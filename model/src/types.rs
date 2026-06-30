//! Domain models representing controller states and status telemetry.

/// Telemetry status of the battery system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    /// Voltage (mV), temperature (mC), and battery state.
    VolTempState(u32, i32, BatteryState),
}

impl Default for BatteryStatus {
    fn default() -> Self {
        Self::VolTempState(0, 0, BatteryState::default())
    }
}

/// Enumeration of battery states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BatteryState {
    /// Battery voltage is normal.
    #[default]
    Ok,
    /// Battery voltage is low.
    Low,
    /// Battery is charging.
    Charging,
    /// Battery is critically low (system runs, but pump is disabled).
    Critical,
}

/// Telemetry status of the motor (pump) system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorStatus {
    /// Pump speed as a percentage (0-100), run status, and temperature (mC).
    SpeedRunTemp(u8, bool, i32),
}

impl Default for MotorStatus {
    fn default() -> Self {
        Self::SpeedRunTemp(0, false, 0)
    }
}

/// Telemetry status of the thermal monitoring system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalStatus {
    /// Temperature (mC) and overheating status.
    TempOverheating(i32, bool),
}

impl Default for ThermalStatus {
    fn default() -> Self {
        Self::TempOverheating(0, false)
    }
}

/// Operating mode of the system (Active or Sleep).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemStatus {
    /// System is fully awake and processing sensors/motor.
    #[default]
    Active,
    /// System is in low-power sleep state.
    Sleep,
}

/// Telemetry data from the battery fuel gauge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelGaugeTelemetry {
    /// Battery cell voltage (mV) and state of charge percentage (0-100).
    VolSoc(u32, u8),
}

impl Default for FuelGaugeTelemetry {
    fn default() -> Self {
        Self::VolSoc(0, 0)
    }
}

/// Telemetry data from the proximity (ToF) sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProximityTelemetry {
    /// Target is detected within active range (value in mm).
    InRange(u16),
    /// Target is out of range (value in mm).
    OutRange(u16),
}

impl Default for ProximityTelemetry {
    fn default() -> Self {
        Self::OutRange(1000)
    }
}

/// State patterns of the indicator system LEDs representing operating status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemLedState {
    /// LED is powered off.
    #[default]
    Off,
    /// Solid green indicating Active/Normal status.
    SolidGreen,
    /// Solid blue indicating low-power Sleep status.
    SolidBlue,
    /// Solid yellow indicating battery is charging.
    SolidYellow,
    /// Solid orange indicating battery level is low.
    SolidOrange,
    /// Four quick red blinks indicating a critical system alert.
    BlinksRedFourTimes,
    /// One red blink once every 30 seconds indicating critical battery low.
    BlinksRedOncePerThirtySeconds,
}
