//! Generic interface for periodic updates/ticks.

#![deny(missing_docs)]

/// Trait representing a peripheral that needs periodic updates.
pub trait Tickable {
    /// Error type returned by the physical hardware.
    type Error;

    /// Updates the state of the peripheral (called periodically).
    fn tick(&mut self) -> Result<(), Self::Error>;
}

/// Wrapper for peripherals that do not need periodic updates.
/// Automatically implements `Tickable` with a no-op `tick()`.
pub struct NoTick<T>(pub T);

impl<T> NoTick<T> {
    /// Creates a new `NoTick` wrapper around the given peripheral.
    pub const fn new(peripheral: T) -> Self {
        Self(peripheral)
    }
}

impl<T> Tickable for NoTick<T> {
    /// Error type is Infallible as the no-op tick never fails.
    type Error = core::convert::Infallible;

    /// No-op implementation of tick.
    fn tick(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T: super::motor::Motor> super::motor::Motor for NoTick<T> {
    /// Error type derived from the underlying motor driver.
    type Error = T::Error;

    /// Delegates speed configuration to the inner motor driver.
    fn set_speed(&mut self, speed: crate::types::MotorSpeed) -> Result<(), Self::Error> {
        self.0.set_speed(speed)
    }

    /// Delegates stop command to the inner motor driver.
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.0.stop()
    }
}

impl<T> core::ops::Deref for NoTick<T> {
    /// Deref target is the underlying peripheral type.
    type Target = T;

    /// Dereferences to the inner peripheral.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for NoTick<T> {
    /// Mutably dereferences to the inner peripheral.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
