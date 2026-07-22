use model::telemetry::TelemetryRecord;
use model::types::*;
use tool_common::{FlashTelemetryParser, TraceTelemetryParser};

struct MockFlash {
    data: Vec<u8>,
}

impl MockFlash {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0xFF; size],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 4096;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for MockFlash {}

fn string_to_key(name: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let bytes = name.as_bytes();
    let len = std::cmp::min(bytes.len(), 32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

fn make_rtt_log(rec: TelemetryRecord, ts: u64) -> String {
    let serialized = rec.serialize(ts);
    let len = serialized[0] as usize;
    let payload = &serialized[1..1 + len];
    let mut s = String::new();
    s.push_str("Device Telemetry: [");
    for (i, &b) in payload.iter().enumerate() {
        if i > 0 {
            s.push_str(", ");
        }
        s.push_str(&b.to_string());
    }
    s.push(']');
    s
}

#[test]
fn test_parse_telemetry_record_log_all_variants() {
    let parser = TraceTelemetryParser::new(3);
    // 1. Battery
    let log = make_rtt_log(
        TelemetryRecord::Battery(BatteryStatus::VolTempState(12000, 25, BatteryState::Ok, 2)),
        1000,
    );
    let events = parser.parse_log(&log, 1000.0).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Battery Voltage (mV)");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 12000);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);
    assert_eq!(events[1]["name"].as_str().unwrap(), "Battery Temp (mC)");
    assert_eq!(events[1]["args"]["value"].as_i64().unwrap(), 25);
    assert_eq!(events[1]["tid"].as_i64().unwrap(), 3);
    assert_eq!(events[2]["name"].as_str().unwrap(), "Battery State Change");
    assert_eq!(events[2]["args"]["state"].as_str().unwrap(), "Ok");
    assert_eq!(events[2]["args"]["active_locks"].as_i64().unwrap(), 2);
    assert_eq!(events[2]["tid"].as_i64().unwrap(), 3);

    // 2. Motor Brake
    let log = make_rtt_log(TelemetryRecord::Motor(MotorStatus::Brake), 1001);
    let events = parser.parse_log(&log, 1001.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Motor Speed");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 0);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 3. Motor Running
    let log = make_rtt_log(
        TelemetryRecord::Motor(MotorStatus::Running(MotorSpeed::new(75).unwrap())),
        1002,
    );
    let events = parser.parse_log(&log, 1002.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Motor Speed");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 75);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 4. Thermal
    let log = make_rtt_log(
        TelemetryRecord::Thermal(ThermalStatus::TempOverheating(20480, true)),
        1003,
    );
    let events = parser.parse_log(&log, 1003.0).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["name"].as_str().unwrap(), "MCU Temperature (mC)");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 20480);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);
    assert_eq!(events[1]["name"].as_str().unwrap(), "Overheating Alarm");
    assert_eq!(events[1]["tid"].as_i64().unwrap(), 3);

    // 5. System
    let log = make_rtt_log(TelemetryRecord::System(SystemStatus::Active), 1004);
    let events = parser.parse_log(&log, 1004.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "System Command");
    assert_eq!(events[0]["args"]["cmd"].as_str().unwrap(), "Active");
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 6. FuelGauge
    let log = make_rtt_log(
        TelemetryRecord::FuelGauge(FuelGaugeTelemetry::VolSoc(3750, 80)),
        1005,
    );
    let events = parser.parse_log(&log, 1005.0).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(
        events[0]["name"].as_str().unwrap(),
        "FuelGauge Voltage (mV)"
    );
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 3750);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);
    assert_eq!(events[1]["name"].as_str().unwrap(), "Battery SoC (%)");
    assert_eq!(events[1]["args"]["value"].as_i64().unwrap(), 80);
    assert_eq!(events[1]["tid"].as_i64().unwrap(), 3);

    // 7. Proximity InRange
    let log = make_rtt_log(
        TelemetryRecord::Proximity(ProximityTelemetry::InRange(Direction::North, 150)),
        1006,
    );
    let events = parser.parse_log(&log, 1006.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Proximity (North)");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 150);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 8. Led
    let log = make_rtt_log(TelemetryRecord::Led(SystemLedState::SolidGreen), 1007);
    let events = parser.parse_log(&log, 1007.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "LED Change");
    assert_eq!(events[0]["args"]["led"].as_str().unwrap(), "SolidGreen");
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 9. Gesture
    let log = make_rtt_log(TelemetryRecord::Gesture(Gesture::DualLongPress), 1008);
    let events = parser.parse_log(&log, 1008.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Gesture Action");
    assert_eq!(
        events[0]["args"]["gesture"].as_str().unwrap(),
        "DualLongPress"
    );
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 10. FlashTelemetry
    let log = make_rtt_log(
        TelemetryRecord::FlashTelemetry(FlashEraseTelemetry {
            sector: 5,
            duration_ms: 250,
            erase_count: 12,
        }),
        1009,
    );
    let events = parser.parse_log(&log, 1009.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0]["name"].as_str().unwrap(),
        "Flash Erase Duration (ms)"
    );
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 250);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 11. ChargerState
    let log = make_rtt_log(TelemetryRecord::ChargerState(ChargeState::Charging), 1010);
    let events = parser.parse_log(&log, 1010.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Charger State");
    assert_eq!(events[0]["args"]["state"].as_str().unwrap(), "Charging");
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 12. PeripheralError
    let log = make_rtt_log(
        TelemetryRecord::PeripheralError(PeripheralError::DeviceNotFound),
        1011,
    );
    let events = parser.parse_log(&log, 1011.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Peripheral Error");
    assert_eq!(
        events[0]["args"]["error"].as_str().unwrap(),
        "DeviceNotFound"
    );
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 13. Boot
    let log = make_rtt_log(TelemetryRecord::Boot(BootReason::SoftwareReset), 1012);
    let events = parser.parse_log(&log, 1012.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "System Boot");
    assert_eq!(
        events[0]["args"]["reason"].as_str().unwrap(),
        "SoftwareReset"
    );
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);

    // 14. PeriodicInterval
    let log = make_rtt_log(
        TelemetryRecord::PeriodicInterval(Device::Motor, PeriodicInterval::UpdateMs(100)),
        1013,
    );
    let events = parser.parse_log(&log, 1013.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0]["name"].as_str().unwrap(),
        "Periodic Interval Update"
    );
    assert_eq!(events[0]["args"]["device"].as_str().unwrap(), "Motor");
    assert_eq!(
        events[0]["args"]["interval"].as_str().unwrap(),
        "UpdateMs(100)"
    );
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);
}

#[test]
fn test_read_telemetry_records_integration() {
    futures::executor::block_on(async {
        let mut flash = MockFlash::new(1024 * 64);
        let flash_range = 0..1024 * 64;
        let mut cache = sequential_storage::cache::NoCache::new();
        let mut buf = vec![0u8; 4096];

        // 1. Store the telemetry header record in telemetry.rrd
        let mut header_bytes = [0u8; 12];
        let cursor = minicbor::encode::write::Cursor::new(&mut header_bytes[1..]);
        let mut encoder = minicbor::Encoder::new(cursor);
        let count = 2u32;
        let next_idx = 2u32;
        encoder.array(2).unwrap();
        encoder.u32(count).unwrap();
        encoder.u32(next_idx).unwrap();
        let header_len = encoder.into_writer().position();
        header_bytes[0] = header_len as u8;

        let key = string_to_key("telemetry.rrd");
        sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &mut buf,
            &key,
            &&header_bytes[..],
        )
        .await
        .unwrap();

        // 2. Prepare chunk data containing 2 serialized telemetry records
        let rec1 =
            TelemetryRecord::Battery(BatteryStatus::VolTempState(3600, 24, BatteryState::Ok, 1));
        let rec2 = TelemetryRecord::Motor(MotorStatus::Running(MotorSpeed::new(50).unwrap()));

        let slot1 = rec1.serialize(500);
        let slot2 = rec2.serialize(600);

        let mut chunk_bytes = vec![0u8; model::telemetry::CHUNK_FILE_SIZE];
        chunk_bytes[..20].copy_from_slice(&slot1);
        chunk_bytes[20..40].copy_from_slice(&slot2);

        let chunk_key = string_to_key("telemetry_0.rrd");
        sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &mut buf,
            &chunk_key,
            &&chunk_bytes[..],
        )
        .await
        .unwrap();

        // 3. Read and parse telemetry records via shared library
        let max_records = 128;
        let parser = FlashTelemetryParser::new(3);
        let records = parser
            .read_records(
                &mut flash,
                flash_range.clone(),
                &mut cache,
                &mut buf,
                max_records,
            )
            .await
            .unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].0, 500);
        assert_eq!(records[0].1, rec1);
        assert_eq!(records[1].0, 600);
        assert_eq!(records[1].1, rec2);
    });
}

#[test]
fn test_read_telemetry_records_corrupted() {
    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();
        let mut buf = vec![0u8; 4096];
        let parser = FlashTelemetryParser::new(3);

        // 1. Telemetry file is too short (< 12 bytes)
        {
            let mut flash = MockFlash::new(1024 * 64);
            let flash_range = 0..1024 * 64;
            let key = string_to_key("telemetry.rrd");
            let header_bytes = [0u8; 8];
            sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
                &mut flash,
                flash_range.clone(),
                &mut cache,
                &mut buf,
                &key,
                &&header_bytes[..],
            )
            .await
            .unwrap();

            let res = parser
                .read_records(&mut flash, flash_range, &mut cache, &mut buf, 128)
                .await;
            assert!(res.is_err());
            assert!(res.unwrap_err().contains("too short"));
        }

        // 2. Telemetry file is oversized (20 bytes)
        {
            let mut flash = MockFlash::new(1024 * 64);
            let flash_range = 0..1024 * 64;
            let key = string_to_key("telemetry.rrd");
            let mut header_bytes = vec![0u8; 20];
            let cursor = minicbor::encode::write::Cursor::new(&mut header_bytes[1..12]);
            let mut encoder = minicbor::Encoder::new(cursor);
            encoder.array(2).unwrap();
            encoder.u32(0).unwrap();
            encoder.u32(0).unwrap();
            let header_len = encoder.into_writer().position();
            header_bytes[0] = header_len as u8;

            sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
                &mut flash,
                flash_range.clone(),
                &mut cache,
                &mut buf,
                &key,
                &&header_bytes[..],
            )
            .await
            .unwrap();

            // Should succeed without panicking
            let res = parser
                .read_records(&mut flash, flash_range, &mut cache, &mut buf, 128)
                .await;
            assert!(res.is_ok());
            assert!(res.unwrap().is_empty());
        }

        // 3. Telemetry file has invalid header CBOR format
        {
            let mut flash = MockFlash::new(1024 * 64);
            let flash_range = 0..1024 * 64;
            let key = string_to_key("telemetry.rrd");
            let mut header_bytes = [0u8; 12];
            header_bytes[0] = 5; // length of CBOR payload
            header_bytes[1..6].copy_from_slice(b"badcb"); // completely invalid CBOR

            sequential_storage::map::store_item::<[u8; 32], &[u8], _>(
                &mut flash,
                flash_range.clone(),
                &mut cache,
                &mut buf,
                &key,
                &&header_bytes[..],
            )
            .await
            .unwrap();

            let res = parser
                .read_records(&mut flash, flash_range, &mut cache, &mut buf, 128)
                .await;
            assert!(res.is_err());
            assert!(res.unwrap_err().contains("Failed to decode"));
        }
    });
}
