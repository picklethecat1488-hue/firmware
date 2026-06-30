//! Concrete driver implementation for the ATtiny816 custom LED driver over I2C.

#![deny(missing_docs)]

use embedded_hal::i2c::I2c;

/// Driver for the ATtiny816 custom NeoPixel LED driver over I2C.
pub struct Attiny816<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Attiny816<I> {
    /// Creates a new ATtiny816 LED driver instance with default address (0x60).
    pub const fn new(i2c: I) -> Self {
        Self {
            i2c,
            address: 0x60,
        }
    }

    /// Sets the color of the connected NeoPixel LED.
    /// Writes the RGB values to the custom I2C device starting at register address 0x00.
    pub fn set_led_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0x00, r, g, b])
    }
}
