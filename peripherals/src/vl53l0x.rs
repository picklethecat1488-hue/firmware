//! Concrete driver implementation for the VL53L0X Time-of-Flight (ToF) proximity sensor.

#![deny(missing_docs)]

use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::ProximitySensor;
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
            .write(self.address, &[0x8A, new_address & 0x7F])
            .map_err(|e| e.to_i2c_error(self.address as u16, 0x8A_u16));
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
                .write(self.address, &[0x0E, low_bytes[0], low_bytes[1]])
                .map_err(|e| e.to_i2c_error(self.address as u16, 0x0E_u16))?;

            // Write SYSTEM_THRESH_HIGH (0x0C) - 16-bit value (MSB first)
            let high_val = self.threshold_mm + self.hysteresis_mm;
            let high_bytes = high_val.to_be_bytes();
            self.i2c
                .write(self.address, &[0x0C, high_bytes[0], high_bytes[1]])
                .map_err(|e| e.to_i2c_error(self.address as u16, 0x0C_u16))?;

            // Write SYSTEM_INTERRUPT_GPIO_CONFIG (0x0A) - 8-bit value
            self.i2c
                .write(self.address, &[0x0A, mode as u8])
                .map_err(|e| e.to_i2c_error(self.address as u16, 0x0A_u16))?;

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
    pub fn clear_interrupt(&mut self) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(self.address, &[0x0B, 0x01])
            .map_err(|e| e.to_i2c_error(self.address as u16, 0x0B_u16));
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
            .write(self.address, &[0x71, 0x54, 0x36])
            .map_err(|e| e.to_i2c_error(self.address as u16, 0x71_u16));
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

    /// Reads the range measurement in millimeters.
    /// Triggers start of measurement and reads the resulting 2-byte range value from register `0x1E`.
    /// Also clears the interrupt register `0x0B` to allow future interrupt cycles.
    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        let res = (|| {
            // Trigger a measurement (write 0x01 to register 0x00 for System Start)
            self.i2c
                .write(self.address, &[0x00, 0x01])
                .map_err(|e| e.to_i2c_error(self.address as u16, 0x00_u16))?;

            // Read 16-bit range result from register 0x1E (High Byte) and 0x1F (Low Byte)
            let mut buf = [0u8; 2];
            self.i2c
                .write_read(self.address, &[0x1E], &mut buf)
                .map_err(|e| e.to_i2c_error(self.address as u16, 0x1E_u16))?;
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
            assert!(
                self.threshold_mm > cal.low + THRESHOLD_ERROR_MM,
                "threshold_mm ({}) must be greater than cal.low ({}) + THRESHOLD_ERROR_MM ({})",
                self.threshold_mm,
                cal.low,
                THRESHOLD_ERROR_MM
            );
            self.calibration = cal;
        }
    }
}
