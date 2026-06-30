//! Concrete driver implementation for the VL53L0X Time-of-Flight (ToF) proximity sensor.

#![deny(missing_docs)]

use embedded_hal::i2c::I2c;
use model::interfaces::ProximitySensor;

/// Driver for the VL53L0X Time-of-Flight sensor communicating over I2C.
pub struct Vl53l0x<I> {
    i2c: I,
    address: u8,
    proximity_callback: Option<fn(bool)>,
}

impl<I: I2c> Vl53l0x<I> {
    /// Creates a new VL53L0X driver instance at the specified address.
    pub const fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            proximity_callback: None,
        }
    }

    /// Sets a new I2C address for the sensor, enabling dynamic re-addressing on shared buses.
    /// This writes register `0x8A` with the new I2C address.
    pub fn set_address(&mut self, new_address: u8) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[0x8A, new_address & 0x7F])?;
        self.address = new_address;
        Ok(())
    }
}

impl<I: I2c> ProximitySensor for Vl53l0x<I> {
    type Error = I::Error;

    /// Reads the range measurement in millimeters.
    /// Triggers start of measurement and reads the resulting 2-byte range value from register `0x1E`.
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        // Trigger a measurement (write 0x01 to register 0x00 for System Start)
        self.i2c.write(self.address, &[0x00, 0x01])?;

        // Read 16-bit range result from register 0x1E (High Byte) and 0x1F (Low Byte)
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &[0x1E], &mut buf)?;
        let distance = u16::from_be_bytes(buf);

        // Optionally fire the callback if registered (for example, target threshold is < 300 mm)
        if let Some(cb) = self.proximity_callback {
            cb(distance < 300);
        }

        Ok(distance)
    }

    /// Registers a callback for proximity detection events.
    fn register_proximity_callback(&mut self, callback: fn(bool)) -> Result<(), Self::Error> {
        self.proximity_callback = Some(callback);
        Ok(())
    }
}
