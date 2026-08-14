//! Board Support Package (BSP) for the Cat Detector project.

/// Pump IA pin (GPIO 19)
pub const PUMP_PIN_IA: u32 = 19;
/// Pump IB pin (GPIO 20)
pub const PUMP_PIN_IB: u32 = 20;
/// I2C SDA pin (GPIO 12)
pub const I2C_SDA_PIN: u32 = 12;
/// I2C SCL pin (GPIO 13)
pub const I2C_SCL_PIN: u32 = 13;
/// UART TX pin (GPIO 0)
pub const UART_TX_PIN: u32 = 0;
/// UART RX pin (GPIO 1)
pub const UART_RX_PIN: u32 = 1;

/// ToF Sensor 1 (North) XSHUT pin (GPIO 4)
pub const TOF_NORTH_XSHUT_PIN: u32 = 4;
/// ToF Sensor 2 (East) XSHUT pin (GPIO 5)
pub const TOF_EAST_XSHUT_PIN: u32 = 5;
/// ToF Sensor 3 (West) XSHUT pin (GPIO 6)
pub const TOF_WEST_XSHUT_PIN: u32 = 6;

/// ToF Sensor 1 (North) Interrupt pin (GPIO 7)
pub const TOF_NORTH_INT_PIN: u32 = 7;
/// ToF Sensor 2 (East) Interrupt pin (GPIO 9)
pub const TOF_EAST_INT_PIN: u32 = 9;
/// ToF Sensor 3 (West) Interrupt pin (GPIO 10)
pub const TOF_WEST_INT_PIN: u32 = 10;

/// ToF Sensor 1 (North) I2C Address (0x30)
pub const TOF_NORTH_I2C_ADDR: u8 = 0x30;
/// ToF Sensor 2 (East) I2C Address (0x31)
pub const TOF_EAST_I2C_ADDR: u8 = 0x31;
/// ToF Sensor 3 (West) I2C Address (0x32)
pub const TOF_WEST_I2C_ADDR: u8 = 0x32;

/// Fuel Gauge Interrupt/Alert pin (GPIO 14)
pub const FUEL_GAUGE_INT_PIN: u32 = 14;

/// The default wake threshold in millimeters under which target presence is detected.
pub const DEFAULT_WAKE_THRESHOLD_MM: u16 = 300;
/// The default press threshold in millimeters under which gesture button presses are detected.
pub const DEFAULT_PRESS_THRESHOLD_MM: u16 = 20;
/// The default near threshold in millimeters for intermediate proximity alerts.
pub const DEFAULT_NEAR_THRESHOLD_MM: u16 = 100;

/// Start address of the filesystem storage partition in flash (offset from start of flash).
pub const STORAGE_PARTITION_START: u32 = 0x1C_0000; // 1.75 MB
/// End address of the filesystem storage partition in flash (2.00 MB limit).
pub const STORAGE_PARTITION_END: u32 = 0x20_0000; // 2.00 MB

/// Start address of the map filesystem partition.
pub const FS_PARTITION_START: u32 = 0x1C_0000;
/// End address of the map filesystem partition.
pub const FS_PARTITION_END: u32 = 0x1D_0000; // 64 KB

/// Start address of the telemetry queue partition.
pub const TELEMETRY_PARTITION_START: u32 = 0x1D_0000;
/// End address of the telemetry queue partition.
pub const TELEMETRY_PARTITION_END: u32 = 0x20_0000; // 192 KB

// Statically verify that filesystem and telemetry partitions are within STORAGE bounds and do not overlap.
platform::assert_partitions! {
    storage_range: (STORAGE_PARTITION_START, STORAGE_PARTITION_END),
    partition_ranges: [
        (FS_PARTITION_START, FS_PARTITION_END),
        (TELEMETRY_PARTITION_START, TELEMETRY_PARTITION_END)
    ]
}

/// Total number of telemetry chunks
pub const NUM_CHUNKS: usize = 77;
/// Total maximum number of records stored
pub const MAX_RECORDS: usize = NUM_CHUNKS * model::telemetry::CHUNK_SIZE;
/// Maximum number of rolling crash logs (modulo limit)
pub const MAX_CRASH_LOGS: u32 = 10;
/// Static working buffer for filesystem and panic handler operations.
/// Shared across the app and shell binaries to avoid duplicate stack/static allocations.
pub static mut FS_BUF: [u8; 8192] = [0u8; 8192];
/// Total QSPI flash memory capacity on the board (2.00 MB).
pub const FLASH_SIZE: usize = 2 * 1024 * 1024;
/// Top address of the stack/SRAM (RP2040 has 264 KB SRAM, ending at 0x2004_0000).
pub const STACK_TOP: u32 = 0x2004_2000;
/// Start address of flash memory mapping (XIP address space).
pub const FLASH_START: u32 = 0x1000_0000;
/// End address of flash memory mapping (FLASH_START + FLASH_SIZE).
pub const FLASH_END: u32 = 0x1020_0000;
/// Flash page write size in bytes.
pub const FLASH_WRITE_SIZE: usize = 1;
/// Flash erase block size in bytes.
pub const FLASH_ERASE_SIZE: usize = 4096;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Thread-safe Mutex wrapping the active I2C peripheral for shared access between tasks.
pub static SHARED_I2C: embassy_sync::mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    platform::i2c::SafeI2c,
> = embassy_sync::mutex::Mutex::new(platform::i2c::SafeI2c::new(
    12,
    13,
    400_000,
    i2c_recovery_fn,
));

/// RawMutex type used by controllers.
pub type MutexRaw = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global temperature sensor mutex.
pub static SHARED_TEMP_SENSOR: embassy_sync::mutex::Mutex<MutexRaw, TempSensorDevice> =
    embassy_sync::mutex::Mutex::new(SafeRp2040TempSensor(None));

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global battery/fuel gauge mutex.
pub static SHARED_BATTERY: embassy_sync::mutex::Mutex<MutexRaw, BatteryDevice> =
    embassy_sync::mutex::Mutex::new(BatteryDevice::new(platform::i2c::SharedI2cWrapper::new(
        &SHARED_I2C,
    )));

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global battery charger mutex.
pub static SHARED_CHARGER: embassy_sync::mutex::Mutex<MutexRaw, ChargerDevice> =
    embassy_sync::mutex::Mutex::new(ChargerDevice::new(platform::i2c::SharedI2cWrapper::new(
        &SHARED_I2C,
    )));

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global panic flash peripheral reference.
pub static mut PANIC_FLASH: Option<FlashDevice> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Core 1 stack size in bytes.
pub const CORE1_STACK_SIZE: usize = 16384;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Type alias for the blocking flash device.
pub type FlashDevice = embassy_rp::flash::Flash<
    'static,
    embassy_rp::peripherals::FLASH,
    embassy_rp::flash::Blocking,
    { crate::FLASH_SIZE },
>;

rp2040::boot_multicore!(crate::Board, CORE1_STACK_SIZE);

rp2040::define_panic_handler!(
    crate::STACK_TOP,
    { crate::FLASH_SIZE },
    { crate::FLASH_START },
    { crate::FLASH_END },
    { crate::FLASH_WRITE_SIZE },
    { crate::FLASH_ERASE_SIZE }
);

/// Default core monitor timeout in milliseconds.
pub const CORE_MONITOR_TIMEOUT_MS: u32 = 10_000;

/// Default core monitor warning threshold percentage.
pub const CORE_MONITOR_WARN_PCT: u32 = 80;

/// The hardware stack guard address for Core 0 (the bottom of Core 0's stack).
pub const CORE0_STACK_BOTTOM: u32 = 0x2003_C000;

platform::define_project_metadata! {
    chip: "rp2040",
    flash_base: 0x10000000,
    flash_write_size: FLASH_WRITE_SIZE,
    flash_erase_size: FLASH_ERASE_SIZE,
    fs_start: FS_PARTITION_START,
    fs_end: FS_PARTITION_END,
    telemetry_start: TELEMETRY_PARTITION_START,
    telemetry_end: TELEMETRY_PARTITION_END
}

// Host implementation
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use host::*;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod host {
    //! Host Board Support Package (BSP) mock.
    //!
    //! Provides mock peripheral drivers, inputs, and outputs to compile
    //! and validate logic on host systems.

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
        pub charger: Option<peripheral::mock::MockCharger>,
        /// Mock battery controller
        pub battery: peripheral::mock::MockBattery,
        /// Mock motor
        pub motor: peripheral::mock::MockMotor,
        /// Mock current sensor
        pub current_sensor: peripheral::mock::DummyCurrentSensor,
        /// Mock North proximity sensor
        pub tof_north: peripheral::mock::DummyProximitySensor,
        /// Mock East proximity sensor
        pub tof_east: peripheral::mock::DummyProximitySensor,
        /// Mock West proximity sensor
        pub tof_west: peripheral::mock::DummyProximitySensor,
        /// Mock LED driver
        pub led_driver: peripheral::mock::MockLed,
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
            let charger = Some(peripheral::mock::MockCharger::new(
                model::types::ChargeState::DoneOrStandbyOrUnplugged,
            ));

            let battery = peripheral::mock::MockBattery::new(3700, 25000);
            let motor = peripheral::mock::MockMotor::new();
            let current_sensor = peripheral::mock::DummyCurrentSensor;

            let tof_north = peripheral::mock::DummyProximitySensor::new(100);
            let tof_east = peripheral::mock::DummyProximitySensor::new(150);
            let tof_west = peripheral::mock::DummyProximitySensor::new(200);

            let led_driver = peripheral::mock::MockLed::new();

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

        fn is_asserted(&self) -> bool {
            false
        }
    }

    impl controller::sensor_controller::DataReadyPin for MockFlex {
        async fn wait_for_data_ready(&mut self) {
            embassy_time::Timer::after_secs(3600 * 24).await;
        }
    }

    /// The battery fuel gauge type.
    pub type BatteryDevice = peripheral::mock::MockBattery;
    /// The battery charger type.
    pub type ChargerDevice = peripheral::mock::MockCharger;
    /// The battery alert pin type.
    pub type AlertPinType = MockFlex;
    /// The motor driver type.
    pub type MotorDevice = peripheral::mock::MockMotor;
    /// The motor current sensor type.
    pub type CurrentSensorDevice = peripheral::mock::DummyCurrentSensor;
    /// The proximity sensor type.
    pub type ProximitySensorDevice = peripheral::mock::DummyProximitySensor;
    /// The proximity sensor interrupt pin type.
    pub type DataReadyPinType = MockFlex;
    /// The LED driver type.
    pub type LedDevice = peripheral::mock::MockLed;
    /// The temperature sensor type.
    pub type TempSensorDevice = Rp2040TempSensor;
}

// Target implementation
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use target::*;

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod target {
    //! Target Board Support Package (BSP) for Raspberry Pi Pico.
    //!
    //! Provides hardware-specific peripheral wrappers, pin initialization,
    //! and lookup mappings for bare-metal deployment.

    #![deny(missing_docs)]

    use embassy_rp::bind_interrupts;
    use embassy_rp::gpio::{Flex, Pin, Pull};
    use embassy_rp::pio::InterruptHandler;
    use embassy_rp::Peripherals;
    use platform::tracing;
    use platform::types::QueueFilesystem;

    bind_interrupts!(struct Irqs {
        PIO0_IRQ_0 => InterruptHandler<embassy_rp::peripherals::PIO0>;
    });

    /// Helper structure containing all pre-initialized board interfaces.
    pub struct Board<'d> {
        /// The onboard flash peripheral
        pub flash: embassy_rp::peripherals::FLASH,
        /// Lookup array containing Flex instances for dynamic GPIO diagnostics
        pub gpio_pins: [Option<Flex<'d>>; 30],
        /// Internal RP2040 temperature sensor
        pub temp_sensor: Option<Rp2040TempSensor>,

        /// Motor driver
        pub motor: peripheral::l9110s::L9110s<Flex<'d>, Flex<'d>>,
        /// Motor current sensor
        pub current_sensor: peripheral::ina219::Ina219<platform::i2c::SharedI2cWrapper<'static>>,
        /// North proximity sensor
        pub tof_north: peripheral::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
        /// East proximity sensor
        pub tof_east: peripheral::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
        /// West proximity sensor
        pub tof_west: peripheral::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>,
        /// Status LED driver
        pub led_driver: peripheral::ws2812::Ws2812<'d, embassy_rp::peripherals::PIO0, 0>,
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
        pub async fn init(p: Peripherals) -> Self {
            // Configure hardware stack guard using Cortex-M MPU
            platform::core_monitor::configure_mpu_stack_guard(crate::CORE0_STACK_BOTTOM);

            // Initialize the I2C0 peripheral inside SHARED_I2C static Mutex
            {
                let mut guard = crate::SHARED_I2C.lock().await;
                guard.initialize();
            }
            let mut i2c = platform::i2c::SharedI2cWrapper::new(&crate::SHARED_I2C);
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
            let mut raw_flash: crate::FlashDevice =
                embassy_rp::flash::Flash::new_blocking(temp_flash);
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
            for &(_name, xshut_pin, addr) in &sensors {
                if let Some(ref mut pin) = gpio_pins[xshut_pin as usize] {
                    pin.set_high();
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    ::embassy_time::Timer::after_millis(2).await;

                    let mut sensor = peripheral::vl53l0x::Vl53l0x::new(
                        &mut i2c,
                        addr,
                        model::types::Direction::North,
                    );
                    let _ = sensor.set_threshold_mm(crate::DEFAULT_WAKE_THRESHOLD_MM);
                    sensor.set_interrupt_mode(peripheral::vl53l0x::InterruptMode::LowLevel);

                    peripheral::init_i2c!(&mut sensor, &mut boot_status);
                }
            }

            let temp_sensor = Some(Rp2040TempSensor::new(p.ADC, p.ADC_TEMP_SENSOR));

            // Configure remaining drivers using local i2c before returning
            {
                let mut sensor = peripheral::max17048::Max17048::new(&mut i2c);
                peripheral::init_i2c!(&mut sensor, &mut boot_status);
            }
            {
                let mut sensor = peripheral::ina219::Ina219::new(&mut i2c);
                peripheral::init_i2c!(&mut sensor, &mut boot_status);
            }

            // Extract pins needed for drivers/controllers
            let mut motor_pin_ia = gpio_pins[crate::PUMP_PIN_IA as usize]
                .take()
                .expect("Motor pin IA must be available");
            let mut motor_pin_ib = gpio_pins[crate::PUMP_PIN_IB as usize]
                .take()
                .expect("Motor pin IB must be available");
            motor_pin_ia.set_as_output();
            motor_pin_ib.set_as_output();
            let motor = peripheral::l9110s::L9110s::new(motor_pin_ia, motor_pin_ib);

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
            let current_sensor = peripheral::ina219::Ina219::new(
                platform::i2c::SharedI2cWrapper::new(&crate::SHARED_I2C),
            );
            let make_tof = |addr, direction| {
                let mut sensor = peripheral::vl53l0x::Vl53l0x::new(
                    platform::i2c::SharedI2cWrapper::new(&crate::SHARED_I2C),
                    addr,
                    direction,
                );
                let _ = sensor.set_threshold_mm(crate::DEFAULT_WAKE_THRESHOLD_MM);
                sensor
            };

            let tof_north = make_tof(crate::TOF_NORTH_I2C_ADDR, model::types::Direction::North);
            let tof_east = make_tof(crate::TOF_EAST_I2C_ADDR, model::types::Direction::East);
            let tof_west = make_tof(crate::TOF_WEST_I2C_ADDR, model::types::Direction::West);

            let pio_config = rp2040::pio::PioInitConfig {
                pio: p.PIO0,
                pin: p.PIN_11,
                irq: Irqs,
            };
            let led_driver = peripheral::init_pio!(
                peripheral::ws2812::Ws2812<embassy_rp::peripherals::PIO0, 0>,
                pio_config,
                &mut boot_status
            );

            let spawner = unsafe {
                use rp2040::PlatformMulticore as _;
                Some(rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core0))
            };

            Self {
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
            use rp2040::PlatformMulticore as _;
            rp2040::Rp2040Multicore.run_executor(cpu_id);
        }

        /// Initialize the Embassy executor for Core 1.
        ///
        /// # Safety
        /// This function must be called only once and prior to spawning Core 1 tasks.
        pub unsafe fn init_executor_core1() {
            use rp2040::PlatformMulticore as _;
            rp2040::Rp2040Multicore.init_executor(platform::types::CpuId::Core1);
        }

        /// Returns the Spawner for Core 1.
        ///
        /// # Safety
        /// This function must be called only after init_executor_core1 has been called.
        pub unsafe fn spawner_core1() -> embassy_executor::Spawner {
            use rp2040::PlatformMulticore as _;
            rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core1)
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

        fn is_asserted(&self) -> bool {
            self.0.is_low()
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
    pub type BatteryDevice =
        peripheral::max17048::Max17048<platform::i2c::SharedI2cWrapper<'static>>;
    /// The battery charger type.
    pub type ChargerDevice =
        peripheral::max17048::Max17048<platform::i2c::SharedI2cWrapper<'static>>;
    /// The battery alert pin type.
    pub type AlertPinType = AlertPinWrapper;
    /// The motor driver type.
    pub type MotorDevice = peripheral::l9110s::L9110s<Flex<'static>, Flex<'static>>;
    /// The motor current sensor type.
    pub type CurrentSensorDevice =
        peripheral::ina219::Ina219<platform::i2c::SharedI2cWrapper<'static>>;
    /// The proximity sensor type.
    pub type ProximitySensorDevice =
        peripheral::vl53l0x::Vl53l0x<platform::i2c::SharedI2cWrapper<'static>>;
    /// The proximity sensor interrupt pin type.
    pub type DataReadyPinType = ProximityPinWrapper;
    /// The LED driver type.
    pub type LedDevice = peripheral::ws2812::Ws2812<'static, embassy_rp::peripherals::PIO0, 0>;
    /// The temperature sensor type.
    pub type TempSensorDevice = SafeRp2040TempSensor;

    /// Concrete RP2040 I2C Recovery implementation.
    pub struct Rp2040I2cRecovery {
        /// The GPIO pin number used for I2C SDA.
        pub sda_pin: u8,
        /// The GPIO pin number used for I2C SCL.
        pub scl_pin: u8,
    }

    impl platform::i2c::PlatformI2cRecovery for Rp2040I2cRecovery {
        unsafe fn recover_i2c_bus(&self) -> Result<(), &'static str> {
            use embassy_rp::gpio::{Flex, Pull};
            let mut scl = Flex::new(unsafe { embassy_rp::gpio::AnyPin::steal(self.scl_pin) });
            let mut sda = Flex::new(unsafe { embassy_rp::gpio::AnyPin::steal(self.sda_pin) });

            scl.set_as_output();
            scl.set_high();

            sda.set_as_input();
            sda.set_pull(Pull::Up);

            // Give pull-up resistor time to charge bus settle (~8 microseconds)
            embassy_time::block_for(embassy_time::Duration::from_micros(8));

            for _ in 0..16 {
                if sda.is_high() {
                    break;
                }
                scl.set_low();
                embassy_time::block_for(embassy_time::Duration::from_micros(400));
                scl.set_high();
                embassy_time::block_for(embassy_time::Duration::from_micros(400));
            }

            scl.set_pull(Pull::Up);
            sda.set_pull(Pull::Up);
            Ok(())
        }
    }

    /// Core 0 i2c recovery callback function.
    pub fn i2c_recovery_fn(sda_pin: u8, scl_pin: u8) {
        unsafe {
            let recovery = Rp2040I2cRecovery { sda_pin, scl_pin };
            let _ = platform::i2c::PlatformI2cRecovery::recover_i2c_bus(&recovery);
        }
    }
}
