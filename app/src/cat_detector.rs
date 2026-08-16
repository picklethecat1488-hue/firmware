//! Application library for the Cat Detector project.
//!
//! Exposes control loop structures, channels, and task orchestration.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

pub use board::cat_detector::{
    get_boot_reason, AlertPinType, BatteryDevice, Board, ChargerDevice, CurrentSensorDevice,
    DataReadyPinType, LedDevice, MotorDevice, MutexRaw, ProximitySensorDevice, Rp2040TempSensor,
    TempSensorDevice, CORE0_STACK_BOTTOM, CORE_MONITOR_TIMEOUT_MS, CORE_MONITOR_WARN_PCT,
    DEFAULT_NEAR_THRESHOLD_MM, DEFAULT_PRESS_THRESHOLD_MM, DEFAULT_WAKE_THRESHOLD_MM, FLASH_END,
    FLASH_ERASE_SIZE, FLASH_SIZE, FLASH_START, FLASH_WRITE_SIZE, FS_BUF, FS_PARTITION_END,
    FS_PARTITION_START, FUEL_GAUGE_INT_PIN, I2C_SCL_PIN, I2C_SDA_PIN, MAX_CRASH_LOGS, MAX_RECORDS,
    NUM_CHUNKS, PUMP_PIN_IA, PUMP_PIN_IB, STACK_TOP, STORAGE_PARTITION_END,
    STORAGE_PARTITION_START, TELEMETRY_PARTITION_END, TELEMETRY_PARTITION_START, TOF_EAST_I2C_ADDR,
    TOF_EAST_INT_PIN, TOF_EAST_XSHUT_PIN, TOF_NORTH_I2C_ADDR, TOF_NORTH_INT_PIN,
    TOF_NORTH_XSHUT_PIN, TOF_WEST_I2C_ADDR, TOF_WEST_INT_PIN, TOF_WEST_XSHUT_PIN, UART_RX_PIN,
    UART_TX_PIN,
};

pub use platform::panic_handler::init as init_panic_handler;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use board::cat_detector::{
    handle_panic, CORE1_STACK_SIZE, PANIC_FLASH, SHARED_BATTERY, SHARED_CHARGER, SHARED_I2C,
    SHARED_TEMP_SENSOR,
};

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

pub use platform::BatteryUpdateAction;
pub use platform::{
    define_core1_getters, define_static_mut_getters, get_static_mut, take_static_mut,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the ThermalController.
pub static mut THERMAL_CTRL: Option<
    controller::thermal_controller::ThermalController<'static, MutexRaw, TempSensorDevice>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the BatteryController.
pub static mut BATTERY_CTRL: Option<
    controller::battery_controller::BatteryController<
        'static,
        MutexRaw,
        BatteryDevice,
        ChargerDevice,
        AlertPinType,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the LedController.
pub static mut LED_CTRL: Option<controller::led_controller::LedController<LedDevice>> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the North SensorController.
pub static mut SENSOR_CTRL_NORTH_CORE0: Option<
    controller::sensor_controller::SensorController<
        'static,
        ProximitySensorDevice,
        MutexRaw,
        DataReadyPinType,
        SystemCommand,
        controller::sensor_controller::ProximityReader,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the East SensorController.
pub static mut SENSOR_CTRL_EAST_CORE0: Option<
    controller::sensor_controller::SensorController<
        'static,
        ProximitySensorDevice,
        MutexRaw,
        DataReadyPinType,
        SystemCommand,
        controller::sensor_controller::ProximityReader,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the West SensorController.
pub static mut SENSOR_CTRL_WEST_CORE0: Option<
    controller::sensor_controller::SensorController<
        'static,
        ProximitySensorDevice,
        MutexRaw,
        DataReadyPinType,
        SystemCommand,
        controller::sensor_controller::ProximityReader,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the MotorController.
pub static mut MOTOR_CTRL_CORE0: Option<
    controller::motor_controller::MotorController<MotorDevice, CurrentSensorDevice>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the SystemController.
pub static mut SYSTEM_CTRL: Option<SystemControllerType> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Synchronously initializes all application subcontrollers from board hardware.
pub async fn init_controllers(board: Board<'static>) {
    unsafe {
        THERMAL_CTRL = Some(
            controller::thermal_controller::ThermalController::new_with_shutdown_and_trap(
                &SHARED_TEMP_SENSOR,
                THERMAL_ACTION_CHANNEL.sender(),
            ),
        );

        let alert_wrapper = board::cat_detector::AlertPinWrapper(board.fuel_gauge_alert_pin);
        BATTERY_CTRL = Some(
            controller::battery_controller::BatteryController::new_with_system_and_alert(
                &SHARED_BATTERY,
                &SHARED_CHARGER,
                SYSTEM_CHANNEL.sender(),
                alert_wrapper,
            ),
        );

        LED_CTRL = Some(controller::led_controller::LedController::new(
            board.led_driver,
        ));

        let mut north =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::North,
                },
                board.tof_north,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(board.pin_north),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        north.bind_command_tx(SENSOR_NORTH_CHANNEL.sender());
        SENSOR_CTRL_NORTH_CORE0 = Some(north);

        let mut east =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::East,
                },
                board.tof_east,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(board.pin_east),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        east.bind_command_tx(SENSOR_EAST_CHANNEL.sender());
        SENSOR_CTRL_EAST_CORE0 = Some(east);

        let mut west =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::West,
                },
                board.tof_west,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(board.pin_west),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        west.bind_command_tx(SENSOR_WEST_CHANNEL.sender());
        SENSOR_CTRL_WEST_CORE0 = Some(west);

        MOTOR_CTRL_CORE0 = Some(controller::motor_controller::MotorController::new(
            board.motor,
            board.current_sensor,
        ));

        SYSTEM_CTRL = Some(controller::SystemController::new(
            create_default_feature_set(),
            TELEMETRY_CHANNEL.sender(),
            crate::get_boot_reason(),
        ));
    }
}

/// Global pointer to the active MotorController on Core 1 (populated during startup).
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static MOTOR_CTRL_CORE1: platform::OnceLock<*mut ()> = platform::OnceLock::new();

/// Global pointer to the active North SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static SENSOR_CTRL_NORTH_CORE1: platform::OnceLock<*mut ()> = platform::OnceLock::new();

/// Global pointer to the active East SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static SENSOR_CTRL_EAST_CORE1: platform::OnceLock<*mut ()> = platform::OnceLock::new();

/// Global pointer to the active West SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static SENSOR_CTRL_WEST_CORE1: platform::OnceLock<*mut ()> = platform::OnceLock::new();

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
    motor: MotorType,
    sensors: (SensorType, SensorType, SensorType),
) -> embassy_executor::Spawner {
    board::cat_detector::boot_core1(core1);

    let spawner_c1 = spawner_core1();
    spawner_c1
        .spawn(bootstrap_core1_task(spawner_c1, motor, sensors))
        .unwrap();

    spawner_c1
}

/// Boots Core 1 peripherals and controllers.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
#[cfg_attr(target_arch = "arm", link_section = ".data.core1_func")]
pub async fn bootstrap_core1_task(
    spawner: embassy_executor::Spawner,
    motor: MotorType,
    sensors: (SensorType, SensorType, SensorType),
) {
    // Configure MPU Stack Guard for Core 1
    let guard_addr =
        board::cat_detector::CORE1_STACK_BOTTOM.load(core::sync::atomic::Ordering::Acquire);
    if guard_addr != 0 {
        platform::core_monitor::configure_mpu_stack_guard(guard_addr);
    }

    // Initialize the core monitor for Core 1
    platform::core_monitor::init_core(
        Some(spawner),
        platform::core_monitor::CpuId::Core1,
        crate::CORE_MONITOR_TIMEOUT_MS,
        crate::CORE_MONITOR_WARN_PCT,
        true,
    );

    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Motor(motor, MOTOR_CHANNEL) register: MOTOR_CTRL_CORE1, generics: (crate::MotorDevice, crate::CurrentSensorDevice),
            Sensor(sensors.0, SENSOR_NORTH_CHANNEL) register: SENSOR_CTRL_NORTH_CORE1, generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.1, SENSOR_EAST_CHANNEL) register: SENSOR_CTRL_EAST_CORE1, generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.2, SENSOR_WEST_CHANNEL) register: SENSOR_CTRL_WEST_CORE1, generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
        }
    }
}

/// The default inactivity timeout in seconds before transitioning to Sleep.
pub const INACTIVITY_TIMEOUT_SECONDS: u32 = 30;
/// The critical state of charge threshold under which battery is considered critical.
pub const CRITICAL_BATTERY_SOC_THRESHOLD: u8 = 10;
/// The state of charge threshold under which battery is considered low.
pub const LOW_BATTERY_SOC_THRESHOLD: u8 = 20;
/// The state of charge threshold under which battery is considered medium.
pub const MID_BATTERY_SOC_THRESHOLD: u8 = 21;
/// The state of charge threshold under which battery is considered high.
pub const HIGH_BATTERY_SOC_THRESHOLD: u8 = 80;
/// The state of charge hysteresis to prevent rapid toggling around thresholds.
pub const BATTERY_SOC_HYSTERESIS: u8 = 2;

/// Temperature threshold in milli-Celsius where the system starts warning/throttling.
pub const OVERHEATING_TEMP_THRESHOLD_MC: i32 = 45000;
/// Temperature threshold in milli-Celsius where the system goes to PowerDown.
pub const CRITICAL_TEMP_THRESHOLD_MC: i32 = 60000;

const _: () = {
    assert!(
        CRITICAL_BATTERY_SOC_THRESHOLD > 0,
        "Critical battery threshold must be nonzero"
    );
};

platform::assert_ascending!(
    CRITICAL_BATTERY_SOC_THRESHOLD,
    LOW_BATTERY_SOC_THRESHOLD,
    MID_BATTERY_SOC_THRESHOLD,
    HIGH_BATTERY_SOC_THRESHOLD,
);

platform::assert_ascending!(OVERHEATING_TEMP_THRESHOLD_MC, CRITICAL_TEMP_THRESHOLD_MC,);

/// Feature set for the Cat Detector app that implements SystemFeatureSet.
#[allow(clippy::type_complexity)]
pub struct CatDetectorFeatureSet<
    MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const N: usize = 16,
> {
    /// Tuple of active system features
    pub features: (
        controller::MotorFeatureConfig<MutexRaw>,
        controller::BatteryFeatureConfig<MutexRaw>,
        controller::ProximityFeatureConfig<MutexRaw>,
        controller::LedFeatureConfig<MutexRaw>,
        controller::ThermalFeatureConfig<MutexRaw>,
    ),
}

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    controller::SystemFeatureSet<MutexRaw, N> for CatDetectorFeatureSet<MutexRaw, N>
{
    type Features = (
        controller::MotorFeatureConfig<MutexRaw>,
        controller::BatteryFeatureConfig<MutexRaw>,
        controller::ProximityFeatureConfig<MutexRaw>,
        controller::LedFeatureConfig<MutexRaw>,
        controller::ThermalFeatureConfig<MutexRaw>,
    );

    fn features(&self) -> &Self::Features {
        &self.features
    }

    fn inactivity_timeout_seconds(&self) -> u32 {
        INACTIVITY_TIMEOUT_SECONDS
    }
}

/// Shared command channel for the Motor Controller.
pub static MOTOR_CHANNEL: controller::MotorChannel<MutexRaw, 4> = controller::MotorChannel::new();
/// Shared command channel for the System Controller.
pub static SYSTEM_CHANNEL: controller::SystemChannel<MutexRaw, 16> =
    controller::SystemChannel::new();
/// Shared command channel for the North Sensor Controller.
pub static SENSOR_NORTH_CHANNEL: controller::SensorChannel<MutexRaw, 4> =
    controller::SensorChannel::new();
/// Shared command channel for the East Sensor Controller.
pub static SENSOR_EAST_CHANNEL: controller::SensorChannel<MutexRaw, 4> =
    controller::SensorChannel::new();
/// Shared command channel for the West Sensor Controller.
pub static SENSOR_WEST_CHANNEL: controller::SensorChannel<MutexRaw, 4> =
    controller::SensorChannel::new();
/// Shared command channel for the Thermal Controller.
pub static THERMAL_CHANNEL: controller::ThermalChannel<MutexRaw, 4> =
    controller::ThermalChannel::new();
/// Shared status update channel from the Thermal Controller to the System Controller.
pub static THERMAL_ACTION_CHANNEL: embassy_sync::channel::Channel<
    MutexRaw,
    controller::types::ThermalUpdateAction,
    4,
> = embassy_sync::channel::Channel::new();
/// Shared command channel for the Battery Controller.
pub static BATTERY_CHANNEL: controller::BatteryChannel<MutexRaw, 4> =
    controller::BatteryChannel::new();
/// Shared command channel for the System LED status updates.
pub static LED_CHANNEL: controller::LedChannel<MutexRaw, 4> = controller::LedChannel::new();
/// Shared command channel for telemetry records.
pub static TELEMETRY_CHANNEL: controller::TelemetryChannel<
    MutexRaw,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = controller::TelemetryChannel::new();

/// Shared command channel for filesystem operations.
pub static FILESYSTEM_CHANNEL: controller::FilesystemChannel<MutexRaw, 16> =
    controller::FilesystemChannel::new();
/// Type alias for the Cat Detector System Controller.
pub type SystemControllerType =
    controller::SystemController<MutexRaw, CatDetectorFeatureSet<MutexRaw, 16>, 16>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// The concrete flash type used for the filesystem partition in production.
pub type FlashDeviceType = platform::flash::TargetFlash<{ FLASH_SIZE }>;

/// Creates the standard CatDetectorFeatureSet configured with the application's actual channels.
pub fn create_default_feature_set(
) -> CatDetectorFeatureSet<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 16> {
    CatDetectorFeatureSet {
        features: (
            controller::MotorFeatureConfig::new(
                Some(MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            controller::BatteryFeatureConfig::new(
                Some(BATTERY_CHANNEL.sender()),
                platform::BatteryManager::new(
                    CRITICAL_BATTERY_SOC_THRESHOLD,
                    BATTERY_SOC_HYSTERESIS,
                    LOW_BATTERY_SOC_THRESHOLD,
                    MID_BATTERY_SOC_THRESHOLD,
                    HIGH_BATTERY_SOC_THRESHOLD,
                ),
            ),
            controller::ProximityFeatureConfig::new(
                &[
                    SENSOR_NORTH_CHANNEL.sender(),
                    SENSOR_EAST_CHANNEL.sender(),
                    SENSOR_WEST_CHANNEL.sender(),
                ],
                DEFAULT_PRESS_THRESHOLD_MM,
                DEFAULT_NEAR_THRESHOLD_MM,
                DEFAULT_WAKE_THRESHOLD_MM,
                controller::GestureAction::TogglePower,
                Some(TELEMETRY_CHANNEL.sender()),
            ),
            controller::LedFeatureConfig::new(Some(LED_CHANNEL.sender())),
            controller::ThermalFeatureConfig::new_with_thresholds(
                Some(THERMAL_CHANNEL.sender()),
                OVERHEATING_TEMP_THRESHOLD_MC,
                CRITICAL_TEMP_THRESHOLD_MC,
            ),
        ),
    }
}

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
impl controller::BlockingMotorReader for MockMotorController {
    fn read_current_ma_blocking(&mut self) -> Result<i32, model::types::PeripheralError> {
        Ok(150)
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl controller::BlockingMotorWriter for MockMotorController {
    fn set_motor_speed(&mut self, _speed: i8) -> Result<(), model::types::PeripheralError> {
        Ok(())
    }
    fn set_motor_speed_rpm(&mut self, _rpm: i32) -> Result<(), model::types::PeripheralError> {
        Ok(())
    }
    fn stop_motor_blocking(&mut self) -> Result<(), model::types::PeripheralError> {
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
impl controller::BlockingProximityReader for MockSensorController {
    async fn read_distance_blocking(
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
    { MAX_RECORDS },
    { model::telemetry::BUFFER_SIZE },
    platform::flash::SharedFlashMutex<platform::BlockingAsyncFlash<FlashDevice>>,
>;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
/// Concrete type for the telemetry controller (mock on host).
pub type TelemetryControllerType = ();

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of TelemetryController.
pub static mut TELEMETRY_CTRL: Option<TelemetryControllerType> = None;

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

/// Global static storage for LedController and SystemController inside the shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub static mut SHELL_SYSTEM_CTRL: Option<SystemControllerType> = None;

/// Setup shell pointers and resolve device handles.
pub async fn get_shell_pointers(
    fs_buffer: &'static mut [u8],
) -> controller::shell_controller::ShellControllerPointers<'static, CatDetectorShellConfig> {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        // 1. Resolve temp sensor pointer
        let temp_sensor_ptr = {
            let mut guard = SHARED_TEMP_SENSOR.lock().await;
            if let Some(ref mut sensor) = guard.0 {
                sensor as *mut Rp2040TempSensor
            } else {
                core::ptr::null_mut()
            }
        };

        // 2. Resolve I2C
        let board_i2c_ptr = {
            let mut guard = SHARED_I2C.lock().await;
            if let Some(ref mut i2c) = guard.i2c {
                i2c as *mut _ as *mut _
            } else {
                core::ptr::null_mut()
            }
        };

        // 3. Resolve Core 1 Motor & Sensors
        let core1_motor_ctrl = get_motor_ctrl_core1().await;
        let board_motor_ptr = &mut core1_motor_ctrl.motor as *mut _;
        let sensor_n = get_sensor_ctrl_north_core1().await as *mut _;
        let sensor_e = get_sensor_ctrl_east_core1().await as *mut _;
        let sensor_w = get_sensor_ctrl_west_core1().await as *mut _;
        let motor_c1 = get_motor_ctrl_core1().await as *mut _;

        // 4. Resolve Flash
        let panic_flash_ptr = unsafe { &mut *core::ptr::addr_of_mut!(PANIC_FLASH) }
            .as_mut()
            .unwrap() as *mut _;

        // Static mut arrays for device registration
        static mut THERMALS: [controller::NamedDevice<ThermalControllerType>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];
        static mut BATTERIES: [controller::NamedDevice<BatteryControllerType>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];
        static mut I2C_BUSES: [controller::NamedDevice<I2cBus>; 1] = [controller::NamedDevice {
            name: "default",
            device: core::ptr::null_mut(),
        }];
        static mut MOTORS: [controller::NamedDevice<MotorDevice>; 1] = [controller::NamedDevice {
            name: "default",
            device: core::ptr::null_mut(),
        }];
        static mut FLASH_PARTITIONS: [controller::NamedPartition<FlashDevice>; 2] = [
            controller::NamedPartition {
                name: "default",
                partition: controller::FlashPartition {
                    flash_ptr: core::ptr::null_mut(),
                    start_address: 0,
                    end_address: 0,
                },
                kind: controller::PartitionKind::Map,
            },
            controller::NamedPartition {
                name: "telemetry",
                partition: controller::FlashPartition {
                    flash_ptr: core::ptr::null_mut(),
                    start_address: 0,
                    end_address: 0,
                },
                kind: controller::PartitionKind::Queue,
            },
        ];
        static mut TEMP_SENSORS_STORAGE: [controller::NamedDevice<Rp2040TempSensor>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];
        static mut SENSORS: [controller::NamedDevice<SensorControllerType>; 3] = [
            controller::NamedDevice {
                name: "north",
                device: core::ptr::null_mut(),
            },
            controller::NamedDevice {
                name: "east",
                device: core::ptr::null_mut(),
            },
            controller::NamedDevice {
                name: "west",
                device: core::ptr::null_mut(),
            },
        ];
        static mut MOTOR_CTRLS: [controller::NamedDevice<MotorControllerType>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];
        static mut LEDS: [controller::NamedDevice<LedControllerType>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];
        static mut SYSTEM_CTRLS: [controller::NamedDevice<SystemControllerType>; 1] =
            [controller::NamedDevice {
                name: "default",
                device: core::ptr::null_mut(),
            }];

        // Populate and construct under a single unsafe block
        unsafe {
            THERMALS[0].device = get_thermal_ctrl();
            BATTERIES[0].device = get_battery_ctrl();
            I2C_BUSES[0].device = board_i2c_ptr;
            MOTORS[0].device = board_motor_ptr;

            FLASH_PARTITIONS[0].partition = controller::FlashPartition {
                flash_ptr: panic_flash_ptr,
                start_address: FS_PARTITION_START,
                end_address: FS_PARTITION_END,
            };
            FLASH_PARTITIONS[1].partition = controller::FlashPartition {
                flash_ptr: panic_flash_ptr,
                start_address: TELEMETRY_PARTITION_START,
                end_address: TELEMETRY_PARTITION_END,
            };

            let temp_sensors: &[controller::NamedDevice<Rp2040TempSensor>] =
                if !temp_sensor_ptr.is_null() {
                    TEMP_SENSORS_STORAGE[0].device = temp_sensor_ptr;
                    &*core::ptr::addr_of!(TEMP_SENSORS_STORAGE)
                } else {
                    &[]
                };

            SENSORS[0].device = sensor_n;
            SENSORS[1].device = sensor_e;
            SENSORS[2].device = sensor_w;

            MOTOR_CTRLS[0].device = motor_c1;
            LEDS[0].device = get_led_ctrl();

            if (*core::ptr::addr_of!(SHELL_SYSTEM_CTRL)).is_none() {
                let feature_set = create_default_feature_set();
                *core::ptr::addr_of_mut!(SHELL_SYSTEM_CTRL) =
                    Some(controller::SystemController::new(
                        feature_set,
                        TELEMETRY_CHANNEL.sender(),
                        model::types::BootReason::Unknown,
                    ));
            }
            SYSTEM_CTRLS[0].device = (*core::ptr::addr_of_mut!(SHELL_SYSTEM_CTRL))
                .as_mut()
                .unwrap();

            controller::shell_controller::ShellControllerPointers::<CatDetectorShellConfig> {
                i2c_buses: &*core::ptr::addr_of!(I2C_BUSES),
                motors: &*core::ptr::addr_of!(MOTORS),
                flash_partitions: &*core::ptr::addr_of!(FLASH_PARTITIONS),
                temp_sensors,
                sensors: &*core::ptr::addr_of!(SENSORS),
                motor_ctrls: &*core::ptr::addr_of!(MOTOR_CTRLS),
                thermals: &*core::ptr::addr_of!(THERMALS),
                batteries: &*core::ptr::addr_of!(BATTERIES),
                system_ctrls: &*core::ptr::addr_of!(SYSTEM_CTRLS),
                leds: &*core::ptr::addr_of!(LEDS),
                fs_buffer,
            }
        }
    }

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        let _ = fs_buffer;
        controller::shell_controller::ShellControllerPointers::<CatDetectorShellConfig>::default()
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Configured layout and resources for filesystem partition storage and crash logs.
pub struct FilesystemStorageConfig {
    /// Flash device reference.
    pub flash: &'static mut FlashDevice,
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
            { FLASH_SIZE },
        >::new_blocking(fs_flash);
        platform::BlockingAsyncFlash(raw_flash)
    }

    /// Initialize the panic handler using this filesystem storage layout.
    pub fn init_panic_handler(&self) {
        // Safe alias of the static buffer, flash, and partition for the panic handler.
        // This is safe because the panic handler only runs after the application halts/panics.
        unsafe {
            let panic_flash = &mut *core::ptr::addr_of_mut!(PANIC_FLASH);
            let fs_buf_panic = &mut *core::ptr::addr_of_mut!(FS_BUF);
            let partition = controller::MapFilesystem(FS_PARTITION_START..FS_PARTITION_END);
            init_panic_handler(
                panic_flash.as_mut().unwrap(),
                partition,
                fs_buf_panic,
                MAX_CRASH_LOGS,
            );
        }
    }

    /// Safely set the shared FLASH mutex.
    pub fn set_flash_mutex(&self, mutex: FlashMutexType) -> &'static FlashMutexType {
        unsafe {
            let ptr = &mut *core::ptr::addr_of_mut!(FLASH_MUTEX);
            *ptr = Some(mutex);
            ptr.as_ref().unwrap()
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Get the unified filesystem storage configuration.
pub fn get_filesystem_config() -> FilesystemStorageConfig {
    FilesystemStorageConfig {
        flash: unsafe { &mut *core::ptr::addr_of_mut!(PANIC_FLASH) }
            .as_mut()
            .unwrap(),
        partition: controller::MapFilesystem(FS_PARTITION_START..FS_PARTITION_END),
        buffer: unsafe { &mut *core::ptr::addr_of_mut!(FS_BUF) },
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
/// Collection of all Core 0 controllers.
pub struct Core0Controllers {
    /// The thermal controller.
    pub thermal: ThermalControllerType,
    /// The battery controller.
    pub battery: BatteryControllerType,
    /// The LED controller.
    pub led: LedControllerType,
    /// The motor controller.
    pub motor: MotorControllerType,
    /// The North proximity sensor controller.
    pub sensor_north: SensorControllerType,
    /// The East proximity sensor controller.
    pub sensor_east: SensorControllerType,
    /// The West proximity sensor controller.
    pub sensor_west: SensorControllerType,
    /// The system controller.
    pub system: SystemControllerType,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely extract all pre-initialized Core 0 controllers.
pub fn take_core0_controllers() -> Core0Controllers {
    Core0Controllers {
        thermal: take_static_mut!(THERMAL_CTRL),
        battery: take_static_mut!(BATTERY_CTRL),
        led: take_static_mut!(LED_CTRL),
        motor: take_static_mut!(MOTOR_CTRL_CORE0),
        sensor_north: take_static_mut!(SENSOR_CTRL_NORTH_CORE0),
        sensor_east: take_static_mut!(SENSOR_CTRL_EAST_CORE0),
        sensor_west: take_static_mut!(SENSOR_CTRL_WEST_CORE0),
        system: take_static_mut!(SYSTEM_CTRL),
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely set and retrieve the static TelemetryController pointer.
pub fn set_telemetry_ctrl(ctrl: TelemetryControllerType) -> &'static mut TelemetryControllerType {
    unsafe {
        let ptr = &mut *core::ptr::addr_of_mut!(TELEMETRY_CTRL);
        *ptr = Some(ctrl);
        ptr.as_mut().unwrap()
    }
}

define_static_mut_getters! {
    get_thermal_ctrl, THERMAL_CTRL, ThermalControllerType;
    get_battery_ctrl, BATTERY_CTRL, BatteryControllerType;
    get_led_ctrl, LED_CTRL, LedControllerType;
    get_system_ctrl, SYSTEM_CTRL, SystemControllerType;
    get_motor_ctrl_core0, MOTOR_CTRL_CORE0, MotorControllerType;
    get_sensor_ctrl_north_core0, SENSOR_CTRL_NORTH_CORE0, SensorControllerType;
    get_sensor_ctrl_east_core0, SENSOR_CTRL_EAST_CORE0, SensorControllerType;
    get_sensor_ctrl_west_core0, SENSOR_CTRL_WEST_CORE0, SensorControllerType;
}

define_core1_getters! {
    get_motor_ctrl_core1, MOTOR_CTRL_CORE1, MotorControllerType;
    get_sensor_ctrl_north_core1, SENSOR_CTRL_NORTH_CORE1, SensorControllerType;
    get_sensor_ctrl_east_core1, SENSOR_CTRL_EAST_CORE1, SensorControllerType;
    get_sensor_ctrl_west_core1, SENSOR_CTRL_WEST_CORE1, SensorControllerType;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Type alias for shared flash mutex
pub type FlashMutexType = embassy_sync::mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    platform::BlockingAsyncFlash<FlashDevice>,
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Shared static FLASH mutex
static mut FLASH_MUTEX: Option<FlashMutexType> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Safely run the executor loop for the specified core.
pub fn run_executor(cpu_id: platform::types::CpuId) -> ! {
    unsafe { Board::run_executor(cpu_id) }
}
