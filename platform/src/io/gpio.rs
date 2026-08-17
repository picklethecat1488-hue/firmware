//! GPIO diagnostic and status utilities.

use core::fmt::Write as _;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_rp::gpio::Flex;

crate::subcommand_enum! {
    /// Subcommands for GPIO diagnostics.
    pub enum GpioSubcommand {
        /// Show all pin directions and levels.
        Status,
        /// Read the level of a single pin.
        Read,
    }
    "status, read"
}

/// Processes GPIO diagnostic CLI subcommands.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub async fn handle_gpio_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R>(
    _resolver: &R,
    subcommand: Option<GpioSubcommand>,
    pin: Option<u32>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing gpio subcommand (expected: status, read)")?;

    // Helper macro to steal a pin and wrap in Flex
    macro_rules! steal_flex {
        ($pin_num:expr) => {
            Flex::new(unsafe { embassy_rp::gpio::AnyPin::steal($pin_num as u8) })
        };
    }

    match cmd {
        GpioSubcommand::Status => {
            let _ = writeln!(writer, "GPIO Pin Status (0..29):");
            let _ = writeln!(writer, "Pin   Level");
            let _ = writeln!(writer, "-----------");
            for p in 0..30 {
                let pin = steal_flex!(p);
                let lvl_str = if pin.is_high() {
                    "High (1)"
                } else {
                    "Low  (0)"
                };
                let _ = writeln!(writer, "GP{:<2}  {}", p, lvl_str);
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                if p >= 30 {
                    return Err("Invalid pin number (must be 0..29)");
                }
                let pin = steal_flex!(p);
                let lvl_val = if pin.is_high() { 1 } else { 0 };
                let _ = writeln!(writer, "GP{} level is {}", p, lvl_val);
            } else {
                let _ = writeln!(writer, "GPIO Pin Levels (0..29):");
                for row in 0..5 {
                    let mut line = heapless::String::<80>::new();
                    for col in 0..6 {
                        let p = row * 6 + col;
                        let pin = steal_flex!(p);
                        let lvl_val = if pin.is_high() { 1 } else { 0 };
                        let _ = write!(line, "GP{:<2}: {}   ", p, lvl_val);
                    }
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
    }
    Ok(())
}

/// Processes GPIO diagnostic CLI subcommands (Mock version for host).
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub async fn handle_gpio_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R>(
    _resolver: &R,
    subcommand: Option<GpioSubcommand>,
    pin: Option<u32>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing gpio subcommand (expected: status, read)")?;
    match cmd {
        GpioSubcommand::Status => {
            let _ = writeln!(writer, "Mock GPIO Pin Status (0..29):");
            for p in 0..30 {
                let _ = writeln!(writer, "GP{:<2}  Low  (0)", p);
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                let _ = writeln!(writer, "GP{} level is 0", p);
            } else {
                let _ = writeln!(writer, "Mock GPIO Pin Levels (0..29):");
                for row in 0..5 {
                    let mut line = heapless::String::<80>::new();
                    for col in 0..6 {
                        let p = row * 6 + col;
                        let _ = write!(line, "GP{:<2}: 0   ", p);
                    }
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
    }
    Ok(())
}
