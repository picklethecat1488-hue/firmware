//! Concrete driver implementation for the VL53L0X Time-of-Flight (ToF) proximity sensor.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal_async::i2c::I2c;
use model::calibration::{ApplyCalibration, Calibration, TwoPointCalibration};
use model::interfaces::{Probeable, ProximitySensor, WaitableMeasurement};
use model::types::{Direction, PeripheralError, SensorReading};

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "VL53L0X" $(, $arg)*);
    };
}

/// Interrupt modes supported by the VL53L0X GPIO pin.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
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
    const SYSTEM_SEQUENCE_CONFIG: u8 = 0x01;
    const SYSTEM_INTERRUPT_GPIO_CONFIG: u8 = 0x0A;
    const SYSTEM_INTERRUPT_CLEAR: u8 = 0x0B;
    const SYSTEM_THRESH_HIGH: u8 = 0x0C;
    const SYSTEM_THRESH_LOW: u8 = 0x0E;
    const RESULT_INTERRUPT_STATUS: u8 = 0x13;
    const RESULT_RANGE_STATUS: u8 = 0x14;
    const RESULT_RANGE_VAL: u8 = 0x1E;
    const FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI: u8 = 0x71;
    const I2C_SLAVE_DEVICE_ADDRESS: u8 = 0x8A;
    const IDENTIFICATION_MODEL_ID: u8 = 0xC0;
}

struct RangeStatus {
    valid: bool,
    min_range: bool,
    max_range: bool,
}

/// Driver for the VL53L0X Time-of-Flight sensor communicating over I2C.
pub struct Vl53l0x<I> {
    i2c: I,
    pub(crate) address: u8,
    pub(crate) threshold_mm: u16,
    hysteresis_mm: u16,
    /// Two-point calibration values mapping raw sensor readings.
    calibration: Option<TwoPointCalibration<u16>>,
    /// Configured interrupt mode.
    interrupt_mode: InterruptMode,
    /// Active crosstalk compensation peak rate in milli-MCPS.
    xtalk_m_mcps: u16,
    /// Pending crosstalk compensation to be applied asynchronously.
    xtalk_pending: Option<u16>,
    /// Sensor direction.
    pub direction: Direction,
}

impl<I: I2c> Vl53l0x<I> {
    /// Minimum physical sensor range limit.
    pub const MIN_RANGE_MM: u16 = 20;
    /// Maximum physical sensor range limit.
    pub const MAX_RANGE_MM: u16 = 1000;

    /// Creates a new VL53L0X driver instance at the specified address.
    pub const fn new(i2c: I, address: u8, direction: Direction) -> Self {
        Self {
            i2c,
            address,
            direction,
            threshold_mm: 300,
            hysteresis_mm: 50,
            calibration: None,
            interrupt_mode: InterruptMode::Disabled,
            xtalk_m_mcps: 0,
            xtalk_pending: None,
        }
    }

    /// Initializes and configures the sensor.
    ///
    /// Changes the sensor's address from the default (0x29) to the configured address if needed,
    /// then configures the wake threshold and GPIO interrupt mode.
    pub async fn init(&mut self) -> Result<(), PeripheralError> {
        let target_address = self.address;
        // Check if the sensor is responsive at the target address.
        // If not, it is likely still at the default address (0x29) and needs to be re-addressed.
        let mut temp_buf = [0u8; 1];
        if self
            .i2c
            .write_read(
                self.address,
                &[Register::IDENTIFICATION_MODEL_ID],
                &mut temp_buf,
            )
            .await
            .is_err()
        {
            self.address = 0x29;
            if let Err(e) = self.set_address(target_address).await {
                self.address = target_address;
                return Err(e);
            }
            self.address = target_address;
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            ::embassy_time::Timer::after_millis(2).await;
        }

        self.configure_interrupt(self.interrupt_mode).await?;

        self.set_enable_ranging_sequence().await?;

        // Configure High Sensitivity mode for dark/matte targets (like black/grey cat fur)
        self.set_timing_budget_200ms().await?;
        self.set_signal_rate_limit(6).await?; // 0.05 MCPS limit (default is 0.25 MCPS)

        Ok(())
    }

    /// Enables the ranging sequence steps (MSRC, DSS, Pre-Range, Final Range).
    async fn set_enable_ranging_sequence(&mut self) -> Result<(), PeripheralError> {
        self.i2c
            .write(self.address, &[Register::SYSTEM_SEQUENCE_CONFIG, 0xFF])
            .await
            .map_err(|e| {
                e.to_i2c_error(self.address as u16, Register::SYSTEM_SEQUENCE_CONFIG as u16)
            })
    }

    /// Sets a new I2C address for the sensor, enabling dynamic re-addressing on shared buses.
    /// This writes register `0x8A` with the new I2C address.
    pub async fn set_address(&mut self, new_address: u8) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(
                self.address,
                &[Register::I2C_SLAVE_DEVICE_ADDRESS, new_address & 0x7F],
            )
            .await
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
    pub fn calibration(&self) -> Option<TwoPointCalibration<u16>> {
        self.calibration
    }

    /// Sets the near distance threshold in millimeters.
    pub fn set_threshold_mm(&mut self, threshold_mm: u16) -> Result<(), PeripheralError> {
        let low = self.calibration.map(|c| c.low).unwrap_or(0);
        if threshold_mm <= low + THRESHOLD_ERROR_MM {
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

    /// Sets the GPIO interrupt mode.
    pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
        self.interrupt_mode = mode;
    }

    /// Configures the GPIO interrupt mode and threshold registers.
    /// Writes low threshold to `SYSTEM_THRESH_LOW` (0x0E), high threshold (low + hysteresis)
    /// to `SYSTEM_THRESH_HIGH` (0x0C), and the mode to `SYSTEM_INTERRUPT_GPIO_CONFIG` (0x0A).
    pub async fn configure_interrupt(
        &mut self,
        mode: InterruptMode,
    ) -> Result<(), PeripheralError> {
        // Write SYSTEM_THRESH_LOW (0x0E) - 16-bit value (MSB first)
        let low_bytes = self.threshold_mm.to_be_bytes();
        self.i2c
            .write(
                self.address,
                &[Register::SYSTEM_THRESH_LOW, low_bytes[0], low_bytes[1]],
            )
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, Register::SYSTEM_THRESH_LOW as u16))?;

        // Write SYSTEM_THRESH_HIGH (0x0C) - 16-bit value (MSB first)
        let high_val = self.threshold_mm + self.hysteresis_mm;
        let high_bytes = high_val.to_be_bytes();
        self.i2c
            .write(
                self.address,
                &[Register::SYSTEM_THRESH_HIGH, high_bytes[0], high_bytes[1]],
            )
            .await
            .map_err(|e| {
                e.to_i2c_error(self.address as u16, Register::SYSTEM_THRESH_HIGH as u16)
            })?;

        // Write SYSTEM_INTERRUPT_GPIO_CONFIG (0x0A) - 8-bit value
        self.i2c
            .write(
                self.address,
                &[Register::SYSTEM_INTERRUPT_GPIO_CONFIG, mode as u8],
            )
            .await
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::SYSTEM_INTERRUPT_GPIO_CONFIG as u16,
                )
            })?;

        // Clear any pending interrupt to start fresh
        self.clear_interrupt().await?;

        self.interrupt_mode = mode;

        Ok(())
    }

    /// Clears the interrupt status register `SYSTEM_INTERRUPT_CLEAR` (0x0B).
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub async fn clear_interrupt(&mut self) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(self.address, &[Register::SYSTEM_INTERRUPT_CLEAR, 0x01])
            .await
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
    pub async fn set_timing_budget_200ms(&mut self) -> Result<(), PeripheralError> {
        let res = self
            .i2c
            .write(
                self.address,
                &[Register::FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI, 0x54, 0x36],
            )
            .await
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

    /// Sets the return signal rate limit check threshold (in fixed-point Q9.7 MCPS format).
    /// Standard is 0.25 MCPS (32). Lowering it (e.g. to 0.05 MCPS (6)) allows detecting dark/matte targets.
    pub async fn set_signal_rate_limit(&mut self, limit_mcps: u16) -> Result<(), PeripheralError> {
        let bytes = limit_mcps.to_be_bytes();
        let res = self
            .i2c
            .write(self.address, &[0x44, bytes[0], bytes[1]])
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, 0x44));
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to set signal rate limit to {} at address 0x{:02x}: {:?}",
                limit_mcps,
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Sets the crosstalk compensation peak rate in milli-MCPS (Mega Counts Per Second * 1000).
    /// Converts the value to Q9.7 fixed-point format and writes to register `0x20`.
    /// A value of 0 disables crosstalk compensation.
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    pub async fn set_crosstalk_compensation(
        &mut self,
        rate_m_mcps: u16,
    ) -> Result<(), PeripheralError> {
        let val = ((rate_m_mcps as u32 * 128) / 1000) as u16;
        let bytes = val.to_be_bytes();
        self.i2c
            .write(self.address, &[0x20, bytes[0], bytes[1]])
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, 0x20))?;
        self.xtalk_m_mcps = rate_m_mcps;
        Ok(())
    }
}

impl<I: I2c> WaitableMeasurement for Vl53l0x<I> {
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        // Poll for measurement completion
        // Set a timeout of 240ms (24 * 10ms)
        for _ in 0..24 {
            let mut status = [0u8; 1];
            self.i2c
                .write_read(
                    self.address,
                    &[Register::RESULT_INTERRUPT_STATUS],
                    &mut status,
                )
                .await
                .map_err(|e| {
                    e.to_i2c_error(
                        self.address as u16,
                        Register::RESULT_INTERRUPT_STATUS as u16,
                    )
                })?;

            #[cfg(all(
                target_arch = "arm",
                target_os = "none",
                feature = "verbose-sensor-logging"
            ))]
            {
                defmt::trace!(
                    "VL53L0X [0x{:02x}] interrupt-status: {:02x}",
                    self.address,
                    status[0]
                );
            }
            if (status[0] & 0x07) == 4 {
                return Ok(());
            }
            ::embassy_time::Timer::after_millis(10).await;
        }
        Err(PeripheralError::DeviceNotAvailable)
    }
}

impl<I: I2c> Vl53l0x<I> {
    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    /// Read range status register 0x14
    async fn read_range_status(&mut self) -> Result<RangeStatus, PeripheralError> {
        let mut status = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::RESULT_RANGE_STATUS], &mut status)
            .await
            .map_err(|e| {
                e.to_i2c_error(self.address as u16, Register::RESULT_RANGE_STATUS as u16)
            })?;
        #[cfg(all(
            target_arch = "arm",
            target_os = "none",
            feature = "verbose-sensor-logging"
        ))]
        {
            defmt::trace!(
                "VL53L0X [0x{:02x}] range-status: {:02x}",
                self.address,
                status[0]
            );
        }
        let range_status_code = (status[0] >> 3) & 0x0F;
        Ok(RangeStatus {
            valid: (status[0] & 0x01) != 0,
            min_range: range_status_code == 3,
            max_range: range_status_code == 4 || range_status_code == 8,
        })
    }

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    /// Reads the raw measured distance and the peak return rate (in Q9.7 register format).
    pub async fn read_raw_distance_and_rate_internal(
        &mut self,
    ) -> Result<(SensorReading, u16), PeripheralError> {
        if let Some(xtalk) = self.xtalk_pending.take() {
            let _ = self.set_crosstalk_compensation(xtalk).await;
        }

        // Temporarily configure interrupt mode to NewSampleReady (4)
        self.i2c
            .write(
                self.address,
                &[
                    Register::SYSTEM_INTERRUPT_GPIO_CONFIG,
                    InterruptMode::NewSampleReady as u8,
                ],
            )
            .await
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::SYSTEM_INTERRUPT_GPIO_CONFIG as u16,
                )
            })?;

        // Trigger a measurement (write 0x01 to register 0x00 for System Start)
        self.i2c
            .write(self.address, &[Register::SYSTEM_START, 0x01])
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, Register::SYSTEM_START as u16))?;

        let wait_res = self.wait_for_measurement().await;

        // Restore the configured interrupt mode
        let restore_res = self
            .i2c
            .write(
                self.address,
                &[
                    Register::SYSTEM_INTERRUPT_GPIO_CONFIG,
                    self.interrupt_mode as u8,
                ],
            )
            .await
            .map_err(|e| {
                e.to_i2c_error(
                    self.address as u16,
                    Register::SYSTEM_INTERRUPT_GPIO_CONFIG as u16,
                )
            });

        wait_res?;
        restore_res?;

        let range_status = self.read_range_status().await?;

        // Read 16-bit range result from register 0x1E (High Byte) and 0x1F (Low Byte)
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[Register::RESULT_RANGE_VAL], &mut buf)
            .await
            .map_err(|e| e.to_i2c_error(self.address as u16, Register::RESULT_RANGE_VAL as u16))?;
        let distance = u16::from_be_bytes(buf);

        // Read peak signal rate (registers 0x1A and 0x1B)
        let mut buf_rate = [0u8; 2];
        let _ = self
            .i2c
            .write_read(self.address, &[0x1A], &mut buf_rate)
            .await;
        let peak_rate = u16::from_be_bytes(buf_rate);

        // Clear interrupt status so the pin can trigger again (write 0x01 to register 0x0B)
        self.clear_interrupt().await?;

        #[cfg(all(
            target_arch = "arm",
            target_os = "none",
            feature = "verbose-sensor-logging"
        ))]
        {
            // Read ambient rate (registers 0x1C and 0x1D)
            let mut buf_ambient = [0u8; 2];
            let _ = self
                .i2c
                .write_read(self.address, &[0x1C], &mut buf_ambient)
                .await;
            let ambient_rate = u16::from_be_bytes(buf_ambient);

            defmt::debug!(
                    "VL53L0X [0x{:02x}] mm-read: Raw Dist = {} mm, Peak Rate = {} Mcps, Ambient Rate = {} Mcps, Range Valid = {}, Min = {}, Max = {}",
                    self.address,
                    distance,
                    peak_rate,
                    ambient_rate,
                    range_status.valid,
                    range_status.min_range,
                    range_status.max_range,
                );
        }

        let reading = if !range_status.valid || range_status.max_range {
            SensorReading::Invalid
        } else if range_status.min_range {
            SensorReading::Proximity(Self::MIN_RANGE_MM)
        } else {
            SensorReading::Proximity(distance)
        };
        Ok((reading, peak_rate))
    }
}

impl<I: I2c> ProximitySensor for Vl53l0x<I> {
    type Error = PeripheralError;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    async fn read_distance_mm(&mut self) -> Result<SensorReading, Self::Error> {
        let (raw_reading, _) = self.read_raw_distance_and_rate_internal().await?;
        Ok(raw_reading)
    }

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    async fn read_raw_distance_and_rate(&mut self) -> Result<(SensorReading, u16), Self::Error> {
        self.read_raw_distance_and_rate_internal().await
    }
}

impl<I: I2c> Calibration for Vl53l0x<I> {
    const CALIBRATION_FILE_NAME: &'static str =
        model::calibration::Vl53l0xCalibration::CALIBRATION_FILE_NAME;

    type Store = model::calibration::Vl53l0xCalibration;

    fn set_calibration(&mut self, store: &Self::Store) {
        let dir = self.direction;
        let cal = store.sensors[dir as usize];
        let xtalk = store.xtalk_m_mcps[dir as usize];
        if self.threshold_mm > cal.low + THRESHOLD_ERROR_MM {
            self.calibration = Some(cal);
            // Defer crosstalk compensation setup to async write path
            self.xtalk_pending = Some(xtalk);
        } else {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!(
                "Invalid proximity calibration (low = {}, threshold_mm = {}). Ignoring and using defaults.",
                cal.low,
                self.threshold_mm
            );
        }
    }

    fn get_calibration(&self) -> Self::Store {
        let mut store = Self::Store::default();
        let dir = self.direction;
        if let Some(cal) = self.calibration {
            store.sensors[dir as usize] = cal;
        }
        store.xtalk_m_mcps[dir as usize] = self.xtalk_m_mcps;
        store
    }
}

impl<I: I2c> ApplyCalibration for Vl53l0x<I> {
    type Input = SensorReading;
    type Output = SensorReading;
    type Error = &'static str;

    #[cfg_attr(
        all(target_arch = "arm", feature = "sensors-core"),
        link_section = ".data.core1_func"
    )]
    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        match reading {
            SensorReading::Proximity(distance) => {
                if let Some(cal) = self.calibration {
                    Ok(SensorReading::Proximity(cal.map(distance)))
                } else {
                    Ok(reading)
                }
            }
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}

impl<I: I2c> Probeable for Vl53l0x<I> {
    type Error = PeripheralError;

    #[tracing::instrument(level = "trace")]
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 1];
        if self
            .i2c
            .write_read(self.address, &[Register::IDENTIFICATION_MODEL_ID], &mut buf)
            .await
            .is_err()
        {
            // If the probe failed at the current address, the sensor might still be at its default
            // boot address of 0x29. Attempt to probe it there.
            self.i2c
                .write_read(0x29, &[Register::IDENTIFICATION_MODEL_ID], &mut buf)
                .await
                .map_err(|e| e.to_i2c_error(0x29, Register::IDENTIFICATION_MODEL_ID as u16))?;
        }
        let id = buf[0] as u16;
        if id == 0xEE {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn reset(&mut self) -> Result<(), Self::Error> {
        // No software-initiated reset register on the VL53L0X.
        // It relies on the hardware XSHUT pin for reset, so this is a no-op.
        Ok(())
    }
}
