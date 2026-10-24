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

    /// Sets the motor PWM duty cycle percentage (0..=100).
    fn set_duty_cycle(&mut self, duty_cycle: u8) -> Result<(), Self::Error> {
        let _ = duty_cycle;
        Ok(())
    }

    /// Returns the active motor PWM duty cycle percentage (0..=100).
    fn duty_cycle(&self) -> u8 {
        100
    }
}
