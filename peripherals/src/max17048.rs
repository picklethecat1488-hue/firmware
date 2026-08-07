//! Concrete driver implementation for the MAX17048 battery fuel gauge.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::{ChargeStatus, FuelGauge, Probeable};
use model::types::{ChargeState, PeripheralError};

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "MAX17048" $(, $arg)*);
    };
}

struct Register;
impl Register {
    const VCELL: u8 = 0x02;
    const SOC: u8 = 0x04;
    const MODE: u8 = 0x06;
    const CONFIG: u8 = 0x0C;
    const VALRT: u8 = 0x14;
    const CRATE: u8 = 0x16;
    const VRESET: u8 = 0x18;
    const STATUS: u8 = 0x1A;
    const CMD: u8 = 0xFE;
}

struct StatusMask;
impl StatusMask {
    const VH: u16 = 1 << 10;
    const VL: u16 = 1 << 11;
    const HD: u16 = 1 << 13;
    const SC: u16 = 1 << 14;
}

struct ConfigMask;
impl ConfigMask {
    const ALRT: u16 = 1 << 5;
    const ALSC: u16 = 1 << 6;
}

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

    /// Initialize the fuel gauge.
    /// Checks the RI (Reset Indicator) bit in STATUS. If set, clears RI and triggers a Quick-Start.
    #[tracing::instrument(level = "trace")]
    pub fn init(&mut self) -> Result<(), PeripheralError> {
        let status = self.read_register(Register::STATUS)?;
        // RI is bit 8 (0x0100)
        if (status & 0x0100) != 0 {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!("MAX17048: Reset detected (RI set). Initializing ModelGauge and triggering Quick-Start.");

            // Clear RI bit by writing 0x00FF (clearing bit 8, keeping lower byte 0xFF)
            self.write_register(Register::STATUS, status & !0x0100)?;

            // Trigger Quick-Start by writing 0x4000 to MODE
            let mode = self.read_register(Register::MODE)?;
            self.write_register(Register::MODE, mode | 0x4000)?;
        }
        Ok(())
    }

    /// Read a 16-bit register value from the device.
    fn read_register(&mut self, reg: u8) -> Result<u16, PeripheralError> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Write a 16-bit register value to the device.
    fn write_register(&mut self, reg: u8, val: u16) -> Result<(), PeripheralError> {
        let bytes = val.to_be_bytes();
        self.i2c
            .write(self.address, &[reg, bytes[0], bytes[1]])
            .map_err(|e| e.to_i2c_error(self.address as u16, reg as u16))?;
        Ok(())
    }
}

impl<I: I2c> FuelGauge for Max17048<I> {
    type Error = PeripheralError;

    /// Reads the battery cell voltage in millivolts (mV).
    /// Formula: VCELL * 78.125 uV
    #[tracing::instrument(level = "trace")]
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        let res = self.read_register(Register::VCELL);
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read cell voltage at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let reg_val = res?;
        // Scale to mV: (reg_val * 78125) / 1000000
        let voltage_mv = (reg_val as u32 * 78125) / 1000000;
        Ok(voltage_mv)
    }

    /// Reads the battery state of charge (percentage 0-100).
    /// Formula: High byte is percentage integer, low byte is fractional.
    #[tracing::instrument(level = "trace")]
    fn read_state_of_charge(&mut self) -> Result<u8, Self::Error> {
        let res = self.read_register(Register::SOC);
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read state of charge at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        let reg_val = res?;
        let soc = (reg_val >> 8) as u8;
        Ok(soc)
    }

    /// Configure voltage and state of charge alerts.
    #[tracing::instrument(level = "trace")]
    fn configure_alerts(
        &mut self,
        voltage_min_mv: u32,
        voltage_max_mv: u32,
        soc_threshold_pct: u8,
        enable_soc_change_alert: bool,
    ) -> Result<(), Self::Error> {
        let res = (|| {
            // Write VALRT.MIN and VALRT.MAX to VALRT register
            let min_val = (voltage_min_mv / 20) as u8;
            let max_val = (voltage_max_mv / 20) as u8;
            let valrt_word = ((min_val as u16) << 8) | (max_val as u16);
            self.write_register(Register::VALRT, valrt_word)?;

            // Configure empty alert threshold (ATHD) and SOC change alert (ALSC) in CONFIG register
            let current_config = self.read_register(Register::CONFIG)?;
            let rcomp = current_config & 0xFF00; // Keep RCOMP (bits 15-8)
            let clamped_soc_threshold = soc_threshold_pct.clamp(1, 32);
            let athd = 32 - clamped_soc_threshold;
            let mut config_lsb = (athd & 0x1F) as u16;
            if enable_soc_change_alert {
                config_lsb |= ConfigMask::ALSC;
            }
            let new_config = rcomp | config_lsb;
            self.write_register(Register::CONFIG, new_config)?;

            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to configure alerts at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Check and clear active alerts.
    /// Returns (has_voltage_alert, has_soc_alert).
    fn check_and_clear_alerts(&mut self) -> Result<(bool, bool), Self::Error> {
        let res = (|| {
            let status = self.read_register(Register::STATUS)?;

            let has_voltage_alert = (status & (StatusMask::VL | StatusMask::VH)) != 0;
            let has_soc_alert = (status & (StatusMask::HD | StatusMask::SC)) != 0;

            let mut new_status = status;

            if has_soc_alert {
                // Clear CONFIG.ALRT in CONFIG register
                let config = self.read_register(Register::CONFIG)?;
                let cleared_config = config & !ConfigMask::ALRT;
                self.write_register(Register::CONFIG, cleared_config)?;

                // Clear status bits (SC and HD)
                new_status &= !(StatusMask::SC | StatusMask::HD);
            }

            if has_voltage_alert {
                // Clear status bits (VL and VH)
                new_status &= !(StatusMask::VL | StatusMask::VH);
            }

            if new_status != status {
                self.write_register(Register::STATUS, new_status)?;
            }

            Ok((has_voltage_alert, has_soc_alert))
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to check and clear alerts at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }
}

impl<I: I2c> ChargeStatus for Max17048<I> {
    type Error = PeripheralError;

    /// Checks the current charge state by reading CRATE and STATUS registers.
    #[tracing::instrument(level = "trace")]
    fn get_charge_state(&mut self) -> Result<ChargeState, Self::Error> {
        let crate_val = self.read_register(Register::CRATE)? as i16;
        let status = self.read_register(Register::STATUS)?;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!("MAX17048: CRATE={}, STATUS=0x{:04X}", crate_val, status);

        if (status & StatusMask::VH) != 0 {
            // VH (Voltage High) alert indicates a recoverable fault (e.g. overvoltage condition)
            Ok(ChargeState::RecoverableFault)
        } else if (status & StatusMask::VL) != 0 {
            // VL (Voltage Low) alert indicates a non-recoverable or critical low voltage condition
            Ok(ChargeState::NonRecoverableFault)
        } else if crate_val > 0 {
            Ok(ChargeState::Charging)
        } else {
            Ok(ChargeState::DoneOrStandbyOrUnplugged)
        }
    }
}

impl<I: I2c> Probeable for Max17048<I> {
    type Error = PeripheralError;

    #[tracing::instrument(level = "trace")]
    fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let id = self.read_register(Register::VRESET)?;
        if (id & 0x00F0) == 0x0010 || (id & 0xFF00) == 0x9600 {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    #[tracing::instrument(level = "trace")]
    fn reset(&mut self) -> Result<(), Self::Error> {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            // Writing 0x5400 to CMD resets the chip. Since the chip resets its I2C interface
            // immediately, it may abort the I2C transaction or fail to ACK the STOP condition,
            // resulting in a bus error (I2COther). We trigger the write, wait for the reset
            // to complete, and verify the chip is alive by reading the ID.
            let _ = self.write_register(Register::CMD, 0x5400);
            ::embassy_time::block_for(::embassy_time::Duration::from_millis(15)); // Wait for reset (datasheet: 15ms)
            self.read_chip_id().map(|_| ())
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        {
            self.write_register(Register::CMD, 0x5400)
        }
    }
}

/// Macro to initialize a MAX17048 fuel gauge during boot.
#[macro_export]
macro_rules! init_max17048 {
    ($i2c:expr, $boot_status:expr) => {{
        let mut fuel_gauge = $crate::max17048::Max17048::new($i2c);
        {
            use ::model::interfaces::BootStatus;
            use ::model::interfaces::Probeable;
            use $crate::ToPeripheralError;
            if let Err(ref e) = fuel_gauge.read_chip_id() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("MAX17048: Probing failed: {:?}", defmt::Debug2Format(e));
                let pe = e.to_peripheral_error();
                $boot_status.record_error(pe);
            }
            if let Err(ref e) = fuel_gauge.init() {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!(
                    "MAX17048: Initialization failed: {:?}",
                    defmt::Debug2Format(e)
                );
                let pe = e.to_peripheral_error();
                $boot_status.record_error(pe);
            }
        }
    }};
}
