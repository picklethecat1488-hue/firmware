use core::sync::atomic::Ordering;
use firmware_lib::core_monitor;

#[test]
fn test_core_monitor_flow() {
    let initial_progress = core_monitor::CORE_MONITORS[0]
        .last_executor_progress
        .load(Ordering::Acquire);

    // Spawn the Embassy executor on a separate OS thread since run() is blocking/infinite
    std::thread::spawn(move || {
        let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
        executor.run(|spawner| {
            core_monitor::init_core(Some(spawner), core_monitor::CpuId::Core0, 100, 80, true);
        });
    });

    // 1. Test heartbeat task runs and updates the progress timestamp
    {
        // Wait and verify that the heartbeat task updates the progress timestamp
        let mut updated = false;
        for _ in 0..50 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            let progress = core_monitor::CORE_MONITORS[0]
                .last_executor_progress
                .load(Ordering::Acquire);
            if progress > initial_progress {
                updated = true;
                break;
            }
        }

        assert!(updated, "Progress timestamp did not update!");
    }

    // 2. Test stuck task detection and callback execution
    {
        // Re-initialize heartbeat monitor with a very short timeout (200ms) and 80% warn threshold (160ms)
        core_monitor::init_core(None, core_monitor::CpuId::Core0, 200, 80, true);

        // Register a callback to verify it runs
        static CALLBACK_RUN: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);
        fn on_stuck() {
            CALLBACK_RUN.store(true, Ordering::Release);
        }

        critical_section::with(|cs| {
            core_monitor::ON_STUCK_DETECTED
                .borrow(cs)
                .replace(Some(on_stuck));
        });

        // Disable heartbeat updates to simulate a stuck/blocked executor
        core_monitor::HEARTBEAT_ACTIVE.store(false, Ordering::Release);

        // Verify it starts as not stuck
        let is_stuck_init = core_monitor::CORE_MONITORS[0]
            .stuck_detected
            .load(Ordering::Acquire);
        assert!(!is_stuck_init);

        // Simulate progress and then simulate stalling (meaning we don't update it anymore)
        let start_ms = embassy_time::Instant::now().as_millis() as u32;
        core_monitor::CORE_MONITORS[0]
            .last_executor_progress
            .store(start_ms, Ordering::Release);

        // Let 100ms pass. Since timeout is 200ms and warn is 160ms, no warning or stuck should be detected.
        std::thread::sleep(std::time::Duration::from_millis(100));
        let is_stuck_mid = core_monitor::CORE_MONITORS[0]
            .stuck_detected
            .load(Ordering::Acquire);
        assert!(!is_stuck_mid);
        assert!(!CALLBACK_RUN.load(Ordering::Acquire));

        // Wait for the background monitor thread to detect the stall and run the callback (timeout = 200ms)
        let mut callback_ran = false;
        for _ in 0..50 {
            if CALLBACK_RUN.load(Ordering::Acquire) {
                callback_ran = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        assert!(callback_ran, "Stuck task callback did not run!");
        let is_stuck_final = core_monitor::CORE_MONITORS[0]
            .stuck_detected
            .load(Ordering::Acquire);
        assert!(is_stuck_final);
    }
}
