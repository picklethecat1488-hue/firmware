//! Generic speed sensor interface for rotational speed measurement.

#![deny(missing_docs)]

/// Trait for reading rotational speed from a sensor (e.g. Hall effect, optical encoder).
pub trait SpeedSensor {
    /// Error type returned by the speed sensor.
    type Error;

    /// Reads the current rotational speed in revolutions per minute (RPM).
    fn read_rpm(&mut self) -> Result<u32, Self::Error>;
}
