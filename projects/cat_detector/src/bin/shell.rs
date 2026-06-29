//! Standalone interactive hardware bringup serial console shell.
//!
//! Provides a real-time command interface over UART0 for sending one-way commands
//! to controllers (fountain, thermal, power) using the embedded-cli parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod target {
    use embassy_executor::Spawner;
    use embassy_rp::uart::UartTx;
    use embedded_cli::cli::{CliBuilder, CliHandle};
    use embedded_cli::command::RawCommand;
    use embedded_cli::service::{CommandProcessor, FromRaw};
    use embedded_cli::Command;
    use {defmt_rtt as _, panic_probe as _};

    use core::fmt::Write as FmtWrite;
    use embedded_io::Write as IoWrite;

    /// Helper struct to write formatted strings directly to UART.
    struct UartWriter<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> {
        uart: UartTx<'d, T, M>,
    }

    impl<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> embedded_io::ErrorType
        for UartWriter<'d, T, M>
    {
        type Error = core::convert::Infallible;
    }

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

    /// Derived command enum representing all supported user commands.
    #[derive(Command)]
    enum CliCommand {
        /// Fountain pump speed control (fountain <speed>)
        Fountain {
            /// Speed value (0-100)
            speed: u8,
        },
        /// Stop the fountain pump
        Stop,
        /// Query battery voltage and status
        Battery,
        /// Query thermal sensor and status
        Thermal,
        /// Show help and usage summary
        Help,
    }

    struct CliProcessor;

    impl<W: IoWrite<Error = E>, E: embedded_io::Error> CommandProcessor<W, E> for CliProcessor {
        fn process<'a>(
            &mut self,
            cli: &mut CliHandle<'_, W, E>,
            raw: RawCommand<'a>,
        ) -> Result<(), embedded_cli::service::ProcessError<'a, E>> {
            let writer = cli.writer();
            match CliCommand::parse(raw) {
                Ok(CliCommand::Fountain { speed }) => {
                    let _ = cat_detector::FOUNTAIN_CHANNEL.try_send(
                        controller::fountain_controller::FountainCommand::SetSpeed(speed),
                    );
                    let _ = core::writeln!(
                        writer,
                        "\r\nSent FountainCommand::SetSpeed({}) to controller",
                        speed
                    );
                }
                Ok(CliCommand::Stop) => {
                    let _ = cat_detector::FOUNTAIN_CHANNEL
                        .try_send(controller::fountain_controller::FountainCommand::Stop);
                    let _ = core::writeln!(writer, "\r\nSent FountainCommand::Stop to controller");
                }
                Ok(CliCommand::Battery) => {
                    let _ = cat_detector::BATTERY_CHANNEL
                        .try_send(controller::battery_controller::BatteryCommand::CheckStatus);
                    let _ = core::writeln!(
                        writer,
                        "\r\nSent BatteryCommand::CheckStatus to controller"
                    );
                }
                Ok(CliCommand::Thermal) => {
                    let _ = cat_detector::THERMAL_CHANNEL
                        .try_send(controller::thermal_controller::ThermalCommand::CheckTemp);
                    let _ =
                        core::writeln!(writer, "\r\nSent ThermalCommand::CheckTemp to controller");
                }
                Ok(CliCommand::Help) => {
                    let _ = core::writeln!(writer, "\r\nCommands:");
                    let _ = core::writeln!(writer, "  fountain <speed> : Set pump speed (0-100)");
                    let _ = core::writeln!(writer, "  stop             : Stop the pump");
                    let _ =
                        core::writeln!(writer, "  battery          : Trigger battery status check");
                    let _ =
                        core::writeln!(writer, "  thermal          : Trigger thermal temp check");
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
        let _ = cli.write(|writer| {
            let _ = core::writeln!(writer, "\r\n--- RP2040 Interactive Bringup Shell ---");
            let _ = core::writeln!(writer, "Type 'help' to print usage.");
            Ok(())
        });

        let mut processor = CliProcessor;

        // Run the main input loop feeding bytes to the embedded-cli processor
        loop {
            let mut rx_byte = [0u8; 1];
            if rx.blocking_read(&mut rx_byte).is_ok() {
                let _ = cli.process_byte::<CliCommand, _>(rx_byte[0], &mut processor);
            }
        }
    }
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
