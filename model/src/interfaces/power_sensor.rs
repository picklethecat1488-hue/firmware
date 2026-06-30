//! Generic power sensor interface.

#![deny(missing_docs)]

/// Trait representing a power monitoring sensor capable of reading current and voltage.
pub trait PowerSensor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current draw in milliamperes (mA).
    fn read_current_ma(&mut self) -> Result<i32, Self::Error>;

    /// Reads the bus voltage in millivolts (mV).
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error>;

    /// Register a callback function that is invoked when a power alert threshold is triggered.
    fn register_alert_callback(&mut self, callback: fn()) -> Result<(), Self::Error>;
}
