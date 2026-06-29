use crate::pump::Pump;
use crate::water_sensor::WaterSensor;

/// A mock implementation of a Pump for unit testing on the host.
#[derive(Default)]
pub struct MockPump {
    /// Currently configured speed of the mock pump.
    pub speed: u8,
    /// Indicates if the mock pump is currently running.
    pub is_running: bool,
}

impl MockPump {
    /// Creates a new inactive mock pump.
    pub const fn new() -> Self {
        Self {
            speed: 0,
            is_running: false,
        }
    }
}

impl Pump for MockPump {
    type Error = ();

    /// Sets mock speed and updates run status.
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error> {
        self.speed = speed;
        self.is_running = speed > 0;
        Ok(())
    }

    /// Stops the mock pump.
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.is_running = false;
        Ok(())
    }
}

/// A mock implementation of a WaterSensor for unit testing on the host.
pub struct MockWaterSensor {
    /// Tracks if water presence is simulated.
    pub water_present: bool,
}

impl MockWaterSensor {
    /// Creates a new mock water sensor with specified initial presence.
    pub const fn new(water_present: bool) -> Self {
        Self { water_present }
    }
}

impl WaterSensor for MockWaterSensor {
    type Error = ();

    /// Returns the simulated water presence value.
    fn is_water_detected(&mut self) -> Result<bool, Self::Error> {
        Ok(self.water_present)
    }
}

/// A mock implementation of a Battery for unit testing on the host or bare-metal tasks.
pub struct MockBattery {
    /// Simulated battery voltage in millivolts.
    pub voltage_mv: u32,
    /// Simulated temperature in milli-degrees Celsius.
    pub temperature_milli_c: i32,
}

impl MockBattery {
    /// Creates a new mock battery with specified defaults.
    pub const fn new(voltage_mv: u32, temperature_milli_c: i32) -> Self {
        Self {
            voltage_mv,
            temperature_milli_c,
        }
    }
}

impl crate::battery::Battery for MockBattery {
    type Error = ();

    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        Ok(self.voltage_mv)
    }

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(self.temperature_milli_c)
    }
}

/// A mock input pin that implements embedded-hal digital input traits.
pub struct DummyInputPin;

impl embedded_hal::digital::ErrorType for DummyInputPin {
    type Error = core::convert::Infallible;
}

impl embedded_hal::digital::InputPin for DummyInputPin {
    /// Always returns true to simulate active input.
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    /// Always returns false.
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

/// A simulated water sensor wrapping a dummy input pin.
pub struct DummyWaterSensor {
    /// The nested dummy input pin.
    pub pin: DummyInputPin,
}

impl WaterSensor for DummyWaterSensor {
    type Error = core::convert::Infallible;

    /// Checks for water by querying the simulated input pin.
    fn is_water_detected(&mut self) -> Result<bool, Self::Error> {
        use embedded_hal::digital::InputPin;
        self.pin.is_high()
    }
}
