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
    /// System booted telemetry.
    #[n(11)]
    Boot(#[n(0)] BootReason),
    /// Periodic update interval changed.
    #[n(12)]
    PeriodicInterval(#[n(0)] Device, #[n(1)] PeriodicInterval),
}

impl TelemetryRecord {
    /// Serialize the record and its timestamp into a fixed array using CBOR.
    pub fn serialize(&self, timestamp_us: u64) -> [u8; TELEMETRY_RECORD_SIZE] {
        let mut bytes = [0u8; TELEMETRY_RECORD_SIZE];
        // We write the CBOR payload starting at index 1 to leave room for the length byte.
        let cursor = minicbor::encode::write::Cursor::new(&mut bytes[1..]);
        let mut encoder = minicbor::Encoder::new(cursor);
        if encoder.array(2).is_ok()
            && encoder.u64(timestamp_us).is_ok()
            && encoder.encode(self).is_ok()
        {
            let len = encoder.into_writer().position();
            if len < TELEMETRY_MAX_SIZE {
                bytes[0] = len as u8;
            }
        }
        bytes
    }

    /// Deserialize the record and its timestamp from a fixed array using CBOR.
    pub fn deserialize(bytes: &[u8; TELEMETRY_RECORD_SIZE]) -> Option<(u64, Self)> {
        let len = bytes[0] as usize;
        if len == 0 || len > TELEMETRY_MAX_SIZE - 1 {
            return None;
        }
        let payload = &bytes[1..1 + len];
        Self::deserialize_from_slice_payload(payload)
    }

    /// Helper to deserialize a raw CBOR payload slice (without length prefix byte).
    fn deserialize_from_slice_payload(payload: &[u8]) -> Option<(u64, Self)> {
        let mut decoder = minicbor::Decoder::new(payload);
        let array_len = decoder.array().ok()??;
        if array_len == 3 {
            let _id = decoder.u32().ok()?;
            let timestamp_us = decoder.u64().ok()?;
            let record = decoder.decode().ok()?;
            Some((timestamp_us, record))
        } else if array_len == 2 {
            let timestamp_us = decoder.u64().ok()?;
            let record = decoder.decode().ok()?;
            Some((timestamp_us, record))
        } else {
            None
        }
    }

    /// Deserialize the record and its timestamp from a slice containing the length byte followed by CBOR.
    pub fn deserialize_from_slice(bytes: &[u8]) -> Option<(u64, Self)> {
        if bytes.is_empty() {
            return None;
        }
        let len = bytes[0] as usize;
        if len == 0 || len != bytes.len() - 1 {
            return None;
        }
        let payload = &bytes[1..];
        Self::deserialize_from_slice_payload(payload)
    }

    /// Returns the static string representation of the variant name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Battery(_) => "Battery",
            Self::Motor(_) => "Motor",
            Self::Thermal(_) => "Thermal",
            Self::System(_) => "System",
            Self::FuelGauge(_) => "FuelGauge",
            Self::Proximity(_) => "Proximity",
            Self::Led(_) => "Led",
            Self::Gesture(_) => "Gesture",
            Self::FlashTelemetry(_) => "FlashTelemetry",
            Self::ChargerState(_) => "ChargerState",
            Self::PeripheralError(_) => "PeripheralError",
            Self::Boot(_) => "Boot",
            Self::PeriodicInterval(_, _) => "PeriodicInterval",
        }
    }

    /// Returns the static variant name string representation for the given telemetry index.
    pub fn name_from_index(idx: usize) -> &'static str {
        match idx {
            0 => "Battery",
            1 => "Motor",
            2 => "Thermal",
            3 => "System",
            4 => "FuelGauge",
            5 => "Proximity",
            6 => "Led",
            7 => "Gesture",
            8 => "FlashTelemetry",
            9 => "ChargerState",
            10 => "PeripheralError",
            11 => "Boot",
            12 => "PeriodicInterval",
            _ => "Unknown",
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl core::fmt::Debug for TelemetryRecord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("TelemetryRecord")
    }
}

/// Size of a serialized telemetry record in bytes
pub const TELEMETRY_RECORD_SIZE: usize = 40;
/// Max size of a telemetry record payload
pub const TELEMETRY_MAX_SIZE: usize = TELEMETRY_RECORD_SIZE;
/// Size of the telemetry header index file (telemetry.rrd) in bytes
pub const TELEMETRY_HEADER_SIZE: usize = 12;
/// Name of the telemetry header index file
pub const TELEMETRY_HEADER_FILE: &str = "telemetry.rrd";

/// Telemetry record chunking constants
pub const CHUNK_SIZE: usize = 64;
/// Size of one chunk in bytes
pub const CHUNK_FILE_SIZE: usize = CHUNK_SIZE * TELEMETRY_RECORD_SIZE;
/// Default size of the telemetry file buffer
pub const BUFFER_SIZE: usize = 3000;
/// Total number of telemetry record types/variants.
pub const NUM_TELEMETRY_VARIANTS: usize = 13;

/// Trait for a telemetry client that handles change detection, filtering, and reporting.
pub trait TelemetryClient<T> {
    /// Reports telemetry data if it has changed significantly.
    fn report(&mut self, data: T);
}

/// Trait for types that can be converted into a TelemetryRecord.
pub trait IntoTelemetryRecord {
    /// Converts the type into a TelemetryRecord.
    fn into_telemetry_record(self) -> TelemetryRecord;
}

macro_rules! impl_into_telemetry {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(
            impl IntoTelemetryRecord for $ty {
                fn into_telemetry_record(self) -> TelemetryRecord {
                    TelemetryRecord::$variant(self)
                }
            }
        )*
    };
}

impl_into_telemetry! {
    BatteryStatus => Battery,
    MotorStatus => Motor,
    ThermalStatus => Thermal,
    SystemStatus => System,
    FuelGaugeTelemetry => FuelGauge,
    ProximityTelemetry => Proximity,
    SystemLedState => Led,
    Gesture => Gesture,
    FlashEraseTelemetry => FlashTelemetry,
    ChargeState => ChargerState,
    PeripheralError => PeripheralError,
    BootReason => Boot,
}
