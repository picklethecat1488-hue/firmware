#![deny(missing_docs)]
#![allow(static_mut_refs)]

//! Generalized modular panic handler for ARMv6+ architectures (e.g. Cortex-M).
//! Automatically dumps panics, stack traces, register dumps, and circular system log buffers
//! to a rolling flash memory partition using target-agnostic flash abstractions.

use core::fmt::Write;

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

/// Log a formatted string to the global circular buffer.
pub fn log_system(args: core::fmt::Arguments) {
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let micros = unsafe { TIME_FN.map(|f| f()) };
            if let Some(t) = micros {
                let _ = core::fmt::write(&mut *buffer, format_args!("[{:010} us] ", t));
            }
        }
        let _ = core::fmt::write(&mut *buffer, args);
        let _ = buffer.write_str("\n");
    });
}

/// Helper macro for logging system events with compile-time module prefixing.
#[macro_export]
macro_rules! log_info {
    ($fmt:literal $(, $arg:expr)* $(,)*) => {
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        $crate::panic_handler::log_system(format_args!(concat!("[", core::module_path!(), "] ", $fmt) $(, $arg)*));
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!($fmt $(, $arg)*);
    }
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
    let mut pc_count = 0;
    let mut current_addr = sp;
    while current_addr < stack_top && pc_count < 16 {
        let val = unsafe { *(current_addr as *const u32) };
        if (FLASH_START..FLASH_END).contains(&val) && val % 2 == 1 {
            pcs[pc_count] = val & !1;
            pc_count += 1;
        }
        current_addr += 4;
    }

    // A single static scratch buffer used for log extraction, CBOR serialization,
    // and as the sequential-storage map operation workspace.
    // Partitioned as:
    //   - scratch[0..1024]: Used for log extraction (1024 bytes), then reused as sequential-storage scratch workspace.
    //   - scratch[1024..2224]: Used for CBOR serialization buffer (1200 bytes).
    static mut SCRATCH_BUF: [u8; 2224] = [0u8; 2224];
    let (log_buf, cbor_buf) = unsafe { SCRATCH_BUF.split_at_mut(1024) };

    // 2. Extract logs from CRASH_LOG_BUFFER into a contiguous slice
    let logs_slice = critical_section::with(|cs| {
        let buffer = CRASH_LOG_BUFFER.borrow(cs).borrow();
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
        &log_buf[..len]
    });

    // Populate CrashDump struct
    let mut backtrace_array = [0u32; 16];
    backtrace_array[..pc_count].copy_from_slice(&pcs[..pc_count]);

    let dump = crate::types::CrashDump {
        revision_hash: env!("GIT_HASH"),
        r0,
        r1,
        r2,
        r3,
        backtrace: backtrace_array,
        backtrace_len: pc_count as u32,
        system_logs: logs_slice,
    };

    // Serialize CrashDump into a buffer
    let mut encoder = minicbor::Encoder::new(minicbor::encode::write::Cursor::new(&mut *cbor_buf));
    let mut encoded_len = 0;
    if encoder.encode(&dump).is_ok() {
        encoded_len = encoder.into_writer().position();
    }
    let encoded_bytes = &cbor_buf[..encoded_len];

    // 3. Write crash log to storage partition using rolling index
    critical_section::with(|cs| {
        if let Some(config) = PANIC_CONFIG.borrow(cs).take() {
            // Build filesystem controller inside panic context using the adapter
            let flash = PanicFlashAsyncAdapter::<WRITE_SIZE, ERASE_SIZE>(config.flash);
            let mut cache = sequential_storage::cache::NoCache::new();
            let buf = log_buf;

            // Convert string path into fixed key
            let string_to_key = |name: &str| -> [u8; 32] {
                let mut k = [0u8; 32];
                let bytes = name.as_bytes();
                let len = core::cmp::min(bytes.len(), 32);
                k[..len].copy_from_slice(&bytes[..len]);
                k
            };

            // Read the rolling crash index
            let mut idx_buf = [0u8; 4];
            let mut mut_flash = flash;
            let current_idx = block_on(async {
                let key = string_to_key("crash_idx");
                if let Ok(Some(bytes)) = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    &mut idx_buf,
                    &key,
                )
                .await
                {
                    if bytes.len() == 4 {
                        return u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                    }
                }
                0u32
            });

            // Write the crash log
            let mut filename = heapless::String::<32>::new();
            let _ = core::fmt::write(&mut filename, format_args!("crash_{}.cbor", current_idx));
            let log_key = string_to_key(filename.as_str());

            block_on(async {
                let _ = sequential_storage::map::store_item(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    buf,
                    &log_key,
                    &encoded_bytes,
                )
                .await;
            });

            // Increment index modulo 5
            let next_idx = (current_idx + 1) % 5;
            let next_bytes = next_idx.to_le_bytes();
            let idx_key = string_to_key("crash_idx");
            block_on(async {
                let _ = sequential_storage::map::store_item(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    buf,
                    &idx_key,
                    &next_bytes,
                )
                .await;
            });

            // Write directory file .dir so fs_tool can find it
            static mut CURRENT_DIR_STR: heapless::String<128> = heapless::String::new();
            let current_dir = unsafe { &mut CURRENT_DIR_STR };
            current_dir.clear();
            let dir_key = string_to_key(".dir");
            let existing_dir = block_on(async {
                sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    buf,
                    &dir_key,
                )
                .await
            });

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
                block_on(async {
                    let _ = sequential_storage::map::store_item(
                        &mut mut_flash,
                        config.range.clone(),
                        &mut cache,
                        buf,
                        &dir_key,
                        &current_dir.as_bytes(),
                    )
                    .await;
                });
            }
        }
    });

    cortex_m::asm::udf();
}
