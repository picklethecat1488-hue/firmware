#![deny(missing_docs)]
#![allow(static_mut_refs)]

//! Generalized modular panic handler for ARMv6+ architectures (e.g. Cortex-M).
//! Automatically dumps panics, stack traces, register dumps, and circular system log buffers
//! to a rolling flash memory partition using target-agnostic flash abstractions.

pub use crate::types::LogBuffer;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use crate::core_monitor;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use crate::types::CpuId;
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use crate::types::PanicConfig;

/// Static safe instance of the log buffer.
pub static CRASH_LOG_BUFFER: critical_section::Mutex<core::cell::RefCell<LogBuffer>> =
    critical_section::Mutex::new(core::cell::RefCell::new(LogBuffer::new()));

/// Helper function to serialize a `CrashDump` structure into CBOR format.
pub fn serialize_crash_dump<'a>(
    dump: &crate::types::CrashDump<'a>,
    buf: &'a mut [u8],
) -> Result<usize, minicbor::encode::Error<minicbor::encode::write::EndOfSlice>> {
    let mut encoder = minicbor::Encoder::new(minicbor::encode::write::Cursor::new(buf));
    encoder.encode(dump)?;
    Ok(encoder.into_writer().position())
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

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global static storing the stack top address of Core 1 (set on boot)
pub static mut CORE1_STACK_TOP: u32 = 0;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(clippy::declare_interior_mutable_const)]
const INIT_STATE: crate::types::CorePanicState = crate::types::CorePanicState {
    panicked: core::sync::atomic::AtomicBool::new(false),
    r0: 0,
    r1: 0,
    r2: 0,
    r3: 0,
    sp: 0,
    lr: 0,
    pc: 0,
    stack_top: 0,
    message: [0u8; 64],
    message_len: 0,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Shared static instance of captured panic state for all cores.
pub static mut PANIC_STATE: [crate::types::CorePanicState; core_monitor::NUM_CORES] =
    [INIT_STATE; core_monitor::NUM_CORES];

/// Initialize the panic handler with flash access, target partition settings, and filesystem buffer.
pub fn init(
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    flash: &'static mut dyn crate::types::PanicFlash,
    #[cfg(all(target_arch = "arm", target_os = "none"))] range: core::ops::Range<u32>,
    #[cfg(all(target_arch = "arm", target_os = "none"))] fs_buf: &'static mut [u8],
    #[cfg(all(target_arch = "arm", target_os = "none"))] max_crash_logs: u32,
) {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    critical_section::with(|cs| {
        PANIC_CONFIG.borrow(cs).replace(Some(PanicConfig {
            flash,
            range,
            fs_buf,
            max_crash_logs,
        }));
    });
}

/// Heuristic stack scanner that walks a slice of stack words and extracts return PCs
/// by verifying that the caller instruction before the return address is a BL or BLX.
pub fn scan_stack<F>(
    stack: &[u32],
    flash_start: u32,
    flash_end: u32,
    pcs: &mut [u32; 32],
    read_mem: F,
) -> usize
where
    F: Fn(u32) -> Option<u32>,
{
    let mut pc_count = 0;
    for &val in stack {
        if pc_count >= 32 {
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
                // 16-bit Thumb BLX: h2 matches 0x4780..0x47FF (BLX <reg>)
                let is_blx_16 = (h2 & 0xFF80) == 0x4780;

                if is_bl_blx_32 || is_blx_16 {
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
    pcs: &mut [u32; 32],
    read_mem: F,
) -> usize
where
    F: Fn(u32) -> Option<u32>,
{
    if sp < stack_top {
        let words = (stack_top - sp) / 4;
        let limit = words.min(crate::types::STACK_SCAN_LIMIT as usize);
        let stack_slice = unsafe { core::slice::from_raw_parts(sp as *const u32, limit) };
        scan_stack(stack_slice, flash_start, flash_end, pcs, read_mem)
    } else {
        0
    }
}

/// Extracts circular system logs from the CRASH_LOG_BUFFER into the provided buffer.
pub fn extract_system_logs(cs: &critical_section::CriticalSection, log_buf: &mut [u8]) -> usize {
    let buffer = CRASH_LOG_BUFFER.borrow(*cs).borrow();
    let mut write_idx = 0;
    let mut read_idx = buffer.tail;

    while read_idx != buffer.head {
        // Read 2-byte length prefix
        let l_high = buffer.buffer[read_idx];
        let l_low = buffer.buffer[(read_idx + 1) % crate::types::CRASH_LOG_BUFFER_SIZE];
        let len = ((l_high as usize) << 8) | (l_low as usize);

        // Advance read_idx past length prefix
        read_idx = (read_idx + 2) % crate::types::CRASH_LOG_BUFFER_SIZE;

        // Copy frame data
        for i in 0..len {
            if write_idx < log_buf.len() {
                log_buf[write_idx] =
                    buffer.buffer[(read_idx + i) % crate::types::CRASH_LOG_BUFFER_SIZE];
                write_idx += 1;
            }
        }

        // Advance read_idx past frame data
        read_idx = (read_idx + len) % crate::types::CRASH_LOG_BUFFER_SIZE;
    }
    write_idx
}

/// Writes the serialized crash dump, increments the rolling index, and updates the directory listing.
pub async fn write_crash_log_to_flash<F>(
    flash: &mut F,
    range: core::ops::Range<u32>,
    cache: &mut sequential_storage::cache::NoCache,
    buf: &mut [u8],
    encoded_bytes: &[u8],
    max_crash_logs: u32,
) -> Result<(), F::Error>
where
    F: embedded_storage_async::nor_flash::NorFlash,
{
    use crate::directory::string_to_key;

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

    match sequential_storage::map::store_item(
        flash,
        range.clone(),
        cache,
        buf,
        &log_key,
        &encoded_bytes,
    )
    .await
    {
        Ok(_) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!(
                "Successfully stored crash log to flash: {}",
                filename.as_str()
            );
        }
        Err(_e) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!(
                "Failed to store crash log to flash: {:?}",
                defmt::Debug2Format(&_e)
            );
            return Ok(());
        }
    }

    // Increment index modulo max_crash_logs
    let next_idx = (current_idx + 1) % max_crash_logs;
    let next_bytes = next_idx.to_le_bytes();
    let idx_key = string_to_key("crash_idx");
    if let Err(_e) = sequential_storage::map::store_item(
        flash,
        range.clone(),
        cache,
        buf,
        &idx_key,
        &&next_bytes[..],
    )
    .await
    {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::error!(
            "Failed to update crash_idx in flash: {:?}",
            defmt::Debug2Format(&_e)
        );
    }

    // Read the directory index (.dir) file
    let dir_key = crate::directory::string_to_key(".dir");
    let existing_dir_res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
        flash,
        range.clone(),
        cache,
        buf,
        &dir_key,
    )
    .await;

    let mut existing_dir_str = "";
    if let Ok(Some(bytes)) = existing_dir_res {
        if let Ok(s) = core::str::from_utf8(bytes) {
            existing_dir_str = s;
        }
    }

    // Use shared directory manager to append if not present
    if let Some(new_dir) = crate::directory::add_to_directory(existing_dir_str, filename.as_str()) {
        // Store updated dir
        if let Err(_e) = sequential_storage::map::store_item(
            flash,
            range.clone(),
            cache,
            buf,
            &dir_key,
            &new_dir.as_bytes(),
        )
        .await
        {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!(
                "Failed to update .dir in flash: {:?}",
                defmt::Debug2Format(&_e)
            );
        }
    }

    Ok(())
}

/// Keeps track of core processor (mainly ARM) state.
#[derive(Default)]
pub struct CoreState {
    /// The value of the r0 register.
    pub r0: u32,
    /// The value of the r1 register.
    pub r1: u32,
    /// The value of the r2 register.
    pub r2: u32,
    /// The value of the r3 register.
    pub r3: u32,
    /// Return PCs backtrace array.
    pub backtrace: [u32; 32],
}

/// Helper function to generate a stable, content-addressable UUID from crash context variables.
/// This ensures that two crashes with the exact same register state, backtrace, and code revision
/// will result in the same UUID, facilitating crash grouping and deduplication.
pub fn generate_uuid(state: &CoreState, revision_hash: &str) -> [u8; 16] {
    let mut uuid = [0u8; 16];

    let mut h1 = 0xcbf29ce484222325u64;
    let mut h2 = 0x84222325cbf29ce4u64;

    let mut feed_u64 = |val: u64| {
        h1 ^= val;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 ^= val.rotate_left(17);
        h2 = h2.wrapping_mul(0x100000001b3);
    };

    feed_u64(state.r0 as u64);
    feed_u64(state.r1 as u64);
    feed_u64(state.r2 as u64);
    feed_u64(state.r3 as u64);
    for &pc in state.backtrace.iter() {
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
#[link_section = ".data.ram_func"]
#[inline(never)]
fn panic_loop() -> ! {
    unsafe {
        // Enable interrupts so Core 1 can service the SIO inter-core interrupt handler in RAM
        core::arch::asm!("cpsie i");
        loop {
            core::arch::asm!("nop");
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Shared panic handler logic executing crash dump logging to flash memory with customizable write/erase sizes.
pub fn handle_panic_with_sizes<
    const FLASH_SIZE: usize,
    const FLASH_START: u32,
    const FLASH_END: u32,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
>(
    info: &core::panic::PanicInfo,
    cpu_id: CpuId,
    stack_top: u32,
) -> ! {
    let mut msg_buf = [0u8; 64];
    struct BufferWriter<'a> {
        buf: &'a mut [u8],
        pos: usize,
    }
    impl<'a> core::fmt::Write for BufferWriter<'a> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();
            let len = bytes.len().min(self.buf.len() - self.pos);
            self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
            self.pos += len;
            Ok(())
        }
    }
    let mut writer = BufferWriter {
        buf: &mut msg_buf,
        pos: 0,
    };
    let _ = core::fmt::write(&mut writer, format_args!("{}", info.message()));
    let message_len = writer.pos as u8;

    let idx = match cpu_id {
        CpuId::Core0 => 0,
        CpuId::Core1 => 1,
    };

    unsafe {
        let ptr = core::ptr::addr_of_mut!(PANIC_STATE);
        (*ptr)[idx].capture_snapshot(stack_top, &msg_buf, message_len);

        (*ptr)[idx]
            .panicked
            .store(true, core::sync::atomic::Ordering::Release);
    }
    core_monitor::set_core_panicked(cpu_id, true);

    unsafe {
        // Disable SysTick timer so it doesn't try to execute exceptions from FLASH
        let syst = &*cortex_m::peripheral::SYST::PTR;
        syst.csr.write(0);
    }

    if cpu_id != CpuId::Core0 {
        // Put the core into a deadloop state until core0 and service the panic request.
        panic_loop();
    }
    log_crash_and_reset_impl::<FLASH_SIZE, FLASH_START, FLASH_END, WRITE_SIZE, ERASE_SIZE>();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
fn log_crash_and_reset_impl<
    const FLASH_SIZE: usize,
    const FLASH_START: u32,
    const FLASH_END: u32,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
>() -> ! {
    let (core0_panicked, core1_panicked) = unsafe {
        let ptr = core::ptr::addr_of!(PANIC_STATE);
        (
            (*ptr)[0]
                .panicked
                .load(core::sync::atomic::Ordering::Acquire),
            (*ptr)[1]
                .panicked
                .load(core::sync::atomic::Ordering::Acquire),
        )
    };

    defmt::error!("--- PANIC DETECTED ---");
    if core0_panicked {
        let state = unsafe {
            let ptr = core::ptr::addr_of!(PANIC_STATE);
            (*ptr)[0].volatile_clone()
        };
        defmt::error!(
            "Core 0: SP={=u32:08x} PC={=u32:08x} LR={=u32:08x}",
            state.sp,
            state.pc,
            state.lr
        );
        if state.message_len > 0 {
            let len = (state.message_len as usize).min(64);
            if let Ok(msg_str) = core::str::from_utf8(&state.message[..len]) {
                defmt::error!("  Message: {}", msg_str);
            }
        }
    }
    if core1_panicked {
        let state = unsafe {
            let ptr = core::ptr::addr_of!(PANIC_STATE);
            (*ptr)[1].volatile_clone()
        };
        defmt::error!(
            "Core 1: SP={=u32:08x} PC={=u32:08x} LR={=u32:08x}",
            state.sp,
            state.pc,
            state.lr
        );
        if state.message_len > 0 {
            let len = (state.message_len as usize).min(64);
            if let Ok(msg_str) = core::str::from_utf8(&state.message[..len]) {
                defmt::error!("  Message: {}", msg_str);
            }
        }
    }

    let generate_core_dump = |state: crate::types::CorePanicState, panicked: bool| {
        let mut backtrace = [0u32; 32];
        let read_mem = |addr: u32| {
            if (FLASH_START..FLASH_END).contains(&addr) && (addr & 1 == 0) {
                // Read as two 16-bit halfwords to prevent alignment HardFaults on Cortex-M0+
                let low = unsafe { *(addr as *const u16) } as u32;
                let high = unsafe { *((addr + 2) as *const u16) } as u32;
                Some((high << 16) | low)
            } else {
                None
            }
        };
        let mut pc_count = 0;
        if state.sp != 0 && state.stack_top != 0 {
            pc_count = scan_stack_from_sp(
                state.sp as usize,
                state.stack_top as usize,
                FLASH_START,
                FLASH_END,
                &mut backtrace,
                read_mem,
            );
        }

        crate::types::CoreDump {
            r0: state.r0,
            r1: state.r1,
            r2: state.r2,
            r3: state.r3,
            sp: state.sp,
            lr: state.lr,
            pc: state.pc,
            backtrace,
            backtrace_len: pc_count as u32,
            panicked,
        }
    };

    let core0_dump = generate_core_dump(
        unsafe {
            let ptr = core::ptr::addr_of!(PANIC_STATE);
            (*ptr)[0].volatile_clone()
        },
        core0_panicked,
    );
    let core1_dump = generate_core_dump(
        unsafe {
            let ptr = core::ptr::addr_of!(PANIC_STATE);
            (*ptr)[1].volatile_clone()
        },
        core1_panicked,
    );

    let primary_dump = if core1_panicked {
        &core1_dump
    } else {
        &core0_dump
    };
    let primary_core_state = CoreState {
        r0: primary_dump.r0,
        r1: primary_dump.r1,
        r2: primary_dump.r2,
        r3: primary_dump.r3,
        backtrace: primary_dump.backtrace,
    };
    let uuid = generate_uuid(&primary_core_state, env!("GIT_HASH"));

    // A single static scratch buffer used for log extraction and CBOR serialization.
    // Partitioned as:
    //   - scratch[0..1500]: Used for log extraction (1500 bytes).
    //   - scratch[1500..2700]: Used for CBOR serialization buffer (1200 bytes).
    static mut SCRATCH_BUF: [u8; 2700] = [0u8; 2700];
    let (log_buf, cbor_buf) = unsafe { SCRATCH_BUF.split_at_mut(1500) };

    // Extract logs from CRASH_LOG_BUFFER into a contiguous slice
    let logs_len = critical_section::with(|cs| extract_system_logs(&cs, log_buf));
    let logs_slice = &log_buf[..logs_len];

    let dump = crate::types::CrashDump {
        revision_hash: env!("GIT_HASH"),
        system_logs: logs_slice,
        uuid,
        cores: [core0_dump, core1_dump],
    };

    // Serialize CrashDump into a buffer
    let mut encoded_len = 0;
    if let Ok(len) = serialize_crash_dump(&dump, cbor_buf) {
        encoded_len = len;
    }
    let encoded_bytes = &cbor_buf[..encoded_len];
    defmt::error!("Crash Dump: {=[u8]:cbor}", encoded_bytes);

    // Write crash log to storage partition using rolling index
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
                    config.fs_buf,
                    encoded_bytes,
                    config.max_crash_logs,
                )
                .await;
            });
        }
    });

    // Give the host RTT client time to read and drain the buffers before CPU reset (1 second delay)
    delay_us(1_000_000, CpuId::Core0);

    cortex_m::peripheral::SCB::sys_reset();
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
fn delay_us(us: u32, cpu_id: CpuId) {
    let freq = core_monitor::CORE_MONITORS[cpu_id as usize]
        .sys_clock_hz
        .load(core::sync::atomic::Ordering::Relaxed);
    let cycles = us.saturating_mul(freq / 1_000_000);
    cortex_m::asm::delay(cycles);
}
