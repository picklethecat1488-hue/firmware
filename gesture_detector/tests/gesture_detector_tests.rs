use gesture_detector::GestureDetector;
use model::types::Gesture;

#[test]
fn test_gesture_detector_debounce() {
    let mut detector = GestureDetector::new(100);

    // 1. All out of range -> returns Some(ProximityNotDetected), no duration
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 1000, 1000), 1_000_000),
        Some(Gesture::ProximityNotDetected)
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 2. Only West in range of proximity (< 300) -> returns Some(ProximityDetected), no long press duration
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 1000, 150), 2_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 3. Both in long press range (< 100) -> starts accumulating (returns Some(ProximityDetected) first)
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 50), 4_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 0);

    // Accumulates 2 seconds (returns Some(ProximityDetected))
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 50), 6_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 2000);

    // 4. One drops out of long press range -> reset to 0 (but proximity is still detected)
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 120), 7_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 0);

    // 5. Both back in long press range -> starts accumulating again (returns Some(ProximityDetected))
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 50), 10_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 0);

    // Accumulates 3 seconds (returns Some(ProximityDetected))
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 50), 13_000_000),
        Some(Gesture::ProximityDetected)
    );
    assert_eq!(detector.press_time_ms(), 3000);

    // Reaches 5 seconds -> triggers Some(DualLongPress) (takes precedence over ProximityDetected)
    assert_eq!(
        detector.update(Gesture::Proximity(1000, 50, 50), 15_000_000),
        Some(Gesture::DualLongPress)
    );
    assert_eq!(detector.press_time_ms(), 5000);

    // 6. Passing an input that is not Proximity should return None
    assert_eq!(detector.update(Gesture::DualLongPress, 16_000_000), None);
}
