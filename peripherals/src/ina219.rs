//! Concrete driver implementation for the INA219 current/power monitor.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::{
    interfaces::{PowerMeasurementMode, PowerSensor, Probeable},
    types::PeripheralError,
};

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "INA219" $(, $arg)*);
    };
}

struct Register;
impl Register {
    const CONFIG: u8 = 0x00;
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
        Self { i2c, address: 0x40 }
    }

    /// Initializes the INA219 by writing the default calibration (e.g. 4096).
    pub fn init(&mut self) -> Result<(), PeripheralError> {
        let res = (|| {
            // Write configuration word (0x399F default settings)
            self.write_register(Register::CONFIG, 0x399F)?;
            // Write calibration word (4096 LSB matches typical mA ranges)
            self.write_register(Register::CALIBRATION, 4096)?;
            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to locate or initialize current/power monitor at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Read a 16-bit register value from the device.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn read_register(&mut self, reg: u8) -> Result<u16, PeripheralError> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Write a 16-bit register value to the device.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn write_register(&mut self, reg: u8, val: u16) -> Result<(), PeripheralError> {
        let bytes = val.to_be_bytes();
        self.i2c
            .write(self.address, &[reg, bytes[0], bytes[1]])
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(())
    }
}

impl<I: I2c> PowerSensor for Ina219<I> {
    type Error = PeripheralError;

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        let res = self.read_register(Register::CURRENT);
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read current register at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let val = res? as i16;
        Ok(val as i32)
    }

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        let res = self.read_register(Register::BUS_VOLTAGE);
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read voltage register at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let reg_val = res?;
        let voltage_mv = ((reg_val >> 3) as u32) * 4;
        Ok(voltage_mv)
    }

    /// Sets the operating mode of the sensor.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_measurement_mode(&mut self, mode: PowerMeasurementMode) -> Result<(), Self::Error> {
        let res = (|| {
            let config = self.read_register(Register::CONFIG)?;
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
            self.write_register(Register::CONFIG, new_config)?;
            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to set measurement mode at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }
}

impl<I: I2c> Probeable for Ina219<I> {
    type Error = PeripheralError;

    #[tracing::instrument(level = "trace")]
    fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let id = self.read_register(Register::CONFIG)?;
        if id == 0x399F {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    #[tracing::instrument(level = "trace")]
    fn reset(&mut self) -> Result<(), Self::Error> {
        self.write_register(Register::CONFIG, 0x8000)
    }
}

/// Macro to initialize an INA219 current monitor during boot.
#[macro_export]
macro_rules! init_ina219 {
    ($i2c:expr, $boot_status:expr) => {{
        let mut current_sensor = $crate::ina219::Ina219::new($i2c);
        {
            use ::model::interfaces::BootStatus;
            use ::model::interfaces::Probeable;
            use $crate::ToPeripheralError;
            if let Err(ref e) = current_sensor.read_chip_id() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("INA219: Probing failed: {:?}", defmt::Debug2Format(e));
                let pe = e.to_peripheral_error();
                $boot_status.record_error(pe);
            }
            if let Err(ref e) = current_sensor.reset() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("INA219: Reset failed: {:?}", defmt::Debug2Format(e));
                let pe = e.to_peripheral_error();
                $boot_status.record_error(pe);
            }
        }
        if let Err(e) = current_sensor.init() {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::warn!("INA219: Init failed: {:?}", defmt::Debug2Format(&e));
            use ::model::interfaces::BootStatus;
            use $crate::ToPeripheralError;
            let pe = e.to_peripheral_error();
            $boot_status.record_error(pe);
        }
    }};
}
