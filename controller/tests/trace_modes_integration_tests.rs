//! End-to-end controller integration tests validating `PERF` span emission and `SENSOR` value report emission.

use controller::filesystem_controller::{FilesystemClient, FilesystemController};
use controller::led_controller::LedController;
use controller::motor_controller::{MotorCommand, MotorController};
use controller::sensor_controller::SensorController;
use controller::telemetry_controller::TelemetryController;
use controller::thermal_controller::ThermalController;
use controller::types::SensorMetadata;
use controller::ThermalState;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use model::calibration::{Calibration, FourPointCalibration, MotorCalibration};
use model::interfaces::NoTick;
use model::telemetry::TelemetryRecord;
use model::types::{
    BatteryState, BatteryStatus, Direction, MotorSpeed, SensorReading, SensorTelemetry,
    SystemLedState, ThermalStatus,
};
use peripheral::mock::{
    MockBattery, MockCurrentSensor, MockLed, MockMotor, MockProximitySensor, MockSpeedSensor,
};
use platform::flash::SharedFlashMutex;
use platform::tracing::{
    clear_host_trace_capture, take_host_captured_spans, take_host_captured_telemetry_records,
    HostTracePhase,
};
use platform::types::{MapFilesystem, QueueFilesystem};

struct TraceTestMockFlash {
    data: [u8; 64 * 1024],
}

impl TraceTestMockFlash {
    const fn new() -> Self {
        Self {
            data: [0xFF; 64 * 1024],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for TraceTestMockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for TraceTestMockFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for TraceTestMockFlash {
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

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for TraceTestMockFlash {}

#[test]
fn test_controller_perf_spans_and_sensor_reports_emission() {
    futures::executor::block_on(async {
        clear_host_trace_capture();

        // 1. Exercise non-sensor controllers (MotorController + LedController + TelemetryController)
        let mut motor_ctrl = MotorController::new(
            NoTick::new(MockMotor::new()),
            MockCurrentSensor::new(165),
            MockSpeedSensor::new(4200),
        );
        motor_ctrl.set_calibration(&MotorCalibration {
            current_ma: FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(5000),
            rpm_limit: Some(0),
        });
        motor_ctrl
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        motor_ctrl.tick_motor().await.unwrap();
        motor_ctrl.update(None).await.unwrap();

        let mut led_ctrl = LedController::new(MockLed::new());
        led_ctrl
            .set_pattern(SystemLedState::SolidGreen)
            .await
            .unwrap();

        static FLASH_MUTEX: Mutex<CriticalSectionRawMutex, TraceTestMockFlash> =
            Mutex::new(TraceTestMockFlash::new());
        static FS_CHANNEL: embassy_sync::channel::Channel<
            CriticalSectionRawMutex,
            controller::filesystem_controller::FsRequest,
            16,
        > = embassy_sync::channel::Channel::new();

        let fs_buf = Box::leak(vec![0u8; 8192].into_boxed_slice());
        let _fs = FilesystemController::new(
            SharedFlashMutex::new(&FLASH_MUTEX),
            MapFilesystem(0..32 * 1024),
            fs_buf,
        );
        let mut telemetry = TelemetryController::new(
            SharedFlashMutex::new(&FLASH_MUTEX),
            QueueFilesystem(32 * 1024..64 * 1024),
            FilesystemClient::new(FS_CHANNEL.sender()),
        );
        let battery_rec =
            TelemetryRecord::Battery(BatteryStatus::VolTempState(4015, 245, BatteryState::Ok, 0));
        telemetry.push_record(battery_rec).await.unwrap();
        telemetry
            .push_record(TelemetryRecord::Proximity(SensorTelemetry::Status(
                Direction::West,
                SensorReading::Proximity(210),
            )))
            .await
            .unwrap();

        // 2. Exercise sensor controllers (Proximity North + East, ThermalController)
        let mut north_sensor = SensorController::new(
            SensorMetadata {
                direction: Direction::North,
            },
            MockProximitySensor::new(118),
            300,
        );
        let north_reading = north_sensor.update().await.unwrap();
        assert_eq!(north_reading, SensorReading::Proximity(118));

        let east_mock = MockProximitySensor::new(8190);
        let mut east_sensor = SensorController::new(
            SensorMetadata {
                direction: Direction::East,
            },
            east_mock,
            300,
        );
        let east_reading = east_sensor.update().await.unwrap();
        assert_eq!(east_reading, SensorReading::Invalid);

        static THERMAL_ACTION_CH: embassy_sync::channel::Channel<
            CriticalSectionRawMutex,
            platform::ThermalUpdateAction,
            4,
        > = embassy_sync::channel::Channel::new();
        let mock_temp = Box::leak(Box::new(Mutex::new(MockBattery::new(3950, 25_250))));
        let mut thermal_ctrl =
            ThermalController::new_with_shutdown_and_trap(mock_temp, THERMAL_ACTION_CH.sender());
        thermal_ctrl
            .update::<controller::telemetry_controller::ThermalTelemetryClient<CriticalSectionRawMutex>>(
                None,
            )
            .await
            .unwrap();
        assert_eq!(thermal_ctrl.state(), ThermalState::Normal);

        // 3. Validate emitted spans & telemetry records for the active compile-time trace mode
        let spans = take_host_captured_spans();
        let records = take_host_captured_telemetry_records();

        if cfg!(feature = "trace-perf") && !cfg!(feature = "trace-sensors") {
            // PERF mode: must emit non-sensor controller/driver spans and non-sensor telemetry,
            // while excluding all sensor spans and sensor telemetry reports.
            assert!(
                spans.iter().any(|s| {
                    s.phase == HostTracePhase::Enter && s.name.contains("motor_controller::update")
                }),
                "Expected motor_controller::update span in PERF mode, got: {:?}",
                spans
            );
            assert!(
                spans.iter().any(|s| {
                    s.phase == HostTracePhase::Exit && s.name.contains("motor_controller::update")
                }),
                "Expected motor_controller::update span exit in PERF mode"
            );
            assert!(
                spans
                    .iter()
                    .any(|s| s.name.contains("telemetry_controller::push_record")),
                "Expected telemetry_controller::push_record span in PERF mode"
            );
            assert!(
                !spans.iter().any(|s| s.name.contains("sensor_controller")
                    || s.name.contains("thermal_controller")),
                "PERF mode must exclude sensor_controller and thermal_controller spans, got: {:?}",
                spans
            );
            assert_eq!(
                records.len(),
                1,
                "PERF mode must emit non-sensor Battery record and exclude Proximity record: {:?}",
                records
            );
            assert_eq!(records[0].1, battery_rec);
        } else if cfg!(feature = "trace-sensors") && !cfg!(feature = "trace-perf") {
            // SENSOR mode: must emit Proximity & Thermal reports with measured values,
            // and zero execution span enter/exit events or non-sensor telemetry traces.
            assert!(
                spans.is_empty(),
                "SENSOR mode must emit zero execution spans, got: {:?}",
                spans
            );
            assert_eq!(
                records.len(),
                3,
                "Expected North Proximity, East Invalid Proximity, and Thermal reports in SENSOR mode, got: {:?}",
                records
            );
            assert_eq!(
                records[0].1,
                TelemetryRecord::Proximity(SensorTelemetry::Status(
                    Direction::North,
                    SensorReading::Proximity(118),
                ))
            );
            assert_eq!(
                records[1].1,
                TelemetryRecord::Proximity(SensorTelemetry::Status(
                    Direction::East,
                    SensorReading::Invalid,
                ))
            );
            assert_eq!(
                records[2].1,
                TelemetryRecord::Thermal(ThermalStatus::TempOverheating(25_250, false))
            );
        } else if !cfg!(feature = "trace-perf") && !cfg!(feature = "trace-sensors") {
            // NONE mode: zero trace overhead
            assert!(spans.is_empty());
            assert!(records.is_empty());
        }
    });
}
