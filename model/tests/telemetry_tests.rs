use model::telemetry::TelemetryRecord;
use model::types::*;

#[test]
fn test_cbor_serialization() {
    let rec = TelemetryRecord::Battery(BatteryStatus::VolTempState(3045, 25, BatteryState::Ok));
    let bytes = rec.serialize(45);
    println!("CBOR bytes: {:?}", bytes);
    let decoded = TelemetryRecord::deserialize(&bytes);
    assert!(decoded.is_some());
    let (ts, record) = decoded.unwrap();
    assert_eq!(ts, 45);
    assert_eq!(record, rec);

    // Test FlashTelemetry serialization
    let erase_rec = TelemetryRecord::FlashTelemetry(FlashEraseTelemetry {
        sector: 16,
        duration_ms: 150,
        erase_count: 4,
    });
    let erase_bytes = erase_rec.serialize(123456);
    let decoded_erase = TelemetryRecord::deserialize(&erase_bytes);
    assert!(decoded_erase.is_some());
    let (ts_erase, record_erase) = decoded_erase.unwrap();
    assert_eq!(ts_erase, 123456);
    assert_eq!(record_erase, erase_rec);
}
