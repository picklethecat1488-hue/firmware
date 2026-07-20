//! Board configuration library for the Cat Detector project.
//!
//! Defines the single source of truth for pin assignments and helper
//! initialization functions for sharing hardware setup between the main
//! controller and bringup shell binaries.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Pump IA pin (GPIO 14)
pub const PUMP_PIN_IA: u32 = 14;
/// Pump IB pin (GPIO 15)
pub const PUMP_PIN_IB: u32 = 15;
/// I2C SDA pin (GPIO 4)
pub const I2C_SDA_PIN: u32 = 4;
/// I2C SCL pin (GPIO 5)
pub const I2C_SCL_PIN: u32 = 5;
/// UART TX pin (GPIO 0)
pub const UART_TX_PIN: u32 = 0;
/// UART RX pin (GPIO 1)
pub const UART_RX_PIN: u32 = 1;

/// ToF Sensor 1 (North) XSHUT pin (GPIO 2)
pub const TOF_NORTH_XSHUT_PIN: u32 = 2;
/// ToF Sensor 2 (East) XSHUT pin (GPIO 3)
pub const TOF_EAST_XSHUT_PIN: u32 = 3;
/// ToF Sensor 3 (West) XSHUT pin (GPIO 6)
pub const TOF_WEST_XSHUT_PIN: u32 = 6;

/// ToF Sensor 1 (North) Interrupt pin (GPIO 7)
pub const TOF_NORTH_INT_PIN: u32 = 7;
/// ToF Sensor 2 (East) Interrupt pin (GPIO 8)
pub const TOF_EAST_INT_PIN: u32 = 8;
/// ToF Sensor 3 (West) Interrupt pin (GPIO 9)
pub const TOF_WEST_INT_PIN: u32 = 9;

/// Fuel Gauge Interrupt/Alert pin (GPIO 10)
pub const FUEL_GAUGE_INT_PIN: u32 = 10;

/// The default wake threshold in millimeters under which target presence is detected.
pub const DEFAULT_WAKE_THRESHOLD_MM: u16 = 300;

/// The default press threshold in millimeters under which gesture button presses are detected.
pub const DEFAULT_PRESS_THRESHOLD_MM: u16 = 20;

/// Charger Status 1 (S1 / STAT1 / FAULT) pin (GPIO 12)
pub const CHARGER_S1_PIN: u32 = 12;
/// Charger Status 2 (S2 / STAT2 / CHG) pin (GPIO 13)
pub const CHARGER_S2_PIN: u32 = 13;

/// Start address of the filesystem storage partition in flash (offset from start of flash).
pub const STORAGE_PARTITION_START: u32 = 0x1C_0000; // 1.75 MB
/// End address of the filesystem storage partition in flash (2.00 MB limit).
pub const STORAGE_PARTITION_END: u32 = 0x20_0000; // 2.00 MB

/// Total number of telemetry chunks
pub const NUM_CHUNKS: usize = 50;
/// Total maximum number of records stored
pub const MAX_RECORDS: usize = NUM_CHUNKS * model::telemetry::CHUNK_SIZE;
/// Maximum number of rolling crash logs (modulo limit)
pub const MAX_CRASH_LOGS: u32 = 10;
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
pub static SHARED_I2C: embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<firmware_lib::i2c::SafeI2c>,
> = embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(firmware_lib::i2c::SafeI2c(
    None,
)));

/// RawMutex type used by controllers.
pub type MutexRaw = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global temperature sensor mutex.
pub static SHARED_TEMP_SENSOR: embassy_sync::mutex::Mutex<MutexRaw, TempSensorDevice> =
    embassy_sync::mutex::Mutex::new(SafeRp2040TempSensor(None));

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global battery/fuel gauge mutex.
pub static SHARED_BATTERY: embassy_sync::mutex::Mutex<MutexRaw, BatteryDevice> =
    embassy_sync::mutex::Mutex::new(BatteryDevice::new(
        firmware_lib::i2c::SharedI2cWrapper::new(&SHARED_I2C),
    ));

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global battery charger mutex.
pub static SHARED_CHARGER: embassy_sync::mutex::Mutex<MutexRaw, ChargerDevice> =
    embassy_sync::mutex::Mutex::new(SafeBq25185(None));

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
        SystemCommand,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the LedController.
pub static mut LED_CTRL: Option<controller::led_controller::LedController<LedDevice>> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global instance of the North SensorController.
pub static mut SENSOR_CTRL_NORTH: Option<
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
pub static mut SENSOR_CTRL_EAST: Option<
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
pub static mut SENSOR_CTRL_WEST: Option<
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
pub static mut MOTOR_CTRL: Option<
    controller::motor_controller::MotorController<MotorDevice, CurrentSensorDevice>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Type alias for the blocking flash device.
pub type FlashDevice = embassy_rp::flash::Flash<
    'static,
    embassy_rp::peripherals::FLASH,
    embassy_rp::flash::Blocking,
    { crate::FLASH_SIZE },
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global panic flash peripheral reference.
pub static mut PANIC_FLASH: Option<FlashDevice> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Synchronously initializes all application subcontrollers from board hardware.
pub async fn init_controllers(board: Board<'static>) {
    let Board {
        flash,
        i2c,
        temp_sensor,
        charger,
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

    SHARED_I2C.lock(|cell| {
        cell.borrow_mut().0 = Some(i2c);
    });

    {
        let mut sensor = SHARED_TEMP_SENSOR.lock().await;
        sensor.0 = temp_sensor;
    }
    {
        let mut chg = SHARED_CHARGER.lock().await;
        chg.0 = charger;
    }

    unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(flash));

        THERMAL_CTRL = Some(
            controller::thermal_controller::ThermalController::new_with_shutdown_and_trap(
                &SHARED_TEMP_SENSOR,
                THERMAL_ACTION_CHANNEL.sender(),
            ),
        );

        let alert_wrapper = AlertPinWrapper(fuel_gauge_alert_pin);
        BATTERY_CTRL = Some(
            controller::battery_controller::BatteryController::new_with_system_and_alert(
                &SHARED_BATTERY,
                &SHARED_CHARGER,
                SYSTEM_CHANNEL.sender(),
                alert_wrapper,
            ),
        );

        LED_CTRL = Some(controller::led_controller::LedController::new(led_driver));

        SENSOR_CTRL_NORTH = Some(
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::North,
                },
                tof_north,
                SYSTEM_CHANNEL.sender(),
                ProximityPinWrapper(pin_north),
                DEFAULT_WAKE_THRESHOLD_MM,
            ),
        );

        SENSOR_CTRL_EAST = Some(
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::East,
                },
                tof_east,
                SYSTEM_CHANNEL.sender(),
                ProximityPinWrapper(pin_east),
                DEFAULT_WAKE_THRESHOLD_MM,
            ),
        );

        SENSOR_CTRL_WEST = Some(
            controller::sensor_controller::SensorController::new_with_fusion_and_interrupt(
                controller::types::SensorMetadata {
                    direction: model::types::Direction::West,
                },
                tof_west,
                SYSTEM_CHANNEL.sender(),
                ProximityPinWrapper(pin_west),
                DEFAULT_WAKE_THRESHOLD_MM,
            ),
        );

        MOTOR_CTRL = Some(controller::motor_controller::MotorController::new(
            motor,
            current_sensor,
        ));
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut CORE1_STACK: embassy_rp::multicore::Stack<4096> = embassy_rp::multicore::Stack::new();

/// Global pointer to the active MotorController on Core 1 (populated during startup).
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut MOTOR_CTRL_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active North SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_NORTH_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active East SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_EAST_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active West SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_WEST_PTR: *mut () = core::ptr::null_mut();

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
pub async fn bootstrap_core1_task(
    spawner: embassy_executor::Spawner,
    mut motor: MotorType,
    mut sensors: (SensorType, SensorType, SensorType),
) {
    // Initialize the core monitor for Core 1
    firmware_lib::core_monitor::init_core(
        Some(spawner),
        firmware_lib::core_monitor::CpuId::Core1,
        crate::CORE_MONITOR_TIMEOUT_MS,
        crate::CORE_MONITOR_WARN_PCT,
        true,
    );

    unsafe {
        let motor_ptr = core::ptr::addr_of_mut!(MOTOR_CTRL_PTR);
        *motor_ptr = &mut motor as *mut _ as *mut ();
        let north_ptr = core::ptr::addr_of_mut!(SENSOR_NORTH_PTR);
        *north_ptr = &mut sensors.0 as *mut _ as *mut ();
        let east_ptr = core::ptr::addr_of_mut!(SENSOR_EAST_PTR);
        *east_ptr = &mut sensors.1 as *mut _ as *mut ();
        let west_ptr = core::ptr::addr_of_mut!(SENSOR_WEST_PTR);
        *west_ptr = &mut sensors.2 as *mut _ as *mut ();
    }

    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Motor(motor, MOTOR_CHANNEL), generics: (crate::MotorDevice, crate::CurrentSensorDevice),
            Sensor(sensors.0, SENSOR_NORTH_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.1, SENSOR_EAST_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.2, SENSOR_WEST_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Boots Core 1 and starts the RAM executor.
pub fn boot_core1(core1: embassy_rp::peripherals::CORE1) {
    unsafe {
        firmware_lib::panic_handler::CORE1_STACK_TOP =
            core::ptr::addr_of!(CORE1_STACK) as u32 + 4096;
        crate::Board::init_executor_core1();
    }

    embassy_rp::multicore::spawn_core1(
        core1,
        unsafe {
            let ptr = core::ptr::addr_of_mut!(CORE1_STACK);
            &mut *ptr
        },
        move || unsafe {
            crate::Board::run_executor(firmware_lib::types::CpuId::Core1);
        },
    );
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Handle a panic, performing multicore checks, resets, and delegating to flash writer.
pub fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    let cpuid_val = unsafe { core::ptr::read_volatile(0xd0000000 as *const u32) };
    let (cpuid, stack_top) = match cpuid_val {
        0 => (firmware_lib::types::CpuId::Core0, 0x2004_2000),
        1 => {
            let top = unsafe { firmware_lib::panic_handler::CORE1_STACK_TOP };
            (
                firmware_lib::types::CpuId::Core1,
                if top != 0 { top } else { 0x2004_0000 },
            )
        }
        _ => loop {
            cortex_m::asm::nop();
        },
    };

    crate::handle_panic_with_sizes::<
        { crate::FLASH_SIZE },
        { crate::FLASH_START },
        { crate::FLASH_END },
        { crate::FLASH_WRITE_SIZE },
        { crate::FLASH_ERASE_SIZE },
    >(info, cpuid, stack_top);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod bsp_target;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use bsp_target::*;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod bsp_host;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use bsp_host::*;

/// System state and orchestration controller.
pub use controller::{
    BatteryFeatureConfig, LedFeatureConfig, MotorFeatureConfig, ProximityEvent,
    ProximityFeatureConfig, SystemCommand, SystemController, SystemFeatureSet,
    ThermalFeatureConfig,
};

/// The default inactivity timeout in seconds before transitioning to Sleep.
pub const INACTIVITY_TIMEOUT_SECONDS: u32 = 30;
/// The state of charge threshold under which battery is considered low.
pub const LOW_BATTERY_SOC_THRESHOLD: u8 = 20;
/// The state of charge threshold under which battery is considered medium.
pub const MID_BATTERY_SOC_THRESHOLD: u8 = 21;
/// The state of charge threshold under which battery is considered high.
pub const HIGH_BATTERY_SOC_THRESHOLD: u8 = 80;

/// The critical state of charge threshold under which battery is considered critical.
pub const CRITICAL_BATTERY_SOC_THRESHOLD: u8 = 10;
/// The state of charge hysteresis to prevent rapid toggling around thresholds.
pub const BATTERY_SOC_HYSTERESIS: u8 = 2;

/// Temperature threshold in milli-Celsius where the system starts warning/throttling.
pub const OVERHEATING_TEMP_THRESHOLD_MC: i32 = 45000;
/// Temperature threshold in milli-Celsius where the system goes to PowerDown.
pub const CRITICAL_TEMP_THRESHOLD_MC: i32 = 60000;

const _: () = {
    assert!(
        LOW_BATTERY_SOC_THRESHOLD > 0,
        "Low battery threshold be nonzero"
    );
    assert!(
        CRITICAL_BATTERY_SOC_THRESHOLD < LOW_BATTERY_SOC_THRESHOLD,
        "Critical battery threshold must be lower than the low battery threshold"
    );
    assert!(
        LOW_BATTERY_SOC_THRESHOLD < MID_BATTERY_SOC_THRESHOLD,
        "Low battery threshold must be lower than the mid battery threshold"
    );
    assert!(
        MID_BATTERY_SOC_THRESHOLD < HIGH_BATTERY_SOC_THRESHOLD,
        "Mid battery threshold must be lower than the high battery threshold"
    );
};

/// Bringup serial command and shell controller.
pub use controller::shell_controller;

pub use firmware_lib::{
    cbor,
    types::{ProjectMetadata, STACK_SCAN_LIMIT},
    BatteryUpdateAction,
};
pub use model::types::SystemStatus;

/// Feature set for the Cat Detector app that implements SystemFeatureSet.
#[allow(clippy::type_complexity)]
pub struct CatDetectorFeatureSet<
    MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static,
    const N: usize,
> {
    /// Tuple of active system features
    pub features: (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
    ),
}

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    controller::SystemFeatureSet<MutexRaw, N> for CatDetectorFeatureSet<MutexRaw, N>
{
    type Features = (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
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
pub static SYSTEM_CHANNEL: controller::SystemChannel<MutexRaw, 4> =
    controller::SystemChannel::new();
/// Shared channel for local gesture events.
pub static GESTURE_CHANNEL: firmware_lib::gesture_detector::GestureChannel<MutexRaw, 4> =
    firmware_lib::gesture_detector::GestureChannel::new();
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
    controller::SystemController<MutexRaw, CatDetectorFeatureSet<MutexRaw, 4>>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// The concrete flash type used for the filesystem partition in production.
pub type FlashDeviceType = controller::filesystem_controller::ProfilingFlash<
    firmware_lib::BlockingAsyncFlash<
        embassy_rp::flash::Flash<
            'static,
            embassy_rp::peripherals::FLASH,
            embassy_rp::flash::Blocking,
            { FLASH_SIZE },
        >,
    >,
>;

/// Re-export the telemetry module from the controller crate
pub use controller::telemetry_controller as telemetry;

/// Default core monitor timeout in milliseconds.
pub const CORE_MONITOR_TIMEOUT_MS: u32 = 10_000;

/// Default core monitor warning threshold percentage.
pub const CORE_MONITOR_WARN_PCT: u32 = 80;

/// Re-export the run_filesystem_task macro from the controller crate
pub use controller::run_filesystem_task;
/// Re-export the run_telemetry_task macro from the controller crate
pub use controller::run_telemetry_task;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use firmware_lib::panic_handler::handle_panic_with_sizes;

/// Re-export the modular panic handler initialization
pub use firmware_lib::panic_handler::init as init_panic_handler;

/// Returns the current system uptime in microseconds since boot (64-bit precision).
pub fn system_time() -> u64 {
    embassy_time::Instant::now().as_micros()
}

const METADATA_WRITER: cbor::ConstCborWriter<128> = ProjectMetadata::serialize(
    "rp2040",
    0x10000000 + STORAGE_PARTITION_START,
    STORAGE_PARTITION_END - STORAGE_PARTITION_START,
    FLASH_WRITE_SIZE as u32,
    FLASH_ERASE_SIZE as u32,
    STACK_SCAN_LIMIT,
);

/// Embedded project metadata for autodetect functionality.
#[used]
#[no_mangle]
#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".rodata.project_metadata"
)]
pub static PROJECT_METADATA: [u8; METADATA_WRITER.len] = cbor::extract_bytes(METADATA_WRITER.buf);

/// Creates the standard CatDetectorFeatureSet configured with the application's actual channels.
pub fn create_default_feature_set(
) -> CatDetectorFeatureSet<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4> {
    CatDetectorFeatureSet {
        features: (
            controller::MotorFeatureConfig::new(
                Some(MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            controller::BatteryFeatureConfig::new(
                Some(BATTERY_CHANNEL.sender()),
                firmware_lib::BatteryManager::new(
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
