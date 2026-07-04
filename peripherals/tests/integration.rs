use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripherals::motor::{GpioMotor, Motor};

struct MockPin {
    is_high: bool,
}

impl ErrorType for MockPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_high = false;
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_high = true;
        Ok(())
    }
}

impl InputPin for MockPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_high)
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high)
    }
}

#[test]
fn test_gpio_motor_compilation() {
    let pin = MockPin { is_high: false };
    let mut motor = GpioMotor::new(pin);
    assert!(motor.stop().is_ok());
    assert!(motor.set_speed(100).is_ok());
}

#[test]
fn test_l9110s_compilation() {
    let pin_ia = MockPin { is_high: false };
    let pin_ib = MockPin { is_high: false };
    let mut motor = peripherals::l9110s::L9110s::new(pin_ia, pin_ib);
    assert!(motor.stop().is_ok());
    assert!(motor.set_speed(100).is_ok());
}

struct DummyI2c;

impl embedded_hal::i2c::ErrorType for DummyI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal::i2c::I2c for DummyI2c {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[test]
fn test_vl53l0x_threshold_validation() {
    use model::calibration::{Calibration, CalibrationType};
    use peripherals::vl53l0x::Vl53l0x;

    let mut sensor = Vl53l0x::new(DummyI2c, 0x30);
    // Default threshold is 300, cal_near is 0.

    // 1. Setting threshold to > cal_near + THRESHOLD_ERROR_MM should succeed.
    assert!(sensor.set_threshold_mm(250).is_ok());

    // 2. Setting threshold to <= cal_near + THRESHOLD_ERROR_MM should return an error.
    let mut s = Vl53l0x::new(DummyI2c, 0x30);
    assert!(s.set_threshold_mm(10).is_err());

    // 3. Setting calibration with threshold_mm > near + THRESHOLD_ERROR_MM should succeed.
    sensor.set_calibration(CalibrationType::ProximityCal(50, 150));

    // 4. Setting calibration with threshold_mm <= near + THRESHOLD_ERROR_MM should panic.
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut s = Vl53l0x::new(DummyI2c, 0x30);
        let _ = s.set_threshold_mm(100);
        s.set_calibration(CalibrationType::ProximityCal(90, 150));
    }));
    assert!(res.is_err());
}
