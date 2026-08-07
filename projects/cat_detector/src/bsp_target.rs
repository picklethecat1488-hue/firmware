//! Target Board Support Package (BSP) for Raspberry Pi Pico.
//!
//! Provides hardware-specific peripheral wrappers, pin initialization,
//! and lookup mappings for bare-metal deployment.

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]

use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Flex, Pin, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::pio::InterruptHandler;
use embassy_rp::Peripherals;
use platform::tracing;
use platform::types::QueueFilesystem;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<embassy_rp::peripherals::PIO0>;
});

/// Helper structure containing all pre-initialized board interfaces.
pub struct Board<'d> {
    /// Blocking I2C0 instance for sensor communications
    pub i2c: I2c<'d, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    /// The onboard flash peripheral
    pub flash: embassy_rp::peripherals::FLASH,
    /// Lookup array containing Flex instances for dynamic GPIO diagnostics
    pub gpio_pins: [Option<Flex<'d>>; 30],
    /// Internal RP2040 temperature sensor
    pub temp_sensor: Option<Rp2040TempSensor>,

    /// Motor driver
    pub motor: peripherals::l9110s::L9110s<Flex<'d>, Flex<'d>>,
    /// Motor current sensor
    pub current_sensor: peripherals::ina219::Ina219<platform::i2c::SharedI2cWrapper<'static>>,
    /// North proximity sensor
    pub tof_north: peripherals::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
    /// East proximity sensor
    pub tof_east: peripherals::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
    /// West proximity sensor
    pub tof_west: peripherals::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
    /// Status LED driver
    pub led_driver: peripherals::ws2812::Ws2812<'d, embassy_rp::peripherals::PIO0, 0>,
    /// Fuel gauge alert/interrupt pin
    pub fuel_gauge_alert_pin: Flex<'d>,
    /// North proximity interrupt pin
    pub pin_north: Flex<'d>,
    /// East proximity interrupt pin
    pub pin_east: Flex<'d>,
    /// West proximity interrupt pin
    pub pin_west: Flex<'d>,
    /// Core 0 executor spawner
    pub spawner: Option<embassy_executor::Spawner>,
}

impl<'d> Board<'d> {
    /// Initialize all hardware components and return the Board interface.
    ///
    /// # Arguments
    /// * `p` - The RP2040 peripheral set.
    #[tracing::instrument(level = "trace", skip(p))]
    pub fn init(p: Peripherals) -> Self {
        // Configure hardware stack guard using Cortex-M MPU
        platform::core_monitor::configure_mpu_stack_guard(crate::CORE0_STACK_BOTTOM);

        // 1. Perform I2C bus unstuck on I2C0 (GP12 SDA, GP13 SCL) using the platform support recovery tool.
        unsafe {
            use platform::rp2040::PlatformI2cRecovery as _;
            let recovery = platform::rp2040::Rp2040I2cRecovery {
                sda_pin: 12,
                scl_pin: 13,
            };
            let _ = recovery.recover_i2c_bus();
        }

        let mut i2c_config = I2cConfig::default();
        i2c_config.frequency = 400_000;
        let mut i2c = I2c::new_blocking(p.I2C0, p.PIN_13, p.PIN_12, i2c_config);
        let mut gpio_pins: [Option<Flex<'d>>; 30] = [
            None, // 0 - UART TX
            None, // 1 - UART RX
            Some(Flex::new(p.PIN_2.degrade())),
            Some(Flex::new(p.PIN_3.degrade())),
            Some(Flex::new(p.PIN_4.degrade())),
            Some(Flex::new(p.PIN_5.degrade())),
            Some(Flex::new(p.PIN_6.degrade())),
            Some(Flex::new(p.PIN_7.degrade())),
            Some(Flex::new(p.PIN_8.degrade())),
            Some(Flex::new(p.PIN_9.degrade())),
            Some(Flex::new(p.PIN_10.degrade())),
            None, // 11 - WS2812 LED (driven via PIO0)
            None, // 12 - I2C SDA
            None, // 13 - I2C SCL
            Some(Flex::new(p.PIN_14.degrade())),
            Some(Flex::new(p.PIN_15.degrade())),
            Some(Flex::new(p.PIN_16.degrade())),
            Some(Flex::new(p.PIN_17.degrade())),
            Some(Flex::new(p.PIN_18.degrade())),
            Some(Flex::new(p.PIN_19.degrade())),
            Some(Flex::new(p.PIN_20.degrade())),
            Some(Flex::new(p.PIN_21.degrade())),
            Some(Flex::new(p.PIN_22.degrade())),
            Some(Flex::new(p.PIN_23.degrade())),
            Some(Flex::new(p.PIN_24.degrade())),
            Some(Flex::new(p.PIN_25.degrade())), // Onboard LED / Pump pin
            Some(Flex::new(p.PIN_26.degrade())),
            Some(Flex::new(p.PIN_27.degrade())),
            Some(Flex::new(p.PIN_28.degrade())),
            Some(Flex::new(p.PIN_29.degrade())),
        ];

        // 2. Assert XSHUT (active low) on all ToF sensors (GP2, GP3, GP6)
        let xshut_pins = [
            crate::TOF_NORTH_XSHUT_PIN,
            crate::TOF_EAST_XSHUT_PIN,
            crate::TOF_WEST_XSHUT_PIN,
        ];
        for &pin_idx in &xshut_pins {
            if let Some(ref mut pin) = gpio_pins[pin_idx as usize] {
                pin.set_as_output();
                pin.set_low();
            }
        }

        // 3. Configure Fuel Gauge Alert pin (GP10) as input with pull-up (active-low, open-drain)
        if let Some(ref mut pin) = gpio_pins[crate::FUEL_GAUGE_INT_PIN as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Up);
        }

        // 4. Configure ToF Sensor Interrupt pins (GP7, GP8, GP9) as inputs with pull-ups (active-low, open-drain)
        let int_pins = [
            crate::TOF_NORTH_INT_PIN,
            crate::TOF_EAST_INT_PIN,
            crate::TOF_WEST_INT_PIN,
        ];
        for &pin_idx in &int_pins {
            if let Some(ref mut pin) = gpio_pins[pin_idx as usize] {
                pin.set_as_input();
                pin.set_pull(Pull::Up);
            }
        }

        // Wait for sensors to register reset state
        cortex_m::asm::delay(20_000);

        let temp_flash = unsafe { core::ptr::read(&p.FLASH) };
        let mut raw_flash: crate::FlashDevice = embassy_rp::flash::Flash::new_blocking(temp_flash);
        let mut boot_status = platform::flash::DirectFlashBootStatus::new(
            &mut raw_flash,
            QueueFilesystem(crate::TELEMETRY_PARTITION_START..crate::TELEMETRY_PARTITION_END),
        );

        let sensors = [
            (
                "North ToF",
                crate::TOF_NORTH_XSHUT_PIN,
                crate::TOF_NORTH_I2C_ADDR,
            ),
            (
                "East ToF",
                crate::TOF_EAST_XSHUT_PIN,
                crate::TOF_EAST_I2C_ADDR,
            ),
            (
                "West ToF",
                crate::TOF_WEST_XSHUT_PIN,
                crate::TOF_WEST_I2C_ADDR,
            ),
        ];
        for &(name, xshut_pin, addr) in &sensors {
            peripherals::init_vl53l0x!(
                &mut i2c,
                gpio_pins,
                name,
                xshut_pin,
                addr,
                crate::DEFAULT_WAKE_THRESHOLD_MM,
                &mut boot_status
            );
        }

        let temp_sensor = Some(Rp2040TempSensor::new(p.ADC, p.ADC_TEMP_SENSOR));

        // Configure remaining drivers using local i2c before returning
        peripherals::init_max17048!(&mut i2c, &mut boot_status);
        peripherals::init_ina219!(&mut i2c, &mut boot_status);

        // Extract pins needed for drivers/controllers
        let motor_pin_ia = gpio_pins[crate::PUMP_PIN_IA as usize]
            .take()
            .expect("Motor pin IA must be available");
        let motor_pin_ib = gpio_pins[crate::PUMP_PIN_IB as usize]
            .take()
            .expect("Motor pin IB must be available");
        let motor = peripherals::l9110s::L9110s::new(motor_pin_ia, motor_pin_ib);

        let fuel_gauge_alert_pin = gpio_pins[crate::FUEL_GAUGE_INT_PIN as usize]
            .take()
            .expect("Fuel gauge alert pin must be available");
        let pin_north = gpio_pins[crate::TOF_NORTH_INT_PIN as usize]
            .take()
            .expect("North ToF interrupt pin must be available");
        let pin_east = gpio_pins[crate::TOF_EAST_INT_PIN as usize]
            .take()
            .expect("East ToF interrupt pin must be available");
        let pin_west = gpio_pins[crate::TOF_WEST_INT_PIN as usize]
            .take()
            .expect("West ToF interrupt pin must be available");

        // Construct final drivers wrapping SHARED_I2C static cell
        let current_sensor = peripherals::ina219::Ina219::new(
            platform::i2c::SharedI2cWrapper::new(&crate::SHARED_I2C),
        );
        let make_tof = |addr| {
            let mut sensor = peripherals::vl53l0x::Vl53l0x::new(
                platform::i2c::SharedI2cWrapper::new(&crate::SHARED_I2C),
                addr,
            );
            let _ = sensor.set_threshold_mm(crate::DEFAULT_WAKE_THRESHOLD_MM);
            sensor
        };

        let tof_north = make_tof(crate::TOF_NORTH_I2C_ADDR);
        let tof_east = make_tof(crate::TOF_EAST_I2C_ADDR);
        let tof_west = make_tof(crate::TOF_WEST_I2C_ADDR);

        let led_driver = peripherals::init_ws2812!(p.PIO0, p.PIN_11, &mut boot_status);

        let spawner = unsafe {
            use platform::rp2040::PlatformMulticore as _;
            Some(platform::rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core0))
        };

        Self {
            i2c,
            flash: p.FLASH,
            gpio_pins,
            temp_sensor,

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
            spawner,
        }
    }

    /// Run the Embassy executor loop for the specified core.
    ///
    /// # Safety
    ///
    /// This function must be called from the main thread of the corresponding core and does not return.
    pub unsafe fn run_executor(cpu_id: platform::types::CpuId) -> ! {
        use platform::rp2040::PlatformMulticore as _;
        platform::rp2040::Rp2040Multicore.run_executor(cpu_id);
    }

    /// Initialize the Embassy executor for Core 1.
    ///
    /// # Safety
    /// This function must be called only once and prior to spawning Core 1 tasks.
    pub unsafe fn init_executor_core1() {
        use platform::rp2040::PlatformMulticore as _;
        platform::rp2040::Rp2040Multicore.init_executor(platform::types::CpuId::Core1);
    }

    /// Returns the Spawner for Core 1.
    ///
    /// # Safety
    /// This function must be called only after init_executor_core1 has been called.
    pub unsafe fn spawner_core1() -> embassy_executor::Spawner {
        use platform::rp2040::PlatformMulticore as _;
        platform::rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core1)
    }
}

/// Temperature sensor utilizing the RP2040 internal ADC temperature sensor.
pub struct Rp2040TempSensor {
    adc: embassy_rp::adc::Adc<'static, embassy_rp::adc::Blocking>,
    channel: embassy_rp::adc::Channel<'static>,
}

impl Rp2040TempSensor {
    /// Creates a new internal temperature sensor.
    pub fn new(
        adc_periph: embassy_rp::peripherals::ADC,
        temp_sensor_periph: embassy_rp::peripherals::ADC_TEMP_SENSOR,
    ) -> Self {
        let adc =
            embassy_rp::adc::Adc::new_blocking(adc_periph, embassy_rp::adc::Config::default());
        let channel = embassy_rp::adc::Channel::new_temp_sensor(temp_sensor_periph);
        Self { adc, channel }
    }
}

impl model::interfaces::TemperatureSensor for Rp2040TempSensor {
    type Error = embassy_rp::adc::Error;

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        let raw_temp = self.adc.blocking_read(&mut self.channel)?;
        let voltage = (raw_temp as f32 * 3.3) / 4095.0;
        let temp_c = 27.0 - (voltage - 0.706) / 0.001721;
        Ok((temp_c * 1000.0) as i32)
    }
}

/// Reads the RP2040 chip reset registers to determine the cause of the boot.
pub fn get_boot_reason() -> model::types::BootReason {
    let reg = unsafe { core::ptr::read_volatile(0x40064008 as *const u32) };
    if (reg & (1 << 8)) != 0 {
        model::types::BootReason::PowerOn
    } else if (reg & (1 << 20)) != 0 {
        model::types::BootReason::Watchdog
    } else if (reg & (1 << 16)) != 0 {
        model::types::BootReason::SoftwareReset
    } else {
        model::types::BootReason::Unknown
    }
}

/// Wrapper around Flex for battery alert pin.
pub struct AlertPinWrapper(pub Flex<'static>);

impl controller::battery_controller::BatteryAlertPin for AlertPinWrapper {
    async fn wait_for_alert(&mut self) {
        self.0.wait_for_low().await;
    }
}

/// Wrapper around Flex for sensor interrupt data ready pin.
pub struct ProximityPinWrapper(pub Flex<'static>);

impl controller::sensor_controller::DataReadyPin for ProximityPinWrapper {
    async fn wait_for_data_ready(&mut self) {
        self.0.wait_for_falling_edge().await;
    }
}

/// Safe wrapper around Rp2040TempSensor to make it thread-safe.
pub struct SafeRp2040TempSensor(pub Option<Rp2040TempSensor>);

impl model::interfaces::TemperatureSensor for SafeRp2040TempSensor {
    type Error = ();

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        if let Some(ref mut sensor) = self.0 {
            sensor.read_temperature_milli_c().map_err(|_| ())
        } else {
            Ok(25000)
        }
    }
}

/// The battery fuel gauge type.
pub type BatteryDevice = peripherals::max17048::Max17048<platform::i2c::SharedI2cWrapper<'static>>;
/// The battery charger type.
pub type ChargerDevice = peripherals::max17048::Max17048<platform::i2c::SharedI2cWrapper<'static>>;
/// The battery alert pin type.
pub type AlertPinType = AlertPinWrapper;
/// The motor driver type.
pub type MotorDevice = peripherals::l9110s::L9110s<Flex<'static>, Flex<'static>>;
/// The motor current sensor type.
pub type CurrentSensorDevice =
    peripherals::ina219::Ina219<platform::i2c::SharedI2cWrapper<'static>>;
/// The proximity sensor type.
pub type ProximitySensorDevice =
    peripherals::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>;
/// The proximity sensor interrupt pin type.
pub type DataReadyPinType = ProximityPinWrapper;
/// The LED driver type.
pub type LedDevice = peripherals::ws2812::Ws2812<'static, embassy_rp::peripherals::PIO0, 0>;
/// The temperature sensor type.
pub type TempSensorDevice = SafeRp2040TempSensor;
