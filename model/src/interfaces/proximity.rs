//! Generic proximity/distance sensor interface.

#![deny(missing_docs)]

use crate::interfaces::WaitableMeasurement;
use crate::types::SensorReading;

/// Trait representing a proximity or distance sensor.
pub trait ProximitySensor: WaitableMeasurement {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current measured distance in millimeters.
    fn read_distance_mm(&mut self) -> Result<SensorReading, Self::Error>;

    /// Reads the raw measured distance in millimeters (ignoring calibration mapping).
    fn read_distance_raw(&mut self) -> Result<SensorReading, Self::Error>;
}
