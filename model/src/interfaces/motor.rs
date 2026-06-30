//! Generic motor interface.

#![deny(missing_docs)]

/// Interface for controlling a DC or geared motor.
pub trait Motor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Sets the motor speed (0 to 255).
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error>;

    /// Stops the motor completely.
    fn stop(&mut self) -> Result<(), Self::Error>;
}
