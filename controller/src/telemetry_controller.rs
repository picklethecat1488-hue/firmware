//! Telemetry storage pipeline and task.

#![deny(missing_docs)]

use crate::filesystem_controller::FilesystemClient;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use model::telemetry::{TelemetryClient, TelemetryRecord};

static TELEMETRY_WRITE_SIGNAL: Signal<CriticalSectionRawMutex, Result<(), ()>> = Signal::new();

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
extern crate std;

/// Returns the current system uptime in microseconds since boot (64-bit precision).
pub fn system_time() -> u64 {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        embassy_time::Instant::now().as_micros()
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        0
    }
}

/// Struct that maintains all of the telemetry state, RRD buffer, and filesystem client reference.
pub struct TelemetryController<
    const MAX_RECORDS: usize = 45,
    const BUFFER_SIZE: usize = { model::telemetry::BUFFER_SIZE },
> {
    file_buf: [u8; BUFFER_SIZE],
    count: u32,
    next_idx: u32,
    time_fn: fn() -> u64,
    fs: FilesystemClient,
    write_pending: bool,
}

/// Type alias for compatibility with the old Telemetry struct name.
pub type Telemetry<
    const MAX_RECORDS: usize = 45,
    const BUFFER_SIZE: usize = { model::telemetry::BUFFER_SIZE },
> = TelemetryController<MAX_RECORDS, BUFFER_SIZE>;

/// Capacity of the telemetry channel queue.
pub const CHANNEL_CAPACITY: usize = 64;

impl Default for TelemetryController<45, { model::telemetry::BUFFER_SIZE }> {
    fn default() -> Self {
        static DUMMY_CHANNEL: embassy_sync::channel::Channel<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            crate::filesystem_controller::FsRequest,
            16,
        > = embassy_sync::channel::Channel::new();
        Self::new(FilesystemClient::new(DUMMY_CHANNEL.sender()), system_time)
    }
}

impl<const MAX_RECORDS: usize, const BUFFER_SIZE: usize>
    TelemetryController<MAX_RECORDS, BUFFER_SIZE>
{
    /// Total size of the telemetry.rrd metadata file (12-byte CBOR header).
    pub const FILE_SIZE: usize = 12;

    /// Interval at which telemetry stats are logged.
    pub const STATS_LOG_INTERVAL: embassy_time::Duration = embassy_time::Duration::from_secs(60);

    /// Interval/timeout for checking/waiting for telemetry updates.
    pub const TELEMETRY_CHECK_INTERVAL: embassy_time::Duration =
        embassy_time::Duration::from_secs(1);

    const _CHECK: () = {
        if BUFFER_SIZE < model::telemetry::CHUNK_FILE_SIZE {
            panic!("Telemetry buffer size is too small for the requested record count");
        }
    };

    /// Creates a new `TelemetryController` instance, initializing the buffer, indices, timestamp function, and filesystem client.
    pub const fn new(fs: FilesystemClient, time_fn: fn() -> u64) -> Self {
        #[allow(path_statements)]
        Self::_CHECK;
        Self {
            file_buf: [0u8; BUFFER_SIZE],
            count: 0,
            next_idx: 0,
            time_fn,
            fs,
            write_pending: false,
        }
    }

    /// Serialize the header indices (count, next_idx) to a 12-byte CBOR payload.
    fn serialize_header(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        let cursor = minicbor::encode::write::Cursor::new(&mut bytes[1..]);
        let mut encoder = minicbor::Encoder::new(cursor);
        if encoder.array(2).is_ok()
            && encoder.u32(self.count).is_ok()
            && encoder.u32(self.next_idx).is_ok()
        {
            let len = encoder.into_writer().position();
            if len <= 11 {
                bytes[0] = len as u8;
            }
        }
        bytes
    }

    /// Deserialize the header indices (count, next_idx) from the 12-byte CBOR payload.
    fn deserialize_header(&mut self) -> bool {
        let len = self.file_buf[0] as usize;
        if len == 0 || len > 11 {
            return false;
        }
        let payload = &self.file_buf[1..1 + len];
        let mut decoder = minicbor::Decoder::new(payload);
        if let Ok(_array_len) = decoder.array() {
            if let Ok(count) = decoder.u32() {
                if let Ok(next_idx) = decoder.u32() {
                    self.count = count;
                    self.next_idx = next_idx;
                    return true;
                }
            }
        }
        false
    }

    /// Flushes any pending asynchronous database update.
    pub async fn flush_pending_write(&mut self) -> Result<(), ()> {
        if self.write_pending {
            self.write_pending = false;
            TELEMETRY_WRITE_SIGNAL.wait().await?;
        }
        Ok(())
    }

    /// Initializes the telemetry buffer from flash storage, or resets it if invalid/missing.
    pub async fn init(&mut self) -> Result<(), ()> {
        let mut temp_buf = [0u8; 12];
        let (len, exists) = match self.fs.read_file("telemetry.rrd", &mut temp_buf).await {
            Ok(Some(bytes)) => {
                let len = bytes.len();
                self.file_buf[..len].copy_from_slice(bytes);
                (len, true)
            }
            _ => (0, false),
        };

        let mut valid = false;
        if exists && len == Self::FILE_SIZE && self.deserialize_header() {
            valid = true;
        }

        if !valid {
            self.count = 0;
            self.next_idx = 0;
            let header = self.serialize_header();
            self.file_buf[0..12].copy_from_slice(&header);
            self.flush_pending_write().await?;
            self.fs
                .start_write_file(
                    "telemetry.rrd",
                    &self.file_buf[..Self::FILE_SIZE],
                    &TELEMETRY_WRITE_SIGNAL,
                )
                .await;
            self.write_pending = true;
            self.flush_pending_write().await?;
        }
        Ok(())
    }

    /// Pushes a telemetry record into the ring buffer and persists it to flash.
    pub async fn push_record(&mut self, record: TelemetryRecord) -> Result<(), ()> {
        let timestamp_us = (self.time_fn)();

        let serialized = record.serialize(timestamp_us);
        let len = serialized[0] as usize;
        if len > 0 && len <= 19 {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::debug!("Writing Telemetry: {=[u8]:cbor}", &serialized[1..1 + len]);
        }

        // Determine which chunk file to write to
        let idx = self.next_idx as usize;
        let chunk_idx = idx / model::telemetry::CHUNK_SIZE;
        let slot_idx = idx % model::telemetry::CHUNK_SIZE;
        let name = model::telemetry::chunk_name(chunk_idx);

        // Read existing chunk data (if any)
        let base_ptr = self.file_buf.as_ptr() as usize;
        self.file_buf.fill(0);
        let mut read_len = 0;
        let mut read_offset = 0;
        if let Ok(Some(bytes)) = self.fs.read_file(name, &mut self.file_buf).await {
            read_len = bytes.len();
            read_offset = bytes.as_ptr() as usize - base_ptr;
        }

        // Copy read bytes to the beginning of self.file_buf once the read reference goes out of scope
        if read_len > 0 && read_offset > 0 {
            self.file_buf
                .copy_within(read_offset..read_offset + read_len, 0);
        }

        // Copy serialized record to chunk slot
        let offset = slot_idx * 20;
        self.file_buf[offset..offset + 20].copy_from_slice(&serialized);

        // Write chunk file
        self.flush_pending_write().await?;
        self.fs
            .start_write_file(
                name,
                &self.file_buf[..model::telemetry::CHUNK_FILE_SIZE],
                &TELEMETRY_WRITE_SIGNAL,
            )
            .await;
        self.write_pending = true;

        // Update metadata and write telemetry.rrd
        self.next_idx = (self.next_idx + 1) % (MAX_RECORDS as u32);
        self.count = core::cmp::min(self.count + 1, MAX_RECORDS as u32);

        let header = self.serialize_header();
        self.flush_pending_write().await?;
        self.file_buf[0..12].copy_from_slice(&header);
        self.fs
            .start_write_file(
                "telemetry.rrd",
                &self.file_buf[..Self::FILE_SIZE],
                &TELEMETRY_WRITE_SIGNAL,
            )
            .await;
        self.write_pending = true;
        self.flush_pending_write().await?;

        Ok(())
    }

    /// Reads all records from the current telemetry state in chronological order.
    pub async fn read_records(&mut self, mut callback: impl FnMut(u64, TelemetryRecord)) -> bool {
        let count = self.count as usize;
        let next_idx = self.next_idx as usize;

        if count > MAX_RECORDS || next_idx > MAX_RECORDS {
            return false;
        }

        let total_iterations = if count < MAX_RECORDS {
            count
        } else {
            MAX_RECORDS
        };
        let mut current_chunk_idx = None;

        for i in 0..total_iterations {
            let idx = if count < MAX_RECORDS {
                i
            } else {
                (next_idx + i) % MAX_RECORDS
            };
            let chunk_idx = idx / model::telemetry::CHUNK_SIZE;
            let slot_idx = idx % model::telemetry::CHUNK_SIZE;

            if current_chunk_idx != Some(chunk_idx) {
                let name = model::telemetry::chunk_name(chunk_idx);
                let base_ptr = self.file_buf.as_ptr() as usize;
                self.file_buf.fill(0);
                let mut read_len = 0;
                let mut read_offset = 0;
                if let Ok(Some(bytes)) = self.fs.read_file(name, &mut self.file_buf).await {
                    read_len = bytes.len();
                    read_offset = bytes.as_ptr() as usize - base_ptr;
                }
                if read_len > 0 && read_offset > 0 {
                    self.file_buf
                        .copy_within(read_offset..read_offset + read_len, 0);
                }
                current_chunk_idx = Some(chunk_idx);
            }

            let offset = slot_idx * 20;
            if offset + 20 <= self.file_buf.len() {
                let slot: &[u8; 20] = self.file_buf[offset..offset + 20].try_into().ok().unwrap();
                if let Some((ts, rec)) = TelemetryRecord::deserialize(slot) {
                    callback(ts, rec);
                }
            }
        }
        true
    }

    /// Starts the controller's main run loop, processing records.
    pub async fn run<const N: usize>(
        &mut self,
        rx: embassy_sync::channel::Receiver<
            'static,
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            TelemetryRecord,
            N,
        >,
    ) -> ! {
        let _ = self.init().await;
        let mut last_print = embassy_time::Instant::now();
        let mut counters = TelemetryCounters::default();

        loop {
            let maybe_record =
                embassy_time::with_timeout(Self::TELEMETRY_CHECK_INTERVAL, rx.receive())
                    .await
                    .ok();

            if let Some(record) = maybe_record {
                counters.record(&record);

                if self.push_record(record).await.is_err() {
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::error!("Telemetry: Failed to push record to flash!");
                    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                    std::eprintln!("Telemetry: Failed to push record to flash!");
                }
            }

            let now = embassy_time::Instant::now();
            if now.duration_since(last_print) >= Self::STATS_LOG_INTERVAL {
                counters.log_stats();
                counters.reset();
                last_print = now;
            }
        }
    }
}

/// Helper structure to track and count processed telemetry records.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelemetryCounters {
    /// Count of updates logged per telemetry category.
    pub counts: [u32; model::telemetry::NUM_TELEMETRY_VARIANTS],
}

impl TelemetryCounters {
    /// Records a new telemetry event and increments the corresponding counter.
    pub fn record(&mut self, record: &TelemetryRecord) {
        let idx = match record {
            TelemetryRecord::Battery(_) => 0,
            TelemetryRecord::Motor(_) => 1,
            TelemetryRecord::Thermal(_) => 2,
            TelemetryRecord::System(_) => 3,
            TelemetryRecord::FuelGauge(_) => 4,
            TelemetryRecord::Proximity(_) => 5,
            TelemetryRecord::Led(_) => 6,
            TelemetryRecord::Gesture(_) => 7,
            TelemetryRecord::FlashTelemetry(_) => 8,
            TelemetryRecord::ChargerState(_) => 9,
            TelemetryRecord::PeripheralError(_) => 10,
            TelemetryRecord::Boot(_) => 11,
        };
        self.counts[idx] += 1;
    }

    /// Computes the total number of telemetry records logged across all categories.
    pub fn total(&self) -> u32 {
        self.counts.iter().sum()
    }

    /// Resets all counters back to zero.
    pub fn reset(&mut self) {
        self.counts.fill(0);
    }

    /// Logs the counters that are greater than zero, showing up to the top 5 counters and the total count.
    pub fn log_stats(&self) {
        let total = self.total();
        if total > 0 {
            let mut active = [(0usize, 0u32); model::telemetry::NUM_TELEMETRY_VARIANTS];
            for (idx, &count) in self.counts.iter().enumerate() {
                active[idx] = (idx, count);
            }
            active.sort_unstable_by_key(|item| core::cmp::Reverse(item.1));

            #[cfg(all(target_arch = "arm", target_os = "none"))]
            {
                let num_active = active.iter().take(5).filter(|item| item.1 > 0).count();
                match num_active {
                    0 => {
                        defmt::info!("Telemetry Stats: Total={}", total);
                    }
                    1 => {
                        defmt::info!(
                            "Telemetry Stats: Total={}, {}: {}",
                            total,
                            TelemetryRecord::name_from_index(active[0].0),
                            active[0].1
                        );
                    }
                    2 => {
                        defmt::info!(
                            "Telemetry Stats: Total={}, {}: {}, {}: {}",
                            total,
                            TelemetryRecord::name_from_index(active[0].0),
                            active[0].1,
                            TelemetryRecord::name_from_index(active[1].0),
                            active[1].1
                        );
                    }
                    3 => {
                        defmt::info!(
                            "Telemetry Stats: Total={}, {}: {}, {}: {}, {}: {}",
                            total,
                            TelemetryRecord::name_from_index(active[0].0),
                            active[0].1,
                            TelemetryRecord::name_from_index(active[1].0),
                            active[1].1,
                            TelemetryRecord::name_from_index(active[2].0),
                            active[2].1
                        );
                    }
                    4 => {
                        defmt::info!(
                            "Telemetry Stats: Total={}, {}: {}, {}: {}, {}: {}, {}: {}",
                            total,
                            TelemetryRecord::name_from_index(active[0].0),
                            active[0].1,
                            TelemetryRecord::name_from_index(active[1].0),
                            active[1].1,
                            TelemetryRecord::name_from_index(active[2].0),
                            active[2].1,
                            TelemetryRecord::name_from_index(active[3].0),
                            active[3].1
                        );
                    }
                    _ => {
                        defmt::info!(
                            "Telemetry Stats: Total={}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}",
                            total,
                            TelemetryRecord::name_from_index(active[0].0),
                            active[0].1,
                            TelemetryRecord::name_from_index(active[1].0),
                            active[1].1,
                            TelemetryRecord::name_from_index(active[2].0),
                            active[2].1,
                            TelemetryRecord::name_from_index(active[3].0),
                            active[3].1,
                            TelemetryRecord::name_from_index(active[4].0),
                            active[4].1
                        );
                    }
                }
            }
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            {
                let mut parts = std::vec::Vec::new();
                for item in active.iter().take(5) {
                    if item.1 > 0 {
                        parts.push(std::format!(
                            "{}={}",
                            TelemetryRecord::name_from_index(item.0),
                            item.1
                        ));
                    }
                }
                std::eprintln!(
                    "Telemetry Stats (1s): Total={}, {}",
                    total,
                    parts.join(", ")
                );
            }
        }
    }
}

/// Telemetry client for thermal status reporting.
pub struct ThermalTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
    last_temp: Option<i32>,
    last_state: Option<crate::thermal_controller::ThermalState>,
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    ThermalTelemetryClient<'a, M, T_CAP>
{
    /// Creates a new `ThermalTelemetryClient`.
    pub fn new(tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>) -> Self {
        Self {
            tx,
            last_temp: None,
            last_state: None,
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<(i32, crate::thermal_controller::ThermalState)>
    for ThermalTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, (temp, state): (i32, crate::thermal_controller::ThermalState)) {
        if let Some(ref tx) = self.tx {
            let send = match (self.last_temp, self.last_state) {
                (Some(last_temp), Some(last_state)) => {
                    (temp - last_temp).abs() >= 1000 || state != last_state
                }
                _ => true,
            };
            if send {
                let overheating = state == crate::thermal_controller::ThermalState::Overheating;
                let status = model::types::ThermalStatus::TempOverheating(temp, overheating);
                let _ = tx.try_send(TelemetryRecord::Thermal(status));
                self.last_temp = Some(temp);
                self.last_state = Some(state);
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!(
                    "Thermal Controller: Temp is {} mC, State: {:?}",
                    temp,
                    state
                );
            }
        }
    }
}

/// Telemetry client for proximity status reporting.
pub struct ProximityTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
    proximity_threshold_mm: u16,
    last_logged_distance: [u16; 3],
    last_logged_in_range: [Option<bool>; 3],
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    ProximityTelemetryClient<'a, M, T_CAP>
{
    /// Creates a new `ProximityTelemetryClient`.
    pub fn new(
        tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
        proximity_threshold_mm: u16,
    ) -> Self {
        Self {
            tx,
            proximity_threshold_mm,
            last_logged_distance: [9999; 3],
            last_logged_in_range: [None; 3],
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<(model::types::Direction, u16)> for ProximityTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, (direction, distance_mm): (model::types::Direction, u16)) {
        if let Some(ref tx) = self.tx {
            let idx = match direction {
                model::types::Direction::North => 0,
                model::types::Direction::East => 1,
                model::types::Direction::West => 2,
            };
            let in_range = distance_mm < self.proximity_threshold_mm;
            let in_range_changed = Some(in_range) != self.last_logged_in_range[idx];
            let distance_changed_significantly =
                (distance_mm as i32 - self.last_logged_distance[idx] as i32).abs() >= 50;

            if in_range_changed || distance_changed_significantly {
                let prox = if in_range {
                    model::types::ProximityTelemetry::InRange(direction, distance_mm)
                } else {
                    model::types::ProximityTelemetry::OutRange(direction, distance_mm)
                };
                let _ = tx.try_send(TelemetryRecord::Proximity(prox));
                self.last_logged_distance[idx] = distance_mm;
                self.last_logged_in_range[idx] = Some(in_range);
            }
        }
    }
}

/// A telemetry client that simply forwards all records to the channel without filtering.
pub struct DefaultTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    T: Clone,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
    map_fn: fn(T) -> TelemetryRecord,
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, T: Clone, const T_CAP: usize>
    DefaultTelemetryClient<'a, M, T, T_CAP>
{
    /// Creates a new `DefaultTelemetryClient`.
    pub fn new(
        tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
        map_fn: fn(T) -> TelemetryRecord,
    ) -> Self {
        Self { tx, map_fn }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, T: Clone, const T_CAP: usize>
    TelemetryClient<T> for DefaultTelemetryClient<'a, M, T, T_CAP>
{
    fn report(&mut self, value: T) {
        if let Some(ref tx) = self.tx {
            let record = (self.map_fn)(value);
            let _ = tx.try_send(record);
        }
    }
}

/// Telemetry client for motor status reporting.
pub struct MotorTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    MotorTelemetryClient<'a, M, T_CAP>
{
    /// Creates a new `MotorTelemetryClient`.
    pub fn new(tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>) -> Self {
        Self { tx }
    }

    /// Reports a peripheral error to telemetry.
    pub fn report_error(&self, err: model::types::PeripheralError) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeripheralError(err));
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::MotorStatus> for MotorTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, status: model::types::MotorStatus) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::Motor(status));
        }
    }
}

/// Telemetry client for battery status reporting.
pub struct BatteryTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    BatteryTelemetryClient<'a, M, T_CAP>
{
    /// Creates a new `BatteryTelemetryClient`.
    pub fn new(tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>) -> Self {
        Self { tx }
    }

    /// Reports a peripheral error to telemetry.
    pub fn report_error(&self, err: model::types::PeripheralError) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeripheralError(err));
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::BatteryStatus> for BatteryTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, status: model::types::BatteryStatus) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::Battery(status));
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::FuelGaugeTelemetry> for BatteryTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, status: model::types::FuelGaugeTelemetry) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::FuelGauge(status));
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::ChargeState> for BatteryTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, status: model::types::ChargeState) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::ChargerState(status));
        }
    }
}

/// Telemetry client for LED status reporting.
pub struct LedTelemetryClient<
    'a,
    M: embassy_sync::blocking_mutex::raw::RawMutex,
    const T_CAP: usize,
> {
    tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>,
    last_state: Option<model::types::SystemLedState>,
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    LedTelemetryClient<'a, M, T_CAP>
{
    /// Creates a new `LedTelemetryClient`.
    pub fn new(tx: Option<embassy_sync::channel::Sender<'a, M, TelemetryRecord, T_CAP>>) -> Self {
        Self {
            tx,
            last_state: None,
        }
    }

    /// Reports a peripheral error to telemetry.
    pub fn report_error(&self, err: model::types::PeripheralError) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeripheralError(err));
        }
    }
}

impl<'a, M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::SystemLedState> for LedTelemetryClient<'a, M, T_CAP>
{
    fn report(&mut self, state: model::types::SystemLedState) {
        if let Some(ref tx) = self.tx {
            let changed = match self.last_state {
                Some(last) => last != state,
                None => true,
            };
            if changed {
                let _ = tx.try_send(TelemetryRecord::Led(state));
                self.last_state = Some(state);
            }
        }
    }
}
