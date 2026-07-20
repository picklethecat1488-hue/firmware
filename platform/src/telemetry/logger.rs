//! Custom defmt logger that writes to both RTT and a circular memory log buffer.

#![allow(static_mut_refs)]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

const BUF_SIZE: usize = 1024;
const MODE_MASK: usize = 0b11;

/// Flag indicating block-if-full RTT write mode.
pub const MODE_BLOCK_IF_FULL: usize = 2;
/// Flag indicating non-blocking ring-buffer RTT write mode.
pub const MODE_NON_BLOCKING_TRIM: usize = 1;

/// Maximum duration in microseconds to block in write/flush mode before dropping logs (2 ms).
const MAX_WRITE_TIMEOUT_US: u64 = 2000;

/// Consecutive timeout counter to detect debugger detachment and fall back to non-blocking.
static CONSECUTIVE_TIMEOUTS: AtomicU32 = AtomicU32::new(0);

/// Maximum consecutive timeouts before assuming the debugger detached and falling back (200ms).
const MAX_TIMEOUTS: u32 = 100;

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
        let start = embassy_time::Instant::now();
        let mut timed_out = false;
        while !bytes.is_empty() {
            let consumed = if self.host_is_connected() {
                self.blocking_write(bytes)
            } else {
                self.nonblocking_write(bytes)
            };
            if consumed != 0 {
                bytes = &bytes[consumed..];
            } else if start.elapsed().as_micros() > MAX_WRITE_TIMEOUT_US {
                timed_out = true;
                break;
            }
        }

        if timed_out {
            let consecutive = CONSECUTIVE_TIMEOUTS.load(Ordering::Relaxed) + 1;
            CONSECUTIVE_TIMEOUTS.store(consecutive, Ordering::Relaxed);
            if consecutive > MAX_TIMEOUTS {
                // Debugger detached. Fall back to non-blocking mode on this channel.
                self.flags.store(MODE_NON_BLOCKING_TRIM, Ordering::Relaxed);
                CONSECUTIVE_TIMEOUTS.store(0, Ordering::Relaxed);
            }
        } else {
            CONSECUTIVE_TIMEOUTS.store(0, Ordering::Relaxed);
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

    /// Reads bytes from the RTT channel. Returns the number of bytes read.
    pub fn read(&self, bytes: &mut [u8]) -> usize {
        if bytes.is_empty() {
            return 0;
        }
        let write = self.write.load(Ordering::Acquire);
        let read = self.read.load(Ordering::Relaxed);
        if read == write {
            return 0;
        }

        let len = if write > read {
            write - read
        } else {
            self.size - read
        };
        let len = len.min(bytes.len());

        unsafe {
            core::ptr::copy_nonoverlapping(self.buffer.add(read), bytes.as_mut_ptr(), len);
        }
        self.read.store((read + len) % self.size, Ordering::Release);
        len
    }

    /// Flushes the RTT channel.
    pub fn flush(&self) {
        if !self.host_is_connected() {
            return;
        }
        let read = || self.read.load(Ordering::Relaxed);
        let write = || self.write.load(Ordering::Relaxed);
        let start = embassy_time::Instant::now();
        let mut timed_out = false;
        while read() != write() {
            if start.elapsed().as_micros() > MAX_WRITE_TIMEOUT_US {
                timed_out = true;
                break;
            }
        }

        if timed_out {
            let consecutive = CONSECUTIVE_TIMEOUTS.load(Ordering::Relaxed) + 1;
            CONSECUTIVE_TIMEOUTS.store(consecutive, Ordering::Relaxed);
            if consecutive > MAX_TIMEOUTS {
                // Debugger detached. Fall back to non-blocking mode on this channel.
                self.flags.store(MODE_NON_BLOCKING_TRIM, Ordering::Relaxed);
                CONSECUTIVE_TIMEOUTS.store(0, Ordering::Relaxed);
            }
        } else {
            CONSECUTIVE_TIMEOUTS.store(0, Ordering::Relaxed);
        }
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

/// Trait for a defmt log writer (RTT, UART, Combined, etc.)
pub trait DefmtLogWriter: Sync + Send {
    /// Writes log bytes to the backend.
    fn write_all(&self, bytes: &[u8]);
    /// Flushes the log backend.
    fn flush(&self) {}
}

/// Default instance of RttLogWriter.
pub static DEFAULT_RTT_WRITER: crate::rtt::RttLogWriter = crate::rtt::RttLogWriter;

pub static mut ACTIVE_WRITER: Option<&'static dyn DefmtLogWriter> = Some(&DEFAULT_RTT_WRITER);

/// Global manager for configuring the defmt logging destination.
pub struct DefmtLogger;

impl DefmtLogger {
    /// Sets the active global writer for defmt logging.
    pub fn set_writer(writer: &'static dyn DefmtLogWriter) {
        critical_section::with(|_| unsafe {
            ACTIVE_WRITER = Some(writer);
        });
    }

    /// Disables defmt logging.
    pub fn disable() {
        critical_section::with(|_| unsafe {
            ACTIVE_WRITER = None;
        });
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
    /// Buffer for accumulating the current frame bytes.
    pub frame_buf: core::cell::UnsafeCell<[u8; 256]>,
    /// Length of currently buffered frame bytes.
    pub frame_len: core::cell::Cell<usize>,
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
            frame_buf: core::cell::UnsafeCell::new([0u8; 256]),
            frame_len: core::cell::Cell::new(0),
        }
    }

    /// Acquires the logger, panicking if called reentrantly.
    pub fn acquire(&self) {
        let restore = unsafe { critical_section::acquire() };
        if self.taken.load(core::sync::atomic::Ordering::Relaxed) {
            unsafe {
                critical_section::release(restore);
            }
            panic!("defmt logger taken reentrantly");
        }
        self.taken
            .store(true, core::sync::atomic::Ordering::Relaxed);
        unsafe {
            self.cs_restore.get().write(restore);
            self.frame_len.set(0);
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

        // Write the completed contiguous frame to CRASH_LOG_BUFFER
        let cs = critical_section::CriticalSection::new();
        let mut buffer = crate::panic_handler::CRASH_LOG_BUFFER
            .borrow(cs)
            .borrow_mut();
        let f_len = self.frame_len.get();
        let f_ptr = self.frame_buf.get() as *const u8;
        let f_slice = core::slice::from_raw_parts(f_ptr, f_len);
        buffer.write_frame(f_slice);

        let restore = self.cs_restore.get().read();
        self.taken
            .store(false, core::sync::atomic::Ordering::Relaxed);
        critical_section::release(restore);
    }

    fn write_encoded(&self, bytes: &[u8]) {
        // Write to RTT immediately
        self.writer.write_all(bytes);

        // Accumulate in frame_buf for circular log writing
        let len = self.frame_len.get();
        let remaining = 256 - len;
        let to_copy = bytes.len().min(remaining);
        if to_copy > 0 {
            unsafe {
                let buf_ptr = self.frame_buf.get() as *mut u8;
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr.add(len), to_copy);
            }
            self.frame_len.set(len + to_copy);
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
defmt::timestamp!("{=u64:us}", embassy_time::Instant::now().as_micros());
