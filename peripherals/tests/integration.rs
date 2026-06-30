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
