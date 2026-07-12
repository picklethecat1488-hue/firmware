//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    app::{
        BATTERY_CHANNEL, FILESYSTEM_CHANNEL, GESTURE_CHANNEL, LED_CHANNEL, MOTOR_CHANNEL,
        SENSOR_EAST_CHANNEL, SENSOR_NORTH_CHANNEL, SENSOR_WEST_CHANNEL, SYSTEM_CHANNEL,
        TELEMETRY_CHANNEL, THERMAL_CHANNEL,
    },
    cat_detector as app,
    controller::{
        telemetry_controller::TelemetryController, BatteryFeatureConfig, GestureAction,
        LedFeatureConfig, MotorFeatureConfig, ProximityFeatureConfig, SystemController,
        ThermalFeatureConfig,
    },
    embassy_executor::Spawner,
    firmware_lib::BatteryManager,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic_with_sizes::<
        { app::FLASH_SIZE },
        { app::STACK_TOP },
        { app::FLASH_START },
        { app::FLASH_END },
        { app::FLASH_WRITE_SIZE },
        { app::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure hardware stack guard using Cortex-M MPU
    app::configure_mpu_stack_guard();

    // Initialize board peripherals and subcontrollers
    let board = app::Board::init(p);
    app::init_controllers(board).await;

    // Route defmt logs to RTT
    firmware_lib::defmt_logger::DefmtLogger::set_writer(
        &firmware_lib::defmt_logger::DEFAULT_RTT_WRITER,
    );
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    defmt::info!("Booting Cat Detector App...");

    // Initialize the modular panic handler
    // Declare statically to avoid stack allocation and stack overflow
    static mut FS_BUF: [u8; 4096] = [0u8; 4096];

    let panic_flash = unsafe { app::PANIC_FLASH.as_mut().unwrap() };
    // Obtain separate static mut references for the panic handler and filesystem controller.
    // This is safe because the panic handler only runs after the application halts/panics.
    let fs_buf_panic = unsafe { &mut *core::ptr::addr_of_mut!(FS_BUF) };
    let fs_buf_controller = unsafe { &mut *core::ptr::addr_of_mut!(FS_BUF) };

    app::init_panic_handler(
        panic_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf_panic,
    );

    // Initialize the FilesystemController using stolen FLASH peripheral (safe because panic handler only reads/writes on panic)
    let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
    let raw_flash = embassy_rp::flash::Flash::<
        _,
        embassy_rp::flash::Blocking,
        { app::FLASH_SIZE },
    >::new_blocking(fs_flash);
    let async_flash = firmware_lib::BlockingAsyncFlash(raw_flash);
    let profiling_flash = controller::filesystem_controller::ProfilingFlash::new(async_flash);
    let mut fs_controller = controller::filesystem_controller::FilesystemController::new(
        profiling_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf_controller,
    );
    fs_controller.set_telemetry(app::TELEMETRY_CHANNEL.sender());

    // Verify and repair/reformat the filesystem if it is corrupted
    let _ = fs_controller.verify_and_repair().await;

    // Read calibration file from flash
    let mut cal_buf = [0u8; 128];
    let proximity_cal = match fs_controller
        .read_file("vl53l0x_cal.cbor", &mut cal_buf)
        .await
    {
        Ok(Some(bytes)) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("vl53l0x calibration: {=[u8]:cbor}", bytes);
            minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes).unwrap_or_default()
        }
        _ => model::calibration::Vl53l0xCalibration::default(),
    };

    let mut motor_cal_buf = [0u8; 128];
    let motor_cal = match fs_controller
        .read_file("motor_cal.cbor", &mut motor_cal_buf)
        .await
    {
        Ok(Some(bytes)) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("motor calibration: {=[u8]:cbor}", bytes);
            minicbor::decode::<model::calibration::MotorCalibration>(bytes).ok()
        }
        _ => None,
    };

    let thermal_ctrl = unsafe { app::THERMAL_CTRL.take().unwrap() };
    let power_ctrl = unsafe { app::BATTERY_CTRL.take().unwrap() };
    let led_ctrl = unsafe { app::LED_CTRL.take().unwrap() };
    let mut controller = unsafe { app::MOTOR_CTRL.take().unwrap() };
    let mut sensor_ctrl_north = unsafe { app::SENSOR_CTRL_NORTH.take().unwrap() };
    let mut sensor_ctrl_east = unsafe { app::SENSOR_CTRL_EAST.take().unwrap() };
    let mut sensor_ctrl_west = unsafe { app::SENSOR_CTRL_WEST.take().unwrap() };

    use model::calibration::Calibration;
    use model::calibration::CalibrationType;

    if let Some(cal) = motor_cal {
        let min_ma = cal.dry_run_limit();
        let max_ma = cal.stall_limit();
        let max_rpm = cal.max_rpm.unwrap_or(0);
        let rpm_limit = cal.rpm_limit.unwrap_or(0);
        controller.set_calibration(CalibrationType::MotorCal {
            current_limits: model::calibration::TwoPointCalibration::new(min_ma, max_ma),
            max_rpm,
            rpm_limit,
        });
    }

    sensor_ctrl_north.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::North],
    ));
    sensor_ctrl_east.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::East],
    ));
    sensor_ctrl_west.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::West],
    ));

    // Initialize SystemController to coordinate all loops
    let feature_set = app::CatDetectorFeatureSet {
        features: (
            MotorFeatureConfig::new(
                Some(app::MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            BatteryFeatureConfig::new(
                Some(app::BATTERY_CHANNEL.sender()),
                BatteryManager::new(
                    app::CRITICAL_BATTERY_SOC_THRESHOLD,
                    app::BATTERY_SOC_HYSTERESIS,
                    app::LOW_BATTERY_SOC_THRESHOLD,
                    app::MID_BATTERY_SOC_THRESHOLD,
                    app::HIGH_BATTERY_SOC_THRESHOLD,
                ),
            ),
            ProximityFeatureConfig::new(
                &[
                    app::SENSOR_NORTH_CHANNEL.sender(),
                    app::SENSOR_EAST_CHANNEL.sender(),
                    app::SENSOR_WEST_CHANNEL.sender(),
                ],
                app::DEFAULT_PRESS_THRESHOLD_MM,
                app::DEFAULT_WAKE_THRESHOLD_MM,
                GestureAction::TogglePower,
                Some(app::TELEMETRY_CHANNEL.sender()),
            ),
            LedFeatureConfig::new(Some(app::LED_CHANNEL.sender())),
            ThermalFeatureConfig::new(Some(app::THERMAL_CHANNEL.sender())),
        ),
    };
    let boot_reason = app::get_boot_reason();

    let system_ctrl =
        SystemController::new(feature_set, app::TELEMETRY_CHANNEL.sender(), boot_reason);

    // Declare telemetry controller statically to avoid stack overflow on the main thread MSP stack
    static mut TELEMETRY_CTRL: Option<
        TelemetryController<1024, { model::telemetry::BUFFER_SIZE }>,
    > = None;

    let client =
        controller::filesystem_controller::FilesystemClient::new(app::FILESYSTEM_CHANNEL.sender());

    let telemetry_ctrl = unsafe {
        TELEMETRY_CTRL = Some(TelemetryController::new(client));
        TELEMETRY_CTRL.as_mut().unwrap()
    };

    // Spawn all application tasks concurrently using the unified macro
    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Thermal(thermal_ctrl, THERMAL_CHANNEL), generics: (app::TempSensorDevice, app::SystemCommand),
            Battery(power_ctrl, BATTERY_CHANNEL), generics: (app::BatteryDevice, app::ChargerDevice, app::AlertPinType, app::SystemCommand),
            Motor(controller, MOTOR_CHANNEL), generics: (app::MotorDevice, app::CurrentSensorDevice),
            Sensor(sensor_ctrl_north, SENSOR_NORTH_CHANNEL), generics: (app::ProximitySensorDevice, app::DataReadyPinType, app::SystemCommand),
            Sensor(sensor_ctrl_east, SENSOR_EAST_CHANNEL), generics: (app::ProximitySensorDevice, app::DataReadyPinType, app::SystemCommand),
            Sensor(sensor_ctrl_west, SENSOR_WEST_CHANNEL), generics: (app::ProximitySensorDevice, app::DataReadyPinType, app::SystemCommand),
            Led(led_ctrl, LED_CHANNEL), generics: (app::LedDevice),
            System(system_ctrl, SYSTEM_CHANNEL, GESTURE_CHANNEL), generics: (app::SystemControllerType),
            Filesystem(fs_controller, FILESYSTEM_CHANNEL), generics: (app::FlashDeviceType),
            Telemetry(telemetry_ctrl, TELEMETRY_CHANNEL), generics: (1024, { controller::telemetry_controller::CHANNEL_CAPACITY }),
        }
    }
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
