//! Concrete driver implementation for the VL53L0X Time-of-Flight (ToF) proximity sensor.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::{Probeable, ProximitySensor};
use model::types::PeripheralError;

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "VL53L0X" $(, $arg)*);
    };
}

/// Interrupt modes supported by the VL53L0X GPIO pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptMode {
    /// Disabled interrupt.
    Disabled = 0,
    /// GPIO interrupt triggers when distance is below low threshold.
    LowLevel = 1,
    /// GPIO interrupt triggers when distance is above high threshold.
    HighLevel = 2,
    /// GPIO interrupt triggers when distance is outside the low/high window.
    OutOfWindow = 3,
    /// GPIO interrupt triggers when a new measurement is ready.
    NewSampleReady = 4,
}

/// The default minimum safety buffer/error margin (in millimeters) between the calibration cover reading
/// and the hardware interrupt threshold, preventing the cover itself from triggering the sensor.
pub const THRESHOLD_ERROR_MM: u16 = 20;

struct Register;
impl Register {
    const SYSTEM_START: u8 = 0x00;
    const SYSTEM_INTERRUPT_GPIO_CONFIG: u8 = 0x0A;
    const SYSTEM_INTERRUPT_CLEAR: u8 = 0x0B;
    const SYSTEM_THRESH_HIGH: u8 = 0x0C;
    const SYSTEM_THRESH_LOW: u8 = 0x0E;
    const RESULT_RANGE_STATUS: u8 = 0x1E;
    const FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI: u8 = 0x71;
    const I2C_SLAVE_DEVICE_ADDRESS: u8 = 0x8A;
    const IDENTIFICATION_MODEL_ID: u8 = 0xC0;
}

/// Driver for the VL53L0X Time-of-Flight sensor communicating over I2C.
pub struct Vl53l0x<I> {
    i2c: I,
    address: u8,
    threshold_mm: u16,
    hysteresis_mm: u16,
    /// Two-point calibration values mapping raw sensor readings.
    calibration: model::calibration::TwoPointCalibration<u16>,
}

impl<I: I2c> Vl53l0x<I> {
    /// Creates a new VL53L0X driver instance at the specified address.
    pub const fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            threshold_mm: 300,
            hysteresis_mm: 50,
            calibration: model::calibration::TwoPointCalibration::new(0, 100),
        }
    }

    /// Initializes and configures the sensor.
    ///
    /// Changes the sensor's address if different from the target `new_address`,
    /// then configures the wake threshold and GPIO interrupt mode.
    pub fn init(
        &mut self,
        new_address: u8,
        threshold_mm: u16,
        interrupt_mode: InterruptMode,
    ) -> Result<(), PeripheralError> {
        if self.address != new_address {
            self.set_address(new_address)?;
        }
        self.set_threshold_mm(threshold_mm)?;
        self.configure_interrupt(interrupt_mode)?;
        Ok(())
    }

    /// Sets a new I2C address for the sensor, enabling dynamic re-addressing on shared buses.
    /// This writes register `0x8A` with the new I2C address.
    pub fn set_address(&mut self, new_address: u8) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(
                self.address,
                &[Register::I2C_SLAVE_DEVICE_ADDRESS, new_address & 0x7F],
            )
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::I2C_SLAVE_DEVICE_ADDRESS as u16,
                )
            });
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to locate or set address to 0x{:02x} (current address: 0x{:02x}): {:?}",
                new_address,
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res?;
        self.address = new_address;
        Ok(())
    }

    /// Gets the near distance threshold in millimeters.
    pub fn threshold_mm(&self) -> u16 {
        self.threshold_mm
    }

    /// Gets the current calibration.
    pub fn calibration(&self) -> model::calibration::TwoPointCalibration<u16> {
        self.calibration
    }

    /// Sets the near distance threshold in millimeters.
    pub fn set_threshold_mm(&mut self, threshold_mm: u16) -> Result<(), PeripheralError> {
        if threshold_mm <= self.calibration.low + THRESHOLD_ERROR_MM {
            return Err(PeripheralError::InvalidConfiguration);
        }
        self.threshold_mm = threshold_mm;
        Ok(())
    }

    /// Gets the hysteresis value in millimeters.
    pub fn hysteresis_mm(&self) -> u16 {
        self.hysteresis_mm
    }

    /// Sets the hysteresis value in millimeters.
    pub fn set_hysteresis_mm(&mut self, hysteresis_mm: u16) {
        self.hysteresis_mm = hysteresis_mm;
    }

    /// Configures the GPIO interrupt mode and threshold registers.
    /// Writes low threshold to `SYSTEM_THRESH_LOW` (0x0E), high threshold (low + hysteresis)
    /// to `SYSTEM_THRESH_HIGH` (0x0C), and the mode to `SYSTEM_INTERRUPT_GPIO_CONFIG` (0x0A).
    pub fn configure_interrupt(&mut self, mode: InterruptMode) -> Result<(), PeripheralError> {
        let res = (|| {
            // Write SYSTEM_THRESH_LOW (0x0E) - 16-bit value (MSB first)
            let low_bytes = self.threshold_mm.to_be_bytes();
            self.i2c
                .write(
                    self.address,
                    &[Register::SYSTEM_THRESH_LOW, low_bytes[0], low_bytes[1]],
                )
                .map_err(|e| {
                    e.to_i2c_error(self.address as u16, Register::SYSTEM_THRESH_LOW as u16)
                })?;

            // Write SYSTEM_THRESH_HIGH (0x0C) - 16-bit value (MSB first)
            let high_val = self.threshold_mm + self.hysteresis_mm;
            let high_bytes = high_val.to_be_bytes();
            self.i2c
                .write(
                    self.address,
                    &[Register::SYSTEM_THRESH_HIGH, high_bytes[0], high_bytes[1]],
                )
                .map_err(|e| {
                    e.to_i2c_error(self.address as u16, Register::SYSTEM_THRESH_HIGH as u16)
                })?;

            // Write SYSTEM_INTERRUPT_GPIO_CONFIG (0x0A) - 8-bit value
            self.i2c
                .write(
                    self.address,
                    &[Register::SYSTEM_INTERRUPT_GPIO_CONFIG, mode as u8],
                )
                .map_err(|e| {
                    e.to_i2c_error(
                        self.address as u16,
                        Register::SYSTEM_INTERRUPT_GPIO_CONFIG as u16,
                    )
                })?;

            // Clear any pending interrupt to start fresh
            self.clear_interrupt()?;

            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to configure interrupt at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Clears the interrupt status register `SYSTEM_INTERRUPT_CLEAR` (0x0B).
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub fn clear_interrupt(&mut self) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(self.address, &[Register::SYSTEM_INTERRUPT_CLEAR, 0x01])
            .map_err(|e| {
                e.to_i2c_error(self.address as u16, Register::SYSTEM_INTERRUPT_CLEAR as u16)
            });
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to clear interrupt at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Sets the measurement timing budget to 200ms (High Accuracy mode).
    /// This writes the calculated timeout value to register `FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI` (0x71).
    pub fn set_timing_budget_200ms(&mut self) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(
                self.address,
                &[Register::FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI, 0x54, 0x36],
            )
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI as u16,
                )
            });
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to set timing budget at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }
}

impl<I: I2c> ProximitySensor for Vl53l0x<I> {
    type Error = PeripheralError;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        let res = (|| {
            // Trigger a measurement (write 0x01 to register 0x00 for System Start)
            self.i2c
                .write(self.address, &[Register::SYSTEM_START, 0x01])
                .map_err(|e| e.to_i2c_error(self.address as u16, Register::SYSTEM_START as u16))?;

            // Read 16-bit range result from register 0x1E (High Byte) and 0x1F (Low Byte)
            let mut buf = [0u8; 2];
            self.i2c
                .write_read(self.address, &[Register::RESULT_RANGE_STATUS], &mut buf)
                .map_err(|e| {
                    e.to_i2c_error(self.address as u16, Register::RESULT_RANGE_STATUS as u16)
                })?;
            let mut distance = u16::from_be_bytes(buf);

            // Clear interrupt status so the pin can trigger again (write 0x01 to register 0x0B)
            self.clear_interrupt()?;

            // Apply two-point calibration
            distance = self.calibration.map(distance, 100);
            Ok(distance)
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to read distance at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }
}

impl<I: I2c> model::calibration::Calibration for Vl53l0x<I> {
    fn set_calibration(&mut self, calibration: model::calibration::CalibrationType) {
        if let model::calibration::CalibrationType::ProximityCal(cal) = calibration {
            if self.threshold_mm > cal.low + THRESHOLD_ERROR_MM {
                self.calibration = cal;
            } else {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!(
                    "Invalid proximity calibration (low = {}, threshold_mm = {}). Ignoring and using defaults.",
                    cal.low,
                    self.threshold_mm
                );
            }
        }
    }
}

impl<I: I2c> Probeable for Vl53l0x<I> {
    type Error = PeripheralError;

    fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::IDENTIFICATION_MODEL_ID], &mut buf)
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::IDENTIFICATION_MODEL_ID as u16,
                )
            })?;
        let id = buf[0] as u16;
        if id == 0xEE {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    fn reset(&mut self) -> Result<(), Self::Error> {
        // No software-initiated reset register on the VL53L0X.
        // It relies on the hardware XSHUT pin for reset, so this is a no-op.
        Ok(())
    }
}
