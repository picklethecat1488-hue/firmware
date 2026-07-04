use firmware_lib::panic_handler::{
    extract_system_logs, generate_uuid, scan_stack, scan_stack_from_sp, write_crash_log_to_flash,
    CoreState, CRASH_LOG_BUFFER,
};
use std::fmt::Write;
use std::sync::Mutex;

static BUFFER_MUTEX: Mutex<()> = Mutex::new(());

/// Log a string to the global circular buffer.
fn log_string(val: &str) {
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        let _ = buffer.write_str(val);
        let _ = buffer.write_str("\n");
    });
}

#[test]
fn test_crash_log_buffer_writing_and_wrapping() {
    let _lock = BUFFER_MUTEX.lock().unwrap();
    // Clear CRASH_LOG_BUFFER first
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        buffer.head = 0;
        buffer.wrapped = false;
        buffer.buffer.fill(0);
    });

    log_string("Event A");
    log_string("Event B");

    critical_section::with(|cs| {
        let buffer = CRASH_LOG_BUFFER.borrow(cs).borrow();
        let end = buffer.head;
        let logged_str = core::str::from_utf8(&buffer.buffer[..end]).unwrap();
        assert!(logged_str.contains("Event A"));
        assert!(logged_str.contains("Event B"));
    });
}

#[test]
fn test_scan_stack_heuristic() {
    // Flash range: 0x10000000..0x10080000
    let flash_start = 0x10000000;
    let flash_end = 0x10080000;

    let stack = [
        0x00001234, // Outside flash
        0x10000101, // Inside flash, odd, has BL instr before it
        0x10000201, // Inside flash, odd, has BLX instr before it
        0x10000301, // Inside flash, odd, has invalid instr before it
        0x10000400, // Inside flash, even (invalid PC)
        0x20000456, // Outside flash (RAM)
    ];

    let mock_read_mem = |addr: u32| -> Option<u32> {
        match addr {
            // Address before 0x10000100 (0x100000FC): return mock 32-bit BL instruction (h1: 0xF000, h2: 0xD800)
            0x100000FC => Some(0xD800F000),
            // Address before 0x10000200 (0x100001FC): return mock 16-bit BX instruction (h2: 0x4720)
            0x100001FC => Some(0x47200000),
            // Other addresses: return dummy non-call instructions
            _ => Some(0x00000000),
        }
    };

    let mut pcs = [0u32; 16];
    let count = scan_stack(&stack, flash_start, flash_end, &mut pcs, mock_read_mem);

    assert_eq!(count, 2);
    // Should clear the LSB for Thumb-mode addresses
    assert_eq!(pcs[0], 0x10000100);
    assert_eq!(pcs[1], 0x10000200);
}

#[test]
fn test_scan_stack_from_sp_integration() {
    let flash_start = 0x10000000;
    let flash_end = 0x10080000;

    let stack_data = [
        0x10000101, // Valid BL return PC
        0x10000201, // Valid BX/BLX return PC
    ];

    let mock_read_mem = |addr: u32| -> Option<u32> {
        match addr {
            0x100000FC => Some(0xD800F000),
            0x100001FC => Some(0x47200000),
            _ => Some(0),
        }
    };

    let mut pcs = [0u32; 16];
    let sp = stack_data.as_ptr() as usize;
    let stack_top = sp + (stack_data.len() * 4);

    let count = scan_stack_from_sp(
        sp,
        stack_top,
        flash_start,
        flash_end,
        &mut pcs,
        mock_read_mem,
    );

    assert_eq!(count, 2);
    assert_eq!(pcs[0], 0x10000100);
    assert_eq!(pcs[1], 0x10000200);

    // If sp >= stack_top, should return 0
    let count_empty = scan_stack_from_sp(
        stack_top,
        sp,
        flash_start,
        flash_end,
        &mut pcs,
        mock_read_mem,
    );
    assert_eq!(count_empty, 0);
}

#[test]
fn test_extract_system_logs_helper() {
    let _lock = BUFFER_MUTEX.lock().unwrap();
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        buffer.head = 0;
        buffer.wrapped = false;
    });

    log_string("Log 1");
    log_string("Log 2");

    let mut extract_buf = [0u8; 1024];
    let len = critical_section::with(|cs| extract_system_logs(&cs, &mut extract_buf));

    let extracted_str = core::str::from_utf8(&extract_buf[..len]).unwrap();
    assert!(extracted_str.contains("Log 1"));
    assert!(extracted_str.contains("Log 2"));
}

struct MockFlash {
    data: Vec<u8>,
}

impl embedded_storage_async::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;
    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        bytes.copy_from_slice(&self.data[start..end]);
        Ok(())
    }
    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;
    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        self.data[start..end].copy_from_slice(bytes);
        Ok(())
    }
    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        self.data[start..end].fill(0xFF);
        Ok(())
    }
}

#[test]
fn test_write_crash_log_to_flash_rolling() {
    let mut flash = MockFlash {
        data: vec![0xFF; 65536], // 64KB mock partition
    };
    let range = 0..65536;
    let mut cache = sequential_storage::cache::NoCache::new();
    let mut scratch = [0u8; 1500];

    // Write crash dump 1
    futures::executor::block_on(async {
        write_crash_log_to_flash(
            &mut flash,
            range.clone(),
            &mut cache,
            &mut scratch,
            b"crash data 1",
        )
        .await
        .unwrap();
    });

    // Write crash dump 2
    futures::executor::block_on(async {
        write_crash_log_to_flash(
            &mut flash,
            range.clone(),
            &mut cache,
            &mut scratch,
            b"crash data 2",
        )
        .await
        .unwrap();
    });

    // Check directories and stored keys
    let string_to_key = |name: &str| -> [u8; 32] {
        let mut k = [0u8; 32];
        let bytes = name.as_bytes();
        let len = core::cmp::min(bytes.len(), 32);
        k[..len].copy_from_slice(&bytes[..len]);
        k
    };

    futures::executor::block_on(async {
        // Check .dir contents
        let mut dir_buf = [0u8; 128];
        let dir_val = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut flash,
            range.clone(),
            &mut cache,
            &mut dir_buf,
            &string_to_key(".dir"),
        )
        .await;

        let dir_str = core::str::from_utf8(dir_val.unwrap().unwrap()).unwrap();
        println!("dir_str is: {:?}", dir_str);
        assert!(dir_str.contains("crash_0.cbor"));
        assert!(dir_str.contains("crash_1.cbor"));

        // Check crash_0.cbor contents
        let mut crash0_buf = [0u8; 128];
        let crash0_val = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut flash,
            range.clone(),
            &mut cache,
            &mut crash0_buf,
            &string_to_key("crash_0.cbor"),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(crash0_val, b"crash data 1");

        // Check crash_1.cbor contents
        let mut crash1_buf = [0u8; 128];
        let crash1_val = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            &mut flash,
            range.clone(),
            &mut cache,
            &mut crash1_buf,
            &string_to_key("crash_1.cbor"),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(crash1_val, b"crash data 2");
    });
}

#[test]
fn test_generate_uuid_properties() {
    let entropy: [u8; 16] = [0xAA; 16];
    let mut state1: CoreState = CoreState {
        r0: 1,
        r1: 2,
        r2: 3,
        r3: 4,
        backtrace: [0u32; 16],
    };
    state1.backtrace[..2].copy_from_slice(&[0x10002000, 0x10003000]);

    let uuid1 = generate_uuid(entropy, 12345, &state1, "hash123");
    let mut state2: CoreState = CoreState {
        r0: 1,
        r1: 2,
        r2: 3,
        r3: 4,
        backtrace: [0u32; 16],
    };
    state2.backtrace[..2].copy_from_slice(&[0x10002000, 0x10003000]);

    let uuid2 = generate_uuid(entropy, 12345, &state2, "hash123");
    let mut state3: CoreState = CoreState {
        r0: 1,
        r1: 2,
        r2: 3,
        r3: 4,
        backtrace: [0u32; 16],
    };
    state3.backtrace[..2].copy_from_slice(&[0x10002000, 0x10003000]);

    let uuid3 = generate_uuid(entropy, 12346, &state3, "hash123"); // different time

    // UUIDv4 checks:
    // Version 4 check:
    assert_eq!(uuid1[6] & 0xF0, 0x40);
    // Variant 1 check (RFC 4122):
    assert_eq!(uuid1[8] & 0xC0, 0x80);

    // Identical inputs produce identical UUIDs
    assert_eq!(uuid1, uuid2);

    // Different inputs produce different UUIDs
    assert_ne!(uuid1, uuid3);
}
