//! Generic battery peripheral traits.

#![deny(missing_docs)]

/// Trait representing a battery peripheral capable of reading voltage and temperature.
pub trait Battery {
    /// Error type for battery transactions.
    type Error;

    /// Reads the current battery voltage in millivolts.
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error>;

    /// Reads the current battery temperature in milli-degrees Celsius.
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error>;
}
