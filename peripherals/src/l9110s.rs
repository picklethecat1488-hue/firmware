//! Concrete driver implementation for the L9110S motor driver.

#![deny(missing_docs)]

use crate::tracing;
use embedded_hal::digital::OutputPin;
use model::interfaces::{Motor, Tickable};
use model::types::MotorSpeed;

/// Driver for the L9110S dual-channel h-bridge motor driver.
pub struct L9110s<P1, P2> {
    pin_ia: P1,
    pin_ib: P2,
    speed: i8,
    tick_counter: u8,
}

impl<P1: OutputPin, P2: OutputPin> L9110s<P1, P2> {
    /// Creates a new L9110S motor driver instance.
    pub const fn new(pin_ia: P1, pin_ib: P2) -> Self {
        Self {
            pin_ia,
            pin_ib,
            speed: 0,
            tick_counter: 0,
        }
    }
}

impl<P1: OutputPin, P2: OutputPin> Motor for L9110s<P1, P2>
where
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
{
    type Error = L9110sError<P1::Error, P2::Error>;

    /// Sets the motor speed.
    #[tracing::instrument(level = "trace", skip(speed))]
    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        let speed_raw = speed.get();
        self.speed = speed_raw;
        self.tick_counter = 0;
        if speed_raw > 0 {
            self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
            self.pin_ia.set_high().map_err(L9110sError::PinIa)?;
        } else if speed_raw < 0 {
            // Reverse direction: IA low, IB high
            self.pin_ia.set_low().map_err(L9110sError::PinIa)?;
            self.pin_ib.set_high().map_err(L9110sError::PinIb)?;
        } else {
            self.stop()?;
        }
        Ok(())
    }

    /// Stops the motor by braking (both IA and IB set to low).
    #[tracing::instrument(level = "trace")]
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.tick_counter = 0;
        self.pin_ia.set_low().map_err(L9110sError::PinIa)?;
        self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
        Ok(())
    }
}

impl<P1: OutputPin, P2: OutputPin> Tickable for L9110s<P1, P2>
where
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
{
    type Error = L9110sError<P1::Error, P2::Error>;

    #[tracing::instrument(level = "trace")]
    fn tick(&mut self) -> Result<(), Self::Error> {
        let abs_speed = self.speed.abs();
        if abs_speed == 0 || abs_speed >= 100 {
            return Ok(());
        }

        self.tick_counter = (self.tick_counter + 1) % 10;
        let threshold = (abs_speed / 10) as u8;
        if self.tick_counter < threshold {
            if self.speed > 0 {
                self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
                self.pin_ia.set_high().map_err(L9110sError::PinIa)?;
            } else {
                self.pin_ia.set_low().map_err(L9110sError::PinIa)?;
                self.pin_ib.set_high().map_err(L9110sError::PinIb)?;
            }
        } else {
            self.pin_ia.set_low().map_err(L9110sError::PinIa)?;
            self.pin_ib.set_low().map_err(L9110sError::PinIb)?;
        }
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
