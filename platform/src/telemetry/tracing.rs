//! Conditional compilation tracing facade.
//! Re-exports `tracing-defmt` when target tracing is enabled, otherwise defines no-op mock versions.

#![allow(unused_imports)]

#[cfg(feature = "tracing")]
pub use tracing_defmt::{self, debug, error, info, trace, warn};

#[cfg(not(feature = "tracing"))]
pub use defmt::{debug, error, info, trace, warn};

pub use tracing_macros::instrument;

/// Trace a telemetry record to the defmt console/RTT buffer when tracing is enabled on target.
pub fn trace_telemetry_record(record: &model::telemetry::TelemetryRecord) {
    #[cfg(all(target_arch = "arm", target_os = "none", feature = "tracing"))]
    {
        let timestamp_us = embassy_time::Instant::now().as_micros();
        let serialized = record.serialize(timestamp_us);
        let len = serialized[0] as usize;
        if len > 0 && len < model::telemetry::TELEMETRY_RECORD_SIZE {
            let payload = &serialized[1..1 + len];
            defmt::trace!("Device Telemetry: {=[u8]}", payload);
        }
    }
    let _ = record;
}
