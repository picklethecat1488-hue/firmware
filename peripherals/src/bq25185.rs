//! Concrete driver implementation for the BQ25185 battery charger.

#![deny(missing_docs)]

use embedded_hal::i2c::I2c;
use model::interfaces::Charger;

/// Driver for the BQ25185 battery charger communicating over I2C.
pub struct Bq25185<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Bq25185<I> {
    /// Creates a new BQ25185 instance with default address 0x6B.
    pub const fn new(i2c: I) -> Self {
        Self {
            i2c,
            address: 0x6B,
        }
    }
}

impl<I: I2c> Charger for Bq25185<I> {
    type Error = I::Error;

    /// Enables or disables charging. Writes to register 0x01 (IC Ctrl).
    /// For the BQ25185, Bit 0 controls charge enable (0 = enable, 1 = disable).
    fn set_charging_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
        let val = if enabled { 0x00 } else { 0x01 }; // 0x01 disables charge
        self.i2c.write(self.address, &[0x01, val])
    }

    /// Checks if charging input is present. Reads status register 0x00.
    fn is_charging_input_present(&mut self) -> Result<bool, Self::Error> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.address, &[0x00], &mut buf)?;
        // If VBUS status is good (e.g. non-zero or specific bits set), return true
        Ok((buf[0] & 0x08) != 0)
    }
}
