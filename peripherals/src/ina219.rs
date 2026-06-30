//! Concrete driver implementation for the INA219 current/power monitor.

#![deny(missing_docs)]

use embedded_hal::i2c::I2c;
use model::interfaces::{CurrentSensor, PowerSensor};

/// Driver for the INA219 current and power monitor communicating over I2C.
pub struct Ina219<I> {
    i2c: I,
    address: u8,
    alert_callback: Option<fn()>,
}

impl<I: I2c> Ina219<I> {
    /// Creates a new INA219 driver instance with the default address (0x40).
    pub const fn new(i2c: I) -> Self {
        Self {
            i2c,
            address: 0x40,
            alert_callback: None,
        }
    }

    /// Initializes the INA219 by writing the default calibration (e.g. 4096).
    pub fn init(&mut self) -> Result<(), I::Error> {
        // Write configuration word (0x399F default settings)
        self.write_register(0x00, 0x399F)?;
        // Write calibration word (4096 LSB matches typical mA ranges)
        self.write_register(0x05, 4096)?;
        Ok(())
    }

    /// Read a 16-bit register value from the device.
    fn read_register(&mut self, reg: u8) -> Result<u16, I::Error> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &[reg], &mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Write a 16-bit register value to the device.
    fn write_register(&mut self, reg: u8, val: u16) -> Result<(), I::Error> {
        let bytes = val.to_be_bytes();
        self.i2c.write(self.address, &[reg, bytes[0], bytes[1]])
    }
}

impl<I: I2c> CurrentSensor for Ina219<I> {
    type Error = I::Error;

    /// Reads the current draw in milliamperes (mA).
    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        let val = self.read_register(0x04)? as i16;
        Ok(val as i32)
    }
}

impl<I: I2c> PowerSensor for Ina219<I> {
    type Error = I::Error;

    /// Reads current in mA.
    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        CurrentSensor::read_current_ma(self)
    }

    /// Reads the bus voltage in millivolts (mV).
    /// Formula: Bus Voltage Register bits 3-15 shift right 3, LSB is 4 mV.
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        let reg_val = self.read_register(0x02)?;
        let voltage_mv = ((reg_val >> 3) as u32) * 4;
        Ok(voltage_mv)
    }

    /// Registers the power alert callback.
    fn register_alert_callback(&mut self, callback: fn()) -> Result<(), Self::Error> {
        self.alert_callback = Some(callback);
        Ok(())
    }
}
