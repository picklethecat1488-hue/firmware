//! Concrete driver implementation for the L9110S motor driver.

#![deny(missing_docs)]

use embedded_hal::digital::OutputPin;
use model::interfaces::Motor;

/// Driver for the L9110S dual-channel h-bridge motor driver.
pub struct L9110s<P1, P2> {
    pin_ia: P1,
    pin_ib: P2,
    speed: u8,
}

impl<P1: OutputPin, P2: OutputPin> L9110s<P1, P2> {
    /// Creates a new L9110S motor driver instance.
    pub const fn new(pin_ia: P1, pin_ib: P2) -> Self {
        Self {
            pin_ia,
            pin_ib,
            speed: 0,
        }
    }
}

impl<P1: OutputPin, P2: OutputPin> Motor for L9110s<P1, P2>
where
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
{
    type Error = L9110sError<P1::Error, P2::Error>;

    /// Sets the motor speed (0-100). In a basic GPIO configuration,
    /// setting any speed > 0 drives IA high and IB low (forward).
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error> {
        self.speed = speed;
        if speed > 0 {
            self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
            self.pin_ia.set_high().map_err(L9110sError::PinIa)?;
        } else {
            self.stop()?;
        }
        Ok(())
    }

    /// Stops the motor by braking (both IA and IB set to low).
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.pin_ia.set_low().map_err(L9110sError::PinIa)?;
        self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
        Ok(())
    }
}

/// Errors returned by the L9110S motor driver.
#[derive(Debug)]
pub enum L9110sError<E1, E2> {
    /// Error controlling the IA input pin.
    PinIa(E1),
    /// Error controlling the IB input pin.
    PinIb(E2),
}
