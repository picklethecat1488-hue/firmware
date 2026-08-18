//! Application library for the Cat Detector project.
//!
//! Exposes control loop structures, channels, and task orchestration.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

pub use board::cat_detector::{
    get_boot_reason, AlertPinType, BatteryDevice, Board, BoardPeripherals, ChargerDevice,
    CurrentSensorDevice, DataReadyPinType, LedDevice, MotorDevice, MutexRaw, ProximitySensorDevice,
    Rp2040TempSensor, TempSensorDevice,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use board::cat_detector::{handle_panic, SHARED_I2C};

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global OnceLock static for Board.
pub static BOARD: platform::OnceLock<Board<'static>> = platform::OnceLock::new();

pub use controller::{
    run_filesystem_task, run_telemetry_task, shell_controller, telemetry_controller as telemetry,
    BatteryFeatureConfig, FilesystemChannel, LedFeatureConfig, MotorFeatureConfig, ProximityEvent,
    ProximityFeatureConfig, SystemCommand, SystemController, SystemFeatureSet, TelemetryChannel,
    ThermalFeatureConfig,
};
pub use model::types::SystemStatus;
pub use platform::core_monitor::{Core1Command, CORE1_COMMAND_CHANNEL};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use platform::core_monitor::core1_command_task;

pub use platform::define_core1_getters;
pub use platform::BatteryUpdateAction;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Synchronously initializes all application subcontrollers from board hardware.
pub async fn init_controllers(
    board: &'static Board<'static>,
    peripherals: BoardPeripherals<'static>,
) -> Controllers {
    let thermal = controller::thermal_controller::ThermalController::new_with_shutdown_and_trap(
        &board.temp_sensor,
        THERMAL_ACTION_CHANNEL.sender(),
    );

    let alert_wrapper = board::cat_detector::AlertPinWrapper(peripherals.fuel_gauge_alert_pin);
    let battery = controller::battery_controller::BatteryController::new_with_system_and_alert(
        &board.battery,
        &board.charger,
        SYSTEM_CHANNEL.sender(),
        alert_wrapper,
    );

    let led = controller::led_controller::LedController::new(peripherals.led_driver);

    let mut sensor_north =
        controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
            controller::types::SensorMetadata {
                direction: model::types::Direction::North,
            },
            peripherals.tof_north,
            SYSTEM_CHANNEL.sender(),
            board::cat_detector::ProximityPinWrapper(peripherals.pin_north),
            Board::DEFAULT_WAKE_THRESHOLD_MM,
        );
    sensor_north.bind_command_tx(SENSOR_NORTH_CHANNEL.sender());

    let mut sensor_east =
        controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
            controller::types::SensorMetadata {
                direction: model::types::Direction::East,
            },
            peripherals.tof_east,
            SYSTEM_CHANNEL.sender(),
            board::cat_detector::ProximityPinWrapper(peripherals.pin_east),
            Board::DEFAULT_WAKE_THRESHOLD_MM,
        );
    sensor_east.bind_command_tx(SENSOR_EAST_CHANNEL.sender());

    let mut sensor_west =
        controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
            controller::types::SensorMetadata {
                direction: model::types::Direction::West,
            },
            peripherals.tof_west,
            SYSTEM_CHANNEL.sender(),
            board::cat_detector::ProximityPinWrapper(peripherals.pin_west),
            Board::DEFAULT_WAKE_THRESHOLD_MM,
        );
    sensor_west.bind_command_tx(SENSOR_WEST_CHANNEL.sender());

    let motor = controller::motor_controller::MotorController::new(
        peripherals.motor,
        peripherals.current_sensor,
    );

    let system = controller::SystemController::new(
        create_default_feature_set(),
        TELEMETRY_CHANNEL.sender(),
        crate::get_boot_reason(),
    );

    Controllers {
        core0: Core0Controllers {
            thermal,
            battery,
            led,
            system,
        },
        core1: Core1Controllers {
            motor,
            sensor_north,
            sensor_east,
            sensor_west,
        },
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_app.rs"));

/// Type alias for the motor controller.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub type MotorType =
    controller::motor_controller::MotorController<crate::MotorDevice, crate::CurrentSensorDevice>;

/// Type alias for the sensor controller.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub type SensorType = controller::sensor_controller::SensorController<
    'static,
    crate::ProximitySensorDevice,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    crate::DataReadyPinType,
    crate::SystemCommand,
    controller::sensor_controller::ProximityReader,
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Boots Core 1 and starts the Core 1 bootstrap task, returning the Spawner for Core 1.
pub fn bootstrap_core1(
    core1: embassy_rp::peripherals::CORE1,
    controllers: Core1Controllers,
) -> embassy_executor::Spawner {
    board::cat_detector::boot_core1(core1);

    let spawner_c1 = spawner_core1();
    spawner_c1
        .spawn(bootstrap_core1_task(spawner_c1, controllers))
        .unwrap();

    spawner_c1
}

/// Boots Core 1 peripherals and controllers.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
#[cfg_attr(target_arch = "arm", link_section = ".data.core1_func")]
pub async fn bootstrap_core1_task(
    spawner: embassy_executor::Spawner,
    core1_controllers: Core1Controllers,
) {
    // Configure MPU Stack Guard for Core 1
    let top = board::cat_detector::CORE1_STACK_TOP.load(core::sync::atomic::Ordering::Acquire);
    if top != board::cat_detector::CORE1_DEFAULT_STACK_TOP {
        platform::core_monitor::configure_mpu_stack_guard(top, Board::CORE1_STACK_SIZE);
    }

    // Initialize the core monitor for Core 1
    platform::core_monitor::init_core(
        Some(spawner),
        platform::core_monitor::CpuId::Core1,
        Board::CORE_MONITOR_TIMEOUT_MS,
        Board::CORE_MONITOR_WARN_PCT,
        true,
    );

    let controllers = Core1Wrapper {
        core1: core1_controllers,
    };

    spawn_core1_controllers!(spawner, controllers);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// The concrete flash type used for the filesystem partition in production.
pub type FlashDeviceType = platform::flash::TargetFlash<{ Board::FLASH_SIZE }>;
// Type aliases for controllers
#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the thermal controller.
pub type ThermalControllerType =
    controller::thermal_controller::ThermalController<'static, MutexRaw, TempSensorDevice>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the thermal controller (mock on host).
pub type ThermalControllerType = ();

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the battery controller.
pub type BatteryControllerType = controller::battery_controller::BatteryController<
    'static,
    MutexRaw,
    BatteryDevice,
    ChargerDevice,
    AlertPinType,
>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the battery controller (mock on host).
pub type BatteryControllerType = ();

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the LED controller.
pub type LedControllerType = controller::led_controller::LedController<LedDevice>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the LED controller (mock on host).
pub type LedControllerType = ();

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the motor controller.
pub type MotorControllerType =
    controller::motor_controller::MotorController<MotorDevice, CurrentSensorDevice>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Mock motor controller structure for the host shell.
pub struct MockMotorController;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl controller::MotorReader for MockMotorController {
    async fn read_motor_current_ma(&mut self) -> Result<i32, model::types::PeripheralError> {
        Ok(150)
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl controller::MotorWriter for MockMotorController {
    async fn set_motor_speed(&mut self, _speed: i8) -> Result<(), model::types::PeripheralError> {
        Ok(())
    }
    async fn set_motor_speed_rpm(
        &mut self,
        _rpm: i32,
    ) -> Result<(), model::types::PeripheralError> {
        Ok(())
    }
    async fn stop_motor(&mut self) -> Result<(), model::types::PeripheralError> {
        Ok(())
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl model::calibration::Calibration for MockMotorController {
    const CALIBRATION_FILE_NAME: &'static str = "motor_cal.cbor";
    type Store = model::calibration::MotorCalibration;
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the motor controller (mock on host).
pub type MotorControllerType = MockMotorController;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the proximity sensor controller.
pub type SensorControllerType = controller::sensor_controller::SensorController<
    'static,
    ProximitySensorDevice,
    MutexRaw,
    DataReadyPinType,
    SystemCommand,
    controller::sensor_controller::ProximityReader,
>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Mock sensor controller structure for the host shell.
pub struct MockSensorController;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl controller::ProximityReader for MockSensorController {
    async fn read_distance(
        &mut self,
    ) -> Result<model::types::SensorReading, model::types::PeripheralError> {
        Ok(model::types::SensorReading::Proximity(100))
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl model::calibration::Calibration for MockSensorController {
    const CALIBRATION_FILE_NAME: &'static str = "vl53l0x_cal.cbor";
    type Store = model::calibration::Vl53l0xCalibration;
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl model::calibration::ApplyCalibration for MockSensorController {
    type Input = model::types::SensorReading;
    type Output = model::types::SensorReading;
    type Error = &'static str;

    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        match reading {
            model::types::SensorReading::Proximity(_) => Ok(reading),
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the proximity sensor controller (mock on host).
pub type SensorControllerType = MockSensorController;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete type for the telemetry controller.
pub type TelemetryControllerType = controller::telemetry_controller::TelemetryController<
    platform::flash::SharedFlashMutex<platform::BlockingAsyncFlash<FlashDevice>>,
>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the telemetry controller (mock on host).
pub type TelemetryControllerType = ();

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Mock flash device for host.
pub struct MockFlash;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl embedded_storage::nor_flash::ErrorType for MockFlash {
    type Error = core::convert::Infallible;
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl embedded_storage::nor_flash::ReadNorFlash for MockFlash {
    const READ_SIZE: usize = 1;
    fn read(&mut self, _address: u32, _buf: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn capacity(&self) -> usize {
        1024
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl embedded_storage::nor_flash::NorFlash for MockFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;
    fn write(&mut self, _address: u32, _buf: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn erase(&mut self, _from: u32, _to: u32) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// The concrete I2C bus type for the shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub type I2cBus =
    embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>;

/// The concrete I2C bus type for the shell (mock on host).
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub type I2cBus = peripheral::mock::DummyI2c;

/// The concrete flash type.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub type FlashDevice = board::cat_detector::FlashDevice;

/// The concrete flash type (mock on host).
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub type FlashDevice = MockFlash;

/// Configuration for the interactive bringup shell.
pub struct CatDetectorShellConfig;

controller::impl_shell_config! {
    CatDetectorShellConfig {
        I2c = I2cBus,
        Motor = MotorDevice,
        Flash = FlashDevice,
        TempSensor = Rp2040TempSensor,
        ThermalCtrl = ThermalControllerType,
        BatteryCtrl = BatteryControllerType,
        SensorCtrl = SensorControllerType,
        MotorCtrl = MotorControllerType,
        SystemCtrl = SystemControllerType,
        LedCtrl = controller::led_controller::LedController<LedDevice>,
    }

    fn trigger_core_panic(
        _resolver: &controller::shell_controller::ShellController<'_, Self>,
        core_id: u32,
    ) -> Result<(), &'static str> {
        platform::core_monitor::trigger_core_panic(core_id)
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Configured layout and resources for filesystem partition storage and crash logs.
pub struct FilesystemStorageConfig {
    /// Partition range mapped for filesystem.
    pub partition: controller::MapFilesystem,
    /// Static mutable buffer for file operations.
    pub buffer: &'static mut [u8],
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl FilesystemStorageConfig {
    /// Safe helper to steal the FLASH peripheral and construct the flash device driver.
    /// This is safe because the panic handler only reads/writes on panic.
    pub fn steal_flash(&self) -> platform::BlockingAsyncFlash<FlashDevice> {
        let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
        let raw_flash = embassy_rp::flash::Flash::<
            _,
            embassy_rp::flash::Blocking,
            { Board::FLASH_SIZE },
        >::new_blocking(fs_flash);
        platform::BlockingAsyncFlash(raw_flash)
    }

    /// Safely set the shared FLASH mutex.
    pub fn set_flash_mutex(&self, mutex: FlashMutexType) -> &'static FlashMutexType {
        if FLASH_MUTEX.set(mutex).is_err() {
            panic!("Failed to set FLASH_MUTEX static");
        }
        FLASH_MUTEX.get().unwrap()
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Get the unified filesystem storage configuration.
pub fn get_filesystem_config() -> FilesystemStorageConfig {
    FilesystemStorageConfig {
        partition: controller::MapFilesystem(Board::FS_PARTITION_START..Board::FS_PARTITION_END),
        buffer: Board::fs_buf(),
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely steal the Core 1 peripheral instance.
pub fn steal_core1_peripheral() -> embassy_rp::peripherals::CORE1 {
    unsafe { embassy_rp::peripherals::CORE1::steal() }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely get the Core 0 Embassy executor spawner.
pub fn spawner_core0() -> embassy_executor::Spawner {
    unsafe {
        use rp2040::PlatformMulticore as _;
        rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core0)
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely get the Core 1 Embassy executor spawner.
pub fn spawner_core1() -> embassy_executor::Spawner {
    unsafe { Board::spawner_core1() }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Combined layout of all controllers mapped by core.
pub struct Controllers {
    /// Controllers running on Core 0.
    pub core0: Core0Controllers,
    /// Controllers running on Core 1.
    pub core1: Core1Controllers,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Collection of all Core 0 controllers.
pub struct Core0Controllers {
    /// The thermal controller.
    pub thermal: ThermalControllerType,
    /// The battery controller.
    pub battery: BatteryControllerType,
    /// The LED controller.
    pub led: LedControllerType,
    /// The system controller.
    pub system: SystemControllerType,
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Combined layout of all controllers mapped by core on host.
pub struct Controllers {
    /// Controllers running on Core 0.
    pub core0: Core0Controllers,
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Dummy Core0Controllers on host.
pub struct Core0Controllers {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Collection of all Core 1 controllers.
pub struct Core1Controllers {
    /// The motor controller.
    pub motor: MotorControllerType,
    /// The North proximity sensor controller.
    pub sensor_north: SensorControllerType,
    /// The East proximity sensor controller.
    pub sensor_east: SensorControllerType,
    /// The West proximity sensor controller.
    pub sensor_west: SensorControllerType,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Wrapper struct to map Core 1 controllers hierarchy under a core1 field.
pub struct Core1Wrapper {
    /// Core 1 controllers.
    pub core1: Core1Controllers,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Type alias for shared flash mutex
pub type FlashMutexType = embassy_sync::mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    platform::BlockingAsyncFlash<FlashDevice>,
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Shared static FLASH mutex
static FLASH_MUTEX: platform::OnceLock<FlashMutexType> = platform::OnceLock::new();

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely run the executor loop for the specified core.
pub fn run_executor(cpu_id: platform::types::CpuId) -> ! {
    unsafe { Board::run_executor(cpu_id) }
}
