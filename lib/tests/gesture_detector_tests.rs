use firmware_lib::gesture_detector::{GestureDetector, ProximityGestureDetector};
use model::types::Gesture;

fn update_detector(
    detector: &mut ProximityGestureDetector,
    n: u16,
    e: u16,
    w: u16,
    time_us: u64,
) -> Option<Gesture> {
    detector.register_distance(model::types::Direction::North, n);
    detector.register_distance(model::types::Direction::East, e);
    detector.update((model::types::Direction::West, w), time_us)
}

#[test]
fn test_gesture_detector_debounce() {
    let mut detector = ProximityGestureDetector::new(20);

    // 1. All out of range -> no change, returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 1000, 1000, 1_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 2. Only West in range of button press (< 20) -> no change, returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 1000, 15, 2_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 3. Both in press range (< 20) -> starts accumulating (returns None)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 4_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // Accumulates 2 seconds -> returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 6_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 2000);

    // 4. One drops out of press range -> reset to 0 (returns None)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 25, 7_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 5. Both back in press range -> starts accumulating again (returns None)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 10_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // Accumulates 3 seconds -> returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 13_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 3000);

    // Reaches 5 seconds -> triggers Some(DualLongPress)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 15_000_000),
        Some(Gesture::DualLongPress)
    );
    assert_eq!(detector.press_time_ms(), 5000);
}
