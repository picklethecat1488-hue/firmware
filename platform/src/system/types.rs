use core::fmt::Write;
use minicbor::{Decode, Encode};

/// Stack scan limit in words (8 KB stack coverage)
pub const STACK_SCAN_LIMIT: u32 = 2048;

/// Capacity of the global circular crash log buffer in bytes.
pub const CRASH_LOG_BUFFER_SIZE: usize = 1024;

/// Global circular log buffer for crash logging.
pub struct LogBuffer {
    /// Internal byte buffer.
    pub buffer: [u8; CRASH_LOG_BUFFER_SIZE],
    /// Current write head.
    pub head: usize,
    /// Index of the oldest valid frame start.
    pub tail: usize,
    /// Whether the buffer has wrapped around.
    pub wrapped: bool,
}

impl LogBuffer {
    /// Creates a new empty LogBuffer.
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; CRASH_LOG_BUFFER_SIZE],
            head: 0,
            tail: 0,
            wrapped: false,
        }
    }

    /// Writes a complete frame of bytes to the circular log buffer.
    /// Each frame is stored with a 2-byte length prefix to allow tracking frame boundaries.
    pub fn write_frame(&mut self, frame: &[u8]) {
        let frame_len = frame.len();
        if frame_len > 1000 {
            // Frame is too large to fit in the buffer, ignore it
            return;
        }
        let total_bytes = 2 + frame_len;

        if self.wrapped || self.head + total_bytes > CRASH_LOG_BUFFER_SIZE {
            self.wrapped = true;
        }

        // If tail is inside the range of bytes we are about to overwrite, advance it
        while self.wrapped && self.circular_distance(self.head, self.tail) < total_bytes {
            // Read length prefix at tail
            let l_high = self.buffer[self.tail];
            let l_low = self.buffer[(self.tail + 1) % CRASH_LOG_BUFFER_SIZE];
            let len = ((l_high as usize) << 8) | (l_low as usize);
            // Advance tail past this frame
            self.tail = (self.tail + 2 + len) % CRASH_LOG_BUFFER_SIZE;
        }

        // Write 2-byte length prefix
        self.buffer[self.head] = (frame_len >> 8) as u8;
        self.buffer[(self.head + 1) % CRASH_LOG_BUFFER_SIZE] = (frame_len & 0xFF) as u8;
        self.head = (self.head + 2) % CRASH_LOG_BUFFER_SIZE;

        // Write frame data
        for &b in frame {
            self.buffer[self.head] = b;
            self.head = (self.head + 1) % CRASH_LOG_BUFFER_SIZE;
        }
    }

    fn circular_distance(&self, from: usize, to: usize) -> usize {
        if to >= from {
            to - from
        } else {
            CRASH_LOG_BUFFER_SIZE - from + to
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
        self.write_frame(s.as_bytes());
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

/// Structure representing the captured panic state of any core.
pub struct CorePanicState {
    /// Flag indicating that a core has panicked.
    pub panicked: core::sync::atomic::AtomicBool,
    /// Saved R0 register.
    pub r0: u32,
    /// Saved R1 register.
    pub r1: u32,
    /// Saved R2 register.
    pub r2: u32,
    /// Saved R3 register.
    pub r3: u32,
    /// Saved SP register.
    pub sp: u32,
    /// Saved LR register.
    pub lr: u32,
    /// Saved PC register.
    pub pc: u32,
    /// Saved stack top.
    pub stack_top: u32,
    /// Saved panic message characters (UTF-8).
    pub message: [u8; 64],
    /// Length of the saved panic message.
    pub message_len: u8,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct BufferWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a> core::fmt::Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len().min(self.buf.len() - self.pos);
        self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
        self.pos += len;
        Ok(())
    }
}

impl CorePanicState {
    /// Create a new empty panic state.
    pub fn new_empty() -> Self {
        Self {
            panicked: core::sync::atomic::AtomicBool::new(false),
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            sp: 0,
            lr: 0,
            pc: 0,
            stack_top: 0,
            message: [0u8; 64],
            message_len: 0,
        }
    }

    /// Capture a snapshot of registers via volatile writes.
    ///
    /// # Safety
    /// This method is unsafe because it performs raw volatile writes to the fields of this struct,
    /// which may reside in shared memory accessed across CPU cores.
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    #[allow(clippy::needless_range_loop)]
    pub unsafe fn capture_snapshot(&mut self, stack_top: u32, message: &[u8; 64], message_len: u8) {
        let r0: u32;
        let r1: u32;
        let r2: u32;
        let r3: u32;
        let sp: u32;
        let lr: u32;
        let pc: u32;
        core::arch::asm!("mov {}, r0", out(reg) r0);
        core::arch::asm!("mov {}, r1", out(reg) r1);
        core::arch::asm!("mov {}, r2", out(reg) r2);
        core::arch::asm!("mov {}, r3", out(reg) r3);
        core::arch::asm!("mov {}, sp", out(reg) sp);
        core::arch::asm!("mov {}, lr", out(reg) lr);
        core::arch::asm!("mov {}, pc", out(reg) pc);

        let self_ptr = self as *mut Self;
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).r0), r0);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).r1), r1);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).r2), r2);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).r3), r3);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).sp), sp);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).lr), lr);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).pc), pc);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).stack_top), stack_top);
        for i in 0..64 {
            core::ptr::write_volatile(core::ptr::addr_of_mut!((*self_ptr).message[i]), message[i]);
        }
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!((*self_ptr).message_len),
            message_len,
        );
    }

    /// Capture a snapshot of registers via volatile writes (host mock).
    ///
    /// # Safety
    /// This is a mock function on host and is always safe to call.
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    pub unsafe fn capture_snapshot(
        &mut self,
        _stack_top: u32,
        _message: &[u8; 64],
        _message_len: u8,
    ) {
    }

    /// Read the register state using volatile reads.
    ///
    /// # Safety
    /// This method is unsafe because it reads from raw pointers pointing to the fields of this struct,
    /// which may be concurrently written to by other CPU cores.
    #[allow(clippy::needless_range_loop)]
    pub unsafe fn volatile_clone(&self) -> Self {
        let self_ptr = self as *const Self;
        let mut msg_buf = [0u8; 64];
        for i in 0..64 {
            msg_buf[i] = core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).message[i]));
        }
        let message_len = core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).message_len));

        Self {
            panicked: core::sync::atomic::AtomicBool::new(
                self.panicked.load(core::sync::atomic::Ordering::Acquire),
            ),
            r0: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).r0)),
            r1: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).r1)),
            r2: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).r2)),
            r3: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).r3)),
            sp: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).sp)),
            lr: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).lr)),
            pc: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).pc)),
            stack_top: core::ptr::read_volatile(core::ptr::addr_of!((*self_ptr).stack_top)),
            message: msg_buf,
            message_len,
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl CorePanicState {
    /// Create a new panic state by capturing the current registers.
    pub fn new(info: &core::panic::PanicInfo) -> Self {
        let mut r0: u32;
        let mut r1: u32;
        let mut r2: u32;
        let mut r3: u32;
        let sp: u32;
        let lr: u32;

        unsafe {
            core::arch::asm!(
                "mov {0}, r0",
                "mov {1}, r1",
                "mov {2}, r2",
                "mov {3}, r3",
                out(reg) r0,
                out(reg) r1,
                out(reg) r2,
                out(reg) r3,
            );
            core::arch::asm!("mov {}, sp", out(reg) sp);
            core::arch::asm!("mov {}, lr", out(reg) lr);
        }

        let mut msg_buf = [0u8; 64];
        let mut writer = BufferWriter {
            buf: &mut msg_buf,
            pos: 0,
        };
        let _ = core::fmt::write(&mut writer, format_args!("{}", info.message()));
        let message_len = writer.pos as u8;

        Self {
            panicked: core::sync::atomic::AtomicBool::new(false),
            r0,
            r1,
            r2,
            r3,
            sp,
            lr,
            pc: 0,
            stack_top: 0,
            message: msg_buf,
            message_len,
        }
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
    /// Maximum number of rolling crash logs (modulo limit)
    pub max_crash_logs: u32,
}

/// Result of a battery update state transition.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct BatteryTransitionResult {
    /// The new battery critical flag value.
    pub new_battery_critical: bool,
    /// The next system status if a transition occurred.
    pub next_status: Option<model::types::SystemStatus>,
}

/// Context info containing state-of-charge measurements.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct BatteryUpdateInfo {
    /// Percentage integer (0-100)
    pub state_of_charge: u8,
    /// Is the charger connected?
    pub charging: bool,
    /// Is there a fault?
    pub is_fault: bool,
}

/// Threshold values for battery safety transitions.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct BatteryThresholds {
    /// Critical SOC percentage limit
    pub critical_threshold: u8,
    /// Recovery hysteresis value
    pub hysteresis: u8,
}

/// Structure representing a serialized crash dump in CBOR.
/// Structure representing a serialized core dump in CBOR.
#[derive(Clone, Copy, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct CoreDump {
    /// Register R0
    #[n(0)]
    pub r0: u32,
    /// Register R1
    #[n(1)]
    pub r1: u32,
    /// Register R2
    #[n(2)]
    pub r2: u32,
    /// Register R3
    #[n(3)]
    pub r3: u32,
    /// Register SP
    #[n(4)]
    pub sp: u32,
    /// Register LR
    #[n(5)]
    pub lr: u32,
    /// Register PC
    #[n(6)]
    pub pc: u32,
    /// Backtrace program counters
    #[n(7)]
    pub backtrace: [u32; 32],
    /// Number of valid entries in the backtrace array
    #[n(8)]
    pub backtrace_len: u32,
    /// Whether this core panicked
    #[n(9)]
    pub panicked: bool,
}

impl Default for CoreDump {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreDump {
    /// Create a new, empty CoreDump.
    pub const fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            sp: 0,
            lr: 0,
            pc: 0,
            backtrace: [0u32; 32],
            backtrace_len: 0,
            panicked: false,
        }
    }
}

/// Structure representing a serialized crash dump in CBOR.
#[derive(Clone, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct CrashDump<'a> {
    /// Git revision hash of the firmware
    #[n(0)]
    pub revision_hash: &'a str,
    /// Circular log buffer raw bytes
    #[cbor(with = "minicbor::bytes")]
    #[n(1)]
    pub system_logs: &'a [u8],
    /// Unique identifier (UUID) for this crash dump
    #[cbor(with = "minicbor::bytes")]
    #[n(2)]
    pub uuid: [u8; 16],
    /// Core dumps for all cores
    #[n(3)]
    pub cores: [CoreDump; 2],
}

/// Project metadata struct embedded in the ELF to allow autodetecting chip/partition layout.
#[derive(Clone, Encode, Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub struct InvalidBootTrapMask;

/// A type-safe mutable bitmask vector representing active boot traps.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub struct BootTrapMask(u32);

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum ThermalUpdateAction {
    /// Clear the thermal boot trap.
    ClearBootTrap,
    /// Alert triggered due to critical temperature.
    AlertTriggered,
}

/// Result of a thermal update transition.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThermalTransitionResult {
    /// The next system status if a transition occurred.
    pub next_status: Option<model::types::SystemStatus>,
    /// Whether the transition requires clearing wake locks.
    pub clear_wake_locks: bool,
}

/// Processor Core Identifier.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum CpuId {
    /// Core 0 (Primary)
    Core0 = 0,
    /// Core 1 (Secondary)
    #[cfg(feature = "dual-core")]
    Core1 = 1,
}

impl CpuId {
    /// Convert a raw usize to CpuId if valid.
    pub const fn from_usize(val: usize) -> Option<Self> {
        match val {
            0 => Some(CpuId::Core0),
            #[cfg(feature = "dual-core")]
            1 => Some(CpuId::Core1),
            _ => None,
        }
    }
}

/// Trait representing execution and stall monitoring capabilities for a core.
pub trait CoreMonitor {
    /// Perform the liveness check.
    fn check_liveness(&self, cpu_id: CpuId);
    /// Retrieve the last executor progress timestamp.
    fn last_progress(&self) -> u32;
    /// Update the executor progress timestamp.
    fn update_progress(&self, now_ms: u32);
    /// Check if stuck task detection is enabled.
    fn is_enabled(&self) -> bool;
    /// Check if stuck task has been detected.
    fn is_stuck(&self) -> bool;
    /// Check if the core has panicked.
    fn is_panicked(&self) -> bool;
    /// Set the panicked flag.
    fn set_panicked(&self, panicked: bool);
    /// Retrieve the CPU ID configured for this core.
    fn cpuid(&self) -> CpuId;
}

/// Health and execution status tracking for a CPU core.
pub struct CoreStatus {
    /// The CPU ID assigned to this core status.
    pub cpu_id: CpuId,
    /// Keeps track of the last time the executor made progress.
    pub last_executor_progress: core::sync::atomic::AtomicU32,
    /// Controls whether stuck task detection is enabled.
    pub stuck_detection_enabled: core::sync::atomic::AtomicBool,
    /// Tracks if a stuck task has been detected.
    pub stuck_detected: core::sync::atomic::AtomicBool,
    /// Flag indicating if the core has panicked.
    pub panicked: core::sync::atomic::AtomicBool,
    /// Stuck task detection timeout in milliseconds.
    pub timeout_ms: core::sync::atomic::AtomicU32,
    /// Warning threshold percentage.
    pub warn_threshold_pct: core::sync::atomic::AtomicU32,
    /// Last warning timestamp.
    pub last_warn_time_ms: core::sync::atomic::AtomicU32,
    /// Cached system clock frequency in Hz.
    pub sys_clock_hz: core::sync::atomic::AtomicU32,
    /// Tracks if host mock monitor thread is spawned for this core.
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    pub mock_thread_spawned: core::sync::atomic::AtomicBool,
}

impl CoreStatus {
    /// Create a new CoreStatus initialized with a specific CPU ID.
    pub const fn new(cpu_id: CpuId) -> Self {
        Self {
            cpu_id,
            last_executor_progress: core::sync::atomic::AtomicU32::new(0),
            stuck_detection_enabled: core::sync::atomic::AtomicBool::new(true),
            stuck_detected: core::sync::atomic::AtomicBool::new(false),
            panicked: core::sync::atomic::AtomicBool::new(false),
            timeout_ms: core::sync::atomic::AtomicU32::new(10_000),
            warn_threshold_pct: core::sync::atomic::AtomicU32::new(80),
            last_warn_time_ms: core::sync::atomic::AtomicU32::new(0),
            sys_clock_hz: core::sync::atomic::AtomicU32::new(125_000_000),
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            mock_thread_spawned: core::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Create a new empty CoreStatus.
    pub const fn new_empty() -> Self {
        Self::new(CpuId::Core0)
    }
}
