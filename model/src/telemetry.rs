use crate::types::*;

/// A telemetry record wrapper for the system.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum TelemetryRecord {
    /// Battery status.
    #[n(0)]
    Battery(#[n(0)] BatteryStatus),
    /// Motor status.
    #[n(1)]
    Motor(#[n(0)] MotorStatus),
    /// Thermal status.
    #[n(2)]
    Thermal(#[n(0)] ThermalStatus),
    /// System status.
    #[n(3)]
    System(#[n(0)] SystemStatus),
    /// Fuel gauge telemetry.
    #[n(4)]
    FuelGauge(#[n(0)] FuelGaugeTelemetry),
    /// Proximity telemetry.
    #[n(5)]
    Proximity(#[n(0)] ProximityTelemetry),
    /// Indicator LED state.
    #[n(6)]
    Led(#[n(0)] SystemLedState),
    /// Detected gesture.
    #[n(7)]
    Gesture(#[n(0)] Gesture),
    /// Flash operations telemetry.
    #[n(8)]
    FlashTelemetry(#[n(0)] FlashEraseTelemetry),
    /// Charger state telemetry.
    #[n(9)]
    ChargerState(#[n(0)] ChargeState),
    /// Peripheral error telemetry.
    #[n(10)]
    PeripheralError(#[n(0)] PeripheralError),
}

impl TelemetryRecord {
    /// Serialize the record and its timestamp into a fixed 20-byte array using CBOR.
    pub fn serialize(&self, timestamp_us: u64) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        // We write the CBOR payload starting at index 1 to leave room for the length byte.
        let cursor = minicbor::encode::write::Cursor::new(&mut bytes[1..]);
        let mut encoder = minicbor::Encoder::new(cursor);
        if encoder.array(2).is_ok()
            && encoder.u64(timestamp_us).is_ok()
            && encoder.encode(self).is_ok()
        {
            let len = encoder.into_writer().position();
            if len <= 19 {
                bytes[0] = len as u8;
            }
        }
        bytes
    }

    /// Deserialize the record and its timestamp from a fixed 20-byte array using CBOR.
    pub fn deserialize(bytes: &[u8; 20]) -> Option<(u64, Self)> {
        let len = bytes[0] as usize;
        if len == 0 || len > 19 {
            return None;
        }
        let payload = &bytes[1..1 + len];
        let mut decoder = minicbor::Decoder::new(payload);
        let _array_len = decoder.array().ok()?;
        let timestamp_us = decoder.u64().ok()?;
        let record = decoder.decode().ok()?;
        Some((timestamp_us, record))
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl core::fmt::Debug for TelemetryRecord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("TelemetryRecord")
    }
}

/// Telemetry record chunking constants
pub const CHUNK_SIZE: usize = 128;
/// Total number of chunks
pub const NUM_CHUNKS: usize = 8;
/// Size of one chunk in bytes
pub const CHUNK_FILE_SIZE: usize = CHUNK_SIZE * 20;

/// Return the file name of a telemetry record chunk
pub fn chunk_name(idx: usize) -> &'static str {
    match idx {
        0 => "telemetry_0.rrd",
        1 => "telemetry_1.rrd",
        2 => "telemetry_2.rrd",
        3 => "telemetry_3.rrd",
        4 => "telemetry_4.rrd",
        5 => "telemetry_5.rrd",
        6 => "telemetry_6.rrd",
        7 => "telemetry_7.rrd",
        _ => "telemetry_0.rrd",
    }
}
