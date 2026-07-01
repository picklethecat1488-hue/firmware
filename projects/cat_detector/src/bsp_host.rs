//! Host Board Support Package (BSP) mock.
//!
//! Provides mock peripheral drivers, inputs, and outputs to compile
//! and validate logic on host systems.

#![cfg(not(all(target_arch = "arm", target_os = "none")))]
#![deny(missing_docs)]

/// Mock pin implementation for host.
#[derive(Default)]
pub struct MockFlex {
    /// Current mock state of the pin (High/Low)
    pub value: bool,
}

impl MockFlex {
    /// Create a new MockFlex pin.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set mock pin state to high.
    pub fn set_high(&mut self) {
        self.value = true;
    }

    /// Set mock pin state to low.
    pub fn set_low(&mut self) {
        self.value = false;
    }

    /// Checks if mock pin state is high.
    pub fn is_high(&self) -> bool {
        self.value
    }
}

impl embedded_hal::digital::ErrorType for MockFlex {
    type Error = core::convert::Infallible;
}

impl embedded_hal::digital::OutputPin for MockFlex {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }
}

/// Mock Board structure for host testing.
pub struct Board {
    /// Lookup array containing MockFlex instances for dynamic GPIO diagnostics
    pub gpio_pins: [Option<MockFlex>; 30],
    /// Mock temperature sensor
    pub temp_sensor: Option<Rp2040TempSensor>,
}

impl Board {
    /// Initialize mock board.
    pub fn init() -> Self {
        let mut gpio_pins: [Option<MockFlex>; 30] = Default::default();
        for item in gpio_pins.iter_mut() {
            *item = Some(MockFlex::new());
        }
        // Mock asserting XSHUT (active low) on ToF sensors (GP2, GP3, GP6)
        if let Some(ref mut pin) = gpio_pins[2] {
            pin.set_low();
        }
        if let Some(ref mut pin) = gpio_pins[3] {
            pin.set_low();
        }
        if let Some(ref mut pin) = gpio_pins[6] {
            pin.set_low();
        }
        let temp_sensor = Some(Rp2040TempSensor);
        Self {
            gpio_pins,
            temp_sensor,
        }
    }
}

/// Mock temperature sensor for host.
pub struct Rp2040TempSensor;

impl model::interfaces::TemperatureSensor for Rp2040TempSensor {
    type Error = core::convert::Infallible;

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(25000)
    }
}
