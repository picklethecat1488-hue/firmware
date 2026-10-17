//! GPIO diagnostic and status utilities.

include!(concat!(env!("OUT_DIR"), "/generated_gpio.rs"));

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
        /// Set the logic level of a single pin (0 or 1).
        Set,
    }
    "status, read, set"
}

/// Checks if a specific GPIO pin is at a high logic level given a bitmask.
#[inline]
pub fn is_pin_high(bitmask: u32, pin: u32) -> bool {
    (bitmask & (1 << pin)) != 0
}

/// Processes GPIO diagnostic CLI subcommands.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub async fn handle_gpio_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R>(
    _resolver: &R,
    subcommand: Option<GpioSubcommand>,
    pin: Option<u32>,
    value: Option<u8>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing gpio subcommand (expected: status, read, set)")?;

    // Helper macro to steal a pin and wrap in ManuallyDrop<Flex>.
    // Suppressing Drop prevents Flex::drop from clearing pad configuration and resetting
    // funcsel to NULL, ensuring GPIO pin directions and levels persist indefinitely.
    macro_rules! steal_flex {
        ($pin_num:expr) => {
            core::mem::ManuallyDrop::new(Flex::new(unsafe {
                embassy_rp::gpio::AnyPin::steal($pin_num as u8)
            }))
        };
    }

    match cmd {
        GpioSubcommand::Status => {
            let _ = writeln!(writer, "GPIO Pin Status (0..29):");
            let _ = writeln!(writer, "Pin   Level");
            let _ = writeln!(writer, "-----------");
            for p in 0..NUM_GPIO_PINS {
                if is_bus_assigned(p) {
                    let _ = writeln!(writer, "GP{:<2}  Alt", p);
                } else {
                    let pin = steal_flex!(p);
                    let lvl_str = if pin.is_high() {
                        "High (1)"
                    } else {
                        "Low  (0)"
                    };
                    let _ = writeln!(writer, "GP{:<2}  {}", p, lvl_str);
                }
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                if p >= NUM_GPIO_PINS {
                    return Err("Invalid pin number (must be 0..29)");
                }
                if is_bus_assigned(p) {
                    let _ = writeln!(
                        writer,
                        "GP{} is configured for alternate function (not GPIO)",
                        p
                    );
                } else {
                    let pin = steal_flex!(p);
                    let lvl_val = if pin.is_high() { 1 } else { 0 };
                    let _ = writeln!(writer, "GP{} level is {}", p, lvl_val);
                }
            } else {
                let _ = writeln!(writer, "GPIO Pin Levels (0..29):");
                for row in 0..5 {
                    let mut line = heapless::String::<80>::new();
                    for col in 0..6 {
                        let p = row * 6 + col;
                        if is_bus_assigned(p) {
                            let _ = write!(line, "GP{:<2}: Alt ", p);
                        } else {
                            let pin = steal_flex!(p);
                            let lvl_val = if pin.is_high() { 1 } else { 0 };
                            let _ = write!(line, "GP{:<2}: {}   ", p, lvl_val);
                        }
                    }
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
        GpioSubcommand::Set => {
            let p = pin.ok_or("Missing pin number (expected: gpio set <pin> <0|1>)")?;
            if p >= NUM_GPIO_PINS {
                return Err("Invalid pin number (must be 0..29)");
            }
            if is_bus_assigned(p) {
                return Err("Cannot set pin configured for alternate function (bus assigned)");
            }
            let val = value.ok_or("Missing pin value (expected: gpio set <pin> <0|1>)")?;
            let mut pin = steal_flex!(p);
            pin.set_as_output();
            if val != 0 {
                pin.set_high();
                let _ = writeln!(writer, "GP{} set to High (1)", p);
            } else {
                pin.set_low();
                let _ = writeln!(writer, "GP{} set to Low (0)", p);
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
    value: Option<u8>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing gpio subcommand (expected: status, read, set)")?;
    match cmd {
        GpioSubcommand::Status => {
            let _ = writeln!(writer, "Mock GPIO Pin Status (0..29):");
            let _ = writeln!(writer, "Pin   Level");
            let _ = writeln!(writer, "-----------");
            for p in 0..NUM_GPIO_PINS {
                if is_bus_assigned(p) {
                    let _ = writeln!(writer, "GP{:<2}  Alt", p);
                } else {
                    let _ = writeln!(writer, "GP{:<2}  Low  (0)", p);
                }
            }
        }
        GpioSubcommand::Read => {
            if let Some(p) = pin {
                if p >= NUM_GPIO_PINS {
                    return Err("Invalid pin number (must be 0..29)");
                }
                if is_bus_assigned(p) {
                    let _ = writeln!(
                        writer,
                        "GP{} is configured for alternate function (not GPIO)",
                        p
                    );
                } else {
                    let _ = writeln!(writer, "GP{} level is 0", p);
                }
            } else {
                let _ = writeln!(writer, "Mock GPIO Pin Levels (0..29):");
                for row in 0..5 {
                    let mut line = heapless::String::<80>::new();
                    for col in 0..6 {
                        let p = row * 6 + col;
                        if is_bus_assigned(p) {
                            let _ = write!(line, "GP{:<2}: Alt ", p);
                        } else {
                            let _ = write!(line, "GP{:<2}: 0   ", p);
                        }
                    }
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
        GpioSubcommand::Set => {
            let p = pin.ok_or("Missing pin number (expected: gpio set <pin> <0|1>)")?;
            if p >= NUM_GPIO_PINS {
                return Err("Invalid pin number (must be 0..29)");
            }
            if is_bus_assigned(p) {
                return Err("Cannot set pin configured for alternate function (bus assigned)");
            }
            let val = value.ok_or("Missing pin value (expected: gpio set <pin> <0|1>)")?;
            if val != 0 {
                let _ = writeln!(writer, "Mock GP{} set to High (1)", p);
            } else {
                let _ = writeln!(writer, "Mock GP{} set to Low (0)", p);
            }
        }
    }
    Ok(())
}
