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

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn write_rtt_raw(bytes: &[u8]) {
    _SEGGER_RTT.up_channel.write_all(bytes);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub(crate) unsafe fn flush_rtt_raw() {
    _SEGGER_RTT.up_channel.flush();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
const BUF_SIZE: usize = 1024;

#[cfg(all(target_arch = "arm", target_os = "none"))]
const MODE_NON_BLOCKING_TRIM: usize = 1;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[repr(C)]
struct Header {
    id: [u8; 16],
    max_up_channels: usize,
    max_down_channels: usize,
    up_channel: crate::defmt_logger::Channel,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
unsafe impl Sync for Header {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[no_mangle]
static mut _SEGGER_RTT: Header = Header {
    id: *b"SEGGER RTT\0\0\0\0\0\0",
    max_up_channels: 1,
    max_down_channels: 0,
    up_channel: crate::defmt_logger::Channel {
        name: NAME.as_ptr(),
        buffer: unsafe { BUFFER.as_mut_ptr() },
        size: BUF_SIZE,
        write: core::sync::atomic::AtomicUsize::new(0),
        read: core::sync::atomic::AtomicUsize::new(0),
        flags: core::sync::atomic::AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
    },
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BUFFER: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

#[cfg(all(target_arch = "arm", target_os = "none"))]
static NAME: [u8; 6] = *b"defmt\0";

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
