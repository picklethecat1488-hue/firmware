//! Generic interface for probing and resetting peripherals.

#![deny(missing_docs)]

/// Trait representing a probeable and software-reset-capable device.
pub trait Probeable {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the device's chip ID.
    fn read_chip_id(&mut self) -> Result<u16, Self::Error>;

    /// Performs a software reset to restore the device to its default state.
    fn reset(&mut self) -> Result<(), Self::Error>;
}
