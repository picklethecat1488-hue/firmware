//! Shared I2C blocking access wrapper structures.

use core::fmt::Write as _;
use embedded_hal::i2c::I2c as _;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::cell::RefCell;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::blocking_mutex::Mutex;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// A wrapper structure containing the initialized I2C0 peripheral on target.
pub struct SafeI2c(
    pub  Option<
        embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    >,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[derive(Clone, Copy)]
/// A unit struct wrapper that implements `embedded_hal::i2c::I2c` by dynamically locking a Shared I2C Mutex.
pub struct SharedI2cWrapper<'a> {
    mutex: &'a Mutex<CriticalSectionRawMutex, RefCell<SafeI2c>>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> SharedI2cWrapper<'a> {
    /// Creates a new SharedI2cWrapper wrapping a Mutex.
    pub const fn new(mutex: &'a Mutex<CriticalSectionRawMutex, RefCell<SafeI2c>>) -> Self {
        Self { mutex }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> embedded_hal::i2c::ErrorType for SharedI2cWrapper<'a> {
    type Error = embassy_rp::i2c::Error;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> embedded_hal::i2c::I2c for SharedI2cWrapper<'a> {
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.read(address, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write(address, write)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write_read(address, write, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.transaction(address, operations)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }
}

crate::subcommand_enum! {
    /// Subcommands for I2C diagnostics.
    pub enum I2cSubcommand {
        /// Scan the I2C bus.
        Scan,
    }
    "scan"
}

/// Trait to resolve I2C buses for platform CLI handlers.
pub trait I2cResolver {
    /// Associated type for the I2C peripheral.
    type I2c: embedded_hal::i2c::I2c;

    /// Resolves a named I2C bus.
    #[allow(clippy::mut_from_ref)]
    fn resolve_i2c(&self, name: Option<&str>) -> Result<&mut Self::I2c, &'static str>;
}

/// Processes I2C diagnostic CLI subcommands.
pub fn handle_i2c_cli<W: embedded_io::Write<Error = E>, E: embedded_io::Error, R: I2cResolver>(
    resolver: &R,
    subcommand: Option<I2cSubcommand>,
    bus: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing i2c subcommand (expected: scan)")?;
    match cmd {
        I2cSubcommand::Scan => {
            let i2c = resolver.resolve_i2c(bus)?;
            let _ = writeln!(writer, "Scanning I2C bus...");
            let _ = writeln!(
                writer,
                "     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f"
            );
            for row in 0..8 {
                let mut line = heapless::String::<64>::new();
                let _ = write!(line, "{:02x}:", row * 16);
                for col in 0..16 {
                    let addr_7bit = row * 16 + col;
                    if !(0x08..=0x77).contains(&addr_7bit) {
                        let _ = write!(line, "   ");
                    } else {
                        // Attempt a single byte read to check for ACK using 7-bit address
                        let mut buf = [0];
                        match i2c.read(addr_7bit, &mut buf) {
                            Ok(_) => {
                                let _ = write!(line, " {:02x}", addr_7bit);
                            }
                            Err(_) => {
                                let _ = write!(line, " --");
                            }
                        }
                    }
                }
                let _ = writeln!(writer, "{}", line);
            }
            Ok(())
        }
    }
}
