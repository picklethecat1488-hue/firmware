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
        TelemetryRecord::Motor(MotorStatus::Running(MotorSpeed::new(75).unwrap(), 150)),
        1002,
    );
    let events = parser.parse_log(&log, 1002.0).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Motor Speed");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 75);
    assert_eq!(events[0]["tid"].as_i64().unwrap(), 3);
    assert_eq!(events[1]["name"].as_str().unwrap(), "Motor Current (mA)");
    assert_eq!(events[1]["args"]["value"].as_i64().unwrap(), 150);
    assert_eq!(events[1]["tid"].as_i64().unwrap(), 3);

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
        TelemetryRecord::Proximity(SensorTelemetry::Status(
            Direction::North,
            SensorReading::Proximity(150),
        )),
        1006,
    );
    let events = parser.parse_log(&log, 1006.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Proximity (North)");
    assert_eq!(events[0]["args"]["value"].as_i64().unwrap(), 150);
    assert_eq!(events[0]["args"]["valid"].as_bool().unwrap(), true);
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
        TelemetryRecord::PeripheralError(PeripheralError::DeviceNotFound(0x1234)),
        1011,
    );
    let events = parser.parse_log(&log, 1011.0).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["name"].as_str().unwrap(), "Peripheral Error");
    assert_eq!(
        events[0]["args"]["error"].as_str().unwrap(),
        "DeviceNotFound(0x1234)"
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

        // 1. Prepare 2 serialized telemetry records
        let rec1 =
            TelemetryRecord::Battery(BatteryStatus::VolTempState(3600, 24, BatteryState::Ok, 1));
        let rec2 = TelemetryRecord::Motor(MotorStatus::Running(MotorSpeed::new(50).unwrap(), 150));

        let slot1 = rec1.serialize(500);
        let slot2 = rec2.serialize(600);

        let len1 = slot1[0] as usize;
        let len2 = slot2[0] as usize;

        sequential_storage::queue::push(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &slot1[..1 + len1],
            true,
        )
        .await
        .unwrap();

        sequential_storage::queue::push(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &slot2[..1 + len2],
            true,
        )
        .await
        .unwrap();

        // 2. Read and parse telemetry records via shared library
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
        assert_eq!(records[0].0, 500); // timestamp
        assert_eq!(records[0].1, rec1); // record
        assert_eq!(records[1].0, 600); // timestamp
        assert_eq!(records[1].1, rec2); // record
    });
}

#[test]
fn test_read_telemetry_records_corrupted_skips() {
    futures::executor::block_on(async {
        let mut flash = MockFlash::new(1024 * 64);
        let flash_range = 0..1024 * 64;
        let mut cache = sequential_storage::cache::NoCache::new();
        let mut buf = vec![0u8; 4096];
        let parser = FlashTelemetryParser::new(3);

        // 1. Push a valid record
        let rec1 = TelemetryRecord::Boot(model::types::BootReason::PowerOn);
        let slot1 = rec1.serialize(100);
        let len1 = slot1[0] as usize;
        sequential_storage::queue::push(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &slot1[..1 + len1],
            true,
        )
        .await
        .unwrap();

        // 2. Push some corrupted/garbage bytes to the queue
        let bad_bytes = [4u8, 0x99, 0x88, 0x77, 0x66]; // Invalid CBOR
        sequential_storage::queue::push(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &bad_bytes,
            true,
        )
        .await
        .unwrap();

        // 3. Push another valid record
        let rec2 = TelemetryRecord::Led(model::types::SystemLedState::Off);
        let slot2 = rec2.serialize(200);
        let len2 = slot2[0] as usize;
        sequential_storage::queue::push(
            &mut flash,
            flash_range.clone(),
            &mut cache,
            &slot2[..1 + len2],
            true,
        )
        .await
        .unwrap();

        // 4. Read back: should successfully skip the corrupted item and return the 2 valid records sorted chronologically
        let res = parser
            .read_records(&mut flash, flash_range, &mut cache, &mut buf, 128)
            .await
            .unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].0, 100);
        assert_eq!(res[0].1, rec1);
        assert_eq!(res[1].0, 200);
        assert_eq!(res[1].1, rec2);
    });
}
