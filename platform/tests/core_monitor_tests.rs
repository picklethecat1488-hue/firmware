use core::sync::atomic::Ordering;
use platform::core_monitor;

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

#[test]
fn test_once_lock_flow() {
    let lock: platform::OnceLock<u32> = platform::OnceLock::new();
    assert_eq!(lock.get(), None);

    assert!(lock.set(42).is_ok());
    assert_eq!(*lock.wait(), 42);
    assert_eq!(lock.get(), Some(&42));

    assert!(lock.set(100).is_err());
}

#[test]
fn test_once_lock_blocking_wait() {
    use std::sync::Arc;
    let lock = Arc::new(platform::OnceLock::<u32>::new());
    let lock_clone = Arc::clone(&lock);

    let handle = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(15));
        assert!(lock_clone.set(99).is_ok());
    });

    assert_eq!(*lock.wait(), 99);
    handle.join().unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
struct SendPtr(pub *mut ());
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

#[test]
fn test_once_lock_integration_pointer_flow() {
    let lock: platform::OnceLock<SendPtr> = platform::OnceLock::new();
    assert!(lock.get().is_none());

    let mut val = 42u32;
    let ptr = SendPtr(&mut val as *mut u32 as *mut ());
    assert!(lock.set(ptr).is_ok());
    assert_eq!(*lock.wait(), ptr);
    assert_eq!(lock.get(), Some(&ptr));

    let mut val2 = 100u32;
    assert!(lock.set(SendPtr(&mut val2 as *mut u32 as *mut ())).is_err());
}

#[test]
fn test_once_lock_integration_pointer_blocking() {
    use std::sync::Arc;
    let lock = Arc::new(platform::OnceLock::<SendPtr>::new());
    let lock_clone = Arc::clone(&lock);

    let mut val = 123u32;
    let ptr = SendPtr(&mut val as *mut u32 as *mut ());

    let handle = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(15));
        assert!(lock_clone.set(ptr).is_ok());
    });

    assert_eq!(*lock.wait(), ptr);
    handle.join().unwrap();
}
