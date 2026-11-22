//! Conditional compilation tracing facade.
//!
//! Supports domain-separated compile-time trace modes:
//! - `trace-perf`: Enables `tracing-defmt` execution spans (excluding sensor spans) and
//!   non-sensor `TelemetryRecord` trace events.
//! - `trace-sensors`: Enables real-time sensor report & value trace events (`Proximity`,
//!   `Thermal`, `Gesture`) while keeping execution span overhead compiled out.
//! - `trace-audio`: Reserved domain feature for future audio subsystem tracing.

#![allow(unused_imports)]

#[cfg(any(
    feature = "trace-perf",
    feature = "trace-sensor-spans",
    feature = "trace-audio"
))]
pub use tracing_defmt::{self, debug, error, info, trace, warn};

#[cfg(not(any(
    feature = "trace-perf",
    feature = "trace-sensor-spans",
    feature = "trace-audio"
)))]
pub use defmt::{debug, error, info, trace, warn};

pub use tracing_macros::{controller_context, instrument};

/// Returns `true` when `record` belongs to the sensor telemetry domain (`Proximity`, `Gesture`, or `Thermal`).
pub const fn is_sensor_telemetry_record(record: &model::telemetry::TelemetryRecord) -> bool {
    matches!(
        record,
        model::telemetry::TelemetryRecord::Proximity(_)
            | model::telemetry::TelemetryRecord::Gesture(_)
            | model::telemetry::TelemetryRecord::Thermal(_)
    )
}

#[inline]
fn emit_device_telemetry_trace(record: &model::telemetry::TelemetryRecord) {
    #[cfg(all(
        target_arch = "arm",
        target_os = "none",
        any(
            feature = "trace-perf",
            feature = "trace-sensors",
            feature = "trace-audio"
        )
    ))]
    {
        let timestamp_us = embassy_time::Instant::now().as_micros();
        let serialized = record.serialize(timestamp_us);
        let len = serialized[0] as usize;
        if len > 0 && len < model::telemetry::TELEMETRY_RECORD_SIZE {
            let payload = &serialized[1..1 + len];
            defmt::trace!("Device Telemetry: {=[u8]}", payload);
        }
    }

    #[cfg(all(
        not(all(target_arch = "arm", target_os = "none")),
        any(
            feature = "trace-perf",
            feature = "trace-sensors",
            feature = "trace-audio"
        )
    ))]
    {
        record_host_telemetry_trace(*record);
    }

    let _ = record;
}

/// Trace a non-sensor system/power/actuator telemetry record when `PERF` tracing (`trace-perf`) is enabled.
///
/// Sensor domain records (`Proximity`, `Gesture`, `Thermal`) are intentionally excluded in `PERF` mode
/// and instead emitted continuously without flash-wear debounce via [`trace_sensor_report`] in `SENSOR` mode.
pub fn trace_telemetry_record(record: &model::telemetry::TelemetryRecord) {
    if cfg!(feature = "trace-perf") && !is_sensor_telemetry_record(record) {
        emit_device_telemetry_trace(record);
    }
}

/// Trace a real-time sensor report (including measured values) when `SENSOR` tracing (`trace-sensors`) is enabled.
pub fn trace_sensor_report(record: &model::telemetry::TelemetryRecord) {
    if cfg!(feature = "trace-sensors") && is_sensor_telemetry_record(record) {
        emit_device_telemetry_trace(record);
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod host_capture {
    use std::sync::{Mutex, OnceLock};

    /// Lifecycle phase of a host-captured execution span transition.
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
    pub enum HostTracePhase {
        /// Span entered (`B`).
        Enter,
        /// Span exited (`E`).
        Exit,
    }

    /// Host-captured span transition recorded during integration tests.
    #[derive(Clone, PartialEq, Eq)]
    #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
    pub struct HostTraceSpanEvent {
        /// Transition phase (`Enter` or `Exit`).
        pub phase: HostTracePhase,
        /// Instrumented span name (including `Core 0:` / `Core 1:` prefix).
        pub name: &'static str,
        /// Rust module path where the span was entered.
        pub module: &'static str,
    }

    #[derive(Default)]
    struct HostTraceCaptureState {
        spans: Vec<HostTraceSpanEvent>,
        telemetry: Vec<(u64, model::telemetry::TelemetryRecord)>,
        next_ts_us: u64,
    }

    fn capture_store() -> &'static Mutex<HostTraceCaptureState> {
        static STORE: OnceLock<Mutex<HostTraceCaptureState>> = OnceLock::new();
        STORE.get_or_init(|| Mutex::new(HostTraceCaptureState::default()))
    }

    /// RAII guard recording span enter and exit transitions during host integration tests.
    pub struct HostSpanGuard {
        name: &'static str,
        module: &'static str,
    }

    impl HostSpanGuard {
        /// Record a span `Enter` transition and return an RAII guard that records `Exit` on drop.
        pub fn enter(name: &'static str, module: &'static str) -> Self {
            if let Ok(mut guard) = capture_store().lock() {
                guard.spans.push(HostTraceSpanEvent {
                    phase: HostTracePhase::Enter,
                    name,
                    module,
                });
            }
            Self { name, module }
        }
    }

    impl Drop for HostSpanGuard {
        fn drop(&mut self) {
            if let Ok(mut guard) = capture_store().lock() {
                guard.spans.push(HostTraceSpanEvent {
                    phase: HostTracePhase::Exit,
                    name: self.name,
                    module: self.module,
                });
            }
        }
    }

    #[cfg(any(
        feature = "trace-perf",
        feature = "trace-sensors",
        feature = "trace-audio"
    ))]
    pub(super) fn record_host_telemetry_trace(record: model::telemetry::TelemetryRecord) {
        if let Ok(mut guard) = capture_store().lock() {
            guard.next_ts_us += 1_000;
            let ts = guard.next_ts_us;
            guard.telemetry.push((ts, record));
        }
    }

    /// Clear all buffered host span and telemetry trace events.
    pub fn clear_host_trace_capture() {
        if let Ok(mut guard) = capture_store().lock() {
            guard.spans.clear();
            guard.telemetry.clear();
            guard.next_ts_us = 0;
        }
    }

    /// Drain and return all host-captured execution span transitions since the last clear.
    pub fn take_host_captured_spans() -> Vec<HostTraceSpanEvent> {
        if let Ok(mut guard) = capture_store().lock() {
            std::mem::take(&mut guard.spans)
        } else {
            Vec::new()
        }
    }

    /// Drain and return all host-captured telemetry records since the last clear.
    pub fn take_host_captured_telemetry_records() -> Vec<(u64, model::telemetry::TelemetryRecord)> {
        if let Ok(mut guard) = capture_store().lock() {
            std::mem::take(&mut guard.telemetry)
        } else {
            Vec::new()
        }
    }
}

#[cfg(all(
    not(all(target_arch = "arm", target_os = "none")),
    any(
        feature = "trace-perf",
        feature = "trace-sensors",
        feature = "trace-audio"
    )
))]
use host_capture::record_host_telemetry_trace;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use host_capture::{
    clear_host_trace_capture, take_host_captured_spans, take_host_captured_telemetry_records,
    HostSpanGuard, HostTracePhase, HostTraceSpanEvent,
};
