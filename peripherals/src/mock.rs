use model::interfaces::{
    ChargeStatus, FuelGauge, LedDriver, Motor, PowerMeasurementMode, PowerSensor, ProximitySensor,
    TemperatureSensor,
};

/// A mock implementation of a Motor for unit testing on the host.
#[derive(Default)]
pub struct MockMotor {
    /// Currently configured speed of the mock motor.
    pub speed: u8,
    /// Indicates if the mock motor is currently running.
    pub is_running: bool,
}

impl MockMotor {
    /// Creates a new inactive mock motor.
    pub const fn new() -> Self {
        Self {
            speed: 0,
            is_running: false,
        }
    }
}

impl Motor for MockMotor {
    type Error = ();

    /// Sets mock speed and updates run status.
    fn set_speed(&mut self, speed: u8) -> Result<(), Self::Error> {
        self.speed = speed;
        self.is_running = speed > 0;
        Ok(())
    }

    /// Stops the mock motor.
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.is_running = false;
        Ok(())
    }
}

/// A mock implementation of a CurrentSensor for unit testing on the host.
pub struct MockCurrentSensor {
    /// Current draw in mA.
    pub current_ma: i32,
}

impl MockCurrentSensor {
    /// Creates a new mock current sensor.
    pub const fn new(current_ma: i32) -> Self {
        Self { current_ma }
    }
}

impl PowerSensor for MockCurrentSensor {
    type Error = ();

    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        Ok(self.current_ma)
    }

    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        Ok(3700)
    }

    fn set_measurement_mode(&mut self, _mode: PowerMeasurementMode) -> Result<(), Self::Error> {
        Ok(())
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
}

impl MockBattery {
    /// Creates a new mock battery with specified defaults.
    pub const fn new(voltage_mv: u32, temperature_milli_c: i32) -> Self {
        Self {
            voltage_mv,
            temperature_milli_c,
            state_of_charge: 50,
        }
    }
}

impl TemperatureSensor for MockBattery {
    type Error = ();

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(self.temperature_milli_c)
    }
}

impl FuelGauge for MockBattery {
    type Error = ();

    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        Ok(self.voltage_mv)
    }

    fn read_state_of_charge(&mut self) -> Result<u8, Self::Error> {
        Ok(self.state_of_charge)
    }
}

/// A simulated current sensor that always returns a healthy current draw.
pub struct DummyCurrentSensor;

impl PowerSensor for DummyCurrentSensor {
    type Error = core::convert::Infallible;

    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        Ok(150) // Simulate a healthy current draw of 150mA
    }

    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error> {
        Ok(3700)
    }

    fn set_measurement_mode(&mut self, _mode: PowerMeasurementMode) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A mock implementation of a ProximitySensor for unit testing.
pub struct MockProximitySensor {
    /// Simulated distance value in millimeters.
    pub distance_mm: u16,
    /// Proximity threshold in millimeters.
    pub threshold_mm: u16,
}

impl MockProximitySensor {
    /// Creates a new mock proximity sensor with specified initial distance.
    pub const fn new(distance_mm: u16) -> Self {
        Self {
            distance_mm,
            threshold_mm: 300,
        }
    }
}

impl ProximitySensor for MockProximitySensor {
    type Error = ();

    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        Ok(self.distance_mm)
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

impl ProximitySensor for DummyProximitySensor {
    type Error = core::convert::Infallible;

    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        Ok(self.distance_mm)
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

    fn get_charge_state(&mut self) -> Result<model::types::ChargeState, Self::Error> {
        Ok(self.state)
    }
}

/// A mock implementation of an LedDriver for unit testing.
pub struct MockLed {
    /// Currently set RGB color.
    pub color: (u8, u8, u8),
}

impl Default for MockLed {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLed {
    /// Creates a new mock LED driver.
    pub const fn new() -> Self {
        Self { color: (0, 0, 0) }
    }
}

impl LedDriver for MockLed {
    type Error = ();

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        self.color = (r, g, b);
        Ok(())
    }
}
