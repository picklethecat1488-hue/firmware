use model::interfaces::{
    Charger, CurrentSensor, FuelGauge, Motor, ProximitySensor, TemperatureSensor,
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

impl CurrentSensor for MockCurrentSensor {
    type Error = ();

    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        Ok(self.current_ma)
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
        Ok(50) // Default 50% state of charge
    }
}

/// A simulated current sensor that always returns a healthy current draw.
pub struct DummyCurrentSensor;

impl CurrentSensor for DummyCurrentSensor {
    type Error = core::convert::Infallible;

    fn read_current_ma(&mut self) -> Result<i32, Self::Error> {
        Ok(150) // Simulate a healthy current draw of 150mA
    }
}

/// A mock implementation of a ProximitySensor for unit testing.
pub struct MockProximitySensor {
    /// Simulated distance value in millimeters.
    pub distance_mm: u16,
    /// Proximity callback function.
    pub callback: Option<fn(bool)>,
}

impl MockProximitySensor {
    /// Creates a new mock proximity sensor with specified initial distance.
    pub const fn new(distance_mm: u16) -> Self {
        Self {
            distance_mm,
            callback: None,
        }
    }
}

impl ProximitySensor for MockProximitySensor {
    type Error = ();

    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        if let Some(cb) = self.callback {
            cb(self.distance_mm < 300);
        }
        Ok(self.distance_mm)
    }

    fn register_proximity_callback(&mut self, callback: fn(bool)) -> Result<(), Self::Error> {
        self.callback = Some(callback);
        Ok(())
    }
}

/// A simulated proximity sensor that returns a default distance.
pub struct DummyProximitySensor {
    /// Distance in millimeters.
    pub distance_mm: u16,
    /// Proximity callback function.
    pub callback: Option<fn(bool)>,
}

impl DummyProximitySensor {
    /// Creates a new dummy proximity sensor.
    pub const fn new(distance_mm: u16) -> Self {
        Self {
            distance_mm,
            callback: None,
        }
    }
}

impl ProximitySensor for DummyProximitySensor {
    type Error = core::convert::Infallible;

    fn read_distance_mm(&mut self) -> Result<u16, Self::Error> {
        if let Some(cb) = self.callback {
            cb(self.distance_mm < 300);
        }
        Ok(self.distance_mm)
    }

    fn register_proximity_callback(&mut self, callback: fn(bool)) -> Result<(), Self::Error> {
        self.callback = Some(callback);
        Ok(())
    }
}

/// A mock implementation of a Charger for unit testing.
pub struct MockCharger {
    /// Tracks if charging is enabled.
    pub charging_enabled: bool,
    /// Tracks if a charging input is present.
    pub input_present: bool,
}

impl MockCharger {
    /// Creates a new MockCharger instance.
    pub const fn new(charging_enabled: bool, input_present: bool) -> Self {
        Self {
            charging_enabled,
            input_present,
        }
    }
}

impl Charger for MockCharger {
    type Error = ();

    fn set_charging_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
        self.charging_enabled = enabled;
        Ok(())
    }

    fn is_charging_input_present(&mut self) -> Result<bool, Self::Error> {
        Ok(self.input_present)
    }
}
