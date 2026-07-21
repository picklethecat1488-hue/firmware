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
            if len <= TELEMETRY_MAX_SIZE - 1 {
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
        let mut decoder = minicbor::Decoder::new(payload);
        let _array_len = decoder.array().ok()?;
        let timestamp_us = decoder.u64().ok()?;
        let record = decoder.decode().ok()?;
        Some((timestamp_us, record))
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
pub const TELEMETRY_RECORD_SIZE: usize = 20;
/// Max size of a telemetry record payload
pub const TELEMETRY_MAX_SIZE: usize = TELEMETRY_RECORD_SIZE;

/// Telemetry record chunking constants
pub const CHUNK_SIZE: usize = 128;
/// Size of one chunk in bytes
pub const CHUNK_FILE_SIZE: usize = CHUNK_SIZE * TELEMETRY_RECORD_SIZE;
/// Default size of the telemetry file buffer
pub const BUFFER_SIZE: usize = 3000;
/// Total number of telemetry record types/variants.
pub const NUM_TELEMETRY_VARIANTS: usize = 13;

/// Macro to lookup the name of a telemetry record chunk.
#[macro_export]
macro_rules! telemetry_chunk_name_lookup {
    ($idx:expr) => {
        match $idx {
            0 => "telemetry_0.rrd",
            1 => "telemetry_1.rrd",
            2 => "telemetry_2.rrd",
            3 => "telemetry_3.rrd",
            4 => "telemetry_4.rrd",
            5 => "telemetry_5.rrd",
            6 => "telemetry_6.rrd",
            7 => "telemetry_7.rrd",
            8 => "telemetry_8.rrd",
            9 => "telemetry_9.rrd",
            10 => "telemetry_10.rrd",
            11 => "telemetry_11.rrd",
            12 => "telemetry_12.rrd",
            13 => "telemetry_13.rrd",
            14 => "telemetry_14.rrd",
            15 => "telemetry_15.rrd",
            16 => "telemetry_16.rrd",
            17 => "telemetry_17.rrd",
            18 => "telemetry_18.rrd",
            19 => "telemetry_19.rrd",
            20 => "telemetry_20.rrd",
            21 => "telemetry_21.rrd",
            22 => "telemetry_22.rrd",
            23 => "telemetry_23.rrd",
            24 => "telemetry_24.rrd",
            25 => "telemetry_25.rrd",
            26 => "telemetry_26.rrd",
            27 => "telemetry_27.rrd",
            28 => "telemetry_28.rrd",
            29 => "telemetry_29.rrd",
            30 => "telemetry_30.rrd",
            31 => "telemetry_31.rrd",
            32 => "telemetry_32.rrd",
            33 => "telemetry_33.rrd",
            34 => "telemetry_34.rrd",
            35 => "telemetry_35.rrd",
            36 => "telemetry_36.rrd",
            37 => "telemetry_37.rrd",
            38 => "telemetry_38.rrd",
            39 => "telemetry_39.rrd",
            40 => "telemetry_40.rrd",
            41 => "telemetry_41.rrd",
            42 => "telemetry_42.rrd",
            43 => "telemetry_43.rrd",
            44 => "telemetry_44.rrd",
            45 => "telemetry_45.rrd",
            46 => "telemetry_46.rrd",
            47 => "telemetry_47.rrd",
            48 => "telemetry_48.rrd",
            49 => "telemetry_49.rrd",
            50 => "telemetry_50.rrd",
            51 => "telemetry_51.rrd",
            52 => "telemetry_52.rrd",
            53 => "telemetry_53.rrd",
            54 => "telemetry_54.rrd",
            55 => "telemetry_55.rrd",
            56 => "telemetry_56.rrd",
            57 => "telemetry_57.rrd",
            58 => "telemetry_58.rrd",
            59 => "telemetry_59.rrd",
            60 => "telemetry_60.rrd",
            61 => "telemetry_61.rrd",
            62 => "telemetry_62.rrd",
            63 => "telemetry_63.rrd",
            64 => "telemetry_64.rrd",
            65 => "telemetry_65.rrd",
            66 => "telemetry_66.rrd",
            67 => "telemetry_67.rrd",
            68 => "telemetry_68.rrd",
            69 => "telemetry_69.rrd",
            70 => "telemetry_70.rrd",
            71 => "telemetry_71.rrd",
            72 => "telemetry_72.rrd",
            73 => "telemetry_73.rrd",
            74 => "telemetry_74.rrd",
            75 => "telemetry_75.rrd",
            76 => "telemetry_76.rrd",
            77 => "telemetry_77.rrd",
            78 => "telemetry_78.rrd",
            79 => "telemetry_79.rrd",
            80 => "telemetry_80.rrd",
            81 => "telemetry_81.rrd",
            82 => "telemetry_82.rrd",
            83 => "telemetry_83.rrd",
            84 => "telemetry_84.rrd",
            _ => "telemetry_0.rrd",
        }
    };
}

/// Return the file name of a telemetry record chunk
pub fn chunk_name(idx: usize) -> &'static str {
    telemetry_chunk_name_lookup!(idx)
}

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
