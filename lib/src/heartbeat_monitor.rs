//! Heartbeat and cooperative task stall monitoring library.

#![deny(missing_docs)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_rt::exception;

/// Keeps track of the last time the executor made progress (in absolute system time milliseconds).
pub static LAST_EXECUTOR_PROGRESS: AtomicU32 = AtomicU32::new(0);

/// Tracks if a stuck task has been detected.
pub static MONITOR_STUCK_DETECTED: AtomicBool = AtomicBool::new(false);

/// Type alias for the stuck task callback mutex.
pub type StuckCallbackMutex = embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<Option<fn()>>,
>;

/// Thread-safe registry for a custom callback when a stuck task is detected.
pub static ON_STUCK_DETECTED: StuckCallbackMutex =
    embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(None));

/// Type alias for the monitor config mutex.
type MonitorConfigMutex = embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<MonitorConfig>,
>;

/// Global config for the heartbeat monitor.
static MONITOR_CONFIG: MonitorConfigMutex =
    embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(MonitorConfig {
        timeout_ms: 10_000,
        warn_threshold_pct: 80,
        last_warn_time_ms: 0,
    }));

struct MonitorConfig {
    timeout_ms: u32,
    warn_threshold_pct: u32,
    last_warn_time_ms: u32,
}

/// Controls whether the heartbeat loop actually updates progress (useful for simulating stuck tasks in testing).
pub static HEARTBEAT_ACTIVE: AtomicBool = AtomicBool::new(true);

/// The heartbeat task that periodically updates the executor progress timestamp.
#[embassy_executor::task]
pub async fn heartbeat_task(interval_ms: u32) -> ! {
    heartbeat_loop(interval_ms).await
}

/// The inner loop of the heartbeat task, exposed for testing and direct execution.
pub async fn heartbeat_loop(interval_ms: u32) -> ! {
    loop {
        if HEARTBEAT_ACTIVE.load(Ordering::Acquire) {
            let now_ms = embassy_time::Instant::now().as_millis() as u32;
            LAST_EXECUTOR_PROGRESS.store(now_ms, Ordering::Release);
        }
        embassy_time::Timer::after(embassy_time::Duration::from_millis(interval_ms as u64)).await;
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
static MONITOR_THREAD_SPAWNED: AtomicBool = AtomicBool::new(false);

/// Initializes the heartbeat monitor parameters and starts the background monitor.
pub fn init(timeout_ms: u32, warn_threshold_pct: u32) {
    let now_ms = embassy_time::Instant::now().as_millis() as u32;
    LAST_EXECUTOR_PROGRESS.store(now_ms, Ordering::Release);
    MONITOR_STUCK_DETECTED.store(false, Ordering::Release);

    critical_section::with(|cs| {
        let mut cfg = MONITOR_CONFIG.borrow(cs).borrow_mut();
        cfg.timeout_ms = timeout_ms;
        cfg.warn_threshold_pct = warn_threshold_pct;
        cfg.last_warn_time_ms = 0;
    });

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        // Configure the SysTick interrupt to fire periodically (every 1 second)
        // System clock on RP2040 is typically 125 MHz.
        // We set SysTick to fire every 1 second (125,000,000 cycles).
        unsafe {
            let syst = &*cortex_m::peripheral::SYST::PTR;
            let ticks = embassy_rp::clocks::clk_sys_freq();
            syst.rvr.write(ticks - 1);
            syst.cvr.write(0);
            syst.csr.write(0x07); // Enable SysTick, source = processor clock, enable interrupt
        }
    }

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        if !MONITOR_THREAD_SPAWNED.swap(true, Ordering::SeqCst) {
            // On host, spawn a background monitoring thread
            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(20));
                unsafe {
                    check_liveness_host();
                }
            });
        }
    }
}

/// Perform the stuck detection check logic on host.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
unsafe fn check_liveness_host() {
    let now_ms = embassy_time::Instant::now().as_millis() as u32;
    let last_progress = LAST_EXECUTOR_PROGRESS.load(Ordering::Acquire);
    let elapsed = now_ms.saturating_sub(last_progress);

    let (timeout_ms, warn_pct) = critical_section::with(|cs| {
        let cfg = MONITOR_CONFIG.borrow(cs).borrow();
        (cfg.timeout_ms, cfg.warn_threshold_pct)
    });

    let warn_threshold = (timeout_ms * warn_pct) / 100;
    if elapsed >= timeout_ms {
        MONITOR_STUCK_DETECTED.store(true, Ordering::Release);
        eprintln!(
            "Warning: Stuck task detected on host! Stalled for {}ms (limit: {}ms)",
            elapsed, timeout_ms
        );
        critical_section::with(|cs| {
            if let Some(cb) = *ON_STUCK_DETECTED.borrow(cs).borrow() {
                cb();
            }
        });
    } else if elapsed >= warn_threshold {
        let should_warn = critical_section::with(|cs| {
            let mut cfg = MONITOR_CONFIG.borrow(cs).borrow_mut();
            if now_ms.saturating_sub(cfg.last_warn_time_ms) >= 1000 {
                cfg.last_warn_time_ms = now_ms;
                true
            } else {
                false
            }
        });
        if should_warn {
            eprintln!(
                "Warning: Heartbeat stalled! Stuck threshold: {}%, elapsed: {}ms",
                warn_pct, elapsed
            );
        }
    }
}

/// Periodic SysTick exception handler that monitors execution progress.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[exception]
unsafe fn SysTick() {
    let now_ms = embassy_time::Instant::now().as_millis() as u32;
    let last_progress = LAST_EXECUTOR_PROGRESS.load(Ordering::Acquire);
    let elapsed = now_ms.saturating_sub(last_progress);

    let (timeout_ms, warn_pct) = critical_section::with(|cs| {
        let cfg = MONITOR_CONFIG.borrow(cs).borrow();
        (cfg.timeout_ms, cfg.warn_threshold_pct)
    });

    let warn_threshold = (timeout_ms * warn_pct) / 100;
    if elapsed >= timeout_ms {
        MONITOR_STUCK_DETECTED.store(true, Ordering::Release);

        // Preempt the stack frame to find where we got stuck
        let sp: u32;
        core::arch::asm!("mov {}, sp", out(reg) sp);
        // On exception entry, hardware pushes 8 words. Link Register (LR) is at sp + 20, PC is at sp + 24.
        let stack_ptr = sp as *const u32;
        let preempted_pc = *stack_ptr.add(6);
        let preempted_lr = *stack_ptr.add(5);

        defmt::error!(
            "Heartbeat Monitor: STUCK TASK DETECTED! Stalled for {}ms (threshold: {}ms). PC: {=u32:08x}, LR: {=u32:08x}. Triggering crash log...",
            elapsed,
            timeout_ms,
            preempted_pc,
            preempted_lr
        );

        critical_section::with(|cs| {
            if let Some(cb) = *ON_STUCK_DETECTED.borrow(cs).borrow() {
                cb();
            }
        });

        // Trigger target-specific crash log dump and system reset
        // Reuse constants mapped in projects/cat_detector/src/lib.rs
        crate::panic_handler::report_stuck_task_with_sizes::<
            { 2 * 1024 * 1024 }, // FLASH_SIZE
            0x2004_2000,         // STACK_TOP
            0x1000_0000,         // FLASH_START
            0x1020_0000,         // FLASH_END
            1,                   // WRITE_SIZE
            4096,                // ERASE_SIZE
        >(sp, preempted_pc, preempted_lr);
    } else if elapsed >= warn_threshold {
        let should_warn = critical_section::with(|cs| {
            let mut cfg = MONITOR_CONFIG.borrow(cs).borrow_mut();
            if now_ms.saturating_sub(cfg.last_warn_time_ms) >= 1000 {
                cfg.last_warn_time_ms = now_ms;
                true
            } else {
                false
            }
        });
        if should_warn {
            defmt::warn!(
                "Heartbeat Monitor: Task execution stalled! Stuck threshold: {}%, elapsed: {}ms",
                warn_pct,
                elapsed
            );
        }
    }
}
