//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    controller::battery_controller::BatteryController,
    controller::motor_controller::MotorController,
    controller::sensor_controller::SensorController,
    controller::thermal_controller::ThermalController,
    cat_detector::system_controller::SystemController,
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    embassy_sync::mutex::Mutex,
    panic_probe as _,
    peripherals::mock::{DummyCurrentSensor, DummyProximitySensor, MockBattery},
    peripherals::motor::GpioMotor,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
// Define raw statically allocated Mutex for thread-safe/multi-core peripheral sharing
static SHARED_BATTERY: Mutex<CriticalSectionRawMutex, MockBattery> =
    Mutex::new(MockBattery::new(3700, 25000));

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Initializing hardware for cat detector...");
    let p = embassy_rp::init(Default::default());

    // Initialize board peripherals using the unified board configuration
    let mut board = cat_detector::Board::init(p);

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
    let sensor_ctrl = SensorController::new(tof_north, tof_east, tof_west);

    let thermal_ctrl = ThermalController::new_with_shutdown(
        &SHARED_BATTERY,
        cat_detector::SYSTEM_CHANNEL.sender(),
        cat_detector::system_controller::SystemCommand::Sleep,
    );
    let power_ctrl = BatteryController::new(&SHARED_BATTERY);

    // Initialize SystemController to coordinate all loops
    let system_ctrl = SystemController::new(
        cat_detector::MOTOR_CHANNEL.sender(),
        cat_detector::SENSOR_CHANNEL.sender(),
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

    controller::run_sensor_task!(
        spawner,
        sensor_task,
        sensor_ctrl,
        cat_detector::SENSOR_CHANNEL.receiver(),
        DummyProximitySensor,
        DummyProximitySensor,
        DummyProximitySensor
    );

    cat_detector::run_system_task!(
        spawner,
        system_task,
        system_ctrl,
        cat_detector::SYSTEM_CHANNEL.receiver()
    );
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
