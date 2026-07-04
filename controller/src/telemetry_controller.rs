//! Telemetry storage pipeline and task.

#![deny(missing_docs)]

use crate::filesystem_controller::FilesystemClient;
use model::telemetry::TelemetryRecord;

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
pub struct TelemetryController<const MAX_RECORDS: usize = 45, const BUFFER_SIZE: usize = 1024> {
    file_buf: [u8; BUFFER_SIZE],
    count: u32,
    next_idx: u32,
    time_fn: fn() -> u64,
    fs: FilesystemClient,
}

/// Type alias for compatibility with the old Telemetry struct name.
pub type Telemetry<const MAX_RECORDS: usize = 45, const BUFFER_SIZE: usize = 1024> =
    TelemetryController<MAX_RECORDS, BUFFER_SIZE>;

impl Default for TelemetryController<45, 1024> {
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
    /// Total size of the RRD file including the 12-byte CBOR header.
    pub const FILE_SIZE: usize = 12 + MAX_RECORDS * 20;

    const _CHECK: () = {
        if BUFFER_SIZE < 12 + MAX_RECORDS * 20 {
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

    /// Initializes the telemetry buffer from flash storage, or resets it if invalid/missing.
    pub async fn init(&mut self) -> Result<(), ()> {
        let base_ptr = self.file_buf.as_ptr() as usize;
        let (start, len) = match self.fs.read_file("telemetry.rrd", &mut self.file_buf).await {
            Ok(Some(bytes)) => {
                let start = bytes.as_ptr() as usize - base_ptr;
                (Some(start), bytes.len())
            }
            _ => (None, 0),
        };

        let exists = if let Some(start) = start {
            if start > 0 {
                self.file_buf.copy_within(start..start + len, 0);
            }
            len == Self::FILE_SIZE
        } else {
            false
        };

        if exists {
            if !self.deserialize_header() {
                self.count = 0;
                self.next_idx = 0;
            }
        } else {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!("Telemetry: telemetry.rrd not found or invalid, initializing...");
            self.file_buf.fill(0);
            self.count = 0;
            self.next_idx = 0;
            let header = self.serialize_header();
            self.file_buf[0..12].copy_from_slice(&header);
            self.fs
                .write_file("telemetry.rrd", &self.file_buf[..Self::FILE_SIZE])
                .await
                .map_err(|_| ())?;
        }
        Ok(())
    }

    /// Pushes a telemetry record into the ring buffer and persists it to flash.
    pub async fn push_record(&mut self, record: TelemetryRecord) -> Result<(), ()> {
        let timestamp_us = (self.time_fn)();

        let base_ptr = self.file_buf.as_ptr() as usize;
        let (start, len) = match self.fs.read_file("telemetry.rrd", &mut self.file_buf).await {
            Ok(Some(bytes)) => {
                let start = bytes.as_ptr() as usize - base_ptr;
                (Some(start), bytes.len())
            }
            _ => (None, 0),
        };

        if let Some(start) = start {
            if start > 0 {
                self.file_buf.copy_within(start..start + len, 0);
            }
            let _ = self.deserialize_header();

            let serialized = record.serialize(timestamp_us);
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!("Writing Telemetry: {=[u8]:cbor}", serialized);

            let offset = 12 + (self.next_idx as usize) * 20;
            if offset + 20 <= self.file_buf.len() {
                self.file_buf[offset..offset + 20].copy_from_slice(&serialized);

                self.next_idx = (self.next_idx + 1) % (MAX_RECORDS as u32);
                self.count = core::cmp::min(self.count + 1, MAX_RECORDS as u32);

                let header = self.serialize_header();
                self.file_buf[0..12].copy_from_slice(&header);

                self.fs
                    .write_file("telemetry.rrd", &self.file_buf[..Self::FILE_SIZE])
                    .await
                    .map_err(|_| ())?;
            }
        }
        Ok(())
    }

    /// Reads all records from the current telemetry state in chronological order.
    pub fn read_records(&self, mut callback: impl FnMut(u64, TelemetryRecord)) -> bool {
        let count = self.count as usize;
        let next_idx = self.next_idx as usize;

        if count > MAX_RECORDS || next_idx > MAX_RECORDS {
            return false;
        }

        if count < MAX_RECORDS {
            for i in 0..count {
                let offset = 12 + i * 20;
                let slot: &[u8; 20] = self.file_buf[offset..offset + 20].try_into().ok().unwrap();
                if let Some((ts, rec)) = TelemetryRecord::deserialize(slot) {
                    callback(ts, rec);
                }
            }
        } else {
            for i in 0..MAX_RECORDS {
                let idx = (next_idx + i) % MAX_RECORDS;
                let offset = 12 + idx * 20;
                let slot: &[u8; 20] = self.file_buf[offset..offset + 20].try_into().ok().unwrap();
                if let Some((ts, rec)) = TelemetryRecord::deserialize(slot) {
                    callback(ts, rec);
                }
            }
        }
        true
    }

    /// Starts the controller's main run loop, processing records.
    pub async fn run(
        mut self,
        rx: embassy_sync::channel::Receiver<
            'static,
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            TelemetryRecord,
            16,
        >,
    ) -> ! {
        let _ = self.init().await;
        loop {
            let record = rx.receive().await;
            let _ = self.push_record(record).await;
        }
    }
}
