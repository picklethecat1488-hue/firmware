//! Generic proximity/distance sensor interface.

#![deny(missing_docs)]

/// Trait representing a proximity or distance sensor.
pub trait ProximitySensor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current measured distance in millimeters.
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error>;

    /// Register a callback function for proximity events.
    /// The callback receives a boolean indicating whether an object is detected.
    fn register_proximity_callback(&mut self, callback: fn(bool)) -> Result<(), Self::Error>;
}
