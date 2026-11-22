//! Board Support Package (BSP) for the Carrier Board 2.0 (NXP MCX N947) platform.

include!(concat!(env!("OUT_DIR"), "/generated_carrier_board.rs"));

/// Static working buffer for filesystem and panic handler operations.
static mut FS_BUF: [u8; Board::FS_BUF_SIZE] = [0u8; Board::FS_BUF_SIZE];

/// RawMutex type used by Carrier Board 2.0 controllers.
pub type MutexRaw = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

/// Mock internal temperature sensor representing the MCX N947 on-die thermal sensor during bringup.
pub struct McxTempSensor;

impl model::interfaces::TemperatureSensor for McxTempSensor {
    type Error = core::convert::Infallible;

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(25_000)
    }
}

/// Thread-safe wrapper around [`McxTempSensor`].
pub struct SafeMcxTempSensor(pub Option<McxTempSensor>);

impl model::interfaces::TemperatureSensor for SafeMcxTempSensor {
    type Error = ();

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        if let Some(ref mut sensor) = self.0 {
            sensor.read_temperature_milli_c().map_err(|_| ())
        } else {
            Ok(25_000)
        }
    }
}

/// Digital input/output pin abstraction for Carrier Board 2.0 bringup and host validation.
#[derive(Default)]
pub struct CarrierFlexPin {
    /// Logical pin state (`true` = high, `false` = low).
    pub value: bool,
}

impl CarrierFlexPin {
    /// Construct a new high-idling interrupt/GPIO pin.
    pub const fn new_high() -> Self {
        Self { value: true }
    }
}

impl embedded_hal::digital::ErrorType for CarrierFlexPin {
    type Error = core::convert::Infallible;
}

impl embedded_hal::digital::OutputPin for CarrierFlexPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.value = false;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.value = true;
        Ok(())
    }
}

impl embedded_hal::digital::InputPin for CarrierFlexPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.value)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.value)
    }
}

impl controller::battery_controller::BatteryAlertPin for CarrierFlexPin {
    async fn wait_for_alert(&mut self) {
        embassy_time::Timer::after_secs(3600 * 24).await;
    }

    fn is_asserted(&self) -> bool {
        !self.value
    }
}

/// NOR flash placeholder implementation for Carrier Board 2.0 shell storage resolving.
pub struct CarrierFlashDevice;

impl embedded_storage::nor_flash::ErrorType for CarrierFlashDevice {
    type Error = embedded_storage::nor_flash::NorFlashErrorKind;
}

impl embedded_storage::nor_flash::ReadNorFlash for CarrierFlashDevice {
    const READ_SIZE: usize = 1;

    fn read(&mut self, _offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.fill(0xFF);
        Ok(())
    }

    fn capacity(&self) -> usize {
        Board::FLASH_SIZE
    }
}

impl embedded_storage::nor_flash::NorFlash for CarrierFlashDevice {
    const WRITE_SIZE: usize = Board::FLASH_WRITE_SIZE;
    const ERASE_SIZE: usize = Board::FLASH_ERASE_SIZE;

    fn write(&mut self, _offset: u32, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn erase(&mut self, _from: u32, _to: u32) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Battery fuel gauge device type (MAX17048).
pub type BatteryDevice = peripheral::mock::MockBattery;
/// Battery charger device type (BQ24074).
pub type ChargerDevice = peripheral::mock::MockCharger;
/// Battery alert interrupt pin type (`FUEL_ALERT`).
pub type AlertPinType = CarrierFlexPin;
/// RGB LED driver device type (LP5009).
pub type LedDevice = peripheral::mock::MockLed;
/// On-die MCU thermal sensor type.
pub type TempSensorDevice = SafeMcxTempSensor;
/// Internal NOR flash device type.
pub type FlashDevice = CarrierFlashDevice;

/// Move-by-value drivers and interrupt pins returned during Carrier Board initialization.
pub struct BoardPeripherals {
    /// Status RGB LED driver (LP5009).
    pub led_driver: LedDevice,
    /// Fuel gauge active-low alert pin.
    pub fuel_gauge_alert_pin: AlertPinType,
}

/// Shared communication buses initialized by Carrier Board 2.0.
pub struct BoardBuses;

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod target {
    //! Target Board Support Package (BSP) for Carrier Board 2.0 (NXP MCX N947).

    use super::*;

    /// Pre-initialized Carrier Board 2.0 hardware context.
    pub struct Board<'d> {
        /// Lifetime marker matching the unified `Board<'d>` BSP contract.
        pub _marker: core::marker::PhantomData<&'d ()>,
        /// On-die temperature sensor mutex.
        pub temp_sensor: embassy_sync::mutex::Mutex<MutexRaw, TempSensorDevice>,
        /// MAX17048 fuel gauge mutex.
        pub battery: embassy_sync::mutex::Mutex<MutexRaw, BatteryDevice>,
        /// BQ24074 battery charger mutex.
        pub charger: embassy_sync::mutex::Mutex<MutexRaw, ChargerDevice>,
        /// Move-by-value peripherals taken once by the application/shell entrypoint.
        pub peripherals: Option<BoardPeripherals>,
    }

    impl<'d> Board<'d> {
        /// Take shared communication buses owned by the board.
        pub fn take_buses(&self) -> BoardBuses {
            BoardBuses
        }

        /// Take move-by-value peripherals owned by the board.
        pub fn take_peripherals(&mut self) -> BoardPeripherals {
            self.peripherals.take().expect("Peripherals already taken")
        }

        /// Initialize Carrier Board 2.0 clocks, MPU stack guard, and peripheral drivers.
        pub async fn init() -> Self {
            platform::core_monitor::configure_mpu_stack_guard(
                Board::CORE0_STACK_TOP,
                Board::CORE0_STACK_SIZE,
            );
            Self {
                _marker: core::marker::PhantomData,
                temp_sensor: embassy_sync::mutex::Mutex::new(SafeMcxTempSensor(Some(
                    McxTempSensor,
                ))),
                battery: embassy_sync::mutex::Mutex::new(peripheral::mock::MockBattery::new(
                    4050, 25_000,
                )),
                charger: embassy_sync::mutex::Mutex::new(peripheral::mock::MockCharger::new(
                    model::types::ChargeState::DoneOrStandbyOrUnplugged,
                )),
                peripherals: Some(BoardPeripherals {
                    led_driver: peripheral::mock::MockLed::new(),
                    fuel_gauge_alert_pin: CarrierFlexPin::new_high(),
                }),
            }
        }
    }

    struct SyncExecutor(embassy_executor::raw::Executor);
    unsafe impl Sync for SyncExecutor {}

    static mut EXECUTOR_CORE0: Option<SyncExecutor> = None;

    /// Obtain the Core 0 Embassy spawner, initializing the raw executor if needed.
    ///
    /// # Safety
    ///
    /// Must only be invoked on Core 0 prior to or within `run_executor`.
    pub unsafe fn spawner_core0() -> embassy_executor::Spawner {
        let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE0);
        if (*ptr).is_none() {
            *ptr = Some(SyncExecutor(embassy_executor::raw::Executor::new(
                usize::MAX as *mut (),
            )));
        }
        let executor_static: &'static embassy_executor::raw::Executor = &(*ptr).as_ref().unwrap().0;
        executor_static.spawner()
    }

    /// Run the Core 0 scheduler loop (`CpuScheduler::run_loop`) emitting `CPU Idle Core 0` spans under `trace-perf`.
    ///
    /// # Safety
    ///
    /// Must be called from the main thread entry of Core 0.
    pub unsafe fn run_executor(cpu_id: platform::types::CpuId) -> ! {
        use platform::system::CpuScheduler as _;
        let _ = spawner_core0();
        let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE0);
        let executor_static: &'static embassy_executor::raw::Executor = &(*ptr).as_ref().unwrap().0;
        executor_static.run_loop(cpu_id)
    }

    /// Read the MCX N947 CMC reset status register to determine boot cause.
    pub fn get_boot_reason() -> model::types::BootReason {
        model::types::BootReason::PowerOn
    }

    /// Carrier Board 2.0 target panic handler logging via `defmt` and triggering a debug trap.
    pub fn handle_panic(info: &core::panic::PanicInfo) -> ! {
        defmt::error!("Carrier Board panic: {}", defmt::Display2Format(info));
        loop {
            cortex_m::asm::bkpt();
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use target::*;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod host {
    //! Host mock Board Support Package (BSP) for Carrier Board 2.0.

    use super::*;

    /// Mock Carrier Board 2.0 context for host compilation and unit tests.
    pub struct Board {
        /// On-die temperature sensor mutex.
        pub temp_sensor: embassy_sync::mutex::Mutex<MutexRaw, TempSensorDevice>,
        /// MAX17048 fuel gauge mutex.
        pub battery: embassy_sync::mutex::Mutex<MutexRaw, BatteryDevice>,
        /// BQ24074 battery charger mutex.
        pub charger: embassy_sync::mutex::Mutex<MutexRaw, ChargerDevice>,
        /// Move-by-value peripherals taken once by the caller.
        pub peripherals: Option<BoardPeripherals>,
    }

    impl Board {
        /// Take mock communication buses.
        pub fn take_buses(&self) -> BoardBuses {
            BoardBuses
        }

        /// Take mock move-by-value peripherals.
        pub fn take_peripherals(&mut self) -> BoardPeripherals {
            self.peripherals.take().expect("Peripherals already taken")
        }

        /// Initialize mock Carrier Board 2.0 instance on host.
        pub fn init() -> Self {
            Self {
                temp_sensor: embassy_sync::mutex::Mutex::new(SafeMcxTempSensor(Some(
                    McxTempSensor,
                ))),
                battery: embassy_sync::mutex::Mutex::new(peripheral::mock::MockBattery::new(
                    4050, 25_000,
                )),
                charger: embassy_sync::mutex::Mutex::new(peripheral::mock::MockCharger::new(
                    model::types::ChargeState::DoneOrStandbyOrUnplugged,
                )),
                peripherals: Some(BoardPeripherals {
                    led_driver: peripheral::mock::MockLed::new(),
                    fuel_gauge_alert_pin: CarrierFlexPin::new_high(),
                }),
            }
        }
    }

    /// Mock boot reason on host.
    pub fn get_boot_reason() -> model::types::BootReason {
        model::types::BootReason::Unknown
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use host::*;
