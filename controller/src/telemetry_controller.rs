//! Telemetry storage pipeline and task.

#![deny(missing_docs)]

use crate::filesystem_controller::FilesystemClient;
use crate::tracing::controller_context;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use model::telemetry::{IntoTelemetryRecord, TelemetryClient, TelemetryRecord};

use crate::{TelemetryReceiver, TelemetrySender};

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
extern crate std;

use core::sync::atomic::Ordering;

#[cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
use core::sync::atomic::AtomicU64;

#[cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
/// Global atomic representing mock time during tests.
pub static TEST_MOCK_TIME: AtomicU64 = AtomicU64::new(0);

fn get_timestamp_us() -> u64 {
    #[cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
    {
        TEST_MOCK_TIME.load(Ordering::Relaxed)
    }
    #[cfg(not(any(test, not(all(target_arch = "arm", target_os = "none")))))]
    {
        embassy_time::Instant::now().as_micros()
    }
}

/// Struct that maintains all of the telemetry state, RRD buffer, and filesystem client reference.
#[controller_context]
pub struct TelemetryController<
    const MAX_RECORDS: usize = 45,
    const BUFFER_SIZE: usize = { model::telemetry::BUFFER_SIZE },
    F = (),
> {
    flash: F,
    flash_range: platform::types::QueueFilesystem,
    #[allow(dead_code)]
    fs: FilesystemClient,
}

/// Type alias for compatibility with the old Telemetry struct name.
pub type Telemetry<
    const MAX_RECORDS: usize = 45,
    const BUFFER_SIZE: usize = { model::telemetry::BUFFER_SIZE },
    F = (),
> = TelemetryController<MAX_RECORDS, BUFFER_SIZE, F>;

/// Capacity of the telemetry channel queue.
pub const CHANNEL_CAPACITY: usize = 64;

impl Default for TelemetryController<45, { model::telemetry::BUFFER_SIZE }, ()> {
    fn default() -> Self {
        static DUMMY_CHANNEL: crate::FilesystemChannel<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            16,
        > = crate::FilesystemChannel::new();
        Self::new(
            (),
            platform::types::QueueFilesystem(0..0),
            FilesystemClient::new(DUMMY_CHANNEL.sender()),
        )
    }
}

impl<const MAX_RECORDS: usize, const BUFFER_SIZE: usize, F>
    TelemetryController<MAX_RECORDS, BUFFER_SIZE, F>
{
    /// Creates a new `TelemetryController` instance.
    pub const fn new(
        flash: F,
        flash_range: platform::types::QueueFilesystem,
        fs: FilesystemClient,
    ) -> Self {
        Self {
            flash,
            flash_range,
            fs,
        }
    }
}

impl<
        const MAX_RECORDS: usize,
        const BUFFER_SIZE: usize,
        F: embedded_storage_async::nor_flash::NorFlash,
    > TelemetryController<MAX_RECORDS, BUFFER_SIZE, F>
{
    /// Interval at which telemetry stats are logged.
    pub const STATS_LOG_INTERVAL: embassy_time::Duration = embassy_time::Duration::from_secs(60);

    /// Interval/timeout for checking/waiting for telemetry updates.
    pub const TELEMETRY_CHECK_INTERVAL: embassy_time::Duration =
        embassy_time::Duration::from_secs(1);

    /// Interval at which pending RAM telemetry records are flushed to flash.
    pub const TELEMETRY_FLUSH_INTERVAL: embassy_time::Duration =
        embassy_time::Duration::from_secs(15);

    /// Pushes a telemetry record into the ring buffer and persists it to flash queue.
    #[crate::tracing::instrument(
        name = "telemetry_controller::push_record",
        level = "info",
        skip(record)
    )]
    pub async fn push_record(&mut self, record: TelemetryRecord) -> Result<(), ()> {
        let timestamp_us = get_timestamp_us();

        let serialized = record.serialize(timestamp_us);

        let len = serialized[0] as usize;
        if len == 0 || len >= model::telemetry::TELEMETRY_MAX_SIZE {
            #[cfg(all(target_arch = "arm", target_os = "none", feature = "tracing"))]
            defmt::warn!("Unsupported telemetry record length: {}", len);
            Err(())
        } else {
            platform::tracing::trace_telemetry_record(&record);

            let mut cache = sequential_storage::cache::NoCache::new();
            platform::flash::TELEMETRY_ENABLED.store(false, Ordering::Relaxed);
            let push_res = sequential_storage::queue::push(
                &mut self.flash,
                self.flash_range.0.clone(),
                &mut cache,
                &serialized[..1 + len],
                true, // allow_overwrite_old_data = true
            )
            .await;
            platform::flash::TELEMETRY_ENABLED.store(true, Ordering::Relaxed);

            push_res.map_err(|_| ())
        }
    }

    /// Reads all records from the current telemetry state in chronological order.
    #[crate::tracing::instrument(
        name = "telemetry_controller::read_records",
        level = "info",
        skip(callback)
    )]
    pub async fn read_records(&mut self, mut callback: impl FnMut(u64, TelemetryRecord)) -> bool {
        let mut cache = sequential_storage::cache::NoCache::new();
        match sequential_storage::queue::iter(
            &mut self.flash,
            self.flash_range.0.clone(),
            &mut cache,
        )
        .await
        {
            Ok(mut iterator) => {
                let mut item_buf = [0u8; model::telemetry::TELEMETRY_RECORD_SIZE];
                while let Ok(Some(entry)) = iterator.next(&mut item_buf).await {
                    let bytes = entry.into_buf();
                    if let Some((ts, rec)) = TelemetryRecord::deserialize_from_slice(bytes) {
                        callback(ts, rec);
                    }
                }
                true
            }
            Err(_) => false,
        }
    }

    /// Starts the controller's main run loop, processing records.
    pub async fn run<const N: usize>(
        &mut self,
        rx: TelemetryReceiver<CriticalSectionRawMutex, N>,
    ) -> ! {
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
            TelemetryRecord::PeriodicInterval(_, _) => 12,
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
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
    last_temp: Option<i32>,
    last_state: Option<crate::ThermalState>,
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    ThermalTelemetryClient<M, T_CAP>
{
    /// Creates a new `ThermalTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>) -> Self {
        Self {
            tx,
            last_temp: None,
            last_state: None,
        }
    }

    /// Reports a periodic interval change to telemetry.
    pub fn report_interval(
        &self,
        device: model::types::Device,
        interval: model::types::PeriodicInterval,
    ) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeriodicInterval(device, interval));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<(i32, crate::ThermalState)> for ThermalTelemetryClient<M, T_CAP>
{
    fn report(&mut self, (temp, state): (i32, crate::ThermalState)) {
        if let Some(ref tx) = self.tx {
            let send = match (self.last_temp, self.last_state) {
                (Some(last_temp), Some(last_state)) => {
                    (temp - last_temp).abs() >= 1000 || state != last_state
                }
                _ => true,
            };
            if send {
                let overheating = state == crate::ThermalState::Overheating;
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
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
    wake_threshold_mm: u16,
    last_logged_distance: [u16; 3],
    last_logged_in_range: [Option<bool>; 3],
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    ProximityTelemetryClient<M, T_CAP>
{
    /// Creates a new `ProximityTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>, wake_threshold_mm: u16) -> Self {
        Self {
            tx,
            wake_threshold_mm,
            last_logged_distance: [9999; 3],
            last_logged_in_range: [None; 3],
        }
    }

    /// Reports a periodic interval change to telemetry.
    pub fn report_interval(
        &self,
        device: model::types::Device,
        interval: model::types::PeriodicInterval,
    ) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeriodicInterval(device, interval));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<(model::types::Direction, u16)> for ProximityTelemetryClient<M, T_CAP>
{
    fn report(&mut self, (direction, distance_mm): (model::types::Direction, u16)) {
        if let Some(ref tx) = self.tx {
            let idx = match direction {
                model::types::Direction::North => 0,
                model::types::Direction::East => 1,
                model::types::Direction::West => 2,
            };
            let in_range = distance_mm < self.wake_threshold_mm;
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
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    T: IntoTelemetryRecord + Clone,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
    _phantom: core::marker::PhantomData<T>,
}

impl<
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        T: IntoTelemetryRecord + Clone,
        const T_CAP: usize,
    > DefaultTelemetryClient<M, T, T_CAP>
{
    /// Creates a new `DefaultTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>) -> Self {
        Self {
            tx,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<
        M: embassy_sync::blocking_mutex::raw::RawMutex,
        T: IntoTelemetryRecord + Clone,
        const T_CAP: usize,
    > TelemetryClient<T> for DefaultTelemetryClient<M, T, T_CAP>
{
    fn report(&mut self, value: T) {
        if let Some(ref tx) = self.tx {
            let record = value.into_telemetry_record();
            let _ = tx.try_send(record);
        }
    }
}

/// Telemetry client for motor status reporting.
pub struct MotorTelemetryClient<
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    MotorTelemetryClient<M, T_CAP>
{
    /// Creates a new `MotorTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>) -> Self {
        Self { tx }
    }

    /// Reports a peripheral error to telemetry.
    pub fn report_error(&self, err: model::types::PeripheralError) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeripheralError(err));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::MotorStatus> for MotorTelemetryClient<M, T_CAP>
{
    fn report(&mut self, status: model::types::MotorStatus) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::Motor(status));
        }
    }
}

/// Telemetry client for battery status reporting.
pub struct BatteryTelemetryClient<
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    BatteryTelemetryClient<M, T_CAP>
{
    /// Creates a new `BatteryTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>) -> Self {
        Self { tx }
    }

    /// Reports a peripheral error to telemetry.
    pub fn report_error(&self, err: model::types::PeripheralError) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeripheralError(err));
        }
    }

    /// Reports a periodic interval change to telemetry.
    pub fn report_interval(
        &self,
        device: model::types::Device,
        interval: model::types::PeriodicInterval,
    ) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::PeriodicInterval(device, interval));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::BatteryStatus> for BatteryTelemetryClient<M, T_CAP>
{
    fn report(&mut self, status: model::types::BatteryStatus) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::Battery(status));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::FuelGaugeTelemetry> for BatteryTelemetryClient<M, T_CAP>
{
    fn report(&mut self, status: model::types::FuelGaugeTelemetry) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::FuelGauge(status));
        }
    }
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::ChargeState> for BatteryTelemetryClient<M, T_CAP>
{
    fn report(&mut self, status: model::types::ChargeState) {
        if let Some(ref tx) = self.tx {
            let _ = tx.try_send(TelemetryRecord::ChargerState(status));
        }
    }
}

/// Telemetry client for LED status reporting.
pub struct LedTelemetryClient<
    M: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const T_CAP: usize,
> {
    tx: Option<TelemetrySender<M, T_CAP>>,
    last_state: Option<model::types::SystemLedState>,
}

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    LedTelemetryClient<M, T_CAP>
{
    /// Creates a new `LedTelemetryClient`.
    pub fn new(tx: Option<TelemetrySender<M, T_CAP>>) -> Self {
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

impl<M: embassy_sync::blocking_mutex::raw::RawMutex, const T_CAP: usize>
    TelemetryClient<model::types::SystemLedState> for LedTelemetryClient<M, T_CAP>
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
