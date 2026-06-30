#![no_std]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

//! Modular panic handler for RP2040 microcontrollers.
//! Automatically dumps panics, stack traces, and circular system log buffers
//! to a rolling flash memory file buffer.

use core::fmt::Write;

/// Type definitions for the panic handler
pub mod types;

pub use types::LogBuffer;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use types::PanicConfig;

/// Static safe instance of the log buffer.
pub static CRASH_LOG_BUFFER: critical_section::Mutex<core::cell::RefCell<LogBuffer>> =
    critical_section::Mutex::new(core::cell::RefCell::new(LogBuffer::new()));

/// Log a formatted string to the global circular buffer.
pub fn log_system(args: core::fmt::Arguments) {
    critical_section::with(|cs| {
        let mut buffer = CRASH_LOG_BUFFER.borrow(cs).borrow_mut();
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            // Read lower 32 bits of RP2040 microsecond hardware timer
            let micros = unsafe { *(0x4005_400c as *const u32) };
            let _ = core::fmt::write(&mut *buffer, format_args!("[{:010} us] ", micros));
        }
        let _ = core::fmt::write(&mut *buffer, args);
        let _ = buffer.write_str("\n");
    });
}

/// Helper macro for logging system events with compile-time module prefixing.
#[macro_export]
macro_rules! log_info {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::log_system(format_args!(concat!("[", core::module_path!(), "] ", $fmt) $(, $arg)*));
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!($fmt $(, $arg)*);
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Adapter exposing a blocking nor-flash driver as an asynchronous nor-flash driver
/// suitable for sequential-storage async filesystem operations.
struct BlockingAsyncFlash<F>(F);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<F: embedded_storage::nor_flash::ErrorType> embedded_storage_async::nor_flash::ErrorType
    for BlockingAsyncFlash<F>
{
    type Error = F::Error;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
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

#[cfg(all(target_arch = "arm", target_os = "none"))]
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

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<F: embedded_storage::nor_flash::MultiwriteNorFlash>
    embedded_storage_async::nor_flash::MultiwriteNorFlash for BlockingAsyncFlash<F>
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

// PanicConfig is imported from types module

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global static reference to PANIC_CONFIG to be taken by the panic handler
pub static PANIC_CONFIG: embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<Option<PanicConfig>>,
> = embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(None));

/// Initialize the panic handler with flash access and target partition settings.
pub fn init(
    #[cfg(all(target_arch = "arm", target_os = "none"))] flash: embassy_rp::peripherals::FLASH,
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
/// Shared panic handler logic executing crash dump logging to flash memory.
pub fn handle_panic<const FLASH_SIZE: usize>(info: &core::panic::PanicInfo) -> ! {
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
    let stack_top = 0x2004_0000;
    let mut pcs = [0u32; 16];
    let mut pc_count = 0;
    let mut current_addr = sp;
    while current_addr < stack_top && pc_count < 16 {
        let val = unsafe { *(current_addr as *const u32) };
        if val >= 0x1000_0000 && val < 0x1020_0000 && val % 2 == 1 {
            pcs[pc_count] = val & !1;
            pc_count += 1;
        }
        current_addr += 4;
    }

    // 2. Format the crash dump output
    static mut CONTENT_BUF: heapless::String<1024> = heapless::String::new();
    let content = unsafe { &mut CONTENT_BUF };
    content.clear();
    let _ = core::fmt::write(content, format_args!("--- PANIC ---\n"));
    if let Some(location) = info.location() {
        let _ = core::fmt::write(
            content,
            format_args!("Location: {}:{}\n", location.file(), location.line()),
        );
    }
    let _ = core::fmt::write(content, format_args!("Message: {}\n\n", info));

    let _ = core::fmt::write(
        content,
        format_args!("Revision Hash: {}\n\n", env!("GIT_HASH")),
    );

    let _ = core::fmt::write(content, format_args!("Registers:\n"));
    let _ = core::fmt::write(content, format_args!("  R0: 0x{:08X}\n", r0));
    let _ = core::fmt::write(content, format_args!("  R1: 0x{:08X}\n", r1));
    let _ = core::fmt::write(content, format_args!("  R2: 0x{:08X}\n", r2));
    let _ = core::fmt::write(content, format_args!("  R3: 0x{:08X}\n\n", r3));

    let _ = core::fmt::write(content, format_args!("Backtrace:\n"));
    for i in 0..pc_count {
        let _ = core::fmt::write(content, format_args!("  0x{:08X}\n", pcs[i]));
    }

    let _ = core::fmt::write(content, format_args!("\nSystem Logs:\n"));
    critical_section::with(|cs| {
        let buffer = CRASH_LOG_BUFFER.borrow(cs).borrow();
        if buffer.wrapped {
            let start = buffer.head;
            if let Ok(s) = core::str::from_utf8(&buffer.buffer[start..]) {
                let _ = content.push_str(s);
            }
        }
        let end = buffer.head;
        if let Ok(s) = core::str::from_utf8(&buffer.buffer[..end]) {
            let _ = content.push_str(s);
        }
    });

    // 3. Write crash log to storage partition using rolling index
    critical_section::with(|cs| {
        if let Some(config) = PANIC_CONFIG.borrow(cs).take() {
            // Build filesystem controller inside panic context
            let raw_flash = embassy_rp::flash::Flash::<
                _,
                embassy_rp::flash::Blocking,
                FLASH_SIZE,
            >::new_blocking(config.flash);
            let flash = BlockingAsyncFlash(raw_flash);
            let mut cache = sequential_storage::cache::NoCache::new();
            static mut SCRATCH_BUF: [u8; 1024] = [0u8; 1024];
            let buf = unsafe { &mut SCRATCH_BUF };

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
            let _ = core::fmt::write(&mut filename, format_args!("crash_{}.log", current_idx));
            let log_key = string_to_key(filename.as_str());

            let _ = block_on(async {
                let _ = sequential_storage::map::store_item(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    buf,
                    &log_key,
                    &content.as_bytes(),
                )
                .await;
            });

            // Increment index modulo 5
            let next_idx = (current_idx + 1) % 5;
            let next_bytes = next_idx.to_le_bytes();
            let idx_key = string_to_key("crash_idx");
            let _ = block_on(async {
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
            let dir_buf = unsafe { &mut SCRATCH_BUF };
            static mut CURRENT_DIR_STR: heapless::String<128> = heapless::String::new();
            let current_dir = unsafe { &mut CURRENT_DIR_STR };
            current_dir.clear();
            let dir_key = string_to_key(".dir");
            let existing_dir = block_on(async {
                sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut mut_flash,
                    config.range.clone(),
                    &mut cache,
                    dir_buf,
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
                let dir_write_buf = unsafe { &mut SCRATCH_BUF };
                let _ = block_on(async {
                    let _ = sequential_storage::map::store_item(
                        &mut mut_flash,
                        config.range.clone(),
                        &mut cache,
                        dir_write_buf,
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

#[cfg(test)]
mod tests;
