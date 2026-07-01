//! Domain models representing controller states and status telemetry.

/// Telemetry status of the battery system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum BatteryStatus {
    /// Voltage (mV), temperature (mC), and battery state.
    #[n(0)]
    VolTempState(#[n(0)] u32, #[n(1)] i32, #[n(2)] BatteryState),
}

impl Default for BatteryStatus {
    fn default() -> Self {
        Self::VolTempState(0, 0, BatteryState::default())
    }
}

/// Enumeration of battery states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
pub enum BatteryState {
    /// Battery voltage is normal.
    #[default]
    #[n(0)]
    Ok,
    /// Battery voltage is low.
    #[n(1)]
    Low,
    /// Battery is charging.
    #[n(2)]
    Charging,
    /// Battery is critically low (system runs, but pump is disabled).
    #[n(3)]
    Critical,
}

/// Telemetry status of the motor (pump) system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum MotorStatus {
    /// Pump speed as a percentage (0-100), run status, and temperature (mC).
    #[n(0)]
    SpeedRunTemp(#[n(0)] u8, #[n(1)] bool, #[n(2)] i32),
}

impl Default for MotorStatus {
    fn default() -> Self {
        Self::SpeedRunTemp(0, false, 0)
    }
}

/// Telemetry status of the thermal monitoring system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum ThermalStatus {
    /// Temperature (mC) and overheating status.
    #[n(0)]
    TempOverheating(#[n(0)] i32, #[n(1)] bool),
}

impl Default for ThermalStatus {
    fn default() -> Self {
        Self::TempOverheating(0, false)
    }
}

/// Operating mode of the system (Active, Sleep, or PowerDown).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
pub enum SystemStatus {
    /// System is powered down for safe transport/boot constraints.
    #[default]
    #[n(0)]
    PowerDown,
    /// System is fully awake and processing sensors/motor.
    #[n(1)]
    Active,
    /// System is in low-power sleep state.
    #[n(2)]
    Sleep,
}

/// Telemetry data from the battery fuel gauge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum FuelGaugeTelemetry {
    /// Battery cell voltage (mV) and state of charge percentage (0-100).
    #[n(0)]
    VolSoc(#[n(0)] u32, #[n(1)] u8),
}

impl Default for FuelGaugeTelemetry {
    fn default() -> Self {
        Self::VolSoc(0, 0)
    }
}

/// Telemetry data from the proximity (ToF) sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum ProximityTelemetry {
    /// Target is detected within active range (value in mm).
    #[n(0)]
    InRange(#[n(0)] u16),
    /// Target is out of range (value in mm).
    #[n(1)]
    OutRange(#[n(0)] u16),
}

impl Default for ProximityTelemetry {
    fn default() -> Self {
        Self::OutRange(1000)
    }
}

/// State patterns of the indicator system LEDs representing operating status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
pub enum SystemLedState {
    /// LED is powered off.
    #[default]
    #[n(0)]
    Off,
    /// Solid green indicating Active/Normal status.
    #[n(1)]
    SolidGreen,
    /// Solid blue indicating low-power Sleep status.
    #[n(2)]
    SolidBlue,
    /// Solid yellow indicating battery is charging.
    #[n(3)]
    SolidYellow,
    /// Solid orange indicating battery level is low.
    #[n(4)]
    SolidOrange,
    /// Four quick red blinks indicating a critical system alert.
    #[n(5)]
    BlinksRedFourTimes,
    /// One red blink once every 30 seconds indicating critical battery low.
    #[n(6)]
    BlinksRedOncePerThirtySeconds,
}

/// Gestures representing proximity sensor states (North, East, West) in mm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
pub enum Gesture {
    /// Proximity readings from North, East, and West sensors in mm (north, east, west).
    #[n(0)]
    Proximity(#[n(0)] u16, #[n(1)] u16, #[n(2)] u16),
    /// A completed dual-sensor long press gesture.
    #[n(1)]
    DualLongPress,
    /// Proximity detection (any sensor < 300 mm).
    #[n(2)]
    ProximityDetected,
    /// Proximity not detected (all sensors >= 300 mm).
    #[n(3)]
    ProximityNotDetected,
}

/// Telemetry data from the flash storage/filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cbor(array)]
pub struct FlashEraseTelemetry {
    /// Erased sector index (offset / 4096).
    #[n(0)]
    pub sector: u32,
    /// Erase duration in milliseconds.
    #[n(1)]
    pub duration_ms: u32,
    /// Total erases since boot.
    #[n(2)]
    pub erase_count: u32,
}

pub use crate::telemetry::TelemetryRecord;
