use controller::filesystem_controller::{FilesystemClient, FilesystemController};
use controller::telemetry_controller::{TelemetryController, TelemetryCounters};
use model::types::{BatteryState, BatteryStatus, BootReason, TelemetryRecord};
use std::sync::atomic::Ordering;

struct MockFlash {
    data: [u8; 1024 * 64],
}

impl MockFlash {
    fn new() -> Self {
        Self {
            data: [0xFF; 1024 * 64],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 4096;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for MockFlash {}

#[test]
fn test_telemetry_controller_ring_buffer() {
    futures::executor::block_on(async {
        let flash = MockFlash::new();
        let buf = Box::leak(vec![0u8; 4096].into_boxed_slice());
        let fs = FilesystemController::new(flash, 0..1024 * 64, buf);

        static FS_CHANNEL: embassy_sync::channel::Channel<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            controller::filesystem_controller::FsRequest,
            16,
        > = embassy_sync::channel::Channel::new();

        let client = FilesystemClient::new(FS_CHANNEL.sender());
        let mut telemetry =
            TelemetryController::<45, { model::telemetry::BUFFER_SIZE }>::new(client);

        let fs_fut =
            controller::filesystem_controller::run_filesystem_task(fs, FS_CHANNEL.receiver());
        let test_fut = async {
            assert!(telemetry.init().await.is_ok());

            // Push 50 records (max is 45)
            for i in 0..50 {
                controller::telemetry_controller::TEST_MOCK_TIME.store(i as u64, Ordering::Relaxed);
                let record = TelemetryRecord::Battery(BatteryStatus::VolTempState(
                    3000 + i as u32,
                    25,
                    BatteryState::Ok,
                    0,
                ));
                assert!(telemetry.push_record(record).await.is_ok());
            }

            // Read records back in chronological order
            let mut count = 0;
            let mut last_ts = 0;
            let success = telemetry
                .read_records(|ts, record| {
                    if count == 0 {
                        assert_eq!(ts, 5);
                    }
                    assert!(ts >= last_ts);
                    last_ts = ts;

                    match record {
                        TelemetryRecord::Battery(BatteryStatus::VolTempState(
                            vol,
                            temp,
                            state,
                            active_locks,
                        )) => {
                            assert_eq!(vol, 3000 + ts as u32);
                            assert_eq!(temp, 25);
                            assert_eq!(state, BatteryState::Ok);
                            assert_eq!(active_locks, 0);
                        }
                        _ => panic!("Expected Battery status"),
                    }
                    count += 1;
                })
                .await;

            assert!(success);
            assert_eq!(count, 45);
        };

        futures::pin_mut!(fs_fut);
        futures::pin_mut!(test_fut);

        futures::future::select(test_fut, fs_fut).await;
    });
}

#[test]
fn test_telemetry_controller_chunked_boundary() {
    futures::executor::block_on(async {
        let flash = MockFlash::new();
        let buf = Box::leak(vec![0u8; 4096].into_boxed_slice());
        let fs = FilesystemController::new(flash, 0..1024 * 64, buf);

        static FS_CHANNEL: embassy_sync::channel::Channel<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            controller::filesystem_controller::FsRequest,
            16,
        > = embassy_sync::channel::Channel::new();

        let client = FilesystemClient::new(FS_CHANNEL.sender());
        // Max records 200 spanning two chunks (chunk 0: index 0..128, chunk 1: index 128..200)
        let mut telemetry =
            TelemetryController::<200, { model::telemetry::BUFFER_SIZE }>::new(client);

        let fs_fut =
            controller::filesystem_controller::run_filesystem_task(fs, FS_CHANNEL.receiver());
        let test_fut = async {
            assert!(telemetry.init().await.is_ok());

            // Push 220 records (capacity 200)
            for i in 0..220 {
                controller::telemetry_controller::TEST_MOCK_TIME.store(i as u64, Ordering::Relaxed);
                let record = TelemetryRecord::Battery(BatteryStatus::VolTempState(
                    4000 + i as u32,
                    30,
                    BatteryState::Ok,
                    0,
                ));
                assert!(telemetry.push_record(record).await.is_ok());
            }

            // Read records back and verify
            let mut count = 0;
            let mut last_ts = 0;
            let success = telemetry
                .read_records(|ts, record| {
                    if count == 0 {
                        // The oldest record should be at index 20 after ring buffer wrapping
                        assert_eq!(ts, 20);
                    }
                    assert!(ts >= last_ts);
                    last_ts = ts;

                    match record {
                        TelemetryRecord::Battery(BatteryStatus::VolTempState(
                            vol,
                            temp,
                            state,
                            active_locks,
                        )) => {
                            assert_eq!(vol, 4000 + ts as u32);
                            assert_eq!(temp, 30);
                            assert_eq!(state, BatteryState::Ok);
                            assert_eq!(active_locks, 0);
                        }
                        _ => panic!("Expected Battery status"),
                    }
                    count += 1;
                })
                .await;

            assert!(success);
            assert_eq!(count, 200);
        };

        futures::pin_mut!(fs_fut);
        futures::pin_mut!(test_fut);

        futures::future::select(test_fut, fs_fut).await;
    });
}

#[test]
fn test_telemetry_counters() {
    let mut counters = TelemetryCounters::default();
    assert_eq!(counters.total(), 0);

    counters.record(&TelemetryRecord::Boot(BootReason::PowerOn));
    counters.record(&TelemetryRecord::Battery(BatteryStatus::VolTempState(
        3000,
        25,
        BatteryState::Ok,
        0,
    )));
    counters.record(&TelemetryRecord::Battery(BatteryStatus::VolTempState(
        3100,
        25,
        BatteryState::Ok,
        0,
    )));

    assert_eq!(counters.total(), 3);
    assert_eq!(counters.counts[11], 1); // Boot
    assert_eq!(counters.counts[0], 2); // Battery
    assert_eq!(counters.counts[1], 0); // Motor

    counters.reset();
    assert_eq!(counters.total(), 0);
    assert_eq!(counters.counts[11], 0);
    assert_eq!(counters.counts[0], 0);
}
