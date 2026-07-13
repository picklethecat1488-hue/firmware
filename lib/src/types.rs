use core::fmt::Write;
use minicbor::{Decode, Encode};

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

    /// Write raw bytes to the circular log buffer.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.buffer[self.head] = b;
            self.head += 1;
            if self.head >= 1024 {
                self.head = 0;
                self.wrapped = true;
            }
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
    /// Static filesystem buffer used as workspace during panic writes
    pub fs_buf: &'static mut [u8],
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

/// Structure representing a serialized crash dump in CBOR.
#[derive(Debug, Clone, minicbor::Encode, minicbor::Decode)]
pub struct CrashDump<'a> {
    /// Git revision hash of the firmware
    #[n(0)]
    pub revision_hash: &'a str,
    /// Register R0
    #[n(1)]
    pub r0: u32,
    /// Register R1
    #[n(2)]
    pub r1: u32,
    /// Register R2
    #[n(3)]
    pub r2: u32,
    /// Register R3
    #[n(4)]
    pub r3: u32,
    /// Backtrace program counters
    #[n(5)]
    pub backtrace: [u32; 32],
    /// Number of valid entries in the backtrace array
    #[n(6)]
    pub backtrace_len: u32,
    /// Circular log buffer raw bytes
    #[cbor(with = "minicbor::bytes")]
    #[n(7)]
    pub system_logs: &'a [u8],
    /// Unique identifier (UUID) for this crash dump
    #[cbor(with = "minicbor::bytes")]
    #[n(8)]
    pub uuid: [u8; 16],
}

/// Stack scan limit in words (8 KB stack coverage)
pub const STACK_SCAN_LIMIT: u32 = 2048;

/// Project metadata struct embedded in the ELF to allow autodetecting chip/partition layout.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ProjectMetadata<'a> {
    /// Chip name (e.g. "rp2040")
    #[n(0)]
    pub chip: &'a str,
    /// The virtual memory flash address of the storage partition
    #[n(1)]
    pub partition_address: u32,
    /// The size of the storage partition in bytes
    #[n(2)]
    pub partition_size: u32,
    /// Flash write alignment/size in bytes
    #[n(3)]
    pub flash_write_size: u32,
    /// Flash erase sector size in bytes
    #[n(4)]
    pub flash_erase_size: u32,
    /// Stack scan limit in words
    #[n(5)]
    pub stack_scan_limit: u32,
}

impl<'a> ProjectMetadata<'a> {
    /// Statically serializes all fields into CBOR format.
    pub const fn serialize(
        chip: &'a str,
        partition_address: u32,
        partition_size: u32,
        flash_write_size: u32,
        flash_erase_size: u32,
        stack_scan_limit: u32,
    ) -> crate::cbor::ConstCborWriter<128> {
        crate::cbor::ConstCborWriter::<128>::new()
            .write_array_header(6)
            .write_str(chip)
            .write_u32(partition_address)
            .write_u32(partition_size)
            .write_u32(flash_write_size)
            .write_u32(flash_erase_size)
            .write_u32(stack_scan_limit)
    }
}

/// Type alias for an Embassy channel.
pub type Channel<M, T, const N: usize> = embassy_sync::channel::Channel<M, T, N>;

/// Type alias for an Embassy channel Sender.
pub type Sender<'a, M, T, const N: usize> = embassy_sync::channel::Sender<'a, M, T, N>;

/// Type alias for an Embassy channel Receiver.
pub type Receiver<'a, M, T, const N: usize> = embassy_sync::channel::Receiver<'a, M, T, N>;

/// Reasons why the system may be trapped in low-power boot state.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[repr(u32)]
pub enum BootTrapReason {
    /// Battery check pending/failing upon boot.
    Battery = 1 << 0,
    /// Thermal check pending/failing upon boot.
    Thermal = 1 << 1,
}

/// Error indicating that an invalid boot trap mask was configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub struct InvalidBootTrapMask;

/// A type-safe mutable bitmask vector representing active boot traps.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub struct BootTrapMask(u32);

impl core::fmt::Debug for BootTrapMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BootTrapMask").field(&self.0).finish()
    }
}

impl BootTrapMask {
    /// Validates that only known BootTrapReason bits are set in the mask.
    pub const fn validate(&self) -> Result<(), InvalidBootTrapMask> {
        let valid_bits = (BootTrapReason::Battery as u32) | (BootTrapReason::Thermal as u32);
        if (self.0 & !valid_bits) != 0 {
            Err(InvalidBootTrapMask)
        } else {
            Ok(())
        }
    }

    /// Creates a new empty BootTrapMask.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a BootTrapMask with a raw bitmask value.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Checks if any boot traps are active.
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Checks if a specific boot trap is active.
    pub const fn has(&self, reason: BootTrapReason) -> bool {
        (self.0 & (reason as u32)) != 0
    }

    /// Adds a boot trap to the mask.
    pub fn add(&mut self, reason: BootTrapReason) {
        self.0 |= reason as u32;
    }

    /// Removes a boot trap from the mask.
    pub fn remove(&mut self, reason: BootTrapReason) {
        self.0 &= !(reason as u32);
    }

    /// Returns the raw bitmask value.
    pub const fn raw(&self) -> u32 {
        self.0
    }
}

/// Actions sent by the thermal controller to the system controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum ThermalUpdateAction {
    /// Clear the thermal boot trap.
    ClearBootTrap,
    /// Alert triggered due to critical temperature.
    AlertTriggered,
}

/// Result of a thermal update transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThermalTransitionResult {
    /// The next system status if a transition occurred.
    pub next_status: Option<model::types::SystemStatus>,
    /// Whether the transition requires clearing wake locks.
    pub clear_wake_locks: bool,
}
