use model::interfaces::NoTick;
use model::types::{Direction, MotorSpeed};
use peripherals::mock::{DummyCurrentSensor, MockMotor, MockProximitySensor};

#[test]
fn test_core_gating_motor_compiles() {
    let mock_motor = MockMotor::new();
    let mut motor_ctrl = controller::motor_controller::MotorController::new(
        NoTick::new(mock_motor),
        DummyCurrentSensor,
    );

    let speed = MotorSpeed::new(50).unwrap();
    motor_ctrl.handle_command(
        controller::motor_controller::MotorCommand::SetSpeed(speed),
        None,
    );

    assert_eq!(motor_ctrl.current_rpm(), 0);
}

#[test]
fn test_core_gating_sensor_compiles() {
    let mock_tof = MockProximitySensor::new(500);
    let mut sensor_ctrl = controller::sensor_controller::SensorController::new(
        controller::types::SensorMetadata {
            direction: Direction::North,
        },
        mock_tof,
        300,
    );

    assert_eq!(sensor_ctrl.latest_data(), 1000);
    let update_res = sensor_ctrl.update();
    assert!(update_res.is_ok());
    assert_eq!(sensor_ctrl.latest_data(), 500);
}

#[test]
fn test_feature_conditional_compilation() {
    let motor_in_ram = cfg!(feature = "motor-core");
    let sensors_in_ram = cfg!(feature = "sensors-core");
    println!(
        "Core gating test: motor-core={}, sensors-core={}",
        motor_in_ram, sensors_in_ram
    );
}
