//! Generic interface for periodic updates/ticks.

#![deny(missing_docs)]

/// Trait representing a peripheral that needs periodic updates.
pub trait Tickable {
    /// Error type returned by the physical hardware.
    type Error;

    /// Desired tick interval in microseconds for this periodic peripheral.
    ///
    /// Peripherals requiring high-frequency periodic updates (such as high-speed BLDC
    /// commutation drivers) can override this method to define their required tick rate.
    /// General peripherals default to 10,000 microseconds (10ms / 100Hz).
    fn tick_interval_us(&self) -> u32 {
        10_000
    }

    /// Updates the state of the peripheral (called periodically).
    async fn tick(&mut self) -> Result<(), Self::Error>;
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

    /// No-op peripherals default to standard 10ms tick interval.
    fn tick_interval_us(&self) -> u32 {
        10_000
    }

    /// No-op implementation of tick.
    async fn tick(&mut self) -> Result<(), Self::Error> {
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

impl<T: super::speed_sensor::SpeedSensor> super::speed_sensor::SpeedSensor for NoTick<T> {
    /// Error type derived from the underlying speed sensor.
    type Error = T::Error;

    /// Delegates RPM reading to the inner speed sensor.
    fn read_rpm(&mut self) -> Result<u32, Self::Error> {
        self.0.read_rpm()
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
