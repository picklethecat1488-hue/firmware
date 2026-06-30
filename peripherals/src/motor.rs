//! Generic motor peripheral traits and GPIO implementation.

#![deny(missing_docs)]

use embedded_hal::digital::OutputPin;
pub use model::interfaces::Motor;

/// A generic platform-agnostic GPIO implementation of a Motor.
pub struct GpioMotor<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> GpioMotor<P> {
    /// Creates a new generic GPIO motor.
    pub const fn new(pin: P) -> Self {
        Self { pin }
    }
}

impl<P: OutputPin> Motor for GpioMotor<P> {
    type Error = P::Error;

    /// Sets motor speed. Since this is GPIO, speed > 0 sets Pin High, and 0 sets Pin Low.
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error> {
        if speed > 0 {
            self.pin.set_high()
        } else {
            self.pin.set_low()
        }
    }

    /// Stops the motor by pulling the GPIO pin Low.
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}
