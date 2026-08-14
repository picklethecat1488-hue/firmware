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
    boot_core1, handle_panic, FlashDevice, CORE1_STACK_SIZE, PANIC_FLASH, SHARED_BATTERY,
    SHARED_CHARGER, SHARED_I2C, SHARED_TEMP_SENSOR,
};

pub use controller::{
    run_filesystem_task, run_telemetry_task, shell_controller, telemetry_controller as telemetry,
    BatteryFeatureConfig, FilesystemChannel, LedFeatureConfig, MotorFeatureConfig, ProximityEvent,
    ProximityFeatureConfig, SystemCommand, SystemController, SystemFeatureSet, TelemetryChannel,
    ThermalFeatureConfig,
};
pub use model::types::SystemStatus;
pub use platform::BatteryUpdateAction;

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
    let Board {
        flash,
        temp_sensor,
        fuel_gauge_alert_pin,
        led_driver,
        tof_north,
        pin_north,
        tof_east,
        pin_east,
        tof_west,
        pin_west,
        motor,
        current_sensor,
        ..
    } = board;

    {
        let mut sensor = SHARED_TEMP_SENSOR.lock().await;
        sensor.0 = temp_sensor;
    }

    unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(flash));

        THERMAL_CTRL = Some(
            controller::thermal_controller::ThermalController::new_with_shutdown_and_trap(
                &SHARED_TEMP_SENSOR,
                THERMAL_ACTION_CHANNEL.sender(),
            ),
        );

        let alert_wrapper = board::cat_detector::AlertPinWrapper(fuel_gauge_alert_pin);
        BATTERY_CTRL = Some(
            controller::battery_controller::BatteryController::new_with_system_and_alert(
                &SHARED_BATTERY,
                &SHARED_CHARGER,
                SYSTEM_CHANNEL.sender(),
                alert_wrapper,
            ),
        );

        LED_CTRL = Some(controller::led_controller::LedController::new(led_driver));

        let mut north =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::North,
                },
                tof_north,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(pin_north),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        north.bind_command_tx(SENSOR_NORTH_CHANNEL.sender());
        SENSOR_CTRL_NORTH_CORE0 = Some(north);

        let mut east =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::East,
                },
                tof_east,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(pin_east),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        east.bind_command_tx(SENSOR_EAST_CHANNEL.sender());
        SENSOR_CTRL_EAST_CORE0 = Some(east);

        let mut west =
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::West,
                },
                tof_west,
                SYSTEM_CHANNEL.sender(),
                board::cat_detector::ProximityPinWrapper(pin_west),
                DEFAULT_WAKE_THRESHOLD_MM,
            );
        west.bind_command_tx(SENSOR_WEST_CHANNEL.sender());
        SENSOR_CTRL_WEST_CORE0 = Some(west);

        MOTOR_CTRL_CORE0 = Some(controller::motor_controller::MotorController::new(
            motor,
            current_sensor,
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
