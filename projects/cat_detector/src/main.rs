//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    cat_detector::system_controller::SystemController,
    controller::battery_controller::BatteryController,
    controller::led_controller::LedController,
    controller::motor_controller::MotorController,
    controller::sensor_controller::SensorController,
    controller::thermal_controller::ThermalController,
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    embassy_sync::mutex::Mutex,
    peripherals::mock::{DummyCurrentSensor, DummyProximitySensor, MockBattery, MockLed},
    peripherals::motor::GpioMotor,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cat_detector::handle_panic::<
        { cat_detector::FLASH_SIZE },
        { cat_detector::STACK_TOP },
        { cat_detector::FLASH_START },
        { cat_detector::FLASH_END },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
// Define raw statically allocated Mutex for thread-safe/multi-core peripheral sharing
static SHARED_BATTERY: Mutex<CriticalSectionRawMutex, MockBattery> =
    Mutex::new(MockBattery::new(3700, 25000));

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    cat_detector::log_info!("Initializing hardware for cat detector...");
    let p = embassy_rp::init(Default::default());

    // Initialize board peripherals using the unified board configuration
    let mut board = cat_detector::Board::init(p);

    // Initialize the modular panic handler
    cat_detector::init_panic_handler(
        board.flash,
        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
    );

    // Initialize the FilesystemController using stolen FLASH peripheral (safe because panic handler only reads/writes on panic)
    let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
    let raw_flash = embassy_rp::flash::Flash::<
        _,
        embassy_rp::flash::Blocking,
        { cat_detector::FLASH_SIZE },
    >::new_blocking(fs_flash);
    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(raw_flash);
    let fs_controller = controller::filesystem_controller::FilesystemController::new(
        async_flash,
        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
    );

    // Extract the motor control pin from the board configuration array
    let motor_pin = board.gpio_pins[cat_detector::LED_PIN as usize]
        .take()
        .expect("Motor pin must be available");

    let motor = GpioMotor::new(motor_pin);
    let current_sensor = DummyCurrentSensor;

    let controller = MotorController::new(motor, current_sensor);

    // Initialize simulated proximity sensors for North, East, West ToFs
    let tof_north = DummyProximitySensor::new(100);
    let tof_east = DummyProximitySensor::new(150);
    let tof_west = DummyProximitySensor::new(200);

    let sensor_ctrl_north = SensorController::new_with_fusion(
        0,
        tof_north,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            sensor_id: id,
            distance_mm: dist,
        },
    );

    let sensor_ctrl_east = SensorController::new_with_fusion(
        1,
        tof_east,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            sensor_id: id,
            distance_mm: dist,
        },
    );

    let sensor_ctrl_west = SensorController::new_with_fusion(
        2,
        tof_west,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            sensor_id: id,
            distance_mm: dist,
        },
    );

    let thermal_ctrl = ThermalController::new_with_shutdown(
        &SHARED_BATTERY,
        cat_detector::SYSTEM_CHANNEL.sender(),
        cat_detector::system_controller::SystemCommand::Sleep,
    );
    let power_ctrl = BatteryController::new(&SHARED_BATTERY);

    // Initialize simulated LED driver and its controller
    let led_driver = MockLed::new();
    let led_ctrl = LedController::new(led_driver);

    // Initialize SystemController to coordinate all loops
    let system_ctrl = SystemController::new(
        cat_detector::MOTOR_CHANNEL.sender(),
        cat_detector::SENSOR_NORTH_CHANNEL.sender(),
        cat_detector::SENSOR_EAST_CHANNEL.sender(),
        cat_detector::SENSOR_WEST_CHANNEL.sender(),
        cat_detector::BATTERY_CHANNEL.sender(),
        cat_detector::THERMAL_CHANNEL.sender(),
        cat_detector::LED_CHANNEL.sender(),
    );

    // Spawn controllers selectively and concurrently using separate macros
    controller::run_thermal_task!(
        spawner,
        thermal_task,
        thermal_ctrl,
        cat_detector::THERMAL_CHANNEL.receiver(),
        MockBattery,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_battery_task!(
        spawner,
        power_task,
        power_ctrl,
        cat_detector::BATTERY_CHANNEL.receiver(),
        MockBattery
    );

    controller::run_motor_task!(
        spawner,
        motor_task,
        controller,
        cat_detector::MOTOR_CHANNEL.receiver(),
        GpioMotor<embassy_rp::gpio::Flex<'static>>,
        DummyCurrentSensor
    );

    // Spawn the three proximity sensor tasks
    controller::run_sensor_task!(
        spawner,
        sensor_north_task,
        sensor_ctrl_north,
        cat_detector::SENSOR_NORTH_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_east_task,
        sensor_ctrl_east,
        cat_detector::SENSOR_EAST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_west_task,
        sensor_ctrl_west,
        cat_detector::SENSOR_WEST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        cat_detector::system_controller::SystemCommand
    );

    // Spawn the LED controller task
    controller::run_led_task!(
        spawner,
        led_task,
        led_ctrl,
        cat_detector::LED_CHANNEL.receiver(),
        MockLed,
        CriticalSectionRawMutex
    );

    cat_detector::run_system_task!(
        spawner,
        system_task,
        system_ctrl,
        cat_detector::SYSTEM_CHANNEL.receiver()
    );

    cat_detector::run_telemetry_task!(
        spawner,
        telemetry_task,
        fs_controller,
        cat_detector::TELEMETRY_CHANNEL.receiver(),
        firmware_lib::panic_handler::BlockingAsyncFlash<
            embassy_rp::flash::Flash<
                embassy_rp::peripherals::FLASH,
                embassy_rp::flash::Blocking,
                { cat_detector::FLASH_SIZE },
            >,
        >
    );
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
