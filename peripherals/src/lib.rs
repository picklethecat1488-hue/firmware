//! Peripherals crate containing platform-agnostic generic driver wrappers.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Generic and platform-specific motor peripherals.
pub mod motor;

/// Concrete driver implementation for the ATtiny816 custom LED driver.
pub mod attiny816;
/// Concrete driver implementation for the BQ25185 battery charger.
pub mod bq25185;
/// Concrete driver implementation for the INA219 current monitor.
pub mod ina219;
/// Concrete driver implementation for the L9110S motor driver.
pub mod l9110s;
/// Concrete driver implementation for the MAX17048 fuel gauge.
pub mod max17048;
/// Concrete driver implementation for the VL53L0X proximity sensor.
pub mod vl53l0x;

/// Implement bus error telemetry forwarding.
use model::types::PeripheralError;

/// Implement the trait to convert HAL errors into telemetry.
pub trait ToPeripheralError {
    /// Convert the bus error to a Peripheral error.
    fn to_peripheral_error(&self) -> PeripheralError;
}

/// Extension trait to convert I2C bus errors into telemetry.
pub trait I2cToPeripheralError {
    /// Convert the I2C bus error to a Peripheral error.
    fn to_peripheral_error(&self) -> PeripheralError;
}

/// Convert I2C bus errors into bus telemetry.
impl<E> I2cToPeripheralError for E
where
    E: embedded_hal::i2c::Error,
{
    #[inline]
    fn to_peripheral_error(&self) -> PeripheralError {
        use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
        match self.kind() {
            ErrorKind::Bus => PeripheralError::I2CBusError,
            ErrorKind::ArbitrationLoss => PeripheralError::I2CArbitrationLoss,
            ErrorKind::Overrun => PeripheralError::I2COverrun,

            // NoAcknowledge contains extra nested context we can decode
            ErrorKind::NoAcknowledge(source) => match source {
                NoAcknowledgeSource::Address => PeripheralError::I2CNackAddress,
                NoAcknowledgeSource::Data => PeripheralError::I2CNackData,
                _ => PeripheralError::I2CNackUnknown,
            },

            ErrorKind::Other => PeripheralError::I2COther,
            _ => PeripheralError::I2CUnknown,
        }
    }
}

/// Implement ToPeripheralError for PeripheralError (no-op conversion).
impl ToPeripheralError for PeripheralError {
    #[inline]
    fn to_peripheral_error(&self) -> PeripheralError {
        *self
    }
}

/// Implement ToPeripheralError for core::convert::Infallible.
impl ToPeripheralError for core::convert::Infallible {
    #[inline]
    fn to_peripheral_error(&self) -> PeripheralError {
        match *self {}
    }
}

/// Implement ToPeripheralError for ().
impl ToPeripheralError for () {
    #[inline]
    fn to_peripheral_error(&self) -> PeripheralError {
        PeripheralError::Unknown
    }
}

/// Implement ToPeripheralError for L9110sError.
impl<E1: core::fmt::Debug, E2: core::fmt::Debug> ToPeripheralError
    for crate::l9110s::L9110sError<E1, E2>
{
    #[inline]
    fn to_peripheral_error(&self) -> PeripheralError {
        PeripheralError::PinError
    }
}

/// Mock implementations of peripherals for host-based testing.
#[cfg(any(test, feature = "mock"))]
pub mod mock;
