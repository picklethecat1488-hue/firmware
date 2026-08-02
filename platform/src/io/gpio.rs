//! GPIO diagnostic and status utilities.

use core::fmt::Write as _;

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
pub fn handle_gpio_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R>(
    _resolver: &R,
    subcommand: Option<GpioSubcommand>,
    pin: Option<u32>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing gpio subcommand (expected: status, read)")?;

    // Read direct hardware registers from RP2040 SIO block via public rp_pac crate
    let gpio_in = rp_pac::SIO.gpio_in(0).read();
    let gpio_oe = rp_pac::SIO.gpio_oe(0).value().read();

    match cmd {
        GpioSubcommand::Status => {
            let _ = writeln!(writer, "GPIO Pin Status (0..29):");
            let _ = writeln!(writer, "Pin   Direction   Level");
            let _ = writeln!(writer, "-----------------------");
            for p in 0..30 {
                let is_out = (gpio_oe & (1 << p)) != 0;
                let is_high = (gpio_in & (1 << p)) != 0;
                let dir_str = if is_out { "Out" } else { "In " };
                let lvl_str = if is_high { "High (1)" } else { "Low  (0)" };
                let _ = writeln!(writer, "GP{:<2}  {:<9}   {}", p, dir_str, lvl_str);
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                if p >= 30 {
                    return Err("Invalid pin number (must be 0..29)");
                }
                let is_out = (gpio_oe & (1 << p)) != 0;
                let is_high = (gpio_in & (1 << p)) != 0;
                let dir_str = if is_out { "Out" } else { "In" };
                let lvl_val = if is_high { 1 } else { 0 };
                let _ = writeln!(writer, "GP{} ({}) is {}", p, dir_str, lvl_val);
            } else {
                let _ = writeln!(writer, "GPIO Pin Levels (0..29):");
                for row in 0..5 {
                    let mut line = heapless::String::<80>::new();
                    for col in 0..6 {
                        let p = row * 6 + col;
                        let is_high = (gpio_in & (1 << p)) != 0;
                        let lvl_val = if is_high { 1 } else { 0 };
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
pub fn handle_gpio_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R>(
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
                let _ = writeln!(writer, "GP{:<2}  In          Low  (0)", p);
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                let _ = writeln!(writer, "GP{} (In) is 0", p);
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
