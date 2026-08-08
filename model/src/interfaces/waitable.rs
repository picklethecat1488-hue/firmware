//! Trait for peripherals that support waiting for a measurement to complete.

#![deny(missing_docs)]

/// Trait for peripherals that support waiting for a measurement to complete.
pub trait WaitableMeasurement {
    /// Wait until a measurement conversion is complete.
    fn wait_for_measurement(&mut self) -> Result<(), crate::types::PeripheralError>;
}
