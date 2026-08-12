//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    app::{
        BATTERY_CHANNEL, FILESYSTEM_CHANNEL, GESTURE_CHANNEL, LED_CHANNEL, SYSTEM_CHANNEL,
        TELEMETRY_CHANNEL, THERMAL_ACTION_CHANNEL, THERMAL_CHANNEL,
    },
    cat_detector as app,
    controller::telemetry_controller::TelemetryController,
    embassy_executor::Spawner,
    platform::core_monitor,
    platform::flash,
    platform::types::{MapFilesystem, QueueFilesystem},
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
// Define shared static FLASH mutex
static mut FLASH_MUTEX: Option<
    embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        platform::BlockingAsyncFlash<app::FlashDevice>,
    >,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[platform::tracing::instrument(name = "boot", level = "info", skip(spawner, p))]
#[embassy_executor::task]
async fn bootstrap_task(spawner: Spawner, p: embassy_rp::Peripherals) {
    let board = app::Board::init(p).await;
    app::init_controllers(board).await;

    // Route defmt logs to RTT
    platform::defmt_logger::DefmtLogger::set_writer(&platform::defmt_logger::DEFAULT_RTT_WRITER);
    defmt::info!("Booting Cat Detector App...");

    let panic_flash = unsafe { app::PANIC_FLASH.as_mut().unwrap() };
    // Obtain separate static mut references for the panic handler and filesystem controller.
    // This is safe because the panic handler only runs after the application halts/panics.
    let fs_buf_panic = unsafe { &mut *core::ptr::addr_of_mut!(app::FS_BUF) };
    let fs_buf_controller = unsafe { &mut *core::ptr::addr_of_mut!(app::FS_BUF) };

    app::init_panic_handler(
        panic_flash,
        MapFilesystem(app::FS_PARTITION_START..app::FS_PARTITION_END),
        fs_buf_panic,
        app::MAX_CRASH_LOGS,
    );

    core_monitor::init_core(
        Some(spawner),
        core_monitor::CpuId::Core0,
        app::CORE_MONITOR_TIMEOUT_MS,
        app::CORE_MONITOR_WARN_PCT,
        true,
    );

    // Initialize the FilesystemController using stolen FLASH peripheral (safe because panic handler only reads/writes on panic)
    let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
    let raw_flash = embassy_rp::flash::Flash::<
        _,
        embassy_rp::flash::Blocking,
        { app::FLASH_SIZE },
    >::new_blocking(fs_flash);
    let mut async_flash = platform::BlockingAsyncFlash(raw_flash);

    use model::calibration::{Calibration, MotorCalibration, Vl53l0xCalibration};

    let mut cal_buf = [0u8; 128];
    let proximity_cal = flash::read_calibration_direct_blocking::<_, Vl53l0xCalibration>(
        &mut async_flash,
        MapFilesystem(app::FS_PARTITION_START..app::FS_PARTITION_END),
        unsafe { &mut *core::ptr::addr_of_mut!(app::FS_BUF) },
        Vl53l0xCalibration::CALIBRATION_FILE_NAME,
        &mut cal_buf,
    )
    .unwrap_or_default();

    let mut motor_cal_buf = [0u8; 128];
    let motor_cal = flash::read_calibration_direct_blocking::<_, MotorCalibration>(
        &mut async_flash,
        MapFilesystem(app::FS_PARTITION_START..app::FS_PARTITION_END),
        unsafe { &mut *core::ptr::addr_of_mut!(app::FS_BUF) },
        MotorCalibration::CALIBRATION_FILE_NAME,
        &mut motor_cal_buf,
    );

    let flash_mutex = unsafe {
        FLASH_MUTEX = Some(embassy_sync::mutex::Mutex::new(async_flash));
        FLASH_MUTEX.as_ref().unwrap()
    };

    let fs_flash_mutex_ref = flash::SharedFlashMutex::new(flash_mutex);
    let profiling_flash =
        controller::filesystem_controller::ProfilingFlash::new(fs_flash_mutex_ref);
    let mut fs_controller = controller::filesystem_controller::FilesystemController::new(
        profiling_flash,
        MapFilesystem(app::FS_PARTITION_START..app::FS_PARTITION_END),
        fs_buf_controller,
    );
    fs_controller.set_telemetry(app::TELEMETRY_CHANNEL.sender());

    // Verify and repair/reformat the filesystem if it is corrupted
    let _ = fs_controller.verify_and_repair().await;

    let client =
        controller::filesystem_controller::FilesystemClient::new(app::FILESYSTEM_CHANNEL.sender());

    let thermal_ctrl = unsafe { app::THERMAL_CTRL.take().unwrap() };
    let power_ctrl = unsafe { app::BATTERY_CTRL.take().unwrap() };
    let led_ctrl = unsafe { app::LED_CTRL.take().unwrap() };
    let mut controller = unsafe { app::MOTOR_CTRL_CORE0.take().unwrap() };
    let mut sensor_ctrl_north = unsafe { app::SENSOR_CTRL_NORTH_CORE0.take().unwrap() };
    let mut sensor_ctrl_east = unsafe { app::SENSOR_CTRL_EAST_CORE0.take().unwrap() };
    let mut sensor_ctrl_west = unsafe { app::SENSOR_CTRL_WEST_CORE0.take().unwrap() };

    if let Some(cal) = motor_cal {
        controller.set_calibration(&cal);
    }

    sensor_ctrl_north.set_calibration(&proximity_cal);
    sensor_ctrl_east.set_calibration(&proximity_cal);
    sensor_ctrl_west.set_calibration(&proximity_cal);

    let system_ctrl = unsafe { app::SYSTEM_CTRL.take().unwrap() };

    static mut TELEMETRY_CTRL: Option<
        TelemetryController<
            { app::MAX_RECORDS },
            { model::telemetry::BUFFER_SIZE },
            flash::SharedFlashMutex<platform::BlockingAsyncFlash<app::FlashDevice>>,
        >,
    > = None;

    let telemetry_flash_mutex_ref = flash::SharedFlashMutex::new(flash_mutex);
    let telemetry_ctrl = unsafe {
        TELEMETRY_CTRL = Some(TelemetryController::new(
            telemetry_flash_mutex_ref,
            QueueFilesystem(app::TELEMETRY_PARTITION_START..app::TELEMETRY_PARTITION_END),
            client,
        ));
        TELEMETRY_CTRL.as_mut().unwrap()
    };

    let core1 = unsafe { embassy_rp::peripherals::CORE1::steal() };
    app::boot_core1(core1);

    let spawner_c1 = unsafe { app::Board::spawner_core1() };
    spawner_c1
        .spawn(app::bootstrap_core1_task(
            spawner_c1,
            controller,
            (sensor_ctrl_north, sensor_ctrl_east, sensor_ctrl_west),
        ))
        .unwrap();

    // Spawn tasks on Core 0
    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Thermal(thermal_ctrl, THERMAL_CHANNEL), generics: (app::TempSensorDevice),
            Battery(power_ctrl, BATTERY_CHANNEL), generics: (app::BatteryDevice, app::ChargerDevice, app::AlertPinType),
            Led(led_ctrl, LED_CHANNEL), generics: (app::LedDevice),
            System(system_ctrl, SYSTEM_CHANNEL, GESTURE_CHANNEL, THERMAL_ACTION_CHANNEL), generics: (app::SystemControllerType),
            Filesystem(fs_controller, FILESYSTEM_CHANNEL), generics: (app::FlashDeviceType),
            Telemetry(telemetry_ctrl, TELEMETRY_CHANNEL), generics: ({ app::MAX_RECORDS }, { controller::telemetry_controller::CHANNEL_CAPACITY }, platform::flash::SharedFlashMutex<platform::BlockingAsyncFlash<app::FlashDevice>>),
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    let spawner_c0 = unsafe {
        use platform::rp2040::PlatformMulticore as _;
        platform::rp2040::Rp2040Multicore.spawner(platform::types::CpuId::Core0)
    };

    spawner_c0.spawn(bootstrap_task(spawner_c0, p)).unwrap();
    unsafe {
        app::Board::run_executor(platform::types::CpuId::Core0);
    }
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
