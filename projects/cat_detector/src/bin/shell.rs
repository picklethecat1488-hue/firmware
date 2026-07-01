//! Standalone interactive hardware bringup serial console shell.
//!
//! Provides a real-time command interface over UART0 for sending one-way commands
//! to controllers (fountain, thermal, power) using the embedded-cli parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]

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
    cat_detector::handle_panic::<
        { cat_detector::FLASH_SIZE },
        { cat_detector::STACK_TOP },
        { cat_detector::FLASH_START },
        { cat_detector::FLASH_END },
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
    let board = cat_detector::Board::init(p);

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

    // Initialize the modular panic handler
    cat_detector::init_panic_handler(
        board.flash,
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
