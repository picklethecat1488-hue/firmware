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
const BUF_SIZE: usize = 1024;

#[cfg(all(target_arch = "arm", target_os = "none"))]
const CLI_BUF_SIZE: usize = 1024;

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

/// Helper function to execute the non-blocking RTT character receive loop,
/// routing bytes into the embedded-cli processor.
pub fn run_rtt_shell_loop<
    C: embedded_cli::service::Autocomplete + embedded_cli::service::Help,
    P: embedded_cli::service::CommandProcessor<RttTxWriter, core::convert::Infallible>,
    B1: embedded_cli::buffer::Buffer,
    B2: embedded_cli::buffer::Buffer,
>(
    _cli: &mut embedded_cli::cli::Cli<RttTxWriter, core::convert::Infallible, B1, B2>,
    _processor: &mut P,
) -> ! {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    loop {
        let mut rx_byte = [0u8; 1];
        if read_rtt(&mut rx_byte) > 0 {
            let _ = _cli.process_byte::<C, _>(rx_byte[0], _processor);
        } else {
            cortex_m::asm::delay(10_000);
        }
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    panic!("run_rtt_shell_loop is not supported on host");
}
