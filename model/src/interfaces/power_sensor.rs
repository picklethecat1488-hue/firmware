//! Generic power sensor interface.

#![deny(missing_docs)]

/// Operating mode of the power sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMeasurementMode {
    /// Power-down mode (quiescent current state).
    PowerDown,
    /// Triggered (one-shot) measurement of bus voltage, shunt current, or both.
    OneShot(bool, bool),
    /// Continuous measurement of bus voltage, shunt current, or both.
    Continuous(bool, bool),
}

/// Trait representing a power monitoring sensor capable of reading current and voltage.
pub trait PowerSensor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current draw in milliamperes (mA).
    fn read_current_ma(&mut self) -> Result<i32, Self::Error>;

    /// Reads the bus voltage in millivolts (mV).
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error>;

    /// Sets the operating mode of the sensor.
    fn set_measurement_mode(&mut self, mode: PowerMeasurementMode) -> Result<(), Self::Error>;
}
