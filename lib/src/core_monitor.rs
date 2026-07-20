//! Core execution and cooperative task stall monitoring library.

#![deny(missing_docs)]

use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_rt::exception;

pub use crate::types::{CoreMonitor, CoreStatus, CpuId};

/// Type alias for the stuck task callback mutex.
pub type StuckCallbackMutex = embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<Option<fn()>>,
>;

/// Thread-safe registry for a custom callback when a stuck task is detected.
pub static ON_STUCK_DETECTED: StuckCallbackMutex =
    embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(None));

/// Configurable number of processor cores.
#[cfg(feature = "dual-core")]
pub const NUM_CORES: usize = 2;
/// Configurable number of processor cores.
#[cfg(not(feature = "dual-core"))]
pub const NUM_CORES: usize = 1;

#[cfg(all(target_arch = "arm", target_os = "none", feature = "dual-core"))]
const SYSTICK_IRQ: usize = 15;

/// Global optional instances for each core.
pub static CORE_MONITORS: [CoreStatus; NUM_CORES] = [
    CoreStatus::new(CpuId::Core0),
    #[cfg(feature = "dual-core")]
    CoreStatus::new(CpuId::Core1),
];

/// Controls whether the heartbeat loop actually updates progress.
pub static HEARTBEAT_ACTIVE: AtomicBool = AtomicBool::new(true);

/// The heartbeat task that periodically updates the executor progress timestamp.
#[embassy_executor::task(pool_size = NUM_CORES)]
pub async fn heartbeat_task(cpu_id: CpuId, interval_ms: u32) -> ! {
    let idx = cpu_id as usize;
    loop {
        if HEARTBEAT_ACTIVE.load(Ordering::Acquire) {
            let now_ms = embassy_time::Instant::now().as_millis() as u32;
            if idx < NUM_CORES {
                CORE_MONITORS[idx].update_progress(now_ms);
            }
        }
        embassy_time::Timer::after(embassy_time::Duration::from_millis(interval_ms as u64)).await;
    }
}

/// Initialize the core monitor for a specific processor core and optionally spawn the heartbeat task.
pub fn init_core(
    spawner: Option<embassy_executor::Spawner>,
    cpu_id: CpuId,
    timeout_ms: u32,
    warn_threshold_pct: u32,
    enabled: bool,
) {
    let idx = cpu_id as usize;
    if idx < NUM_CORES {
        let monitor = &CORE_MONITORS[idx];
        assert!(
            monitor.cpuid() == cpu_id,
            "Core ID mismatch during initialization"
        );
        let now_ms = embassy_time::Instant::now().as_millis() as u32;
        monitor
            .last_executor_progress
            .store(now_ms, Ordering::Release);
        monitor
            .stuck_detection_enabled
            .store(enabled, Ordering::Release);
        monitor.stuck_detected.store(false, Ordering::Release);
        monitor.panicked.store(false, Ordering::Release);
        monitor.timeout_ms.store(timeout_ms, Ordering::Release);
        monitor
            .warn_threshold_pct
            .store(warn_threshold_pct, Ordering::Release);
        monitor.last_warn_time_ms.store(0, Ordering::Release);
    }
    init_systick(cpu_id);

    if let Some(spawner) = spawner {
        let heartbeat_interval = (timeout_ms / 10).clamp(10, 1000);
        spawner
            .spawn(heartbeat_task(cpu_id, heartbeat_interval))
            .unwrap();
    }
}

/// Mark a specific core as panicked.
pub fn set_core_panicked(cpu_id: CpuId, panicked: bool) {
    let idx = cpu_id as usize;
    if idx < NUM_CORES {
        CORE_MONITORS[idx].set_panicked(panicked);
    }
}

fn init_systick(cpu_id: CpuId) {
    let idx = cpu_id as usize;
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        let _ = cpu_id;
        unsafe {
            #[cfg(feature = "dual-core")]
            if cpu_id != CpuId::Core0 {
                init_multicore_monitor(cpu_id);
            }

            let syst = &*cortex_m::peripheral::SYST::PTR;
            let sys_clock_hz = embassy_rp::clocks::clk_sys_freq();
            if idx < NUM_CORES {
                CORE_MONITORS[idx]
                    .sys_clock_hz
                    .store(sys_clock_hz, Ordering::Relaxed);
            }
            syst.rvr.write(sys_clock_hz - 1);
            syst.cvr.write(0);
            syst.csr.write(0x07);
        }
    }

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        if idx < NUM_CORES
            && !CORE_MONITORS[idx]
                .mock_thread_spawned
                .swap(true, Ordering::SeqCst)
        {
            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(20));
                unsafe {
                    check_liveness_host(cpu_id);
                }
            });
        }
    }
}

/// Perform the stuck detection check logic on host.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
unsafe fn check_liveness_host(cpu_id: CpuId) {
    let idx = cpu_id as usize;
    if idx < NUM_CORES {
        CORE_MONITORS[idx].check_liveness(cpu_id);
    }
}

macro_rules! define_systick_handler {
    ($name:ident, $table_name:ident, $cpu:expr) => {
        #[cfg(all(target_arch = "arm", target_os = "none", feature = "dual-core"))]
        #[repr(align(256))]
        #[allow(dead_code)]
        struct AlignedTable([u32; 48]);

        #[cfg(all(target_arch = "arm", target_os = "none", feature = "dual-core"))]
        #[link_section = ".ram_vectors"]
        static mut $table_name: AlignedTable = AlignedTable([0; 48]);

        #[cfg(all(target_arch = "arm", target_os = "none", feature = "dual-core"))]
        #[no_mangle]
        /// SysTick exception handler for the designated secondary core.
        pub extern "C" fn $name() {
            if $cpu == CpuId::Core0 {
                panic!("Secondary core SysTick handler cannot be registered for Core 0");
            }
            let idx = $cpu as usize;
            CORE_MONITORS[idx].check_liveness($cpu);
        }
    };
}

#[cfg(all(target_arch = "arm", target_os = "none", feature = "dual-core"))]
unsafe fn init_multicore_monitor(cpu_id: CpuId) {
    let current_vtor = (*cortex_m::peripheral::SCB::PTR).vtor.read();
    let src = current_vtor as *const u32;
    let dest = match cpu_id {
        CpuId::Core1 => core::ptr::addr_of_mut!(CORE1_VECTOR_TABLE) as *mut u32,
        _ => return,
    };
    for i in 0..48 {
        core::ptr::write_volatile(dest.add(i), core::ptr::read_volatile(src.add(i)));
    }
    // Vector SYSTICK_IRQ is SysTick
    let handler = match cpu_id {
        CpuId::Core1 => systick_handler_core1 as *const () as u32,
        _ => return,
    };
    core::ptr::write_volatile(dest.add(SYSTICK_IRQ), handler);
    (*cortex_m::peripheral::SCB::PTR).vtor.write(dest as u32);
}

/// Periodic SysTick exception handler for Core 0.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[exception]
unsafe fn SysTick() {
    #[allow(clippy::needless_range_loop)]
    for i in 1..NUM_CORES {
        if CORE_MONITORS[i].is_panicked() {
            panic!("Core {} panicked", i);
        }
    }

    CORE_MONITORS[0].check_liveness(CpuId::Core0);
}

define_systick_handler!(systick_handler_core1, CORE1_VECTOR_TABLE, CpuId::Core1);

impl CoreMonitor for CoreStatus {
    fn check_liveness(&self, cpu_id: CpuId) {
        let _idx = cpu_id as usize;
        let now_ms = embassy_time::Instant::now().as_millis() as u32;
        let last_progress = self.last_executor_progress.load(Ordering::Acquire);
        let elapsed = now_ms.saturating_sub(last_progress);

        let timeout_ms = self.timeout_ms.load(Ordering::Acquire);
        let warn_pct = self.warn_threshold_pct.load(Ordering::Acquire);
        let warn_threshold = (timeout_ms * warn_pct) / 100;

        if self.stuck_detection_enabled.load(Ordering::Acquire) && elapsed >= timeout_ms {
            self.stuck_detected.store(true, Ordering::Release);

            #[cfg(all(target_arch = "arm", target_os = "none"))]
            {
                // Preempt the stack frame to find where we got stuck
                let sp: u32;
                unsafe {
                    core::arch::asm!("mov {}, sp", out(reg) sp);
                }
                let stack_ptr = sp as *const u32;
                let preempted_pc = unsafe { *stack_ptr.add(6) };
                let preempted_lr = unsafe { *stack_ptr.add(5) };

                defmt::error!(
                    "Core Monitor: STUCK TASK DETECTED on Core {}! Stalled for {}ms (threshold: {}ms). PC: {=u32:08x}, LR: {=u32:08x}. Triggering panic...",
                    _idx,
                    elapsed,
                    timeout_ms,
                    preempted_pc,
                    preempted_lr
                );
                panic!("stuck task");
            }
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            {
                eprintln!(
                    "Warning: Stuck task detected on host! Stalled for {}ms (limit: {}ms)",
                    elapsed, timeout_ms
                );
                // Trigger stuck callback on host
                critical_section::with(|cs| {
                    if let Some(cb) = *ON_STUCK_DETECTED.borrow(cs).borrow() {
                        cb();
                    }
                });
            }
        } else if self.stuck_detection_enabled.load(Ordering::Acquire) && elapsed >= warn_threshold
        {
            let should_warn = {
                let last_warn = self.last_warn_time_ms.load(Ordering::Acquire);
                if now_ms.saturating_sub(last_warn) >= 1000 {
                    self.last_warn_time_ms.store(now_ms, Ordering::Release);
                    true
                } else {
                    false
                }
            };
            if should_warn {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!(
                    "Core Monitor: Task execution stalled on Core {}! Stuck threshold: {}%, elapsed: {}ms",
                    _idx,
                    warn_pct,
                    elapsed
                );
                #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                eprintln!(
                    "Warning: Heartbeat stalled! Stuck threshold: {}%, elapsed: {}ms",
                    warn_pct, elapsed
                );
            }
        }
    }

    fn last_progress(&self) -> u32 {
        self.last_executor_progress.load(Ordering::Acquire)
    }

    fn update_progress(&self, now_ms: u32) {
        self.last_executor_progress.store(now_ms, Ordering::Release);
    }

    fn is_enabled(&self) -> bool {
        self.stuck_detection_enabled.load(Ordering::Acquire)
    }

    fn is_stuck(&self) -> bool {
        self.stuck_detected.load(Ordering::Acquire)
    }

    fn is_panicked(&self) -> bool {
        self.panicked.load(Ordering::Acquire)
    }

    fn set_panicked(&self, panicked: bool) {
        self.panicked.store(panicked, Ordering::Release);
    }

    fn cpuid(&self) -> CpuId {
        self.cpu_id
    }
}
