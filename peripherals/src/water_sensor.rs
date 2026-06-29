use embedded_hal::digital::InputPin;

/// Interface for checking a water presence sensor.
pub trait WaterSensor {
    /// Error type returned by the physical hardware.
    type Error;

    /// Checks if water is currently detected by the sensor.
    fn is_water_detected(&mut self) -> Result<bool, Self::Error>;
}

/// A generic platform-agnostic GPIO implementation of a WaterSensor.
pub struct GpioWaterSensor<P: InputPin> {
    pin: P,
}

impl<P: InputPin> GpioWaterSensor<P> {
    /// Creates a new generic GPIO water sensor.
    pub const fn new(pin: P) -> Self {
        Self { pin }
    }
}

impl<P: InputPin> WaterSensor for GpioWaterSensor<P> {
    type Error = P::Error;

    /// Checks if water is detected by reading if the GPIO pin is High.
    fn is_water_detected(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}
