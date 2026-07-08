use embassy_time::Duration;
use firmware_lib::periodic_timer::PeriodicTimer;

#[test]
fn test_periodic_timer_flow() {
    let interval = Duration::from_millis(50);
    let mut timer = PeriodicTimer::new(interval);

    // 1. Initially it should not be expired
    assert!(!timer.expired());
    assert!(timer.remaining_ms() <= 50);

    // 2. Wait 60ms
    std::thread::sleep(std::time::Duration::from_millis(60));

    // 3. Now it should be expired
    assert!(timer.expired());
    assert_eq!(timer.remaining_ms(), 0);

    // 4. Read elapsed time and reset
    let elapsed = timer.elapsed_ms_and_reset();
    assert!(elapsed >= 60);

    // 5. After reset, it should not be expired again
    assert!(!timer.expired());
    assert!(timer.remaining_ms() <= 50);

    // 6. expired_and_reset should return None initially
    assert_eq!(timer.expired_and_reset(), None);

    // 7. Wait 60ms again
    std::thread::sleep(std::time::Duration::from_millis(60));

    // 8. expired_and_reset should return Some(elapsed) and reset the timer
    let res = timer.expired_and_reset();
    assert!(res.is_some());
    assert!(res.unwrap() >= 60);

    // 9. Now it should not be expired again
    assert_eq!(timer.expired_and_reset(), None);
}
