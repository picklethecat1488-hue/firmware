//! Shared I2C blocking access wrapper structures.

use core::fmt::Write as _;
use embedded_hal_async::i2c::I2c;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::mutex::Mutex;

#[cfg(all(target_arch = "arm", target_os = "none"))]
embassy_rp::bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C0>;
});

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// A wrapper structure containing the initialized I2C0 peripheral on target.
pub struct SafeI2c {
    /// Active I2C0 async driver instance
    pub i2c: Option<
        embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>,
    >,
    /// The GPIO pin number used for I2C SDA
    pub sda_pin: u8,
    /// The GPIO pin number used for I2C SCL
    pub scl_pin: u8,
    /// The I2C clock frequency in Hz
    pub frequency: u32,
    /// Recovery function pointer
    pub recovery_fn: fn(sda_pin: u8, scl_pin: u8),
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl SafeI2c {
    /// Creates a new SafeI2c with target pins and frequency cached.
    pub const fn new(
        sda_pin: u8,
        scl_pin: u8,
        frequency: u32,
        recovery_fn: fn(sda_pin: u8, scl_pin: u8),
    ) -> Self {
        Self {
            i2c: None,
            sda_pin,
            scl_pin,
            frequency,
            recovery_fn,
        }
    }

    /// Performs bus recovery and initializes the blocking I2C0 driver.
    pub fn initialize(&mut self) {
        // Run bus recovery sequence first to unstuck any locked device
        (self.recovery_fn)(self.sda_pin, self.scl_pin);

        // Steal control of the I2C0 peripheral and pins
        let i2c0 = unsafe { embassy_rp::peripherals::I2C0::steal() };
        let pin_scl = unsafe { embassy_rp::peripherals::PIN_13::steal() };
        let pin_sda = unsafe { embassy_rp::peripherals::PIN_12::steal() };

        let mut config = embassy_rp::i2c::Config::default();
        config.frequency = self.frequency;

        self.i2c = Some(embassy_rp::i2c::I2c::new_async(
            i2c0, pin_scl, pin_sda, Irqs, config,
        ));
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Mock SafeI2c for host compilation.
pub struct SafeI2c {
    /// Cached configuration parameters
    pub sda_pin: u8,
    /// Cached configuration parameters
    pub scl_pin: u8,
    /// Cached configuration parameters
    pub frequency: u32,
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl SafeI2c {
    /// Creates a new SafeI2c mock.
    pub const fn new(sda_pin: u8, scl_pin: u8, frequency: u32) -> Self {
        Self {
            sda_pin,
            scl_pin,
            frequency,
        }
    }

    /// Mock initialize method.
    pub fn initialize(&mut self) {}
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// The default timeout for I2C transactions to detect a stuck bus.
pub const I2C_TIMEOUT: ::embassy_time::Duration = ::embassy_time::Duration::from_millis(50);

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[derive(Clone, Copy)]
/// A unit struct wrapper that implements `embedded_hal_async::i2c::I2c` by dynamically locking a Shared I2C Mutex.
pub struct SharedI2cWrapper<'a> {
    mutex: &'a Mutex<CriticalSectionRawMutex, SafeI2c>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> SharedI2cWrapper<'a> {
    /// Creates a new SharedI2cWrapper wrapping a Mutex.
    pub const fn new(mutex: &'a Mutex<CriticalSectionRawMutex, SafeI2c>) -> Self {
        Self { mutex }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> embedded_hal_async::i2c::ErrorType for SharedI2cWrapper<'a> {
    type Error = embassy_rp::i2c::Error;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> embedded_hal_async::i2c::I2c for SharedI2cWrapper<'a> {
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let mut guard = self.mutex.lock().await;
        if let Some(ref mut i2c) = guard.i2c {
            let fut = i2c.read(address, read);
            match ::embassy_time::with_timeout(I2C_TIMEOUT, fut).await {
                Ok(res) => res,
                Err(_) => {
                    // Timeout! Bus is stuck. Free the instance, recover, and re-initialize.
                    guard.i2c = None;
                    guard.initialize();
                    Err(embassy_rp::i2c::Error::Abort(
                        embassy_rp::i2c::AbortReason::Other(0),
                    ))
                }
            }
        } else {
            Err(embassy_rp::i2c::Error::Abort(
                embassy_rp::i2c::AbortReason::Other(0),
            ))
        }
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let mut guard = self.mutex.lock().await;
        if let Some(ref mut i2c) = guard.i2c {
            let fut = i2c.write(address, write);
            match ::embassy_time::with_timeout(I2C_TIMEOUT, fut).await {
                Ok(res) => res,
                Err(_) => {
                    guard.i2c = None;
                    guard.initialize();
                    Err(embassy_rp::i2c::Error::Abort(
                        embassy_rp::i2c::AbortReason::Other(0),
                    ))
                }
            }
        } else {
            Err(embassy_rp::i2c::Error::Abort(
                embassy_rp::i2c::AbortReason::Other(0),
            ))
        }
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut guard = self.mutex.lock().await;
        if let Some(ref mut i2c) = guard.i2c {
            let fut = i2c.write_read(address, write, read);
            match ::embassy_time::with_timeout(I2C_TIMEOUT, fut).await {
                Ok(res) => res,
                Err(_) => {
                    guard.i2c = None;
                    guard.initialize();
                    Err(embassy_rp::i2c::Error::Abort(
                        embassy_rp::i2c::AbortReason::Other(0),
                    ))
                }
            }
        } else {
            Err(embassy_rp::i2c::Error::Abort(
                embassy_rp::i2c::AbortReason::Other(0),
            ))
        }
    }

    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut guard = self.mutex.lock().await;
        if let Some(ref mut i2c) = guard.i2c {
            let fut = i2c.transaction(address, operations);
            match ::embassy_time::with_timeout(I2C_TIMEOUT, fut).await {
                Ok(res) => res,
                Err(_) => {
                    guard.i2c = None;
                    guard.initialize();
                    Err(embassy_rp::i2c::Error::Abort(
                        embassy_rp::i2c::AbortReason::Other(0),
                    ))
                }
            }
        } else {
            Err(embassy_rp::i2c::Error::Abort(
                embassy_rp::i2c::AbortReason::Other(0),
            ))
        }
    }
}

/// Minimum valid 7-bit I2C peripheral address (excludes reserved `0x00..=0x07` group).
pub const MIN_I2C_7BIT_ADDR: u8 = 0x08;

/// Maximum valid 7-bit I2C peripheral address (excludes reserved `0x78..=0x7F` 10-bit/HS group).
pub const MAX_I2C_7BIT_ADDR: u8 = 0x77;

/// Maximum unsigned value representable as a single-byte I2C register address (`0x00FF`).
pub const MAX_8BIT_REG_ADDR: u16 = 0x00FF;

/// Width-typed I2C register offset supporting both 8-bit (`1` byte) and 16-bit big-endian (`2` bytes) addressing.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum RegisterAddress {
    /// Single-byte 8-bit register offset (`[offset]`).
    Reg8(u8),
    /// Two-byte big-endian 16-bit register offset (`[hi, lo]`).
    Reg16(u16),
}

crate::subcommand_enum! {
    /// Subcommands for I2C diagnostics and direct register inspection.
    pub enum I2cSubcommand {
        /// Scan the I2C bus for responsive 7-bit addresses.
        Scan,
        /// Read an 8-bit register value (`i2c peek8 <addr> <reg> [--reg16] [-b|--bus <name>]`).
        Peek8 = "peek8",
        /// Read a 16-bit big-endian register value (`i2c peek16 <addr> <reg> [--reg16] [-b|--bus <name>]`).
        Peek16 = "peek16",
        /// Write an 8-bit register value (`i2c poke8 <addr> <reg> <val> [--reg16] [-b|--bus <name>]`).
        Poke8 = "poke8",
        /// Write a 16-bit big-endian register value (`i2c poke16 <addr> <reg> <val> [--reg16] [-b|--bus <name>]`).
        Poke16 = "poke16",
    }
    "scan, peek8, peek16, poke8, poke16"
}

/// Parses an unsigned 8-bit integer from a decimal (`"54"`) or hexadecimal (`"0x36"`) CLI token.
pub fn parse_u8_hex_or_dec(raw: &str) -> Result<u8, &'static str> {
    let trimmed = raw.trim();
    if let Some(hex_body) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u8::from_str_radix(hex_body, 16)
            .map_err(|_| "Invalid 8-bit hex value (expected 0x00..=0xFF)")
    } else {
        trimmed
            .parse::<u8>()
            .map_err(|_| "Invalid 8-bit integer (expected 0..=255 or 0x00..=0xFF)")
    }
}

/// Parses an unsigned 16-bit integer from a decimal (`"38684"`) or hexadecimal (`"0x971C"`) CLI token.
pub fn parse_u16_hex_or_dec(raw: &str) -> Result<u16, &'static str> {
    let trimmed = raw.trim();
    if let Some(hex_body) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u16::from_str_radix(hex_body, 16)
            .map_err(|_| "Invalid 16-bit hex value (expected 0x0000..=0xFFFF)")
    } else {
        trimmed
            .parse::<u16>()
            .map_err(|_| "Invalid 16-bit integer (expected 0..=65535 or 0x0000..=0xFFFF)")
    }
}

/// Parses an 8-bit or 16-bit big-endian [`RegisterAddress`], promoting to [`RegisterAddress::Reg16`]
/// whenever `force_reg16` is `true` or the parsed offset exceeds `0xFF`.
pub fn parse_register_address(
    raw: Option<&str>,
    force_reg16: bool,
    usage: &'static str,
) -> Result<RegisterAddress, &'static str> {
    let val = parse_u16_hex_or_dec(raw.ok_or(usage)?)?;
    if force_reg16 || val > MAX_8BIT_REG_ADDR {
        Ok(RegisterAddress::Reg16(val))
    } else {
        Ok(RegisterAddress::Reg8(val as u8))
    }
}

fn parse_7bit_i2c_addr(raw: Option<&str>, usage: &'static str) -> Result<u8, &'static str> {
    let token = raw.ok_or(usage)?;
    let addr = parse_u8_hex_or_dec(token)?;
    if !(MIN_I2C_7BIT_ADDR..=MAX_I2C_7BIT_ADDR).contains(&addr) {
        return Err("Invalid 7-bit I2C address (must be 0x08..=0x77)");
    }
    Ok(addr)
}

/// Trait to resolve I2C buses for platform CLI handlers.
pub trait I2cResolver {
    /// Associated type for the I2C peripheral.
    type I2c: embedded_hal_async::i2c::I2c;

    /// Resolves a named I2C bus.
    #[allow(clippy::mut_from_ref)]
    fn resolve_i2c(&self, name: Option<&str>) -> Result<&mut Self::I2c, &'static str>;
}

/// Processes I2C diagnostic and register peek/poke CLI subcommands.
#[allow(clippy::too_many_arguments)]
pub async fn handle_i2c_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    R: I2cResolver,
>(
    resolver: &R,
    subcommand: Option<I2cSubcommand>,
    reg16: bool,
    bus: Option<&str>,
    arg1: Option<&str>,
    arg2: Option<&str>,
    arg3: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand
        .ok_or("Missing i2c subcommand (expected: scan, peek8, peek16, poke8, poke16)")?;
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
                    if !(MIN_I2C_7BIT_ADDR..=MAX_I2C_7BIT_ADDR).contains(&addr_7bit) {
                        let _ = write!(line, "   ");
                    } else {
                        // Attempt a single byte read to check for ACK using 7-bit address
                        let mut buf = [0];
                        match i2c.read(addr_7bit, &mut buf).await {
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
        I2cSubcommand::Peek8 => {
            const USAGE: &str = "Usage: i2c peek8 <addr> <reg> [--reg16] [-b|--bus <name>]";
            let addr = parse_7bit_i2c_addr(arg1, USAGE)?;
            let reg = parse_register_address(arg2, reg16, USAGE)?;
            let i2c = resolver.resolve_i2c(bus)?;
            let mut buf = [0u8; 1];
            match reg {
                RegisterAddress::Reg8(r8) => {
                    i2c.write_read(addr, &[r8], &mut buf)
                        .await
                        .map_err(|_| "I2C peek8 transaction failed (NACK or bus timeout)")?;
                    let val = buf[0];
                    let _ = writeln!(
                        writer,
                        "I2C[0x{:02x}] reg 0x{:02x} = 0x{:02x} ({})",
                        addr, r8, val, val
                    );
                }
                RegisterAddress::Reg16(r16) => {
                    let reg_bytes = r16.to_be_bytes();
                    i2c.write_read(addr, &reg_bytes, &mut buf)
                        .await
                        .map_err(|_| "I2C peek8 transaction failed (NACK or bus timeout)")?;
                    let val = buf[0];
                    let _ = writeln!(
                        writer,
                        "I2C[0x{:02x}] reg 0x{:04x} = 0x{:02x} ({})",
                        addr, r16, val, val
                    );
                }
            }
            Ok(())
        }
        I2cSubcommand::Peek16 => {
            const USAGE: &str = "Usage: i2c peek16 <addr> <reg> [--reg16] [-b|--bus <name>]";
            let addr = parse_7bit_i2c_addr(arg1, USAGE)?;
            let reg = parse_register_address(arg2, reg16, USAGE)?;
            let i2c = resolver.resolve_i2c(bus)?;
            let mut buf = [0u8; 2];
            match reg {
                RegisterAddress::Reg8(r8) => {
                    i2c.write_read(addr, &[r8], &mut buf)
                        .await
                        .map_err(|_| "I2C peek16 transaction failed (NACK or bus timeout)")?;
                    let val = u16::from_be_bytes(buf);
                    let _ = writeln!(
                        writer,
                        "I2C[0x{:02x}] reg 0x{:02x} = 0x{:04x} ({}, bytes: [0x{:02x}, 0x{:02x}])",
                        addr, r8, val, val, buf[0], buf[1]
                    );
                }
                RegisterAddress::Reg16(r16) => {
                    let reg_bytes = r16.to_be_bytes();
                    i2c.write_read(addr, &reg_bytes, &mut buf)
                        .await
                        .map_err(|_| "I2C peek16 transaction failed (NACK or bus timeout)")?;
                    let val = u16::from_be_bytes(buf);
                    let _ = writeln!(
                        writer,
                        "I2C[0x{:02x}] reg 0x{:04x} = 0x{:04x} ({}, bytes: [0x{:02x}, 0x{:02x}])",
                        addr, r16, val, val, buf[0], buf[1]
                    );
                }
            }
            Ok(())
        }
        I2cSubcommand::Poke8 => {
            const USAGE: &str = "Usage: i2c poke8 <addr> <reg> <val> [--reg16] [-b|--bus <name>]";
            let addr = parse_7bit_i2c_addr(arg1, USAGE)?;
            let reg = parse_register_address(arg2, reg16, USAGE)?;
            let val = parse_u8_hex_or_dec(arg3.ok_or(USAGE)?)?;
            let i2c = resolver.resolve_i2c(bus)?;
            match reg {
                RegisterAddress::Reg8(r8) => {
                    i2c.write(addr, &[r8, val])
                        .await
                        .map_err(|_| "I2C poke8 transaction failed (NACK or bus timeout)")?;
                    let _ = writeln!(
                        writer,
                        "Wrote 0x{:02x} ({}) to I2C[0x{:02x}] reg 0x{:02x}",
                        val, val, addr, r8
                    );
                }
                RegisterAddress::Reg16(r16) => {
                    let rb = r16.to_be_bytes();
                    i2c.write(addr, &[rb[0], rb[1], val])
                        .await
                        .map_err(|_| "I2C poke8 transaction failed (NACK or bus timeout)")?;
                    let _ = writeln!(
                        writer,
                        "Wrote 0x{:02x} ({}) to I2C[0x{:02x}] reg 0x{:04x}",
                        val, val, addr, r16
                    );
                }
            }
            Ok(())
        }
        I2cSubcommand::Poke16 => {
            const USAGE: &str = "Usage: i2c poke16 <addr> <reg> <val> [--reg16] [-b|--bus <name>]";
            let addr = parse_7bit_i2c_addr(arg1, USAGE)?;
            let reg = parse_register_address(arg2, reg16, USAGE)?;
            let val = parse_u16_hex_or_dec(arg3.ok_or(USAGE)?)?;
            let bytes = val.to_be_bytes();
            let i2c = resolver.resolve_i2c(bus)?;
            match reg {
                RegisterAddress::Reg8(r8) => {
                    i2c.write(addr, &[r8, bytes[0], bytes[1]])
                        .await
                        .map_err(|_| "I2C poke16 transaction failed (NACK or bus timeout)")?;
                    let _ = writeln!(
                        writer,
                        "Wrote 0x{:04x} ({}) to I2C[0x{:02x}] reg 0x{:02x}",
                        val, val, addr, r8
                    );
                }
                RegisterAddress::Reg16(r16) => {
                    let rb = r16.to_be_bytes();
                    i2c.write(addr, &[rb[0], rb[1], bytes[0], bytes[1]])
                        .await
                        .map_err(|_| "I2C poke16 transaction failed (NACK or bus timeout)")?;
                    let _ = writeln!(
                        writer,
                        "Wrote 0x{:04x} ({}) to I2C[0x{:02x}] reg 0x{:04x}",
                        val, val, addr, r16
                    );
                }
            }
            Ok(())
        }
    }
}

/// Trait for platform I2C bus recovery.
pub trait PlatformI2cRecovery {
    /// Perform a bus recovery sequence to free stuck devices on the bus.
    ///
    /// # Safety
    /// This function steals the pins and therefore must only be called when the I2C peripheral is disabled.
    unsafe fn recover_i2c_bus(&self) -> Result<(), &'static str>;
}

/// Trait for sharing I2C access safely across tasks and cores.
pub trait PlatformI2cAccess {
    /// The error type associated with this I2C bus.
    type Error: embedded_hal_async::i2c::Error;

    /// The type of I2C bus implementation returned.
    type I2c<'a>: embedded_hal_async::i2c::I2c<Error = Self::Error>
    where
        Self: 'a;

    /// Get a shared reference to the I2C bus.
    fn get_i2c(&self) -> Self::I2c<'_>;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl PlatformI2cAccess
    for &'static embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        SafeI2c,
    >
{
    type Error = embassy_rp::i2c::Error;
    type I2c<'a>
        = SharedI2cWrapper<'a>
    where
        Self: 'a;

    fn get_i2c(&self) -> Self::I2c<'_> {
        SharedI2cWrapper::new(self)
    }
}
