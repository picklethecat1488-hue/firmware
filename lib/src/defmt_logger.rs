//! Custom defmt logger that writes to both RTT and a circular memory log buffer.

#![allow(static_mut_refs)]

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

const BUF_SIZE: usize = 1024;
const MODE_MASK: usize = 0b11;
const MODE_BLOCK_IF_FULL: usize = 2;
const MODE_NON_BLOCKING_TRIM: usize = 1;

#[repr(C)]
struct Channel {
    name: *const u8,
    buffer: *mut u8,
    size: usize,
    write: AtomicUsize,
    read: AtomicUsize,
    flags: AtomicUsize,
}

impl Channel {
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

    fn blocking_write(&self, bytes: &[u8]) -> usize {
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
        self.write_impl(bytes, write, BUF_SIZE)
    }

    fn write_impl(&self, bytes: &[u8], cursor: usize, available: usize) -> usize {
        let len = bytes.len().min(available);
        unsafe {
            if cursor + len > BUF_SIZE {
                let pivot = BUF_SIZE - cursor;
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.add(cursor), pivot);
                core::ptr::copy_nonoverlapping(bytes.as_ptr().add(pivot), self.buffer, len - pivot);
            } else {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.buffer.add(cursor), len);
            }
        }
        self.write
            .store(cursor.wrapping_add(len) % BUF_SIZE, Ordering::Release);
        len
    }

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
            BUF_SIZE - write_cursor - 1
        } else {
            BUF_SIZE - write_cursor
        }
    }
}

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

struct RttEncoder {
    taken: AtomicBool,
    cs_restore: UnsafeCell<critical_section::RestoreState>,
    encoder: UnsafeCell<defmt::Encoder>,
}

impl RttEncoder {
    const fn new() -> RttEncoder {
        RttEncoder {
            taken: AtomicBool::new(false),
            cs_restore: UnsafeCell::new(critical_section::RestoreState::invalid()),
            encoder: UnsafeCell::new(defmt::Encoder::new()),
        }
    }

    fn acquire(&self) {
        let restore = unsafe { critical_section::acquire() };
        if self.taken.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }
        self.taken.store(true, Ordering::Relaxed);
        unsafe {
            self.cs_restore.get().write(restore);
            let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
            encoder.start_frame(|b| {
                write_encoded(b);
            });
        }
    }

    unsafe fn write(&self, bytes: &[u8]) {
        let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
        encoder.write(bytes, |b| {
            write_encoded(b);
        });
    }

    unsafe fn flush(&self) {
        _SEGGER_RTT.up_channel.flush();
    }

    unsafe fn release(&self) {
        if !self.taken.load(Ordering::Relaxed) {
            panic!("defmt release out of context")
        }
        let encoder: &mut defmt::Encoder = &mut *self.encoder.get();
        encoder.end_frame(|b| {
            write_encoded(b);
        });
        let restore = self.cs_restore.get().read();
        self.taken.store(false, Ordering::Relaxed);
        critical_section::release(restore);
    }
}

unsafe impl Sync for RttEncoder {}

static RTT_ENCODER: RttEncoder = RttEncoder::new();

fn write_encoded(bytes: &[u8]) {
    unsafe {
        _SEGGER_RTT.up_channel.write_all(bytes);
    }
    critical_section::with(|cs| {
        let mut buffer = crate::panic_handler::CRASH_LOG_BUFFER
            .borrow(cs)
            .borrow_mut();
        buffer.write_bytes(bytes);
    });
}

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
