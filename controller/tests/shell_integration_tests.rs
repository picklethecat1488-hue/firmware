use controller::motor_controller::MotorCommand;
use controller::shell_controller::DefaultShellCli as CliCommand;
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
        MutexRaw: CriticalSectionRawMutex,
        Flash = MockFlash,
        Motor = MockMotor,
        I2c = DummyI2c,
        TempSensor = MockTempSensor,
        BatteryCtrl = MockBatteryCtrl,
        ThermalCtrl = MockThermalCtrl,
        SensorCtrl = MockSensorCtrl,
        MotorCtrl = MockMotorCtrl,
        SystemCtrl = embassy_sync::channel::Sender<'static, CriticalSectionRawMutex, SystemCommand, 4>,
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
    fn read_distance_blocking(&mut self) -> Result<u16, PeripheralError> {
        Ok(self.distance)
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
    fn stop(&mut self) -> Result<(), PeripheralError> {
        self.speed.set(0);
        let _ = MOTOR_CHANNEL.try_send(MotorCommand::Stop);
        Ok(())
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
static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();

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
    };

    let mut shell = ShellController::<TestConfig>::new(pointers);

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // Help command first
    for b in b"help\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 1. Motor command
    for b in b"motor speed 42\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert_eq!(motor_ctrl.speed.get(), 42);

    // 2. Stop command
    for b in b"motor stop\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        MOTOR_CHANNEL.try_receive(),
        Ok(MotorCommand::Stop)
    ));

    // 3. Battery command
    for b in b"battery status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 4. Thermal command
    for b in b"thermal status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 5. Proximity command
    for b in b"sensor status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 8. Activity command
    for b in b"system activity\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        system_chan_check(),
        Ok(SystemCommand::ActivityDetected)
    ));

    // 9. McuTemp command
    for b in b"thermal mcu\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 10. CalNear command
    for b in b"sensor cal_near east\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 11. CalFar command
    for b in b"sensor cal_far west\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 12. CalMotor command
    for b in b"motor calibrate water_100ml\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    for b in b"uart\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 13. Help command
    for b in b"help\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
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

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // Verify commands fail gracefully when pointers/controllers are missing
    for b in b"motor speed 42\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    for b in b"battery status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    for b in b"thermal status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    for b in b"sensor status\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
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

    let pointers = ShellControllerPointers::<TestConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        batteries,
        thermals,
        sensors,
        motor_ctrls,
        temp_sensors,
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
