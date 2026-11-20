//! Integration tests for domain-separated `NONE`, `PERF`, and `SENSOR` firmware tracing modes.

use model::telemetry::TelemetryRecord;
use model::types::{
    BatteryState, BatteryStatus, Direction, Gesture, GestureSource, SensorReading, SensorTelemetry,
    ThermalStatus,
};
use platform::tracing::{
    self, clear_host_trace_capture, is_sensor_telemetry_record, take_host_captured_spans,
    take_host_captured_telemetry_records, trace_sensor_report, trace_telemetry_record,
    HostTracePhase,
};
use platform::types::FirmwareTraceMode;
use std::sync::{Mutex, OnceLock};

fn test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[tracing::instrument(level = "debug")]
fn instrumented_perf_fn(x: u32) -> u32 {
    tracing::debug!("Inside perf instrumented function with x = {}", x);
    x + 1
}

#[tracing::instrument(domain = "sensor", level = "trace")]
fn instrumented_sensor_fn(mm: u16) -> u16 {
    mm.saturating_sub(5)
}

#[test]
fn test_sensor_vs_perf_record_classification() {
    let prox = TelemetryRecord::Proximity(SensorTelemetry::Status(
        Direction::North,
        SensorReading::Proximity(145),
    ));
    let therm = TelemetryRecord::Thermal(ThermalStatus::TempOverheating(24_800, false));
    let gest = TelemetryRecord::Gesture(Gesture::DualLongPress(GestureSource::Proximity));
    let batt =
        TelemetryRecord::Battery(BatteryStatus::VolTempState(4010, 250, BatteryState::Ok, 0));

    assert!(is_sensor_telemetry_record(&prox));
    assert!(is_sensor_telemetry_record(&therm));
    assert!(is_sensor_telemetry_record(&gest));
    assert!(!is_sensor_telemetry_record(&batt));
}

#[test]
fn test_tracing_facade_mode_gating() {
    let _guard = test_lock().lock().unwrap();
    clear_host_trace_capture();

    assert_eq!(instrumented_perf_fn(42), 43);
    assert_eq!(instrumented_sensor_fn(125), 120);

    let batt =
        TelemetryRecord::Battery(BatteryStatus::VolTempState(4020, 250, BatteryState::Ok, 0));
    let prox = TelemetryRecord::Proximity(SensorTelemetry::Status(
        Direction::North,
        SensorReading::Proximity(120),
    ));
    let therm = TelemetryRecord::Thermal(ThermalStatus::TempOverheating(25_100, false));

    trace_telemetry_record(&batt);
    trace_telemetry_record(&prox);
    trace_sensor_report(&prox);
    trace_sensor_report(&therm);
    trace_sensor_report(&batt);

    let spans = take_host_captured_spans();
    let records = take_host_captured_telemetry_records();

    if cfg!(feature = "trace-perf") && !cfg!(feature = "trace-sensors") {
        assert_eq!(FirmwareTraceMode::active(), FirmwareTraceMode::Perf);
        assert!(spans
            .iter()
            .any(|s| s.phase == HostTracePhase::Enter && s.name == "Core 0: instrumented_perf_fn"));
        assert!(spans
            .iter()
            .any(|s| s.phase == HostTracePhase::Exit && s.name == "Core 0: instrumented_perf_fn"));
        assert!(
            !spans
                .iter()
                .any(|s| s.name.contains("instrumented_sensor_fn")),
            "PERF mode must exclude sensor-domain spans"
        );
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].1, batt);
    } else if cfg!(feature = "trace-sensors") && !cfg!(feature = "trace-perf") {
        assert_eq!(FirmwareTraceMode::active(), FirmwareTraceMode::Sensor);
        assert!(
            spans.is_empty(),
            "SENSOR mode must not emit execution span transitions: {:?}",
            spans
        );
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].1, prox);
        assert_eq!(records[1].1, therm);
    } else if !cfg!(feature = "trace-perf") && !cfg!(feature = "trace-sensors") {
        assert_eq!(FirmwareTraceMode::active(), FirmwareTraceMode::None);
        assert!(spans.is_empty());
        assert!(records.is_empty());
    } else {
        // Full `tracing` feature (`trace-perf` + `trace-sensors` + `trace-sensor-spans`)
        assert!(spans
            .iter()
            .any(|s| s.name == "Core 0: instrumented_perf_fn"));
        assert!(spans
            .iter()
            .any(|s| s.name == "Core 0: instrumented_sensor_fn"));
        assert_eq!(records.len(), 3);
    }
}
