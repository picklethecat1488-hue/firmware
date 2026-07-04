//! Concrete driver implementation for the MAX17048 battery fuel gauge.

#![deny(missing_docs)]

use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::FuelGauge;
use model::types::PeripheralError;

/// Driver for the MAX17048 fuel gauge communicating over I2C.
pub struct Max17048<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Max17048<I> {
    /// Creates a new MAX17048 driver instance with the default I2C address (0x36).
    pub const fn new(i2c: I) -> Self {
        Self { i2c, address: 0x36 }
    }

    /// Read a 16-bit register value from the device.
    fn read_register(&mut self, reg: u8) -> Result<u16, PeripheralError> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .map_err(|e| e.to_peripheral_error())?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Write a 16-bit register value to the device.
    fn write_register(&mut self, reg: u8, val: u16) -> Result<(), PeripheralError> {
        let bytes = val.to_be_bytes();
        self.i2c
            .write(self.address, &[reg, bytes[0], bytes[1]])
            .map_err(|e| e.to_peripheral_error())?;
        Ok(())
    }
}

impl<I: I2c> FuelGauge for Max17048<I> {
    type Error = PeripheralError;

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

    /// Configure voltage and state of charge alerts.
    fn configure_alerts(
        &mut self,
        voltage_min_mv: u32,
        voltage_max_mv: u32,
        soc_threshold_pct: u8,
        enable_soc_change_alert: bool,
    ) -> Result<(), Self::Error> {
        // Write VALRT.MIN and VALRT.MAX to VALRT register (0x14)
        let min_val = (voltage_min_mv / 20) as u8;
        let max_val = (voltage_max_mv / 20) as u8;
        let valrt_word = ((min_val as u16) << 8) | (max_val as u16);
        self.write_register(0x14, valrt_word)?;

        // Configure empty alert threshold (ATHD) and SOC change alert (ALSC) in CONFIG register (0x0C)
        let current_config = self.read_register(0x0C)?;
        let rcomp = current_config & 0xFF00; // Keep RCOMP (bits 15-8)
        let clamped_soc_threshold = soc_threshold_pct.clamp(1, 32);
        let athd = 32 - clamped_soc_threshold;
        let mut config_lsb = athd & 0x1F;
        if enable_soc_change_alert {
            config_lsb |= 1 << 6;
        }
        let new_config = rcomp | (config_lsb as u16);
        self.write_register(0x0C, new_config)?;

        Ok(())
    }

    /// Check and clear active alerts.
    /// Returns (has_voltage_alert, has_soc_alert).
    fn check_and_clear_alerts(&mut self) -> Result<(bool, bool), Self::Error> {
        let status = self.read_register(0x1A)?;

        // VL = bit 11, VH = bit 10
        let has_voltage_alert = (status & ((1 << 11) | (1 << 10))) != 0;
        // HD = bit 13, SC = bit 14
        let has_soc_alert = (status & ((1 << 13) | (1 << 14))) != 0;

        let mut new_status = status;

        if has_soc_alert {
            // Clear CONFIG.ALRT (bit 5) in CONFIG register (0x0C)
            let config = self.read_register(0x0C)?;
            let cleared_config = config & !(1 << 5);
            self.write_register(0x0C, cleared_config)?;

            // Clear status bits (SC and HD)
            new_status &= !((1 << 14) | (1 << 13));
        }

        if has_voltage_alert {
            // Clear status bits (VL and VH)
            new_status &= !((1 << 11) | (1 << 10));
        }

        if new_status != status {
            self.write_register(0x1A, new_status)?;
        }

        Ok((has_voltage_alert, has_soc_alert))
    }
}
