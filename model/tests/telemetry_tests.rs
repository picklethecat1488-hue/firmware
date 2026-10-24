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

    // Test Boot serialization
    let boot_rec = TelemetryRecord::Boot(BootReason::Watchdog);
    let boot_bytes = boot_rec.serialize(789);
    let decoded_boot = TelemetryRecord::deserialize(&boot_bytes);
    assert!(decoded_boot.is_some());
    let (ts_boot, record_boot) = decoded_boot.unwrap();
    assert_eq!(ts_boot, 789);
    assert_eq!(record_boot, boot_rec);

    // Test Thermal serialization
    let thermal_rec = TelemetryRecord::Thermal(ThermalStatus::TempOverheating(28000, false));
    let thermal_bytes = thermal_rec.serialize(9999);
    let decoded_thermal = TelemetryRecord::deserialize(&thermal_bytes);
    assert!(decoded_thermal.is_some());
    let (ts_thermal, record_thermal) = decoded_thermal.unwrap();
    assert_eq!(ts_thermal, 9999);
    assert_eq!(record_thermal, thermal_rec);
}

#[test]
fn test_telemetry_records_size_bounds() {
    let max_ts = u64::MAX; // worst-case 9-byte timestamp

    let records = vec![
        TelemetryRecord::Battery(BatteryStatus::VolTempState(
            u32::MAX,
            i32::MAX,
            BatteryState::Critical,
            u32::MAX,
        )),
        TelemetryRecord::Motor(MotorStatus::Running(MotorTelemetry::new(
            MotorSpeed::MAX,
            3000,
            3000,
            i32::MAX,
            i32::MAX,
            1000,
        ))),
        TelemetryRecord::Motor(MotorStatus::Brake),
        TelemetryRecord::Thermal(ThermalStatus::TempOverheating(i32::MAX, true)),
        TelemetryRecord::System(SystemStatus::Active),
        TelemetryRecord::FuelGauge(FuelGaugeTelemetry::VolSoc(u32::MAX, u8::MAX)),
        TelemetryRecord::Proximity(SensorTelemetry::Status(
            Direction::West,
            SensorReading::Proximity(u16::MAX),
        )),
        TelemetryRecord::Proximity(SensorTelemetry::Status(
            Direction::East,
            SensorReading::Invalid,
        )),
        TelemetryRecord::Led(SystemLedState::SolidOrange),
        TelemetryRecord::Gesture(Gesture::DualLongPress(GestureSource::Proximity)),
        TelemetryRecord::FlashTelemetry(FlashEraseTelemetry {
            sector: u32::MAX,
            duration_ms: u32::MAX,
            erase_count: u32::MAX,
        }),
        TelemetryRecord::ChargerState(ChargeState::Charging),
        TelemetryRecord::PeripheralError(PeripheralError::I2CNackAddress(u16::MAX, u16::MAX)),
        TelemetryRecord::Boot(BootReason::Watchdog),
        TelemetryRecord::PeriodicInterval(Device::Battery, PeriodicInterval::UpdateMs(u32::MAX)),
    ];

    for rec in records {
        let bytes = rec.serialize(max_ts);
        let len = bytes[0] as usize;
        assert!(
            len > 0,
            "Serialization failed (len is 0) for record: {:?}",
            rec
        );
        assert!(
            len < model::telemetry::TELEMETRY_MAX_SIZE,
            "Record {:?} exceeded max size limit of {}. Serialized len: {}",
            rec,
            model::telemetry::TELEMETRY_MAX_SIZE,
            len
        );
    }
}

#[test]
fn test_backwards_compatibility() {
    // Manually serialize an old format [id, timestamp, record]
    let rec = TelemetryRecord::Boot(BootReason::PowerOn);
    let mut bytes = [0u8; model::telemetry::TELEMETRY_RECORD_SIZE];
    let cursor = minicbor::encode::write::Cursor::new(&mut bytes[1..]);
    let mut encoder = minicbor::Encoder::new(cursor);
    assert!(encoder.array(3).is_ok());
    assert!(encoder.u32(42).is_ok()); // ID = 42
    assert!(encoder.u64(12345).is_ok());
    assert!(encoder.encode(rec).is_ok());
    let len = encoder.into_writer().position();
    bytes[0] = len as u8;

    // Verify it deserializes and successfully parses timestamp and record
    let decoded = TelemetryRecord::deserialize(&bytes);
    assert!(decoded.is_some());
    let (ts, decoded_rec) = decoded.unwrap();
    assert_eq!(ts, 12345);
    assert_eq!(decoded_rec, rec);
}

#[test]
fn test_motor_telemetry_modeling() {
    use model::types::{
        calculate_estimated_rpm, calculate_modeled_peak_current_ma, calculate_modeled_torque_unm,
        MotorSpeed, MotorTelemetry, MOTOR_COMMUTATION_STEPS_PER_REV, MOTOR_KT_UNM_PER_A,
        MOTOR_PHASE_INDUCTANCE_UH, MOTOR_POLE_PAIRS,
    };

    // 1. Motor constants
    assert_eq!(MOTOR_POLE_PAIRS, 6);
    assert_eq!(MOTOR_COMMUTATION_STEPS_PER_REV, 36);
    assert_eq!(MOTOR_KT_UNM_PER_A, 682);
    assert_eq!(MOTOR_PHASE_INDUCTANCE_UH, 10);

    // 2. Estimated RPM from commutation step period
    // At 500 µs step period (2,000 Hz step rate): 5,000,000 / (3 * 500) = 3333 RPM
    assert_eq!(calculate_estimated_rpm(500), 3333);
    // At 1,000 µs step period (1,000 Hz): 5,000,000 / (3 * 1000) = 1666 RPM
    assert_eq!(calculate_estimated_rpm(1000), 1666);
    // At 10,000 µs step period (100 Hz): 5,000,000 / (3 * 10000) = 166 RPM
    assert_eq!(calculate_estimated_rpm(10_000), 166);
    // Stopped
    assert_eq!(calculate_estimated_rpm(0), 0);

    // 3. Modeled torque from current: τ = (682 * I_mA) / 1000
    // At 1000 mA (1.0 A): 682 µN·m
    assert_eq!(calculate_modeled_torque_unm(1000), 682);
    // At 1420 mA (full speed current): (1420 * 682) / 1000 = 968 µN·m
    assert_eq!(calculate_modeled_torque_unm(1420), 968);
    // At 0 mA or negative: 0 µN·m
    assert_eq!(calculate_modeled_torque_unm(0), 0);
    assert_eq!(calculate_modeled_torque_unm(-50), 0);

    // 4. Modeled peak current from average current and ON time (800 ns = 14 cycles)
    // ΔI_ripple = (4000 * 800) / (10 * 1000) = 320 mA
    // At I_avg = 500 mA (CCM): 500 + 160 = 660 mA
    assert_eq!(calculate_modeled_peak_current_ma(500, 800), 660);
    // At I_avg = 0 mA: max(0 + 160, 320) = 320 mA
    assert_eq!(calculate_modeled_peak_current_ma(0, 800), 320);
    // Stopped (on_time_ns = 0): 0 mA
    assert_eq!(calculate_modeled_peak_current_ma(0, 0), 0);

    // 5. MotorTelemetry struct instantiation and field checks
    let telem = MotorTelemetry::new(MotorSpeed::new(50).unwrap(), 1600, 1666, 450, 610, 306);
    assert_eq!(telem.speed.get(), 50);
    assert_eq!(telem.measured_rpm, 1600);
    assert_eq!(telem.estimated_rpm, 1666);
    assert_eq!(telem.current_ma, 450);
    assert_eq!(telem.modeled_peak_current_ma, 610);
    assert_eq!(telem.modeled_torque_unm, 306);
}
