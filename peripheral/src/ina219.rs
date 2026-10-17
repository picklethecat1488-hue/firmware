//! Concrete driver implementation for the INA219 current/power monitor.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal_async::i2c::I2c;
use model::{
    interfaces::{PowerMeasurementMode, PowerSensor, Probeable, WaitableMeasurement},
    types::PeripheralError,
};

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "INA219" $(, $arg)*);
    };
}

/// Default I2C 7-bit address for the INA219 current/power monitor.
pub const DEFAULT_I2C_ADDRESS: u8 = 0x40;

/// Default configuration word (0x399F: 32V bus range, +-320mV shunt range, 12-bit ADC, continuous).
pub const DEFAULT_CONFIG: u16 = 0x399F;

/// Software reset configuration word (bit 15 RST = 1).
pub const RESET_CONFIG: u16 = 0x8000;

/// Default calibration register value for 0.1 ohm shunt resistor (0.1 mA / LSB).
pub const DEFAULT_CALIBRATION: u16 = 4096;

/// Scale factor to convert 0.1 mA counts from INA219 current/shunt registers to integer milliamperes.
pub const CURRENT_LSB_DIVISOR: i32 = 10;

/// Conversion ready (CNVR) bit mask in the Bus Voltage Register (0x02, bit 1).
pub const BUS_VOLTAGE_CNVR_MASK: u16 = 0x0002;

/// Bus voltage bit shift to extract 13-bit voltage data from Bus Voltage Register (bits 3..15).
pub const BUS_VOLTAGE_SHIFT: u32 = 3;

/// Bus voltage LSB resolution in millivolts (4 mV / LSB).
pub const BUS_VOLTAGE_LSB_MV: u32 = 4;

/// Timeout in milliseconds for waiting for a measurement conversion.
pub const MEASUREMENT_TIMEOUT_MS: u32 = 100;

struct Register;
impl Register {
    const CONFIG: u8 = 0x00;
    const SHUNT_VOLTAGE: u8 = 0x01;
    const BUS_VOLTAGE: u8 = 0x02;
    const CURRENT: u8 = 0x04;
    const CALIBRATION: u8 = 0x05;
}

/// Driver for the INA219 current and power monitor communicating over I2C.
pub struct Ina219<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Ina219<I> {
    /// Creates a new INA219 driver instance with the default address (0x40).
    pub const fn new(i2c: I) -> Self {
        Self {
            i2c,
            address: DEFAULT_I2C_ADDRESS,
        }
    }

    /// Initializes the INA219 by writing the default calibration (e.g. 4096).
    pub async fn init(&mut self) -> Result<(), PeripheralError> {
        // Write configuration word (0x399F default settings)
        self.write_register(Register::CONFIG, DEFAULT_CONFIG)
            .await?;
        // Write calibration word (4096 LSB matches typical 0.1 mA per count)
        self.write_register(Register::CALIBRATION, DEFAULT_CALIBRATION)
            .await?;
        Ok(())
    }

    /// Read a 16-bit register value from the device.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    async fn read_register(&mut self, reg: u8) -> Result<u16, PeripheralError> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Write a 16-bit register value to the device.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    async fn write_register(&mut self, reg: u8, val: u16) -> Result<(), PeripheralError> {
        let bytes = val.to_be_bytes();
        self.i2c
            .write(self.address, &[reg, bytes[0], bytes[1]])
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(())
    }
}

impl<I: I2c> WaitableMeasurement for Ina219<I> {
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        // Poll CNVR bit (bit 1) in the Bus Voltage Register (0x02)
        // Set a timeout of 100ms (100 * 1ms)
        for _ in 0..MEASUREMENT_TIMEOUT_MS {
            let bus_voltage = self.read_register(Register::BUS_VOLTAGE).await?;
            if (bus_voltage & BUS_VOLTAGE_CNVR_MASK) != 0 {
                return Ok(());
            }
            ::embassy_time::Timer::after_millis(1).await;
        }
        Err(PeripheralError::DeviceNotAvailable)
    }
}

impl<I: I2c> PowerSensor for Ina219<I> {
    type Error = PeripheralError;

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    async fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        self.wait_for_measurement().await?;
        let res = self.read_register(Register::CURRENT).await;
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read current register at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let val = res? as i16;
        if val != 0 {
            Ok((val as i32) / CURRENT_LSB_DIVISOR)
        } else {
            // Fallback to shunt voltage register in case calibration register was lost/cleared
            let shunt_raw = self.read_register(Register::SHUNT_VOLTAGE).await? as i16;
            if shunt_raw != 0 {
                // Restore calibration register
                let _ = self
                    .write_register(Register::CALIBRATION, DEFAULT_CALIBRATION)
                    .await;
                Ok((shunt_raw as i32) / CURRENT_LSB_DIVISOR)
            } else {
                Ok(0)
            }
        }
    }

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    async fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        self.wait_for_measurement().await?;
        let res = self.read_register(Register::BUS_VOLTAGE).await;
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read voltage register at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let reg_val = res?;
        let voltage_mv = ((reg_val >> BUS_VOLTAGE_SHIFT) as u32) * BUS_VOLTAGE_LSB_MV;
        Ok(voltage_mv)
    }

    /// Sets the operating mode of the sensor.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    async fn set_measurement_mode(
        &mut self,
        mode: PowerMeasurementMode,
    ) -> Result<(), Self::Error> {
        let config = self.read_register(Register::CONFIG).await?;
        let mode_val = match mode {
            PowerMeasurementMode::PowerDown => 0,
            PowerMeasurementMode::OneShot(voltage, current) => match (voltage, current) {
                (false, true) => 1,
                (true, false) => 2,
                (true, true) => 3,
                (false, false) => 4,
            },
            PowerMeasurementMode::Continuous(voltage, current) => match (voltage, current) {
                (false, true) => 5,
                (true, false) => 6,
                (true, true) => 7,
                (false, false) => 4,
            },
        };
        let new_config = (config & 0xFFF8) | mode_val;
        self.write_register(Register::CONFIG, new_config).await?;
        if mode != PowerMeasurementMode::PowerDown {
            self.write_register(Register::CALIBRATION, DEFAULT_CALIBRATION)
                .await?;
        }
        Ok(())
    }
}

impl<I: I2c> Probeable for Ina219<I> {
    type Error = PeripheralError;

    #[tracing::instrument(level = "trace")]
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let id = self.read_register(Register::CONFIG).await?;
        if (id & 0xFFF8) == 0x3998 {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn reset(&mut self) -> Result<(), Self::Error> {
        self.write_register(Register::CONFIG, RESET_CONFIG).await
    }
}
