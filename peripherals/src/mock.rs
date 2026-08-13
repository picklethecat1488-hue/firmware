use crate::tracing;
use model::interfaces::{
    ChargeStatus, FuelGauge, LedDriver, Motor, PowerMeasurementMode, PowerSensor, Probeable,
    ProximitySensor, TemperatureSensor, Tickable, WaitableMeasurement,
};
use model::types::{MotorSpeed, PeripheralError, SensorReading};

/// A mock implementation of a Motor for unit testing on the host.
pub struct MockMotor {
    /// Currently configured speed of the mock motor.
    pub speed: i8,
    /// Indicates if the mock motor is currently running.
    pub is_running: bool,
    /// Whether motor driver commands should fail.
    pub should_fail: bool,
}

impl Default for MockMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMotor {
    /// Creates a new inactive mock motor.
    pub const fn new() -> Self {
        Self {
            speed: 0,
            is_running: false,
            should_fail: false,
        }
    }
}

impl Motor for MockMotor {
    type Error = ();

    /// Sets mock speed and updates run status.
    #[tracing::instrument(core1 = "core1", level = "trace", skip(speed))]
    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            let speed_raw = speed.get();
            self.speed = speed_raw;
            self.is_running = speed_raw != 0;
            Ok(())
        }
    }

    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn stop(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            self.speed = 0;
            self.is_running = false;
            Ok(())
        }
    }
}

/// A mock implementation of a CurrentSensor for unit testing on the host.
pub struct MockCurrentSensor {
    /// Current draw in mA.
    pub current_ma: i32,
    /// Whether power sensor commands should fail.
    pub should_fail: bool,
}

impl MockCurrentSensor {
    /// Creates a new mock current sensor.
    pub const fn new(current_ma: i32) -> Self {
        Self {
            current_ma,
            should_fail: false,
        }
    }
}

impl WaitableMeasurement for MockCurrentSensor {
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        if self.should_fail {
            Err(PeripheralError::DeviceNotAvailable)
        } else {
            Ok(())
        }
    }
}

impl PowerSensor for MockCurrentSensor {
    type Error = ();

    async fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(self.current_ma)
        }
    }

    async fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(3700)
        }
    }

    async fn set_measurement_mode(
        &mut self,
        _mode: PowerMeasurementMode,
    ) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

impl Probeable for MockCurrentSensor {
    type Error = ();
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(0x399F)
        }
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

/// A mock implementation of a Battery for unit testing on the host or bare-metal tasks.
pub struct MockBattery {
    /// Simulated battery voltage in millivolts.
    pub voltage_mv: u32,
    /// Simulated temperature in milli-degrees Celsius.
    pub temperature_milli_c: i32,
    /// Simulated state of charge in percent.
    pub state_of_charge: u8,
    /// Whether battery commands/reads should fail.
    pub should_fail: bool,
}

impl MockBattery {
    /// Creates a new mock battery with specified defaults.
    pub const fn new(voltage_mv: u32, temperature_milli_c: i32) -> Self {
        Self {
            voltage_mv,
            temperature_milli_c,
            state_of_charge: 50,
            should_fail: false,
        }
    }
}

impl TemperatureSensor for MockBattery {
    type Error = ();

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(self.temperature_milli_c)
        }
    }
}

impl Tickable for MockBattery {
    type Error = ();

    async fn tick(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

impl FuelGauge for MockBattery {
    type Error = ();

    async fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(self.voltage_mv)
        }
    }

    async fn read_state_of_charge(&mut self) -> Result<u8, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(self.state_of_charge)
        }
    }

    async fn configure_alerts(
        &mut self,
        _voltage_min_mv: u32,
        _voltage_max_mv: u32,
        _soc_threshold_pct: u8,
        _enable_soc_change_alert: bool,
    ) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }

    async fn check_and_clear_alerts(&mut self) -> Result<(bool, bool), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok((false, false))
        }
    }
}

impl Probeable for MockBattery {
    type Error = ();
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(0x0010)
        }
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

/// A simulated current sensor that always returns a healthy current draw.
pub struct DummyCurrentSensor;

impl WaitableMeasurement for DummyCurrentSensor {
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        Ok(())
    }
}

impl PowerSensor for DummyCurrentSensor {
    type Error = core::convert::Infallible;

    async fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        Ok(150) // Simulate a healthy current draw of 150mA
    }

    async fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        Ok(3700)
    }

    async fn set_measurement_mode(
        &mut self,
        _mode: PowerMeasurementMode,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Probeable for DummyCurrentSensor {
    type Error = core::convert::Infallible;
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        Ok(0x399F)
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A mock implementation of a ProximitySensor for unit testing.
pub struct MockProximitySensor {
    /// Simulated distance value in millimeters.
    pub distance_mm: u16,
    /// Proximity threshold in millimeters.
    pub threshold_mm: u16,
    /// Whether distance reading should fail.
    pub should_fail: bool,
}

impl MockProximitySensor {
    /// Creates a new mock proximity sensor with specified initial distance.
    pub const fn new(distance_mm: u16) -> Self {
        Self {
            distance_mm,
            threshold_mm: 300,
            should_fail: false,
        }
    }
}

impl model::calibration::Calibration for MockProximitySensor {
    const CALIBRATION_FILE_NAME: &'static str = "dummy_cal.cbor";
    type Store = model::calibration::Vl53l0xCalibration;
}

impl model::calibration::ApplyCalibration for MockProximitySensor {
    type Input = SensorReading;
    type Output = SensorReading;
    type Error = &'static str;

    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        match reading {
            SensorReading::Proximity(_) => Ok(reading),
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}

impl WaitableMeasurement for MockProximitySensor {
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        if self.should_fail {
            Err(PeripheralError::DeviceNotAvailable)
        } else {
            Ok(())
        }
    }
}

impl ProximitySensor for MockProximitySensor {
    type Error = ();

    async fn read_distance_mm(&mut self) -> Result<SensorReading, Self::Error> {
        if self.should_fail {
            Err(())
        } else if self.distance_mm == 0 || self.distance_mm == 8190 {
            Ok(SensorReading::Invalid)
        } else {
            Ok(SensorReading::Proximity(self.distance_mm))
        }
    }

    async fn read_raw_distance_and_rate(&mut self) -> Result<(SensorReading, u16), Self::Error> {
        let reading = self.read_distance_mm().await?;
        Ok((reading, 2500)) // Return a dummy peak rate of 2500 (19.5 Mcps)
    }
}

impl Probeable for MockProximitySensor {
    type Error = ();
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(0xEE)
        }
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

/// A simulated proximity sensor that returns a default distance.
pub struct DummyProximitySensor {
    /// Distance in millimeters.
    pub distance_mm: u16,
    /// Proximity threshold in millimeters.
    pub threshold_mm: u16,
}

impl DummyProximitySensor {
    /// Creates a new dummy proximity sensor.
    pub const fn new(distance_mm: u16) -> Self {
        Self {
            distance_mm,
            threshold_mm: 300,
        }
    }
}

impl WaitableMeasurement for DummyProximitySensor {
    async fn wait_for_measurement(&mut self) -> Result<(), PeripheralError> {
        Ok(())
    }
}

impl ProximitySensor for DummyProximitySensor {
    type Error = core::convert::Infallible;

    async fn read_distance_mm(&mut self) -> Result<SensorReading, Self::Error> {
        if self.distance_mm == 0 || self.distance_mm == 8190 {
            Ok(SensorReading::Invalid)
        } else {
            Ok(SensorReading::Proximity(self.distance_mm))
        }
    }

    async fn read_raw_distance_and_rate(&mut self) -> Result<(SensorReading, u16), Self::Error> {
        let reading = self.read_distance_mm().await?;
        Ok((reading, 2500)) // Return a dummy peak rate of 2500 (19.5 Mcps)
    }
}

impl Probeable for DummyProximitySensor {
    type Error = core::convert::Infallible;
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        Ok(0xEE)
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl model::calibration::Calibration for DummyProximitySensor {
    const CALIBRATION_FILE_NAME: &'static str = "dummy_cal.cbor";
    type Store = model::calibration::Vl53l0xCalibration;
}

impl model::calibration::ApplyCalibration for DummyProximitySensor {
    type Input = SensorReading;
    type Output = SensorReading;
    type Error = &'static str;

    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        match reading {
            SensorReading::Proximity(_) => Ok(reading),
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}

/// A mock implementation of a ChargeStatus for unit testing.
pub struct MockCharger {
    /// The mock charger state.
    pub state: model::types::ChargeState,
}

impl MockCharger {
    /// Creates a new MockCharger instance.
    pub const fn new(state: model::types::ChargeState) -> Self {
        Self { state }
    }
}

impl ChargeStatus for MockCharger {
    type Error = ();

    async fn get_charge_state(&mut self) -> Result<model::types::ChargeState, Self::Error> {
        Ok(self.state)
    }
}

impl Probeable for MockCharger {
    type Error = ();
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        Ok(0x0010)
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A mock implementation of an LedDriver for unit testing.
pub struct MockLed {
    /// Currently set RGB color.
    pub color: (u8, u8, u8),
    /// Whether LED driver commands should fail.
    pub should_fail: bool,
}

impl Default for MockLed {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLed {
    /// Creates a new mock LED driver.
    pub const fn new() -> Self {
        Self {
            color: (0, 0, 0),
            should_fail: false,
        }
    }
}

impl LedDriver for MockLed {
    type Error = ();

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            self.color = (r, g, b);
            Ok(())
        }
    }
}

impl Probeable for MockLed {
    type Error = ();
    async fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(0x86)
        }
    }
    async fn reset(&mut self) -> Result<(), Self::Error> {
        if self.should_fail {
            Err(())
        } else {
            Ok(())
        }
    }
}

/// A dummy/no-op I2C driver.
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct DummyI2c;

impl embedded_hal_async::i2c::ErrorType for DummyI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::i2c::I2c for DummyI2c {
    async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op motor driver.
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct DummyMotor;

impl Motor for DummyMotor {
    type Error = core::convert::Infallible;
    fn set_speed(&mut self, _speed: MotorSpeed) -> Result<(), Self::Error> {
        Ok(())
    }
    fn stop(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op flash driver.
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct DummyFlash;

impl embedded_storage::nor_flash::ErrorType for DummyFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage::nor_flash::ReadNorFlash for DummyFlash {
    const READ_SIZE: usize = 1;
    fn read(&mut self, _offset: u32, _bytes: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn capacity(&self) -> usize {
        0
    }
}

impl embedded_storage::nor_flash::NorFlash for DummyFlash {
    const WRITE_SIZE: usize = 1;
    const ERASE_SIZE: usize = 4096;
    fn write(&mut self, _offset: u32, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn erase(&mut self, _from: u32, _to: u32) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op temperature sensor.
#[derive(Clone, Copy, Default)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct DummyTempSensor;

impl TemperatureSensor for DummyTempSensor {
    type Error = core::convert::Infallible;
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(0)
    }
}
