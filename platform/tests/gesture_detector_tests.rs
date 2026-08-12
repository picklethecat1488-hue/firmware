use model::types::Gesture;
use platform::gesture_detector::{GestureDetector, ProximityGestureDetector};

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
    let mut detector = ProximityGestureDetector::new(20, 100, 300);

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

    // Accumulates 1 second -> returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 5_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 1000);

    // 4. One drops out of press range -> reset to 0 (returns None)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 25, 6_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 5. Both back in press range -> starts accumulating again (returns None)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 7_000_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 0);

    // Accumulates 1.5 seconds -> returns None
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 8_500_000),
        None
    );
    assert_eq!(detector.press_time_ms(), 1500);

    // Reaches 2 seconds -> triggers Some(DualLongPress)
    assert_eq!(
        update_detector(&mut detector, 1000, 15, 15, 9_000_000),
        Some(Gesture::DualLongPress)
    );
    assert_eq!(detector.press_time_ms(), 2000);
}

#[test]
fn test_gesture_detector_proximity_states() {
    use model::types::Direction;
    use platform::gesture_detector::ProximityState;

    let mut detector = ProximityGestureDetector::new(20, 100, 300);

    // Initial state: OutOfRange
    for tracker in &detector.trackers {
        assert_eq!(tracker.state, ProximityState::OutOfRange);
    }

    // 1. Enter InRange (< 300)
    detector.register_distance(Direction::East, 250);
    assert_eq!(detector.trackers[1].state, ProximityState::InRange);

    // 2. Enter Near (< 100)
    detector.register_distance(Direction::East, 80);
    assert_eq!(detector.trackers[1].state, ProximityState::Near);

    // 3. Enter Down (<= 20)
    detector.register_distance(Direction::East, 15);
    assert_eq!(detector.trackers[1].state, ProximityState::Down);

    // 4. Back to Near
    detector.register_distance(Direction::East, 50);
    assert_eq!(detector.trackers[1].state, ProximityState::Near);

    // 5. Back to InRange
    detector.register_distance(Direction::East, 120);
    assert_eq!(detector.trackers[1].state, ProximityState::InRange);

    // 6. Back to OutOfRange
    detector.register_distance(Direction::East, 400);
    assert_eq!(detector.trackers[1].state, ProximityState::OutOfRange);
}
