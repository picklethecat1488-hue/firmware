//! Generic proximity/distance sensor interface.

#![deny(missing_docs)]

/// Trait representing a proximity or distance sensor.
pub trait ProximitySensor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current measured distance in millimeters.
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error>;

    /// Reads the raw measured distance in millimeters (ignoring calibration mapping).
    fn read_distance_raw(&mut self) -> Result<u16, Self::Error>;
}
