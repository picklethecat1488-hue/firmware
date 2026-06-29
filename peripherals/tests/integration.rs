use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use peripherals::pump::{GpioPump, Pump};
use peripherals::water_sensor::{GpioWaterSensor, WaterSensor};

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
fn test_gpio_pump_compilation() {
    let pin = MockPin { is_high: false };
    let mut pump = GpioPump::new(pin);
    assert!(pump.stop().is_ok());
    assert!(pump.set_speed(100).is_ok());
}

#[test]
fn test_gpio_water_sensor_compilation() {
    let pin = MockPin { is_high: true };
    let mut sensor = GpioWaterSensor::new(pin);
    assert!(sensor.is_water_detected().unwrap());
}
