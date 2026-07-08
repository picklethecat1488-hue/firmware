use model::telemetry::TelemetryRecord;
use model::types::*;

#[test]
fn test_cbor_serialization() {
    let rec = TelemetryRecord::Battery(BatteryStatus::VolTempState(3045, 25, BatteryState::Ok, 0));
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

#[test]
fn test_header_serialization() {
    let mut bytes = [0u8; 12];
    let cursor = minicbor::encode::write::Cursor::new(&mut bytes[1..]);
    let mut encoder = minicbor::Encoder::new(cursor);
    let count = 0u32;
    let next_idx = 0u32;
    let ok =
        encoder.array(2).is_ok() && encoder.u32(count).is_ok() && encoder.u32(next_idx).is_ok();
    let len = encoder.into_writer().position();
    if ok && len <= 11 {
        bytes[0] = len as u8;
    }
    println!("OK: {}, len: {}, bytes: {:?}", ok, len, bytes);
}
