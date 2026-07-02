//! Standalone interactive hardware bringup serial console shell.
//!
//! Provides a real-time command interface over UART0 for sending one-way commands
//! to controllers (fountain, thermal, power) using the embedded-cli parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_rp::uart::UartTx,
    embedded_cli::cli::{CliBuilder, CliHandle},
    embedded_cli::command::RawCommand,
    embedded_cli::service::{CommandProcessor, FromRaw},
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cat_detector::handle_panic_with_sizes::<
        { cat_detector::FLASH_SIZE },
        { cat_detector::STACK_TOP },
        { cat_detector::FLASH_START },
        { cat_detector::FLASH_END },
        { cat_detector::FLASH_WRITE_SIZE },
        { cat_detector::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::fmt::Write as FmtWrite;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embedded_io::Write as IoWrite;

/// Helper struct to write formatted strings directly to UART.
#[cfg(all(target_arch = "arm", target_os = "none"))]
struct UartWriter<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> {
    uart: UartTx<'d, T, M>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> embedded_io::ErrorType
    for UartWriter<'d, T, M>
{
    type Error = core::convert::Infallible;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> IoWrite
    for UartWriter<'d, T, M>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, core::convert::Infallible> {
        let _ = self.uart.blocking_write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cat_detector::CliCommand;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_I2C: Option<
    *mut embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_MOTOR: Option<
    *mut peripherals::motor::GpioMotor<embassy_rp::gpio::Flex<'static>>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut PANIC_FLASH: Option<
    embassy_rp::flash::Flash<
        'static,
        embassy_rp::peripherals::FLASH,
        embassy_rp::flash::Blocking,
        { cat_detector::FLASH_SIZE },
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct CliProcessor;

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<W: IoWrite<Error = E>, E: embedded_io::Error> CommandProcessor<W, E> for CliProcessor {
    fn process<'a>(
        &mut self,
        cli: &mut CliHandle<'_, W, E>,
        raw: RawCommand<'a>,
    ) -> Result<(), embedded_cli::service::ProcessError<'a, E>> {
        let writer = cli.writer();
        match CliCommand::parse(raw) {
            Ok(CliCommand::Motor { speed }) => {
                let _ = cat_detector::MOTOR_CHANNEL
                    .try_send(controller::motor_controller::MotorCommand::SetSpeed(speed));
                let _ = core::writeln!(
                    writer,
                    "\r\nSent MotorCommand::SetSpeed({}) to controller",
                    speed
                );
            }
            Ok(CliCommand::Stop) => {
                let _ = cat_detector::MOTOR_CHANNEL
                    .try_send(controller::motor_controller::MotorCommand::Stop);
                let _ = core::writeln!(writer, "\r\nSent MotorCommand::Stop to controller");
            }
            Ok(CliCommand::Battery) => {
                let _ = cat_detector::BATTERY_CHANNEL
                    .try_send(controller::battery_controller::BatteryCommand::CheckStatus);
                let _ =
                    core::writeln!(writer, "\r\nSent BatteryCommand::CheckStatus to controller");
            }
            Ok(CliCommand::Thermal) => {
                let _ = cat_detector::THERMAL_CHANNEL
                    .try_send(controller::thermal_controller::ThermalCommand::CheckTemp);
                let _ = core::writeln!(writer, "\r\nSent ThermalCommand::CheckTemp to controller");
            }
            Ok(CliCommand::Proximity) => {
                let _ = cat_detector::SENSOR_NORTH_CHANNEL
                    .try_send(controller::sensor_controller::SensorCommand::ReadSensors);
                let _ = cat_detector::SENSOR_EAST_CHANNEL
                    .try_send(controller::sensor_controller::SensorCommand::ReadSensors);
                let _ = cat_detector::SENSOR_WEST_CHANNEL
                    .try_send(controller::sensor_controller::SensorCommand::ReadSensors);
                let _ = core::writeln!(
                    writer,
                    "\r\nSent SensorCommand::ReadSensors to all three sensor controllers"
                );
            }
            Ok(CliCommand::Wake) => {
                let _ = cat_detector::SYSTEM_CHANNEL
                    .try_send(cat_detector::system_controller::SystemCommand::Wake);
                let _ = core::writeln!(writer, "\r\nSent SystemCommand::Wake to controller");
            }
            Ok(CliCommand::Sleep) => {
                let _ = cat_detector::SYSTEM_CHANNEL
                    .try_send(cat_detector::system_controller::SystemCommand::Sleep);
                let _ = core::writeln!(writer, "\r\nSent SystemCommand::Sleep to controller");
            }
            Ok(CliCommand::Activity) => {
                let _ = cat_detector::SYSTEM_CHANNEL
                    .try_send(cat_detector::system_controller::SystemCommand::ActivityDetected);
                let _ = core::writeln!(
                    writer,
                    "\r\nSent SystemCommand::ActivityDetected to controller"
                );
            }
            Ok(CliCommand::Crash) => {
                panic!("Simulated crash dump flow");
            }
            Ok(CliCommand::CalNear { direction }) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                {
                    let i2c = unsafe { &mut *BOARD_I2C.unwrap() };
                    let (addr, name) = match direction {
                        cat_detector::SensorDirection::North => (0x30, "North"),
                        cat_detector::SensorDirection::East => (0x31, "East"),
                        cat_detector::SensorDirection::West => (0x32, "West"),
                    };

                    let d_raw = {
                        let mut sensor = peripherals::vl53l0x::Vl53l0x::new(&mut *i2c, addr);
                        use model::interfaces::ProximitySensor;
                        sensor.read_distance_mm().unwrap_or(1000)
                    };

                    let _ = core::writeln!(
                        writer,
                        "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                        name,
                        d_raw
                    );

                    let flash_ref = unsafe { PANIC_FLASH.as_mut().unwrap() };
                    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                    let mut fs = controller::filesystem_controller::FilesystemController::new(
                        async_flash,
                        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
                    );

                    let mut buf = [0u8; 128];
                    let mut proximity_cal =
                        match embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                        {
                            Ok(Some(bytes)) => {
                                minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes)
                                    .unwrap_or_default()
                            }
                            _ => model::calibration::Vl53l0xCalibration::default(),
                        };

                    match direction {
                        cat_detector::SensorDirection::North => proximity_cal.north_near = d_raw,
                        cat_detector::SensorDirection::East => proximity_cal.east_near = d_raw,
                        cat_detector::SensorDirection::West => proximity_cal.west_near = d_raw,
                    }

                    let mut write_buf = [0u8; 128];
                    let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                    let mut encoder = minicbor::Encoder::new(cursor);
                    encoder.encode(proximity_cal).unwrap();
                    let len = encoder.into_writer().position();

                    match embassy_futures::block_on(
                        fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                    ) {
                        Ok(_) => {
                            let _ = core::writeln!(
                                writer,
                                "Saved cover calibration for {} to flash.",
                                name
                            );
                        }
                        Err(_) => {
                            let _ = core::writeln!(writer, "Error saving calibration to flash.");
                        }
                    }
                }
            }
            Ok(CliCommand::CalFar { direction }) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                {
                    let i2c = unsafe { &mut *BOARD_I2C.unwrap() };
                    let (addr, name) = match direction {
                        cat_detector::SensorDirection::North => (0x30, "North"),
                        cat_detector::SensorDirection::East => (0x31, "East"),
                        cat_detector::SensorDirection::West => (0x32, "West"),
                    };

                    let d_raw = {
                        let mut sensor = peripherals::vl53l0x::Vl53l0x::new(&mut *i2c, addr);
                        use model::interfaces::ProximitySensor;
                        sensor.read_distance_mm().unwrap_or(1000)
                    };

                    let _ = core::writeln!(
                        writer,
                        "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                        name,
                        d_raw
                    );

                    let flash_ref = unsafe { PANIC_FLASH.as_mut().unwrap() };
                    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                    let mut fs = controller::filesystem_controller::FilesystemController::new(
                        async_flash,
                        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
                    );

                    let mut buf = [0u8; 128];
                    let mut proximity_cal =
                        match embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                        {
                            Ok(Some(bytes)) => {
                                minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes)
                                    .unwrap_or_default()
                            }
                            _ => model::calibration::Vl53l0xCalibration::default(),
                        };

                    match direction {
                        cat_detector::SensorDirection::North => proximity_cal.north_100 = d_raw,
                        cat_detector::SensorDirection::East => proximity_cal.east_100 = d_raw,
                        cat_detector::SensorDirection::West => proximity_cal.west_100 = d_raw,
                    }

                    let mut write_buf = [0u8; 128];
                    let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                    let mut encoder = minicbor::Encoder::new(cursor);
                    encoder.encode(proximity_cal).unwrap();
                    let len = encoder.into_writer().position();

                    match embassy_futures::block_on(
                        fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                    ) {
                        Ok(_) => {
                            let _ = core::writeln!(
                                writer,
                                "Saved 100mm calibration for {} to flash.",
                                name
                            );
                        }
                        Err(_) => {
                            let _ = core::writeln!(writer, "Error saving calibration to flash.");
                        }
                    }
                }
            }
            Ok(CliCommand::CalMotor { state }) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                {
                    let motor = unsafe { &mut *BOARD_MOTOR.unwrap() };
                    let _ = core::writeln!(writer, "\r\nStarting motor for calibration...");
                    use model::interfaces::Motor;
                    let _ = motor.set_speed(100);

                    let _ = core::writeln!(writer, "Waiting 1 second for motor to ramp up...");
                    embassy_futures::block_on(embassy_time::Timer::after(
                        embassy_time::Duration::from_millis(1000),
                    ));

                    let i2c = unsafe { &mut *BOARD_I2C.unwrap() };
                    let mut current_sensor = peripherals::ina219::Ina219::new(&mut *i2c);
                    if let Err(e) = current_sensor.init() {
                        let _ =
                            core::writeln!(writer, "Warning: Failed to initialize INA219: {:?}", e);
                    }

                    let mut sum = 0;
                    for _ in 0..5 {
                        use model::interfaces::PowerSensor;
                        sum += current_sensor.read_current_ma().unwrap_or(0);
                        embassy_futures::block_on(embassy_time::Timer::after(
                            embassy_time::Duration::from_millis(100),
                        ));
                    }
                    let current = sum / 5;

                    let name = match state {
                        cat_detector::MotorCalState::Empty => "Empty",
                        cat_detector::MotorCalState::Water100ml => "100ml",
                        cat_detector::MotorCalState::Full => "Full",
                    };

                    let _ = core::writeln!(
                        writer,
                        "Stopping motor and recording measured current for {} state: {} mA",
                        name,
                        current
                    );
                    let _ = motor.stop();

                    let flash_ref = unsafe { PANIC_FLASH.as_mut().unwrap() };
                    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                    let mut fs = controller::filesystem_controller::FilesystemController::new(
                        async_flash,
                        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
                    );

                    let mut buf = [0u8; 128];
                    let mut cal =
                        match embassy_futures::block_on(fs.read_file("motor_cal.cbor", &mut buf)) {
                            Ok(Some(bytes)) => {
                                minicbor::decode::<model::calibration::MotorCalibration>(bytes)
                                    .unwrap_or_default()
                            }
                            _ => model::calibration::MotorCalibration::default(),
                        };

                    match state {
                        cat_detector::MotorCalState::Empty => cal.empty_current_ma = current,
                        cat_detector::MotorCalState::Water100ml => {
                            cal.water_100ml_current_ma = current
                        }
                        cat_detector::MotorCalState::Full => cal.full_current_ma = current,
                    }

                    let mut write_buf = [0u8; 128];
                    let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                    let mut encoder = minicbor::Encoder::new(cursor);
                    encoder.encode(cal).unwrap();
                    let len = encoder.into_writer().position();

                    match embassy_futures::block_on(
                        fs.write_file("motor_cal.cbor", &write_buf[..len]),
                    ) {
                        Ok(_) => {
                            let _ = core::writeln!(
                                writer,
                                "Saved motor {} calibration to flash.",
                                name
                            );
                        }
                        Err(_) => {
                            let _ = core::writeln!(writer, "Error saving calibration to flash.");
                        }
                    }
                }
            }
            Ok(CliCommand::Help) => {
                let _ = core::writeln!(writer, "\r\nCommands:");
                let _ = core::writeln!(writer, "  motor <speed>    : Set motor speed (0-100)");
                let _ = core::writeln!(writer, "  stop             : Stop the motor");
                let _ = core::writeln!(writer, "  battery          : Trigger battery status check");
                let _ = core::writeln!(writer, "  thermal          : Trigger thermal temp check");
                let _ = core::writeln!(
                    writer,
                    "  proximity        : Trigger proximity sensors check"
                );
                let _ = core::writeln!(writer, "  wake             : Wake system to active state");
                let _ = core::writeln!(writer, "  sleep            : Force system to sleep state");
                let _ = core::writeln!(
                    writer,
                    "  activity         : Simulate user/cat activity event"
                );
                let _ = core::writeln!(
                    writer,
                    "  crash            : Trigger a panic to test crash dump"
                );
                let _ = core::writeln!(
                    writer,
                    "  cal_near <north|east|west> : Calibrate sensor cover (0mm)"
                );
                let _ = core::writeln!(
                    writer,
                    "  cal_far <north|east|west>  : Calibrate sensor 100mm target"
                );
                let _ = core::writeln!(
                    writer,
                    "  cal_motor <empty|100ml|full> : Calibrate motor current levels"
                );
                let _ = core::writeln!(writer, "  help             : Show this help summary");
            }
            Err(e) => {
                let _ = core::writeln!(writer, "\r\nError parsing command: {:?}", e);
            }
        }
        Ok(())
    }
}

/// Main application entry point for the bringup shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    let p = embassy_rp::init(Default::default());

    // Initialize board peripherals using the unified board configuration
    let mut board = cat_detector::Board::init(p);

    // Extract the motor control pin from the board configuration array
    let motor_pin = board.gpio_pins[cat_detector::LED_PIN as usize]
        .take()
        .expect("Motor pin must be available");

    let mut motor = peripherals::motor::GpioMotor::new(motor_pin);

    unsafe {
        BOARD_I2C = Some(&mut board.i2c as *mut _ as *mut _);
        BOARD_MOTOR = Some(&mut motor as *mut _);
    }

    // Split the UART into TX and RX parts to satisfy the borrow checker
    let (tx, mut rx) = board.uart.split();
    let writer = UartWriter { uart: tx };

    let mut cli = CliBuilder::default()
        .writer(writer)
        .prompt("\r\nshell> ")
        .build()
        .map_err(|_| ())
        .unwrap();

    // Print welcome text using the CLI's internal writer
    let banner = r#"
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
"#;
    let _ = cli.write(|writer| {
        let _ = core::writeln!(writer, "{}", banner);
        let _ = core::writeln!(writer, "Type 'help' to print usage.");
        Ok(())
    });

    // Register system time function for the panic handler
    cat_detector::set_time_fn(cat_detector::system_time);

    // Initialize the modular panic handler
    let panic_flash = unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(board.flash));
        PANIC_FLASH.as_mut().unwrap()
    };
    cat_detector::init_panic_handler(
        panic_flash,
        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
    );

    let mut processor = CliProcessor;

    // Run the main input loop feeding bytes to the embedded-cli processor
    loop {
        let mut rx_byte = [0u8; 1];
        if rx.blocking_read(&mut rx_byte).is_ok() {
            let _ = cli.process_byte::<CliCommand, _>(rx_byte[0], &mut processor);
        }
    }
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
