use model::calibration::{
    FourPointCalibration, FourPointRef, TwoPointCalibration, Vl53l0xCalibration,
};
use model::types::Direction;

#[test]
fn test_two_point_calibration_map() {
    // Standard mapping: low < high
    let cal = TwoPointCalibration::new_with_range(20, 120, 0, 100);
    assert_eq!(cal.map(20), 0);
    assert_eq!(cal.map(120), 100);
    assert_eq!(cal.map(70), 50);
    assert_eq!(cal.map(10), 0);
    assert_eq!(cal.map(170), 150);

    // Inverse mapping: low > high
    let cal_inv = TwoPointCalibration::new_with_range(100, 20, 0, 100);
    assert_eq!(cal_inv.map(100), 0);
    assert_eq!(cal_inv.map(20), 100);
    assert_eq!(cal_inv.map(60), 50);
    assert_eq!(cal_inv.map(110), 0);
    assert_eq!(cal_inv.map(10), 100);
}

#[test]
fn test_two_point_calibration_edge_cases() {
    // Equal low and high: low == high
    let cal_equal = TwoPointCalibration::new_with_range(50, 50, 0, 100);
    assert_eq!(cal_equal.map(0), 0);
    assert_eq!(cal_equal.map(50), 50);
    assert_eq!(cal_equal.map(100), 100);

    // Test custom min_range and max_range
    let cal_custom = TwoPointCalibration::new_with_range(20, 120, 10, 50);
    assert_eq!(cal_custom.map(20), 10);
    assert_eq!(cal_custom.map(120), 50);
    assert_eq!(cal_custom.map(70), 30);

    // Scale is 0 (min_range == max_range)
    let cal = TwoPointCalibration::new_with_range(10, 110, 5, 5);
    assert_eq!(cal.map(60), 5);
}

#[test]
fn test_four_point_calibration() {
    let mut cal = FourPointCalibration::new(10, 20, 30, 40);

    // Test indexing
    assert_eq!(cal[FourPointRef::Low], 10);
    assert_eq!(cal[FourPointRef::Mid], 20);
    assert_eq!(cal[FourPointRef::High], 30);
    assert_eq!(cal[FourPointRef::Overload], 40);

    // Test indexing mut
    cal[FourPointRef::Low] = 15;
    cal[FourPointRef::Mid] = 25;
    cal[FourPointRef::High] = 35;
    cal[FourPointRef::Overload] = 45;

    assert_eq!(cal[FourPointRef::Low], 15);
    assert_eq!(cal[FourPointRef::Mid], 25);
    assert_eq!(cal[FourPointRef::High], 35);
    assert_eq!(cal[FourPointRef::Overload], 45);
}

#[test]
fn test_vl53l0x_calibration() {
    let mut cal = Vl53l0xCalibration::default();

    // Default values should be (0, 0) for all sensors
    assert_eq!(cal[Direction::North].low, 0);
    assert_eq!(cal[Direction::North].high, 0);

    let sensor_cal = TwoPointCalibration::new(10, 100);
    cal[Direction::North] = sensor_cal;
    cal[Direction::East] = TwoPointCalibration::new(20, 120);
    cal[Direction::West] = TwoPointCalibration::new(30, 130);

    assert_eq!(cal[Direction::North].low, 10);
    assert_eq!(cal[Direction::North].high, 100);
    assert_eq!(cal[Direction::East].low, 20);
    assert_eq!(cal[Direction::West].low, 30);
}

#[test]
fn test_cbor_serialization_structure() {
    let mut cal = Vl53l0xCalibration::default();
    cal[Direction::North] = TwoPointCalibration::new(38, 20);
    cal[Direction::East] = TwoPointCalibration::new(42, 20);
    cal[Direction::West] = TwoPointCalibration::new(48, 20);

    let mut buf = [0u8; 128];
    let cursor = minicbor::encode::write::Cursor::new(&mut buf[..]);
    let mut encoder = minicbor::Encoder::new(cursor);
    encoder.encode(cal).unwrap();
    let len = encoder.into_writer().position();

    // Now let's decode it back
    let decoded = minicbor::decode::<Vl53l0xCalibration>(&buf[..len]).unwrap();
    assert_eq!(decoded[Direction::North].low, 38);
    assert_eq!(decoded[Direction::North].high, 20);
    assert_eq!(decoded[Direction::East].low, 42);
    assert_eq!(decoded[Direction::East].high, 20);
    assert_eq!(decoded[Direction::West].low, 48);
    assert_eq!(decoded[Direction::West].high, 20);
}
