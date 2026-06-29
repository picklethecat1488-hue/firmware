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
