//! Generic LED driver interface.

#![deny(missing_docs)]

/// Trait representing an RGB LED indicator.
pub trait LedDriver {
    /// Error type returned by the physical hardware.
    type Error;

    /// Sets the color of the LED using RGB values.
    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error>;
}
