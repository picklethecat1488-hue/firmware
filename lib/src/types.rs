use core::fmt::Write;

/// Global circular log buffer for crash logging.
pub struct LogBuffer {
    /// Internal byte buffer.
    pub buffer: [u8; 1024],
    /// Current write head.
    pub head: usize,
    /// Whether the buffer has wrapped around.
    pub wrapped: bool,
}

impl LogBuffer {
    /// Creates a new empty LogBuffer.
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; 1024],
            head: 0,
            wrapped: false,
        }
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for LogBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        for &b in bytes {
            self.buffer[self.head] = b;
            self.head += 1;
            if self.head >= 1024 {
                self.head = 0;
                self.wrapped = true;
            }
        }
        Ok(())
    }
}

/// A target-agnostic trait representing a blocking flash storage driver.
#[allow(clippy::result_unit_err)]
pub trait PanicFlash: Send {
    /// Read data from the flash storage.
    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), ()>;
    /// Write data to the flash storage.
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), ()>;
    /// Erase a range of flash memory.
    fn erase(&mut self, from: u32, to: u32) -> Result<(), ()>;
    /// Capacity of the flash partition.
    fn capacity(&self) -> usize;
}

impl<F> PanicFlash for F
where
    F: embedded_storage::nor_flash::NorFlash + embedded_storage::nor_flash::ReadNorFlash + Send,
{
    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), ()> {
        embedded_storage::nor_flash::ReadNorFlash::read(self, offset, bytes).map_err(|_| ())
    }
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), ()> {
        embedded_storage::nor_flash::NorFlash::write(self, offset, bytes).map_err(|_| ())
    }
    fn erase(&mut self, from: u32, to: u32) -> Result<(), ()> {
        embedded_storage::nor_flash::NorFlash::erase(self, from, to).map_err(|_| ())
    }
    fn capacity(&self) -> usize {
        embedded_storage::nor_flash::ReadNorFlash::capacity(self)
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Configuration parameters for the panic handler
pub struct PanicConfig {
    /// Flash driver reference
    pub flash: &'static mut dyn PanicFlash,
    /// Offset range in flash partition used for filesystem
    pub range: core::ops::Range<u32>,
}

/// Result of a battery update state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryTransitionResult {
    /// The new battery critical flag value.
    pub new_battery_critical: bool,
    /// The next system status if a transition occurred.
    pub next_status: Option<model::types::SystemStatus>,
}

/// Context info containing state-of-charge measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryUpdateInfo {
    /// Percentage integer (0-100)
    pub state_of_charge: u8,
    /// Is the charger connected?
    pub charging: bool,
    /// Is there a fault?
    pub is_fault: bool,
}

/// Threshold values for battery safety transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryThresholds {
    /// Critical SOC percentage limit
    pub critical_threshold: u8,
    /// Recovery hysteresis value
    pub hysteresis: u8,
}
