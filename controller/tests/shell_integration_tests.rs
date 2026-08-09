#![allow(static_mut_refs)]
use controller::motor_controller::MotorCommand;
controller::declare_shell_commands! {
    CliCommand (CliCommandProcessor) {
        Battery,
        Thermal,
        Motor,
        Sensor,
        Fs,
        System,
    }
}
use controller::shell_controller::{ShellController, ShellControllerPointers};
use controller::system_controller::SystemCommand;
use controller::{
    BlockingBatteryReader, BlockingMotorReader, BlockingMotorWriter, BlockingProximityReader,
    BlockingThermalReader,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_cli::cli::CliBuilder;
use model::types::PeripheralError;

struct TestConfig;
controller::impl_shell_config! {
    TestConfig {
        Flash = MockFlash,
        Motor = MockMotor,
        I2c = DummyI2c,
        TempSensor = MockTempSensor,
        BatteryCtrl = MockBatteryCtrl,
        ThermalCtrl = MockThermalCtrl,
        SensorCtrl = MockSensorCtrl,
        MotorCtrl = MockMotorCtrl,
        SystemCtrl = embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, SystemCommand, 16>,
    }
}

struct DummyWriter {
    output: std::vec::Vec<u8>,
}

impl DummyWriter {
    fn new() -> Self {
        Self {
            output: std::vec::Vec::new(),
        }
    }
}

impl embedded_io::ErrorType for DummyWriter {
    type Error = core::convert::Infallible;
}

impl embedded_io::Write for DummyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.output.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct DummyI2c;

impl embedded_hal::i2c::ErrorType for DummyI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal::i2c::I2c for DummyI2c {
    fn read(&mut self, _address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        read.fill(0);
        Ok(())
    }
    fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        read.fill(0);
        Ok(())
    }
    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct MockMotor;

impl model::interfaces::Motor for MockMotor {
    type Error = core::convert::Infallible;
    fn set_speed(&mut self, _speed: model::types::MotorSpeed) -> Result<(), Self::Error> {
        Ok(())
    }
    fn stop(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

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

impl embedded_storage::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

struct MockBatteryCtrl;
impl BlockingBatteryReader for MockBatteryCtrl {
    fn read_battery_blocking(&self) -> Result<(u32, u8), PeripheralError> {
        Ok((3800, 80))
    }
}

struct MockThermalCtrl;
impl BlockingThermalReader for MockThermalCtrl {
    fn read_temperature_blocking(&self) -> Result<i32, PeripheralError> {
        Ok(25000)
    }
}

struct MockSensorCtrl {
    distance: u16,
}
impl BlockingProximityReader for MockSensorCtrl {
    fn read_distance_blocking(&mut self) -> Result<model::types::SensorReading, PeripheralError> {
        if self.distance == u16::MAX || self.distance >= 8190 {
            Ok(model::types::SensorReading::Invalid)
        } else {
            Ok(model::types::SensorReading::Proximity(self.distance))
        }
    }

    fn latest_distance(&self) -> model::types::SensorReading {
        if self.distance == u16::MAX || self.distance >= 8190 {
            model::types::SensorReading::Invalid
        } else {
            model::types::SensorReading::Proximity(self.distance)
        }
    }

    fn send_command(
        &mut self,
        cmd: controller::sensor_controller::SensorCommand,
    ) -> Result<(), PeripheralError> {
        match cmd {
            controller::sensor_controller::SensorCommand::ReadRawSensorsWithSignal(sig_ptr)
            | controller::sensor_controller::SensorCommand::ReadSensorsWithSignal(sig_ptr) => {
                let sig = unsafe { &*sig_ptr.0 };
                let reading = if self.distance == u16::MAX || self.distance >= 8190 {
                    model::types::SensorReading::Invalid
                } else {
                    model::types::SensorReading::Proximity(self.distance)
                };
                let _ = sig.set(Ok(reading));
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl model::calibration::Calibration for MockSensorCtrl {
    const CALIBRATION_FILE_NAME: &'static str = "vl53l0x_cal.cbor";
    type Store = model::calibration::Vl53l0xCalibration;

    fn get_from_store(
        store: &Self::Store,
        direction: model::types::Direction,
    ) -> Option<model::calibration::CalibrationType> {
        Some(model::calibration::CalibrationType::ProximityCal(
            store.sensors[direction as usize],
            store.xtalk_m_mcps[direction as usize],
        ))
    }

    fn update_store(
        store: &mut Self::Store,
        direction: model::types::Direction,
        calibration: model::calibration::CalibrationType,
    ) {
        if let model::calibration::CalibrationType::ProximityCal(cal, xtalk) = calibration {
            store.sensors[direction as usize] = cal;
            store.xtalk_m_mcps[direction as usize] = xtalk;
        }
    }
}

struct MockMotorCtrl {
    speed: core::cell::Cell<i8>,
}
impl BlockingMotorReader for MockMotorCtrl {
    fn read_current_ma_blocking(&mut self) -> Result<i32, PeripheralError> {
        Ok(120)
    }
}
impl BlockingMotorWriter for MockMotorCtrl {
    fn set_motor_speed(&mut self, speed: i8) -> Result<(), PeripheralError> {
        self.speed.set(speed);
        Ok(())
    }
    fn set_motor_speed_rpm(&mut self, rpm: i32) -> Result<(), PeripheralError> {
        let speed_val = (rpm * 100) / 3000;
        self.set_motor_speed(speed_val.clamp(-100, 100) as i8)
    }
    fn stop_motor_blocking(&mut self) -> Result<(), PeripheralError> {
        self.speed.set(0);
        let _ = MOTOR_CHANNEL.try_send(MotorCommand::Stop);
        Ok(())
    }
}

impl model::calibration::Calibration for MockMotorCtrl {
    const CALIBRATION_FILE_NAME: &'static str = "motor_cal.cbor";
    type Store = model::calibration::MotorCalibration;

    fn get_from_store(
        store: &Self::Store,
        _direction: model::types::Direction,
    ) -> Option<model::calibration::CalibrationType> {
        Some(model::calibration::CalibrationType::MotorCal {
            current_limits: model::calibration::TwoPointCalibration::new(
                store.current_ma.low,
                store.current_ma.overload,
            ),
            max_rpm: store.max_rpm.unwrap_or(0),
            rpm_limit: store.rpm_limit.unwrap_or(0),
        })
    }

    fn update_store(
        store: &mut Self::Store,
        _direction: model::types::Direction,
        calibration: model::calibration::CalibrationType,
    ) {
        if let model::calibration::CalibrationType::MotorCal {
            current_limits,
            max_rpm,
            rpm_limit,
        } = calibration
        {
            store.current_ma.low = current_limits.low;
            store.current_ma.overload = current_limits.high;
            store.max_rpm = Some(max_rpm);
            store.rpm_limit = Some(rpm_limit);
        }
    }
}

struct MockTempSensor;
impl model::interfaces::TemperatureSensor for MockTempSensor {
    type Error = core::convert::Infallible;
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(22000)
    }
}

static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 16> = Channel::new();

#[test]
fn test_shell_controller_integration_each_command() {
    let mut i2c = DummyI2c;
    let mut motor = MockMotor;
    let mut flash = MockFlash::new();
    let mut battery_ctrl = MockBatteryCtrl;
    let mut thermal_ctrl = MockThermalCtrl;
    let mut sensor_north_ctrl = MockSensorCtrl { distance: 100 };
    let mut sensor_east_ctrl = MockSensorCtrl { distance: 200 };
    let mut sensor_west_ctrl = MockSensorCtrl { distance: 300 };
    let mut motor_ctrl = MockMotorCtrl {
        speed: core::cell::Cell::new(0),
    };
    let mut temp_sensor = MockTempSensor;

    let i2c_buses = &[controller::NamedDevice {
        name: "default",
        device: &mut i2c as *mut _,
    }];
    let motors = &[controller::NamedDevice {
        name: "default",
        device: &mut motor as *mut _,
    }];
    let flash_partitions = &[controller::NamedPartition {
        name: "default",
        partition: controller::FlashPartition {
            flash_ptr: &mut flash as *mut _,
            start_address: 0,
            end_address: 1024 * 64,
        },
        kind: controller::PartitionKind::Map,
    }];
    let batteries = &[controller::NamedDevice {
        name: "default",
        device: &mut battery_ctrl as *mut _,
    }];
    let thermals = &[controller::NamedDevice {
        name: "default",
        device: &mut thermal_ctrl as *mut _,
    }];
    let sensors = &[
        controller::NamedDevice {
            name: "north",
            device: &mut sensor_north_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "east",
            device: &mut sensor_east_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "west",
            device: &mut sensor_west_ctrl as *mut _,
        },
    ];
    let motor_ctrls = &[controller::NamedDevice {
        name: "default",
        device: &mut motor_ctrl as *mut _,
    }];
    let temp_sensors = &[controller::NamedDevice {
        name: "default",
        device: &mut temp_sensor as *mut _,
    }];

    let mut system_sender = SYSTEM_CHANNEL.sender();
    let system_ctrls = &[controller::NamedDevice {
        name: "default",
        device: &mut system_sender as *mut _,
    }];

    static mut TEST_FS_BUF_1: [u8; 4096] = [0u8; 4096];
    let pointers = ShellControllerPointers::<TestConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        batteries,
        thermals,
        sensors,
        motor_ctrls,
        temp_sensors,
        system_ctrls,
        leds: &[],
        fs_buffer: unsafe { &mut TEST_FS_BUF_1 },
    };

    let mut shell = ShellController::<TestConfig>::new(pointers);
    let mut shell_proc = CliCommandProcessor::new(&mut shell);

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // Help command first
    for b in b"help\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 1. Motor command
    for b in b"motor speed 42\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }
    assert_eq!(motor_ctrl.speed.get(), 42);

    // 2. Stop command
    for b in b"motor stop\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }
    assert!(matches!(
        MOTOR_CHANNEL.try_receive(),
        Ok(MotorCommand::Stop)
    ));

    // 3. Battery command
    for b in b"battery status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 4. Thermal command
    for b in b"thermal status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 5. Proximity command
    for b in b"sensor status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 8. Activity command
    for b in b"system activity\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }
    assert!(matches!(
        system_chan_check(),
        Ok(SystemCommand::ActivityDetected)
    ));

    // 9. McuTemp command
    for b in b"thermal mcu\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 10. CalNear command
    for b in b"sensor cal_near east\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 11. CalFar command
    for b in b"sensor cal_far west\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 12. CalMotor command
    for b in b"motor calibrate water_100ml\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"fs format\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"fs ls\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"uart\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    // 13. Help command
    for b in b"help\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }
}

fn system_chan_check() -> Result<SystemCommand, embassy_sync::channel::TryReceiveError> {
    SYSTEM_CHANNEL.try_receive()
}

#[test]
fn test_shell_controller_with_missing_controllers() {
    // Using module-level TestConfig

    let pointers = ShellControllerPointers::<TestConfig>::default();

    let mut shell = ShellController::<TestConfig>::new(pointers);
    let mut shell_proc = CliCommandProcessor::new(&mut shell);

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // Verify commands fail gracefully when pointers/controllers are missing
    for b in b"motor speed 42\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"battery status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"thermal status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    for b in b"sensor status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }
}

controller::declare_shell_commands! {
    TestWrapperCli (TestWrapperCliProcessor) {
        Motor,
        Sensor,
        Fs,
        System,
    }
}

#[test]
fn test_wrapper_processor_integration() {
    let mut i2c = DummyI2c;
    let mut motor = MockMotor;
    let mut flash = MockFlash::new();
    let mut battery_ctrl = MockBatteryCtrl;
    let mut thermal_ctrl = MockThermalCtrl;
    let mut sensor_north_ctrl = MockSensorCtrl { distance: 100 };
    let mut sensor_east_ctrl = MockSensorCtrl { distance: 200 };
    let mut sensor_west_ctrl = MockSensorCtrl { distance: 300 };
    let mut motor_ctrl = MockMotorCtrl {
        speed: core::cell::Cell::new(0),
    };
    let mut temp_sensor = MockTempSensor;

    let i2c_buses = &[controller::NamedDevice {
        name: "default",
        device: &mut i2c as *mut _,
    }];
    let motors = &[controller::NamedDevice {
        name: "default",
        device: &mut motor as *mut _,
    }];
    let flash_partitions = &[controller::NamedPartition {
        name: "default",
        partition: controller::FlashPartition {
            flash_ptr: &mut flash as *mut _,
            start_address: 0,
            end_address: 1024 * 64,
        },
        kind: controller::PartitionKind::Map,
    }];
    let batteries = &[controller::NamedDevice {
        name: "default",
        device: &mut battery_ctrl as *mut _,
    }];
    let thermals = &[controller::NamedDevice {
        name: "default",
        device: &mut thermal_ctrl as *mut _,
    }];
    let sensors = &[
        controller::NamedDevice {
            name: "north",
            device: &mut sensor_north_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "east",
            device: &mut sensor_east_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "west",
            device: &mut sensor_west_ctrl as *mut _,
        },
    ];
    let motor_ctrls = &[controller::NamedDevice {
        name: "default",
        device: &mut motor_ctrl as *mut _,
    }];
    let temp_sensors = &[controller::NamedDevice {
        name: "default",
        device: &mut temp_sensor as *mut _,
    }];

    static mut TEST_FS_BUF_2: [u8; 4096] = [0u8; 4096];
    let pointers = ShellControllerPointers::<TestConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        batteries,
        thermals,
        sensors,
        motor_ctrls,
        temp_sensors,
        fs_buffer: unsafe { &mut TEST_FS_BUF_2 },
        ..Default::default()
    };

    let mut shell = ShellController::<TestConfig>::new(pointers);

    let mut wrapper_proc = TestWrapperCliProcessor::new(&mut shell);

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // Send a motor command via the wrapper processor
    for b in b"motor speed 77\n" {
        let _ = cli.process_byte::<TestWrapperCli, _>(*b, &mut wrapper_proc);
    }
    assert_eq!(motor_ctrl.speed.get(), 77);
}

#[test]
fn test_fs_buffer_guard_locking() {
    use controller::shell_controller::ShellDeviceResolver;

    // Test Case 1: Locking configured buffer
    static mut TEST_BUF: [u8; 128] = [0u8; 128];
    let pointers = ShellControllerPointers::<TestConfig> {
        fs_buffer: unsafe { &mut TEST_BUF },
        ..Default::default()
    };
    let shell = ShellController::<TestConfig>::new(pointers);

    // Initial lock should succeed
    {
        let mut guard = shell.lock_fs_buffer().expect("Initial lock failed");
        assert_eq!(guard.len(), 128);

        // Modify buffer through guard DerefMut
        guard[0] = 42;
        guard[1] = 99;
        assert_eq!(guard[0], 42);
        assert_eq!(guard[1], 99);

        // Attempting to lock again while held should fail
        let second_lock = shell.lock_fs_buffer();
        match second_lock {
            Err(e) => assert_eq!(e, "Filesystem scratch buffer is already locked"),
            _ => panic!("Expected second lock to fail"),
        }
    } // Guard dropped here, lock should release

    // Lock after drop should succeed
    {
        let guard = shell.lock_fs_buffer().expect("Lock after release failed");
        assert_eq!(guard[0], 42);
    }

    // Test Case 2: Unconfigured buffer
    let unconfigured_pointers = ShellControllerPointers::<TestConfig>::default();
    let unconfigured_shell = ShellController::<TestConfig>::new(unconfigured_pointers);
    let lock_res = unconfigured_shell.lock_fs_buffer();
    match lock_res {
        Err(e) => assert_eq!(e, "Filesystem scratch buffer is not configured"),
        _ => panic!("Expected lock on unconfigured buffer to fail"),
    }
}

#[derive(Clone)]
struct SharedWriter {
    output: std::sync::Arc<std::sync::Mutex<std::vec::Vec<u8>>>,
}

impl SharedWriter {
    fn new() -> Self {
        Self {
            output: std::sync::Arc::new(std::sync::Mutex::new(std::vec::Vec::new())),
        }
    }
}

impl embedded_io::ErrorType for SharedWriter {
    type Error = core::convert::Infallible;
}

impl embedded_io::Write for SharedWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.output.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_sensor_calibration_bounds_checking() {
    let mut i2c = DummyI2c;
    let mut motor = MockMotor;
    let mut flash = MockFlash::new();
    let mut battery_ctrl = MockBatteryCtrl;
    let mut thermal_ctrl = MockThermalCtrl;
    // Set distance to 950 (which is >= MAX_CALIBRATION_RAW_MM (900))
    let mut sensor_north_ctrl = MockSensorCtrl { distance: 950 };
    let mut sensor_east_ctrl = MockSensorCtrl { distance: 200 };
    let mut sensor_west_ctrl = MockSensorCtrl { distance: 300 };
    let mut motor_ctrl = MockMotorCtrl {
        speed: core::cell::Cell::new(0),
    };
    let mut temp_sensor = MockTempSensor;

    let i2c_buses = &[controller::NamedDevice {
        name: "default",
        device: &mut i2c as *mut _,
    }];
    let motors = &[controller::NamedDevice {
        name: "default",
        device: &mut motor as *mut _,
    }];
    let flash_partitions = &[controller::NamedPartition {
        name: "default",
        partition: controller::FlashPartition {
            flash_ptr: &mut flash as *mut _,
            start_address: 0,
            end_address: 1024 * 64,
        },
        kind: controller::PartitionKind::Map,
    }];
    let batteries = &[controller::NamedDevice {
        name: "default",
        device: &mut battery_ctrl as *mut _,
    }];
    let thermals = &[controller::NamedDevice {
        name: "default",
        device: &mut thermal_ctrl as *mut _,
    }];
    let sensors = &[
        controller::NamedDevice {
            name: "north",
            device: &mut sensor_north_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "east",
            device: &mut sensor_east_ctrl as *mut _,
        },
        controller::NamedDevice {
            name: "west",
            device: &mut sensor_west_ctrl as *mut _,
        },
    ];
    let motor_ctrls = &[controller::NamedDevice {
        name: "default",
        device: &mut motor_ctrl as *mut _,
    }];
    let temp_sensors = &[controller::NamedDevice {
        name: "default",
        device: &mut temp_sensor as *mut _,
    }];

    let mut system_sender = SYSTEM_CHANNEL.sender();
    let system_ctrls = &[controller::NamedDevice {
        name: "default",
        device: &mut system_sender as *mut _,
    }];

    static mut TEST_FS_BUF_2: [u8; 4096] = [0u8; 4096];
    let pointers = ShellControllerPointers::<TestConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        batteries,
        thermals,
        sensors,
        motor_ctrls,
        temp_sensors,
        system_ctrls,
        leds: &[],
        fs_buffer: unsafe { &mut TEST_FS_BUF_2 },
    };

    let mut shell = ShellController::<TestConfig>::new(pointers);
    let mut shell_proc = CliCommandProcessor::new(&mut shell);

    // 1. Test CalNear out of bounds
    let shared_writer = SharedWriter::new();
    let mut cli = CliBuilder::default()
        .writer(shared_writer.clone())
        .build()
        .unwrap();

    for b in b"sensor cal_near north\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    let output_bytes = shared_writer.output.lock().unwrap();
    let output_str = String::from_utf8_lossy(&output_bytes);
    assert!(
        output_str.contains("Target too far") || output_str.contains("Target out of range"),
        "Unexpected output: {}",
        output_str
    );

    // 2. Test CalFar out of bounds
    let shared_writer_far = SharedWriter::new();
    let mut cli_far = CliBuilder::default()
        .writer(shared_writer_far.clone())
        .build()
        .unwrap();

    for b in b"sensor cal_far north\n" {
        let _ = cli_far.process_byte::<CliCommand, _>(*b, &mut shell_proc);
    }

    let output_bytes_far = shared_writer_far.output.lock().unwrap();
    let output_str_far = String::from_utf8_lossy(&output_bytes_far);
    assert!(
        output_str_far.contains("Target too far") || output_str_far.contains("Target out of range"),
        "Unexpected output: {}",
        output_str_far
    );
}
