//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    app::{
        BATTERY_CHANNEL, FILESYSTEM_CHANNEL, LED_CHANNEL, SYSTEM_CHANNEL, TELEMETRY_CHANNEL,
        THERMAL_ACTION_CHANNEL, THERMAL_CHANNEL,
    },
    cat_detector as app,
    controller::telemetry_controller::TelemetryController,
    embassy_executor::Spawner,
    platform::core_monitor,
    platform::flash,
    platform::types::QueueFilesystem,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[platform::tracing::instrument(name = "boot", level = "info", skip(spawner, p))]
#[embassy_executor::task]
async fn bootstrap_task(spawner: Spawner, p: embassy_rp::Peripherals) {
    let mut board = app::Board::init(p).await;
    let peripherals = board.take_peripherals();
    if app::BOARD.set(board).is_err() {
        panic!("Failed to set BOARD static");
    }
    let board_ref = app::BOARD.get().unwrap();
    let mut controllers = app::init_controllers(board_ref, peripherals).await;

    // Route defmt logs to RTT
    platform::defmt_logger::DefmtLogger::set_writer(&platform::defmt_logger::DEFAULT_RTT_WRITER);
    defmt::info!("Booting Cat Detector App...");

    let fs_cfg = app::get_filesystem_config();

    core_monitor::init_core(
        Some(spawner),
        core_monitor::CpuId::Core0,
        app::CORE_MONITOR_TIMEOUT_MS,
        app::CORE_MONITOR_WARN_PCT,
        true,
    );

    // Initialize the flash device using the stolen FLASH peripheral
    let mut async_flash = fs_cfg.steal_flash();

    use model::calibration::{Calibration, MotorCalibration, Vl53l0xCalibration};

    let mut cal_buf = [0u8; 128];
    let proximity_cal = flash::read_calibration_direct_blocking::<_, Vl53l0xCalibration>(
        &mut async_flash,
        fs_cfg.partition.clone(),
        fs_cfg.buffer,
        Vl53l0xCalibration::CALIBRATION_FILE_NAME,
        &mut cal_buf,
    )
    .unwrap_or_default();

    let mut motor_cal_buf = [0u8; 128];
    let motor_cal = flash::read_calibration_direct_blocking::<_, MotorCalibration>(
        &mut async_flash,
        fs_cfg.partition.clone(),
        fs_cfg.buffer,
        MotorCalibration::CALIBRATION_FILE_NAME,
        &mut motor_cal_buf,
    );

    let flash_mutex = fs_cfg.set_flash_mutex(embassy_sync::mutex::Mutex::new(async_flash));

    let fs_flash_mutex_ref = flash::SharedFlashMutex::new(flash_mutex);
    let profiling_flash =
        controller::filesystem_controller::ProfilingFlash::new(fs_flash_mutex_ref);
    let mut fs_controller = controller::filesystem_controller::FilesystemController::new(
        profiling_flash,
        fs_cfg.partition.clone(),
        fs_cfg.buffer,
    );
    fs_controller.set_telemetry(app::TELEMETRY_CHANNEL.sender());

    // Verify and repair/reformat the filesystem if it is corrupted
    let _ = fs_controller.verify_and_repair().await;

    let client =
        controller::filesystem_controller::FilesystemClient::new(app::FILESYSTEM_CHANNEL.sender());

    if let Some(cal) = motor_cal {
        controllers.core1.motor.set_calibration(&cal);
    }

    controllers
        .core1
        .sensor_north
        .set_calibration(&proximity_cal);
    controllers
        .core1
        .sensor_east
        .set_calibration(&proximity_cal);
    controllers
        .core1
        .sensor_west
        .set_calibration(&proximity_cal);

    let telemetry_flash_mutex_ref = flash::SharedFlashMutex::new(flash_mutex);
    let telemetry_ctrl = TelemetryController::new(
        telemetry_flash_mutex_ref,
        QueueFilesystem(app::TELEMETRY_PARTITION_START..app::TELEMETRY_PARTITION_END),
        client,
    );

    let core1 = app::steal_core1_peripheral();
    let sensors = (
        controllers.core1.sensor_north,
        controllers.core1.sensor_east,
        controllers.core1.sensor_west,
    );
    app::bootstrap_core1(core1, controllers.core1.motor, sensors);

    // Spawn tasks on Core 0
    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Thermal(controllers.core0.thermal, THERMAL_CHANNEL), generics: (app::TempSensorDevice),
            Battery(controllers.core0.battery, BATTERY_CHANNEL), generics: (app::BatteryDevice, app::ChargerDevice, app::AlertPinType),
            Led(controllers.core0.led, LED_CHANNEL), generics: (app::LedDevice),
            System(controllers.core0.system, SYSTEM_CHANNEL, THERMAL_ACTION_CHANNEL), generics: (app::SystemControllerType),
            Filesystem(fs_controller, FILESYSTEM_CHANNEL), generics: (app::FlashDeviceType),
            Telemetry(telemetry_ctrl, TELEMETRY_CHANNEL), generics: ({ controller::telemetry_controller::CHANNEL_CAPACITY }, platform::flash::SharedFlashMutex<platform::BlockingAsyncFlash<app::FlashDevice>>),
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    let spawner_c0 = app::spawner_core0();

    spawner_c0.spawn(bootstrap_task(spawner_c0, p)).unwrap();
    app::run_executor(platform::types::CpuId::Core0);
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
