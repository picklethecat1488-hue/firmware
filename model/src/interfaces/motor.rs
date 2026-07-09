//! Generic motor interface.

#![deny(missing_docs)]

use crate::types::MotorSpeed;

/// Interface for controlling a DC or geared motor.
pub trait Motor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Sets the motor speed.
    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error>;

    /// Stops the motor completely.
    fn stop(&mut self) -> Result<(), Self::Error>;
}
