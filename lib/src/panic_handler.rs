#![deny(missing_docs)]
#![allow(static_mut_refs)]

//! Generalized modular panic handler for ARMv6+ architectures (e.g. Cortex-M).
//! Automatically dumps panics, stack traces, register dumps, and circular system log buffers
//! to a rolling flash memory partition using target-agnostic flash abstractions.

pub use crate::types::LogBuffer;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use crate::types::PanicConfig;

/// Static safe instance of the log buffer.
pub static CRASH_LOG_BUFFER: critical_section::Mutex<core::cell::RefCell<LogBuffer>> =
    critical_section::Mutex::new(core::cell::RefCell::new(LogBuffer::new()));

/// Static function pointer for retrieving microsecond-level system time.
static mut TIME_FN: Option<fn() -> u64> = None;

/// Sets the function used to retrieve the system time for logs.
pub fn set_time_fn(f: fn() -> u64) {
    critical_section::with(|_| unsafe {
        TIME_FN = Some(f);
    });
}

/// Helper function to serialize a `CrashDump` structure into CBOR format.
pub fn serialize_crash_dump<'a>(
    dump: &crate::types::CrashDump<'a>,
    buf: &'a mut [u8],
) -> Result<usize, minicbor::encode::Error<minicbor::encode::write::EndOfSlice>> {
    let mut encoder = minicbor::Encoder::new(minicbor::encode::write::Cursor::new(buf));
    encoder.encode(dump)?;
    Ok(encoder.into_writer().position())
}

/// Adapter exposing a blocking nor-flash driver as an asynchronous nor-flash driver
pub struct BlockingAsyncFlash<F>(pub F);

impl<F: embedded_storage::nor_flash::ErrorType> embedded_storage_async::nor_flash::ErrorType
    for BlockingAsyncFlash<F>
{
    type Error = F::Error;
}

impl<F: embedded_storage::nor_flash::ReadNorFlash> embedded_storage_async::nor_flash::ReadNorFlash
    for BlockingAsyncFlash<F>
{
    const READ_SIZE: usize = F::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::ReadNorFlash::read(&mut inner, offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::NorFlash
    for BlockingAsyncFlash<F>
{
    const WRITE_SIZE: usize = F::WRITE_SIZE;
    const ERASE_SIZE: usize = F::ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::write(&mut inner, offset, bytes)
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let mut inner = &mut self.0;
        embedded_storage::nor_flash::NorFlash::erase(&mut inner, from, to)
    }
}

impl<F: embedded_storage::nor_flash::NorFlash> embedded_storage_async::nor_flash::MultiwriteNorFlash
    for BlockingAsyncFlash<F>
{
}
/// Error type for `PanicFlashAsyncAdapter`.
#[derive(Debug, Copy, Clone)]
pub struct PanicFlashError;

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl embedded_storage::nor_flash::NorFlashError for PanicFlashError {
    fn kind(&self) -> embedded_storage::nor_flash::NorFlashErrorKind {
        embedded_storage::nor_flash::NorFlashErrorKind::Other
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Adapter exposing a target-agnostic PanicFlash trait object as an asynchronous nor-flash driver
/// suitable for sequential-storage async filesystem operations.
pub struct PanicFlashAsyncAdapter<'a, const WRITE_SIZE: usize = 256, const ERASE_SIZE: usize = 4096>(
    pub &'a mut dyn crate::types::PanicFlash,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage_async::nor_flash::ErrorType
    for PanicFlashAsyncAdapter<'a, WRITE_SIZE, ERASE_SIZE>
{
    type Error = PanicFlashError;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage_async::nor_flash::ReadNorFlash
    for PanicFlashAsyncAdapter<'a, WRITE_SIZE, ERASE_SIZE>
{
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.0.read(offset, bytes).map_err(|_| PanicFlashError)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage_async::nor_flash::NorFlash
    for PanicFlashAsyncAdapter<'a, WRITE_SIZE, ERASE_SIZE>
{
    const WRITE_SIZE: usize = WRITE_SIZE;
    const ERASE_SIZE: usize = ERASE_SIZE;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write(offset, bytes).map_err(|_| PanicFlashError)
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.0.erase(from, to).map_err(|_| PanicFlashError)
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'a, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage_async::nor_flash::MultiwriteNorFlash
    for PanicFlashAsyncAdapter<'a, WRITE_SIZE, ERASE_SIZE>
{
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Micro-executor function to block synchronously on async operations,
/// using a custom no-op waker loop.
fn block_on<F: core::future::Future>(future: F) -> F::Output {
    use core::pin::pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    let mut future = pin!(future);

    unsafe fn dummy_clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    unsafe fn dummy_wake(_: *const ()) {}
    unsafe fn dummy_wake_by_ref(_: *const ()) {}
    unsafe fn dummy_drop(_: *const ()) {}

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(dummy_clone, dummy_wake, dummy_wake_by_ref, dummy_drop);

    let raw_waker = RawWaker::new(core::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(res) => return res,
            Poll::Pending => {}
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global static reference to PANIC_CONFIG to be taken by the panic handler
pub static PANIC_CONFIG: embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<Option<PanicConfig>>,
> = embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(None));

/// Initialize the panic handler with flash access and target partition settings.
pub fn init(
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    flash: &'static mut dyn crate::types::PanicFlash,
    #[cfg(all(target_arch = "arm", target_os = "none"))] range: core::ops::Range<u32>,
) {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    critical_section::with(|cs| {
        PANIC_CONFIG
            .borrow(cs)
            .replace(Some(PanicConfig { flash, range }));
    });
}

/// Heuristic stack scanner that walks a slice of stack words and extracts return PCs
/// by verifying that the caller instruction before the return address is a BL or BLX.
pub fn scan_stack<F>(
    stack: &[u32],
    flash_start: u32,
    flash_end: u32,
    pcs: &mut [u32; 16],
    read_mem: F,
) -> usize
where
    F: Fn(u32) -> Option<u32>,
{
    let mut pc_count = 0;
    for &val in stack {
        if pc_count >= 16 {
            break;
        }
        if (flash_start..flash_end).contains(&val) && (val & 1 == 1) {
            let ret_addr = val & !1;
            // Check if the instruction before the return address is BL or BLX
            if let Some(instr) = read_mem(ret_addr.saturating_sub(4)) {
                let h1 = instr as u16;
                let h2 = (instr >> 16) as u16;
                // 32-bit Thumb BL/BLX: h1 starts with 11110 (0xF000..0xF7FF), h2 starts with 11011 or 11111 (BL) or 11001 or 11101 (BLX)
                let is_bl_blx_32 = (h1 & 0xF800 == 0xF000) && (h2 & 0xC800 == 0xC800);
                // 16-bit Thumb BX/BLX: h2 matches 0x47xx pattern (0x4700..0x47FF)
                let is_bx_blx_16 = h2 & 0xFF00 == 0x4700;

                if is_bl_blx_32 || is_bx_blx_16 {
                    pcs[pc_count] = ret_addr;
                    pc_count += 1;
                }
            }
        }
    }
    pc_count
}

/// Helper function to scan stack memory from a stack pointer up to stack top, returning the collected return PCs.
pub fn scan_stack_from_sp<F>(
    sp: usize,
    stack_top: usize,
    flash_start: u32,
    flash_end: u32,
    pcs: &mut [u32; 16],
    read_mem: F,
) -> usize
where
    F: Fn(u32) -> Option<u32>,
{
    if sp < stack_top {
        let words = (stack_top - sp) / 4;
        let limit = words.min(256);
        let stack_slice = unsafe { core::slice::from_raw_parts(sp as *const u32, limit) };
        scan_stack(stack_slice, flash_start, flash_end, pcs, read_mem)
    } else {
        0
    }
}

/// Extracts circular system logs from the CRASH_LOG_BUFFER into the provided buffer.
pub fn extract_system_logs(cs: &critical_section::CriticalSection, log_buf: &mut [u8]) -> usize {
    let buffer = CRASH_LOG_BUFFER.borrow(*cs).borrow();
    let mut len = 0;
    if buffer.wrapped {
        let part1 = &buffer.buffer[buffer.head..];
        let len1 = part1.len();
        log_buf[..len1].copy_from_slice(part1);
        len += len1;
        let part2 = &buffer.buffer[..buffer.head];
        let len2 = part2.len();
        log_buf[len..len + len2].copy_from_slice(part2);
        len += len2;
    } else {
        let part = &buffer.buffer[..buffer.head];
        let len1 = part.len();
        log_buf[..len1].copy_from_slice(part);
        len += len1;
    }
    len
}

/// Writes the serialized crash dump, increments the rolling index, and updates the directory listing.
pub async fn write_crash_log_to_flash<F>(
    flash: &mut F,
    range: core::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    buf: &mut [u8],
    encoded_bytes: &[u8],
) -> Result<(), F::Error>
where
    F: embedded_storage_async::nor_flash::NorFlash,
{
    // Convert string path into fixed key
    let string_to_key = |name: &str| -> [u8; 32] {
        let mut k = [0u8; 32];
        let bytes = name.as_bytes();
        let len = core::cmp::min(bytes.len(), 32);
        k[..len].copy_from_slice(&bytes[..len]);
        k
    };

    // Read the rolling crash index
    let current_idx = if let Ok(Some(bytes)) =
        sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
            flash,
            range.clone(),
            cache,
            buf,
            &string_to_key("crash_idx"),
        )
        .await
    {
        if bytes.len() == 4 {
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            0
        }
    } else {
        0
    };

    // Write the crash log
    let mut filename = heapless::String::<32>::new();
    let _ = core::fmt::write(&mut filename, format_args!("crash_{}.cbor", current_idx));
    let log_key = string_to_key(filename.as_str());

    let _ = sequential_storage::map::store_item(
        flash,
        range.clone(),
        cache,
        buf,
        &log_key,
        &encoded_bytes,
    )
    .await;

    // Increment index modulo 5
    let next_idx = (current_idx + 1) % 5;
    let next_bytes = next_idx.to_le_bytes();
    let idx_key = string_to_key("crash_idx");
    let _ = sequential_storage::map::store_item(
        flash,
        range.clone(),
        cache,
        buf,
        &idx_key,
        &&next_bytes[..],
    )
    .await;

    // Write directory file .dir so host_fs can find it
    let mut current_dir = heapless::String::<128>::new();
    let dir_key = string_to_key(".dir");
    let existing_dir = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        range.clone(),
        cache,
        buf,
        &dir_key,
    )
    .await;

    if let Ok(Some(bytes)) = existing_dir {
        if let Ok(s) = core::str::from_utf8(bytes) {
            let _ = current_dir.push_str(s);
        }
    }

    // Check if filename is already in directory
    let mut found = false;
    for entry in current_dir.split('\n') {
        if entry == filename.as_str() {
            found = true;
            break;
        }
    }

    if !found {
        if !current_dir.is_empty() {
            let _ = current_dir.push_str("\n");
        }
        let _ = current_dir.push_str(filename.as_str());

        // Store updated dir
        let _ = sequential_storage::map::store_item(
            flash,
            range.clone(),
            cache,
            buf,
            &dir_key,
            &current_dir.as_bytes(),
        )
        .await;
    }

    Ok(())
}

/// Helper function to mix hardware entropy and context variables into a unique UUID.
#[allow(clippy::too_many_arguments)]
pub fn generate_uuid(
    entropy: [u8; 16],
    micros: u64,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    backtrace: &[u32],
    revision_hash: &str,
) -> [u8; 16] {
    let mut uuid = entropy;

    let mut h1 = 0xcbf29ce484222325u64;
    let mut h2 = 0x84222325cbf29ce4u64;

    let mut feed_u64 = |val: u64| {
        h1 ^= val;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 ^= val.rotate_left(17);
        h2 = h2.wrapping_mul(0x100000001b3);
    };

    feed_u64(micros);
    feed_u64(r0 as u64);
    feed_u64(r1 as u64);
    feed_u64(r2 as u64);
    feed_u64(r3 as u64);
    for &pc in backtrace {
        feed_u64(pc as u64);
    }
    for &b in revision_hash.as_bytes() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 ^= (b as u64).rotate_left(9);
        h2 = h2.wrapping_mul(0x100000001b3);
    }

    let h1_bytes = h1.to_be_bytes();
    let h2_bytes = h2.to_be_bytes();
    for i in 0..8 {
        uuid[i] ^= h1_bytes[i];
        uuid[i + 8] ^= h2_bytes[i];
    }

    uuid[6] = (uuid[6] & 0x0F) | 0x40; // Version 4
    uuid[8] = (uuid[8] & 0x3F) | 0x80; // Variant 1

    uuid
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Shared panic handler logic executing crash dump logging to flash memory with customizable write/erase sizes.
pub fn handle_panic_with_sizes<
    const FLASH_SIZE: usize,
    const STACK_TOP: u32,
    const FLASH_START: u32,
    const FLASH_END: u32,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
>(
    entropy: [u8; 16],
    _info: &core::panic::PanicInfo,
) -> ! {
    let r0: u32;
    let r1: u32;
    let r2: u32;
    let r3: u32;
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
    }

    cortex_m::interrupt::disable();

    // 1. Walk the stack to capture return PCs (heuristic stack scanner)
    let sp: u32;
    unsafe {
        core::arch::asm!("mov {}, sp", out(reg) sp);
    }
    let stack_top = STACK_TOP;
    let mut pcs = [0u32; 16];
    let read_mem = |addr: u32| {
        if (FLASH_START..FLASH_END).contains(&addr) {
            Some(unsafe { *(addr as *const u32) })
        } else {
            None
        }
    };
    let pc_count = scan_stack_from_sp(
        sp as usize,
        stack_top as usize,
        FLASH_START,
        FLASH_END,
        &mut pcs,
        read_mem,
    );

    // A single static scratch buffer used for log extraction, CBOR serialization,
    // and as the sequential-storage map operation workspace.
    // Partitioned as:
    //   - scratch[0..1500]: Used for log extraction (1500 bytes), then reused as sequential-storage scratch/fetch workspace.
    //   - scratch[1500..2700]: Used for CBOR serialization buffer (1200 bytes).
    static mut SCRATCH_BUF: [u8; 2700] = [0u8; 2700];
    let (log_buf, cbor_buf) = unsafe { SCRATCH_BUF.split_at_mut(1500) };

    // 2. Extract logs from CRASH_LOG_BUFFER into a contiguous slice
    let logs_len = critical_section::with(|cs| extract_system_logs(&cs, log_buf));
    let logs_slice = &log_buf[..logs_len];

    // Populate CrashDump struct
    let mut backtrace_array = [0u32; 16];
    backtrace_array[..pc_count].copy_from_slice(&pcs[..pc_count]);

    let micros = unsafe { TIME_FN.map(|f| f()) }.unwrap_or(0);
    let uuid = generate_uuid(
        entropy,
        micros,
        r0,
        r1,
        r2,
        r3,
        &backtrace_array[..pc_count],
        env!("GIT_HASH"),
    );

    let dump = crate::types::CrashDump {
        revision_hash: env!("GIT_HASH"),
        r0,
        r1,
        r2,
        r3,
        backtrace: backtrace_array,
        backtrace_len: pc_count as u32,
        system_logs: logs_slice,
        uuid,
    };

    // Serialize CrashDump into a buffer
    let mut encoded_len = 0;
    if let Ok(len) = serialize_crash_dump(&dump, cbor_buf) {
        encoded_len = len;
    }
    let encoded_bytes = &cbor_buf[..encoded_len];

    // 3. Write crash log to storage partition using rolling index
    critical_section::with(|cs| {
        if let Some(config) = PANIC_CONFIG.borrow(cs).take() {
            // Build filesystem controller inside panic context using the adapter
            let mut flash = PanicFlashAsyncAdapter::<WRITE_SIZE, ERASE_SIZE>(config.flash);
            let mut cache = sequential_storage::cache::NoCache::new();
            block_on(async {
                let _ = write_crash_log_to_flash(
                    &mut flash,
                    config.range.clone(),
                    &mut cache,
                    log_buf,
                    encoded_bytes,
                )
                .await;
            });
        }
    });

    cortex_m::asm::udf();
}
