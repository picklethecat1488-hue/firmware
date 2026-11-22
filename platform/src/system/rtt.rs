//! SWD Real-Time Transfer (RTT) logging backend for defmt.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use crate::defmt_logger::DefmtLogWriter;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use crate::defmt_logger::{RttProtocol, RttWriter};

/// A log writer that outputs to the SWD RTT channel.
pub struct RttLogWriter;

impl DefmtLogWriter for RttLogWriter {
    fn write_all(&self, _bytes: &[u8]) {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        unsafe {
            write_rtt_raw(_bytes);
        }
    }
    fn flush(&self) {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        unsafe {
            flush_rtt_raw();
        }
    }
}

/// Helper adapter to write to RTT and expose embedded-io::Write.
pub struct RttTxWriter;

impl embedded_io::ErrorType for RttTxWriter {
    type Error = core::convert::Infallible;
}

impl embedded_io::Write for RttTxWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        unsafe {
            write_rtt_cli(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        unsafe {
            flush_rtt_cli();
        }
        Ok(())
    }
}

/// Reads bytes from the RTT down channel.
/// Returns the number of bytes read.
pub fn read_rtt(buf: &mut [u8]) -> usize {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe {
        _SEGGER_RTT.down_channels[0].read(buf)
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        let _ = buf;
        0
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn write_rtt_raw(bytes: &[u8]) {
    _SEGGER_RTT.up_channels[0].write_all(bytes);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn flush_rtt_raw() {
    _SEGGER_RTT.up_channels[0].flush();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn write_rtt_cli(bytes: &[u8]) {
    _SEGGER_RTT.up_channels[1].write_all(bytes);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn flush_rtt_cli() {
    _SEGGER_RTT.up_channels[1].flush();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
const BUF_SIZE: usize = 32768;

#[cfg(all(target_arch = "arm", target_os = "none"))]
const CLI_BUF_SIZE: usize = 4096;

#[cfg(all(target_arch = "arm", target_os = "none"))]
const CLI_DOWN_BUF_SIZE: usize = 256;

#[cfg(all(target_arch = "arm", target_os = "none"))]
const MODE_NON_BLOCKING_TRIM: usize = 1;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[repr(C)]
struct Header {
    id: [u8; 16],
    max_up_channels: usize,
    max_down_channels: usize,
    up_channels: [crate::defmt_logger::Channel; 2],
    down_channels: [crate::defmt_logger::Channel; 1],
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
unsafe impl Sync for Header {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[no_mangle]
#[used]
static mut _SEGGER_RTT: Header = Header {
    id: *b"SEGGER RTT\0\0\0\0\0\0",
    max_up_channels: 2,
    max_down_channels: 1,
    up_channels: [
        crate::defmt_logger::Channel {
            name: NAME.as_ptr(),
            buffer: unsafe { BUFFER.as_mut_ptr() },
            size: BUF_SIZE,
            write: core::sync::atomic::AtomicUsize::new(0),
            read: core::sync::atomic::AtomicUsize::new(0),
            flags: core::sync::atomic::AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
        },
        crate::defmt_logger::Channel {
            name: CLI_UP_NAME.as_ptr(),
            buffer: unsafe { CLI_UP_BUFFER.as_mut_ptr() },
            size: CLI_BUF_SIZE,
            write: core::sync::atomic::AtomicUsize::new(0),
            read: core::sync::atomic::AtomicUsize::new(0),
            flags: core::sync::atomic::AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
        },
    ],
    down_channels: [crate::defmt_logger::Channel {
        name: CLI_DOWN_NAME.as_ptr(),
        buffer: unsafe { CLI_DOWN_BUFFER.as_mut_ptr() },
        size: CLI_DOWN_BUF_SIZE,
        write: core::sync::atomic::AtomicUsize::new(0),
        read: core::sync::atomic::AtomicUsize::new(0),
        flags: core::sync::atomic::AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
    }],
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BUFFER: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

#[cfg(all(target_arch = "arm", target_os = "none"))]
static NAME: [u8; 6] = *b"defmt\0";

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut CLI_UP_BUFFER: [u8; CLI_BUF_SIZE] = [0u8; CLI_BUF_SIZE];

#[cfg(all(target_arch = "arm", target_os = "none"))]
static CLI_UP_NAME: [u8; 4] = *b"cli\0";

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut CLI_DOWN_BUFFER: [u8; CLI_DOWN_BUF_SIZE] = [0u8; CLI_DOWN_BUF_SIZE];

#[cfg(all(target_arch = "arm", target_os = "none"))]
static CLI_DOWN_NAME: [u8; 4] = *b"cli\0";

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct TargetRttWriter;

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl RttWriter for TargetRttWriter {
    fn write_all(&self, bytes: &[u8]) {
        unsafe {
            if let Some(writer) = crate::defmt_logger::ACTIVE_WRITER {
                writer.write_all(bytes);
            }
        }
    }
    fn flush(&self) {
        unsafe {
            if let Some(writer) = crate::defmt_logger::ACTIVE_WRITER {
                writer.flush();
            }
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static RTT_ENCODER: RttProtocol<TargetRttWriter> = RttProtocol::new(TargetRttWriter);

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[defmt::global_logger]
struct Logger;

#[cfg(all(target_arch = "arm", target_os = "none"))]
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        RTT_ENCODER.acquire();
    }
    unsafe fn write(bytes: &[u8]) {
        RTT_ENCODER.write(bytes);
    }
    unsafe fn flush() {
        RTT_ENCODER.flush();
    }
    unsafe fn release() {
        RTT_ENCODER.release();
    }
}

/// Global signal to notify the CLI runloop that RTT input is available.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub static RTT_SIGNAL: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    (),
> = embassy_sync::signal::Signal::new();

/// Bit flag in `_SEGGER_RTT.down_channels[0].flags` asserted when the target shell loop is idle.
///
/// Bits `0..=1` are reserved by SEGGER RTT for channel buffer mode (`0..=2`); upper bits are
/// ignored by SEGGER RTT readers and used for lock-free single-writer (MCU) / single-reader (host)
/// boot and shell signaling without polling console text or guessing binary names.
pub const RTT_SHELL_IDLE_FLAG: u32 = 1 << 8;

/// Bit shift of the 2-bit [`TargetBootState`] field in `_SEGGER_RTT.down_channels[0].flags` (`bits 9..=10`).
pub const RTT_BOOT_STATE_SHIFT: u32 = 9;

/// Bitmask isolating the 2-bit [`TargetBootState`] field in `_SEGGER_RTT.down_channels[0].flags`.
pub const RTT_BOOT_STATE_MASK: u32 = 0x3 << RTT_BOOT_STATE_SHIFT;

/// Bit shift of the 16-bit wrapping idle-transition epoch in `_SEGGER_RTT.down_channels[0].flags`.
pub const RTT_SHELL_EPOCH_SHIFT: u32 = 16;

/// Mask preserving SEGGER RTT channel mode bits (`0..=1`).
pub const RTT_CHANNEL_MODE_MASK: u32 = 0x3;

/// Lifecycle boot status published by every target firmware binary in `_SEGGER_RTT.down_channels[0].flags`.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[repr(u8)]
pub enum TargetBootState {
    /// Power-on or hardware reset prior to `Board::init`.
    Reset = 0,
    /// Target bootstrap task is initializing peripherals and domain controllers.
    Booting = 1,
    /// Target boot completed and main executor / runtime tasks are active.
    Ready = 2,
    /// Unrecoverable panic or fatal fault occurred.
    Faulted = 3,
}

impl TargetBootState {
    /// Convert a raw 2-bit code (`0..=3`) into a [`TargetBootState`].
    pub const fn from_bits(bits: u32) -> Self {
        match bits & 0x3 {
            1 => Self::Booting,
            2 => Self::Ready,
            3 => Self::Faulted,
            _ => Self::Reset,
        }
    }

    /// Canonical lowercase status token serialized in `host_cli boot-status` and NDJSON events.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Reset => "reset",
            Self::Booting => "booting",
            Self::Ready => "ready",
            Self::Faulted => "faulted",
        }
    }

    /// Return `true` once the firmware binary has reached [`TargetBootState::Ready`].
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Encode RTT `down_channels[0].flags` combining channel `mode`, `boot_state`, `idle` state, and wrapping `epoch`.
pub const fn encode_rtt_status_flags(
    mode: u32,
    boot_state: TargetBootState,
    idle: bool,
    epoch: u16,
) -> u32 {
    let base = mode & RTT_CHANNEL_MODE_MASK;
    let boot_bits = ((boot_state as u32) & 0x3) << RTT_BOOT_STATE_SHIFT;
    let idle_bit = if idle { RTT_SHELL_IDLE_FLAG } else { 0 };
    base | boot_bits | idle_bit | ((epoch as u32) << RTT_SHELL_EPOCH_SHIFT)
}

/// Encode RTT `down_channels[0].flags` combining channel `mode`, `idle` state, and wrapping `epoch` (with `Ready` when `idle` is true).
pub const fn encode_rtt_shell_flags(mode: u32, idle: bool, epoch: u16) -> u32 {
    let boot_state = if idle {
        TargetBootState::Ready
    } else {
        TargetBootState::Reset
    };
    encode_rtt_status_flags(mode, boot_state, idle, epoch)
}

/// Decode [`TargetBootState`] from a 32-bit RTT `down_channels[0].flags` word.
pub const fn decode_rtt_boot_state(flags: u32) -> TargetBootState {
    TargetBootState::from_bits((flags & RTT_BOOT_STATE_MASK) >> RTT_BOOT_STATE_SHIFT)
}

/// Decode `(is_idle, epoch)` from a 32-bit RTT `down_channels[0].flags` word.
pub const fn decode_rtt_shell_flags(flags: u32) -> (bool, u16) {
    let idle = (flags & RTT_SHELL_IDLE_FLAG) != 0;
    let epoch = (flags >> RTT_SHELL_EPOCH_SHIFT) as u16;
    (idle, epoch)
}

/// Update the RTT control block's [`TargetBootState`] while preserving channel mode, shell-idle bit, and epoch.
pub fn set_rtt_boot_state(state: TargetBootState) {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe {
        let prev = _SEGGER_RTT.down_channels[0]
            .flags
            .load(core::sync::atomic::Ordering::Relaxed) as u32;
        let mode = prev & RTT_CHANNEL_MODE_MASK;
        let (was_idle, prev_epoch) = decode_rtt_shell_flags(prev);
        let next_idle = if matches!(state, TargetBootState::Faulted) {
            false
        } else {
            was_idle
        };
        let next_flags = encode_rtt_status_flags(mode, state, next_idle, prev_epoch) as usize;
        _SEGGER_RTT.down_channels[0]
            .flags
            .store(next_flags, core::sync::atomic::Ordering::Release);
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        let _ = state;
    }
}

/// Update the RTT control block's shell-idle flag (and advance its epoch when transitioning to idle).
pub fn set_rtt_shell_idle(idle: bool) {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe {
        let prev = _SEGGER_RTT.down_channels[0]
            .flags
            .load(core::sync::atomic::Ordering::Relaxed) as u32;
        let mode = prev & RTT_CHANNEL_MODE_MASK;
        let prev_boot = decode_rtt_boot_state(prev);
        let next_boot = if matches!(prev_boot, TargetBootState::Faulted) {
            TargetBootState::Faulted
        } else if idle {
            TargetBootState::Ready
        } else {
            prev_boot
        };
        let (was_idle, prev_epoch) = decode_rtt_shell_flags(prev);
        let next_epoch = if idle && !was_idle {
            prev_epoch.wrapping_add(1)
        } else {
            prev_epoch
        };
        let next_flags = encode_rtt_status_flags(mode, next_boot, idle, next_epoch) as usize;
        _SEGGER_RTT.down_channels[0]
            .flags
            .store(next_flags, core::sync::atomic::Ordering::Release);
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        let _ = idle;
    }
}

/// Check if the RTT down channel has unread data.
pub fn rtt_has_input() -> bool {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe {
        let write = _SEGGER_RTT.down_channels[0]
            .write
            .load(core::sync::atomic::Ordering::Acquire);
        let read = _SEGGER_RTT.down_channels[0]
            .read
            .load(core::sync::atomic::Ordering::Relaxed);
        read != write
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    false
}

/// Helper macro to execute the RTT character receive loop,
/// routing bytes into the embedded-cli processor, and automatically executing any
/// pending async commands while signaling idle state via `RTT_SHELL_IDLE_FLAG`.
#[macro_export]
macro_rules! run_rtt_shell_loop {
    ($cli:expr, $proc:expr, $cmd_type:ty, $prompt:expr) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        loop {
            if !$crate::rtt::rtt_has_input() {
                $crate::rtt::set_rtt_shell_idle(true);
                $crate::rtt::RTT_SIGNAL.wait().await;
            }
            $crate::rtt::RTT_SIGNAL.reset();
            $crate::rtt::set_rtt_shell_idle(false);

            let mut rx_byte = [0u8; 1];
            while $crate::rtt::read_rtt(&mut rx_byte) > 0 {
                let _ = $cli.process_byte::<$cmd_type, _>(rx_byte[0], $proc);
            }
            if $proc.controller.has_pending_command() {
                let mut raw_writer = $crate::rtt::RttTxWriter;
                let mut writer = ::embedded_cli::writer::Writer::new(&mut raw_writer);
                if let Err(err) = $proc.controller.execute_pending(&mut writer).await {
                    let _ = $cli.set_prompt($prompt);
                    let _ = $cli.write(|cli_writer| {
                        use core::fmt::Write as _;
                        let _ = core::writeln!(cli_writer, "Command failed: {}", err);
                        Ok::<(), core::convert::Infallible>(())
                    });
                } else {
                    let _ = $cli.set_prompt($prompt);
                }
            }
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        panic!("run_rtt_shell_loop is not supported on host");
    };
}
