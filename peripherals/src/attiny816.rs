//! Concrete driver implementation for the ATtiny816 custom LED driver over I2C.

#![deny(missing_docs)]

use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::LedDriver;
use model::types::PeripheralError;

const BASE_NEOPIXEL: u8 = 0x0E;

/// Driver for the ATtiny816 custom NeoPixel LED driver over I2C.
pub struct Attiny816<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Attiny816<I> {
    /// Creates a new ATtiny816 LED driver instance with default address (0x60).
    pub const fn new(i2c: I) -> Self {
        Self { i2c, address: 0x60 }
    }

    /// Initializes the NeoPixel driver on pin 14 with a buffer of 1 pixel (3 bytes).
    pub fn init(&mut self) -> Result<(), PeripheralError> {
        // 1. Set Output Pin to 14
        self.i2c
            .write(self.address, &[BASE_NEOPIXEL, 0x01, 14])
            .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
        // 2. Set Buffer Length (3 bytes for 1 RGB NeoPixel)
        self.i2c
            .write(self.address, &[BASE_NEOPIXEL, 0x03, 0, 3])
            .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
        Ok(())
    }

    /// Sets the color of the connected NeoPixel LED.
    /// Writes the GRB values to offset 0 and sends the show command.
    pub fn set_led_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), PeripheralError> {
        // 3. Write data to buffer (offset 0, standard GRB sequence)
        self.i2c
            .write(self.address, &[BASE_NEOPIXEL, 0x04, 0, 0, g, r, b])
            .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
        // 4. Send show command
        self.i2c
            .write(self.address, &[BASE_NEOPIXEL, 0x05])
            .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
        Ok(())
    }
}

impl<I: I2c> LedDriver for Attiny816<I> {
    type Error = PeripheralError;

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        self.set_led_color(r, g, b)
    }
}
