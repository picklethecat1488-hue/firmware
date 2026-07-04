use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripherals::motor::{GpioMotor, Motor};

struct MockPin<'a> {
    is_high: &'a core::cell::Cell<bool>,
}

impl<'a> ErrorType for MockPin<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for MockPin<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_high.set(false);
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_high.set(true);
        Ok(())
    }
}

impl<'a> InputPin for MockPin<'a> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_high.get())
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high.get())
    }
}

#[test]
fn test_gpio_motor_functional() {
    let pin_state = core::cell::Cell::new(false);
    let pin = MockPin {
        is_high: &pin_state,
    };
    let mut motor = GpioMotor::new(pin);

    // 1. Initially low
    assert!(!pin_state.get());

    // 2. Setting speed > 0 drives pin high
    assert!(motor.set_speed(100).is_ok());
    assert!(pin_state.get());

    // 3. Setting speed == 0 drives pin low
    assert!(motor.set_speed(0).is_ok());
    assert!(!pin_state.get());

    // 4. Stopping drives pin low
    assert!(motor.set_speed(50).is_ok());
    assert!(pin_state.get());
    assert!(motor.stop().is_ok());
    assert!(!pin_state.get());
}

#[test]
fn test_l9110s_functional() {
    let pin_ia_state = core::cell::Cell::new(false);
    let pin_ib_state = core::cell::Cell::new(false);
    let pin_ia = MockPin {
        is_high: &pin_ia_state,
    };
    let pin_ib = MockPin {
        is_high: &pin_ib_state,
    };
    let mut motor = peripherals::l9110s::L9110s::new(pin_ia, pin_ib);

    // 1. Initially both low
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 2. Setting speed > 0 drives pin_ia high and pin_ib low
    assert!(motor.set_speed(100).is_ok());
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 3. Setting speed == 0 brakes both pins to low
    assert!(motor.set_speed(0).is_ok());
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());

    // 4. Stopping brakes both pins to low
    assert!(motor.set_speed(50).is_ok());
    assert!(pin_ia_state.get());
    assert!(!pin_ib_state.get());
    assert!(motor.stop().is_ok());
    assert!(!pin_ia_state.get());
    assert!(!pin_ib_state.get());
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
