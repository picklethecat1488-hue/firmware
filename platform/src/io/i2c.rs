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
    type I2c: embedded_hal_async::i2c::I2c;

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
                        let read_fut = i2c.read(addr_7bit, &mut buf);
                        match embassy_futures::block_on(read_fut) {
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
