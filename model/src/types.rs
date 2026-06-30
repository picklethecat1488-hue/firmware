//! Domain models representing controller states and status telemetry.

/// Telemetry status of the battery system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BatteryStatus {
    /// Voltage in millivolts (mV).
    pub voltage_mv: u32,
    /// Temperature in millicelsius (mC).
    pub temp_mc: i32,
    /// Current battery state.
    pub state: BatteryState,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MotorStatus {
    /// Pump speed as a percentage (0-100).
    pub speed_percent: u8,
    /// Whether the motor is currently running.
    pub is_running: bool,
    /// Current temperature of the motor in millicelsius (mC).
    pub temp_mc: i32,
}

/// Telemetry status of the thermal monitoring system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ThermalStatus {
    /// Temperature in millicelsius (mC).
    pub temp_mc: i32,
    /// Whether the system is currently overheating.
    pub is_overheating: bool,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FuelGaugeTelemetry {
    /// Battery cell voltage in millivolts (mV).
    pub voltage_mv: u32,
    /// Battery state of charge as a percentage (0-100).
    pub state_of_charge: u8,
}

/// Telemetry data from the proximity (ToF) sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProximityTelemetry {
    /// Measured distance from the North sensor in millimeters.
    pub distance_north_mm: u16,
    /// Measured distance from the East sensor in millimeters.
    pub distance_east_mm: u16,
    /// Measured distance from the West sensor in millimeters.
    pub distance_west_mm: u16,
}

/// State of the indicator system LEDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SystemLedState {
    /// Red channel value (0-255).
    pub r: u8,
    /// Green channel value (0-255).
    pub g: u8,
    /// Blue channel value (0-255).
    pub b: u8,
}
