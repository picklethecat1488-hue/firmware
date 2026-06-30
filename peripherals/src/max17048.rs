//! Concrete driver implementation for the MAX17048 battery fuel gauge.

#![deny(missing_docs)]

use embedded_hal::i2c::I2c;
use model::interfaces::{TemperatureSensor, FuelGauge};

/// Driver for the MAX17048 fuel gauge communicating over I2C.
pub struct Max17048<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Max17048<I> {
    /// Creates a new MAX17048 driver instance with the default I2C address (0x36).
    pub const fn new(i2c: I) -> Self {
        Self {
            i2c,
            address: 0x36,
        }
    }

    /// Read a 16-bit register value from the device.
    fn read_register(&mut self, reg: u8) -> Result<u16, I::Error> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &[reg], &mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}

impl<I: I2c> FuelGauge for Max17048<I> {
    type Error = I::Error;

    /// Reads the battery cell voltage in millivolts (mV).
    /// Formula: VCELL * 78.125 uV
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        let reg_val = self.read_register(0x02)?;
        // Scale to mV: (reg_val * 78125) / 1000000
        let voltage_mv = (reg_val as u32 * 78125) / 1000000;
        Ok(voltage_mv)
    }

    /// Reads the battery state of charge (percentage 0-100).
    /// Formula: High byte is percentage integer, low byte is fractional.
    fn read_state_of_charge(&mut self) -> Result<u8, Self::Error> {
        let reg_val = self.read_register(0x04)?;
        let soc = (reg_val >> 8) as u8;
        Ok(soc)
    }
}

impl<I: I2c> TemperatureSensor for Max17048<I> {
    type Error = I::Error;

    /// Reads the battery temperature (since MAX17048 doesn't have an internal temp sensor, returns a default 25°C in millicelsius).
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(25000)
    }
}
