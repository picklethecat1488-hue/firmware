use app::shell_controller::ShellController;
use app::system_controller::SystemCommand;
use app::CliCommand;
use cat_detector as app;
use controller::motor_controller::MotorCommand;
use controller::{
    BlockingBatteryReader, BlockingMotorReader, BlockingProximityReader, BlockingThermalReader,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_cli::cli::CliBuilder;

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
    fn set_speed(&mut self, _speed: u8) -> Result<(), Self::Error> {
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
    fn read_battery_blocking(&self) -> Option<(u32, u8)> {
        Some((3800, 80))
    }
}

struct MockThermalCtrl;
impl BlockingThermalReader for MockThermalCtrl {
    fn read_temperature_blocking(&self) -> Option<i32> {
        Some(25000)
    }
}

struct MockSensorCtrl {
    distance: u16,
}
impl BlockingProximityReader for MockSensorCtrl {
    fn read_distance_blocking(&mut self) -> Option<u16> {
        Some(self.distance)
    }
}

struct MockMotorCtrl;
impl BlockingMotorReader for MockMotorCtrl {
    fn read_current_ma_blocking(&mut self) -> Option<i32> {
        Some(120)
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
    let mut motor_ctrl = MockMotorCtrl;
    let mut temp_sensor = MockTempSensor;

    let mut shell = ShellController::<
        _,
        4,
        _,
        _,
        _,
        MockBatteryCtrl,
        MockThermalCtrl,
        MockSensorCtrl,
        MockMotorCtrl,
        MockTempSensor,
    >::new(
        MOTOR_CHANNEL.sender(),
        SYSTEM_CHANNEL.sender(),
        Some(&mut i2c as *mut _),
        Some(&mut motor as *mut _),
        Some(&mut flash as *mut _),
        Some(&mut battery_ctrl as *mut _),
        Some(&mut thermal_ctrl as *mut _),
        Some(&mut sensor_north_ctrl as *mut _),
        Some(&mut sensor_east_ctrl as *mut _),
        Some(&mut sensor_west_ctrl as *mut _),
        Some(&mut motor_ctrl as *mut _),
        Some(&mut temp_sensor as *mut _),
        0,
        1024 * 64,
    );

    let writer = DummyWriter::new();
    let mut cli = CliBuilder::default().writer(writer).build().unwrap();

    // 1. Motor command
    for b in b"motor 42\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        MOTOR_CHANNEL.try_receive(),
        Ok(MotorCommand::SetSpeed(42))
    ));

    // 2. Stop command
    for b in b"stop\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        MOTOR_CHANNEL.try_receive(),
        Ok(MotorCommand::Stop)
    ));

    // 3. Battery command
    for b in b"battery\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 4. Thermal command
    for b in b"thermal\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 5. Proximity command
    for b in b"proximity\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 6. Wake command
    for b in b"wake\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        SYSTEM_CHANNEL.try_receive(),
        Ok(SystemCommand::Wake)
    ));

    // 7. Sleep command
    for b in b"sleep\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(system_chan_check(), Ok(SystemCommand::Sleep)));

    // 8. Activity command
    for b in b"activity\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }
    assert!(matches!(
        system_chan_check(),
        Ok(SystemCommand::ActivityDetected)
    ));

    // 9. McuTemp command
    for b in b"mcu_temp\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 10. CalNear command
    for b in b"cal_near east\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 11. CalFar command
    for b in b"cal_far west\n" {
        let _ = cli.process_byte::<CliCommand, _>(*b, &mut shell);
    }

    // 12. CalMotor command
    for b in b"cal_motor water_100ml\n" {
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
