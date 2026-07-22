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
    /// Mock charger driver instance
    pub charger: Option<peripherals::mock::MockCharger>,
    /// Mock battery controller
    pub battery: peripherals::mock::MockBattery,
    /// Mock motor
    pub motor: peripherals::mock::MockMotor,
    /// Mock current sensor
    pub current_sensor: peripherals::mock::DummyCurrentSensor,
    /// Mock North proximity sensor
    pub tof_north: peripherals::mock::DummyProximitySensor,
    /// Mock East proximity sensor
    pub tof_east: peripherals::mock::DummyProximitySensor,
    /// Mock West proximity sensor
    pub tof_west: peripherals::mock::DummyProximitySensor,
    /// Mock LED driver
    pub led_driver: peripherals::mock::MockLed,
    /// Mock fuel gauge alert pin
    pub fuel_gauge_alert_pin: MockFlex,
    /// Mock North proximity interrupt pin
    pub pin_north: MockFlex,
    /// Mock East proximity interrupt pin
    pub pin_east: MockFlex,
    /// Mock West proximity interrupt pin
    pub pin_west: MockFlex,
    /// Core 0 executor spawner
    pub spawner: Option<embassy_executor::Spawner>,
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
        let charger = Some(peripherals::mock::MockCharger::new(
            model::types::ChargeState::DoneOrStandbyOrUnplugged,
        ));

        let battery = peripherals::mock::MockBattery::new(3700, 25000);
        let motor = peripherals::mock::MockMotor::new();
        let current_sensor = peripherals::mock::DummyCurrentSensor;

        let tof_north = peripherals::mock::DummyProximitySensor::new(100);
        let tof_east = peripherals::mock::DummyProximitySensor::new(150);
        let tof_west = peripherals::mock::DummyProximitySensor::new(200);

        let led_driver = peripherals::mock::MockLed::new();

        let fuel_gauge_alert_pin = MockFlex::new();
        let pin_north = MockFlex::new();
        let pin_east = MockFlex::new();
        let pin_west = MockFlex::new();

        Self {
            gpio_pins,
            temp_sensor,
            charger,
            battery,
            motor,
            current_sensor,
            tof_north,
            tof_east,
            tof_west,
            led_driver,
            fuel_gauge_alert_pin,
            pin_north,
            pin_east,
            pin_west,
            spawner: None,
        }
    }

    /// Run the executor loop for the specified core (dummy on host).
    ///
    /// # Safety
    ///
    /// This is a dummy method on host and is always safe to call.
    pub unsafe fn run_executor(_cpu_id: platform::types::CpuId) -> ! {
        loop {
            std::thread::yield_now();
        }
    }

    /// Mock initialization of the Embassy executor for Core 1.
    ///
    /// # Safety
    ///
    /// This is a mock function on host and is always safe to call.
    pub unsafe fn init_executor_core1() {}

    /// Mock spawner for Core 1.
    ///
    /// # Safety
    ///
    /// This is a mock function on host and is always safe to call.
    pub unsafe fn spawner_core1() -> embassy_executor::Spawner {
        panic!("Core 1 spawner should not be called on host");
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

/// Mock boot reason for host.
pub fn get_boot_reason() -> model::types::BootReason {
    model::types::BootReason::Unknown
}

impl controller::battery_controller::BatteryAlertPin for MockFlex {
    async fn wait_for_alert(&mut self) {
        embassy_time::Timer::after_secs(3600 * 24).await;
    }
}

impl controller::sensor_controller::DataReadyPin for MockFlex {
    async fn wait_for_data_ready(&mut self) {
        embassy_time::Timer::after_secs(3600 * 24).await;
    }
}

/// The battery fuel gauge type.
pub type BatteryDevice = peripherals::mock::MockBattery;
/// The battery charger type.
pub type ChargerDevice = peripherals::mock::MockCharger;
/// The battery alert pin type.
pub type AlertPinType = MockFlex;
/// The motor driver type.
pub type MotorDevice = peripherals::mock::MockMotor;
/// The motor current sensor type.
pub type CurrentSensorDevice = peripherals::mock::DummyCurrentSensor;
/// The proximity sensor type.
pub type ProximitySensorDevice = peripherals::mock::DummyProximitySensor;
/// The proximity sensor interrupt pin type.
pub type DataReadyPinType = MockFlex;
/// The LED driver type.
pub type LedDevice = peripherals::mock::MockLed;
/// The temperature sensor type.
pub type TempSensorDevice = Rp2040TempSensor;
