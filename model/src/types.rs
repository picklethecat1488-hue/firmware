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
    /// Measured distance range values (in mm) for North, East, and West sensors.
    Triple(u16, u16, u16),
}

impl Default for ProximityTelemetry {
    fn default() -> Self {
        Self::Triple(0, 0, 0)
    }
}

/// State of the indicator system LEDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemLedState {
    /// Red, green, and blue channel intensity values (0-255).
    Rgb(u8, u8, u8),
}

impl Default for SystemLedState {
    fn default() -> Self {
        Self::Rgb(0, 0, 0)
    }
}
