#![allow(static_mut_refs)]

use core::sync::atomic::{AtomicUsize, Ordering};
use platform::defmt_logger::{Channel, MODE_BLOCK_IF_FULL, MODE_NON_BLOCKING_TRIM};

#[test]
fn test_channel_write_wrapping() {
    let mut buf = [0u8; 16];
    let chan = Channel {
        name: core::ptr::null(),
        buffer: buf.as_mut_ptr(),
        size: 16,
        write: AtomicUsize::new(14),
        read: AtomicUsize::new(0),
        flags: AtomicUsize::new(MODE_NON_BLOCKING_TRIM),
    };

    // Write 4 bytes: should wrap and write 2 bytes at the end, 2 at the beginning
    chan.write_all(&[0xAA, 0xBB, 0xCC, 0xDD]);

    assert_eq!(chan.write.load(Ordering::Relaxed), 2);
    assert_eq!(buf[14], 0xAA);
    assert_eq!(buf[15], 0xBB);
    assert_eq!(buf[0], 0xCC);
    assert_eq!(buf[1], 0xDD);
}

#[test]
fn test_channel_blocking_write_full() {
    let mut buf = [0u8; 16];
    let chan = Channel {
        name: core::ptr::null(),
        buffer: buf.as_mut_ptr(),
        size: 16,
        write: AtomicUsize::new(0),
        read: AtomicUsize::new(0),
        flags: AtomicUsize::new(MODE_BLOCK_IF_FULL),
    };

    // Write 15 bytes to fill the buffer (leaving 1 empty slot for the cursor property)
    let written = chan.blocking_write(&[0x11; 15]);
    assert_eq!(written, 15);
    assert_eq!(chan.write.load(Ordering::Relaxed), 15);

    // Try writing more when full: should write 0 bytes
    let written_extra = chan.blocking_write(&[0x22; 5]);
    assert_eq!(written_extra, 0);
}

use platform::defmt_logger::{RttProtocol, RttWriter};
use std::cell::RefCell;
use std::rc::Rc;

struct MockWriter {
    written: Rc<RefCell<Vec<u8>>>,
    flushed: Rc<RefCell<bool>>,
}

impl RttWriter for MockWriter {
    fn write_all(&self, bytes: &[u8]) {
        self.written.borrow_mut().extend_from_slice(bytes);
    }
    fn flush(&self) {
        self.flushed.replace(true);
    }
}

#[test]
fn test_rtt_protocol_flow() {
    // Clear CRASH_LOG_BUFFER first
    critical_section::with(|cs| {
        let mut buffer = platform::panic_handler::CRASH_LOG_BUFFER
            .borrow(cs)
            .borrow_mut();
        buffer.head = 0;
        buffer.wrapped = false;
        buffer.buffer.fill(0);
    });

    let written = Rc::new(RefCell::new(Vec::new()));
    let flushed = Rc::new(RefCell::new(false));
    let writer = MockWriter {
        written: Rc::clone(&written),
        flushed: Rc::clone(&flushed),
    };

    let protocol = RttProtocol::new(writer);

    // Acquire the logger (starts a frame)
    protocol.acquire();

    // Verify taken flag is set
    assert!(protocol.taken.load(core::sync::atomic::Ordering::Relaxed));

    // Write bytes
    unsafe {
        protocol.write(&[0x11, 0x22, 0x33]);
    }

    // Release the logger (ends frame)
    unsafe {
        protocol.release();
    }

    // Verify taken flag is cleared
    assert!(!protocol.taken.load(core::sync::atomic::Ordering::Relaxed));

    // Check mock writer captured the output
    let written_bytes = written.borrow();
    assert!(!written_bytes.is_empty());
    assert!(written_bytes.contains(&0x11));
    assert!(written_bytes.contains(&0x22));
    assert!(written_bytes.contains(&0x33));

    // Check dual-logging to CRASH_LOG_BUFFER worked
    let mut crash_log_buf = [0u8; 128];
    let logs_len = critical_section::with(|cs| {
        platform::panic_handler::extract_system_logs(&cs, &mut crash_log_buf)
    });
    let crash_slice = &crash_log_buf[..logs_len];
    assert!(!crash_slice.is_empty());
    assert!(crash_slice.contains(&0x11));
}

#[test]
fn test_rtt_protocol_reentrancy_panic() {
    let writer = MockWriter {
        written: Rc::new(RefCell::new(Vec::new())),
        flushed: Rc::new(RefCell::new(false)),
    };
    let protocol = RttProtocol::new(writer);
    protocol.acquire();

    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        protocol.acquire();
    }));

    // Explicitly release the lock so other tests can proceed
    unsafe {
        protocol.release();
    }

    assert!(res.is_err());
}

use platform::defmt_logger::{DefmtLogWriter, DefmtLogger};
use std::sync::{Arc, Mutex};

struct TestLoggerWriter {
    written: Arc<Mutex<Vec<u8>>>,
}

impl DefmtLogWriter for TestLoggerWriter {
    fn write_all(&self, bytes: &[u8]) {
        self.written.lock().unwrap().extend_from_slice(bytes);
    }
}

#[test]
fn test_defmt_logger_state_manager() {
    let written = Arc::new(Mutex::new(Vec::new()));
    let writer = Box::leak(Box::new(TestLoggerWriter {
        written: Arc::clone(&written),
    }));

    // 1. Set our test writer
    DefmtLogger::set_writer(writer);

    // 2. Verify active writer matches and receives write calls
    unsafe {
        if let Some(w) = platform::defmt_logger::ACTIVE_WRITER {
            w.write_all(&[0xDE, 0xAD, 0xBE, 0xEF]);
        }
    }
    assert_eq!(*written.lock().unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);

    // 3. Disable writer
    DefmtLogger::disable();
    unsafe {
        assert!(platform::defmt_logger::ACTIVE_WRITER.is_none());
    }

    // 4. Restore DEFAULT_RTT_WRITER for other tests
    DefmtLogger::set_writer(&platform::defmt_logger::DEFAULT_RTT_WRITER);
    unsafe {
        assert!(platform::defmt_logger::ACTIVE_WRITER.is_some());
    }
}
