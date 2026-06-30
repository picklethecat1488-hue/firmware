//! Generic fuel gauge interface.

#![deny(missing_docs)]

/// Trait representing a battery fuel gauge capable of reading state of charge.
pub trait FuelGauge {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current battery voltage in millivolts (mV).
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error>;

    /// Reads the current state of charge as a percentage (0-100).
    fn read_state_of_charge(&mut self) -> Result<u8, Self::Error>;
}
