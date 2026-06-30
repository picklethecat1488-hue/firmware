//! Generic temperature sensor interface.

#![deny(missing_docs)]

/// Trait representing a temperature sensor.
pub trait TemperatureSensor {
    /// Error type for temperature sensor transactions.
    type Error;

    /// Reads the current temperature in milli-degrees Celsius.
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error>;
}
