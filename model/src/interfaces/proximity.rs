//! Generic proximity/distance sensor interface.

#![deny(missing_docs)]

use crate::interfaces::WaitableMeasurement;

/// Trait representing a proximity or distance sensor.
pub trait ProximitySensor: WaitableMeasurement {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current measured distance in millimeters.
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error>;

    /// Reads the raw measured distance in millimeters (ignoring calibration mapping).
    fn read_distance_raw(&mut self) -> Result<u16, Self::Error>;

    /// Reads diagnostic information: (raw_distance, range_status, peak_signal_rate).
    fn read_diagnostics(&mut self) -> Result<(u16, u8, u16), Self::Error>;
}
