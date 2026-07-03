//! Custom defmt logger that writes to both RTT and a circular memory log buffer.

#![allow(static_mut_refs)]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

const BUF_SIZE: usize = 1024;
const MODE_MASK: usize = 0b11;

/// Flag indicating block-if-full RTT write mode.
pub const MODE_BLOCK_IF_FULL: usize = 2;
/// Flag indicating non-blocking ring-buffer RTT write mode.
pub const MODE_NON_BLOCKING_TRIM: usize = 1;

/// A single RTT channel.
#[repr(C)]
pub struct Channel {
    /// Pointer to the name of the channel.
    pub name: *const u8,
    /// Pointer to the underlying ring buffer.
    pub buffer: *mut u8,
    /// Capacity of the buffer in bytes.
    pub size: usize,
    /// The write index cursor.
    pub write: AtomicUsize,
    /// The read index cursor.
    pub read: AtomicUsize,
    /// The control flags for blocking or non-blocking modes.
    pub flags: AtomicUsize,
}

impl Channel {
    /// Writes all the bytes to the RTT channel.
    pub fn write_all(&self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let consumed = if self.host_is_connected() {
                self.blocking_write(bytes)
            } else {
                self.nonblocking_write(bytes)
            };
            if consumed != 0 {
                bytes = &bytes[consumed..];
            }
        }
    }

    /// Performs a blocking write of bytes.
    pub fn blocking_write(&self, bytes: &[u8]) -> usize {
        if bytes.is_empty() {
            return 0;
        }
        let read = self.read.load(Ordering::Relaxed);
        let write = self.write.load(Ordering::Acquire);
        let available = self.available_buffer_size(read, write);
        if available == 0 {
            return 0;
        }
        self.write_impl(bytes, write, available)
    }

    fn nonblocking_write(&self, bytes: &[u8]) -> usize {
        let write = self.write.load(Ordering::Acquire);
        self.write_impl(bytes, write, self.size)
    }

    fn write_impl(&self, bytes: &[u8], cursor: usize, available: usize) -> usize {
        let len = bytes.len().min(available);
        unsafe {
            if cursor + len > self.size {
                let pivot = self.size - cursor;
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.add(cursor), pivot);
                core::ptr::copy_nonoverlapping(bytes.as_ptr().add(pivot), self.buffer, len - pivot);
            } else {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.add(cursor), len);
            }
        }
        self.write
            .store(cursor.wrapping_add(len) % self.size, Ordering::Release);
        len
    }

    /// Flushes the RTT channel.
    pub fn flush(&self) {
        if !self.host_is_connected() {
            return;
        }
        let read = || self.read.load(Ordering::Relaxed);
        let write = || self.write.load(Ordering::Relaxed);
        while read() != write() {}
    }

    fn host_is_connected(&self) -> bool {
        self.flags.load(Ordering::Relaxed) & MODE_MASK == MODE_BLOCK_IF_FULL
    }

    fn available_buffer_size(&self, read_cursor: usize, write_cursor: usize) -> usize {
        if read_cursor > write_cursor {
            read_cursor - write_cursor - 1
        } else if read_cursor == 0 {
            self.size - write_cursor - 1
        } else {
            self.size - write_cursor
        }
    }
}

/// Trait representing an RTT writing backend.
pub trait RttWriter {
    /// Writes all bytes to the backend.
    fn write_all(&self, bytes: &[u8]);
    /// Flushes any pending writes to the backend.
    fn flush(&self);
}

/// A generic RTT protocol handler that manages frame encoding, reentrancy guards,
/// and dual-logging to both the writer and the circular crash log buffer.
pub struct RttProtocol<W: RttWriter> {
    /// The writing backend.
    pub writer: W,
    /// Guard to protect against reentrant logging calls.
    pub taken: core::sync::atomic::AtomicBool,
    /// Restore state for the critical section lock.
    pub cs_restore: core::cell::UnsafeCell<critical_section::RestoreState>,
    /// Underlying defmt frame encoder.
    pub encoder: core::cell::UnsafeCell<defmt::Encoder>,
}

unsafe impl<W: RttWriter> Sync for RttProtocol<W> {}

impl<W: RttWriter> RttProtocol<W> {
    /// Creates a new RttProtocol instance wrapping the specified writer.
    pub const fn new(writer: W) -> Self {
        Self {
            writer,
            taken: core::sync::atomic::AtomicBool::new(false),
            cs_restore: core::cell::UnsafeCell::new(critical_section::RestoreState::invalid()),
            encoder: core::cell::UnsafeCell::new(defmt::Encoder::new()),
        }
    }

    /// Acquires the logger, panicking if called reentrantly.
    pub fn acquire(&self) {
        let restore = unsafe { critical_section::acquire() };
        if self.taken.load(core::sync::atomic::Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }
        self.taken
            .store(true, core::sync::atomic::Ordering::Relaxed);
        unsafe {
            self.cs_restore.get().write(restore);
            let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
            encoder.start_frame(|b| {
                self.write_encoded(b);
            });
        }
    }

    /// Encodes and writes target log payload bytes.
    ///
    /// # Safety
    /// Must only be called after acquiring the logger.
    pub unsafe fn write(&self, bytes: &[u8]) {
        let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
        encoder.write(bytes, |b| {
            self.write_encoded(b);
        });
    }

    /// Flushes the RTT channel.
    ///
    /// # Safety
    /// Must only be called after acquiring the logger.
    pub unsafe fn flush(&self) {
        self.writer.flush();
    }

    /// Finalizes the defmt frame and releases the logger.
    ///
    /// # Safety
    /// Must only be called after acquiring the logger.
    pub unsafe fn release(&self) {
        if !self.taken.load(core::sync::atomic::Ordering::Relaxed) {
            panic!("defmt release out of context")
        }
        let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
        encoder.end_frame(|b| {
            self.write_encoded(b);
        });
        let restore = self.cs_restore.get().read();
        self.taken
            .store(false, core::sync::atomic::Ordering::Relaxed);
        critical_section::release(restore);
    }

    fn write_encoded(&self, bytes: &[u8]) {
        self.writer.write_all(bytes);
        critical_section::with(|cs| {
            let mut buffer = crate::panic_handler::CRASH_LOG_BUFFER
                .borrow(cs)
                .borrow_mut();
            buffer.write_bytes(bytes);
        });
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod target_logger {
    use super::*;

    #[repr(C)]
    struct Header {
        id: [u8; 16],
        max_up_channels: usize,
        max_down_channels: usize,
        up_channel: Channel,
    }

    unsafe impl Sync for Header {}

    #[no_mangle]
    static mut _SEGGER_RTT: Header = Header {
        id: *b"SEGGER RTT\0\0\0\0\0\0",
        max_up_channels: 1,
        max_down_channels: 0,
        up_channel: Channel {
            name: NAME.as_ptr(),
            buffer: unsafe { BUFFER.as_mut_ptr() },
            size: BUF_SIZE,
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
            flags: AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
        },
    };

    static mut BUFFER: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

    static NAME: [u8; 6] = *b"defmt\0";

    struct TargetRttWriter;

    impl RttWriter for TargetRttWriter {
        fn write_all(&self, bytes: &[u8]) {
            unsafe {
                _SEGGER_RTT.up_channel.write_all(bytes);
            }
        }
        fn flush(&self) {
            unsafe {
                _SEGGER_RTT.up_channel.flush();
            }
        }
    }

    static RTT_ENCODER: RttProtocol<TargetRttWriter> = RttProtocol::new(TargetRttWriter);

    #[defmt::global_logger]
    struct Logger;

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
}
