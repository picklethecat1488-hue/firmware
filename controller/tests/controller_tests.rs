use controller::motor_controller::{MotorController, MotorCommand};
use controller::state_machine::MotorState;
use peripherals::mock::{MockMotor, MockCurrentSensor};

#[test]
fn test_motor_controller_flow() {
    let motor = MockMotor::new();
    let sensor = MockCurrentSensor::new(150); // healthy current: 150mA
    let mut controller = MotorController::new(motor, sensor);

    assert_eq!(controller.state(), MotorState::Off);

    // Turn on the motor using handle_command
    controller.handle_command(MotorCommand::SetSpeed(100));
    assert_eq!(controller.state(), MotorState::Ramping);
    assert_eq!(controller.motor.speed, 100);

    // Run update under normal load -> transitions from Ramping to On
    controller.update().unwrap();
    assert_eq!(controller.state(), MotorState::On);
    assert_eq!(controller.motor.speed, 100);

    // Simulate dry run (low current draw)
    controller.current_sensor.current_ma = 10; // below 15mA threshold
    controller.update().unwrap();
    assert_eq!(controller.state(), MotorState::Off);
    assert_eq!(controller.motor.speed, 0); // motor should be stopped

    // Restart the motor
    controller.current_sensor.current_ma = 150; // reset to healthy current
    controller.handle_command(MotorCommand::SetSpeed(100));
    assert_eq!(controller.state(), MotorState::Ramping);
    assert_eq!(controller.motor.speed, 100);

    // Transition to On
    controller.update().unwrap();
    assert_eq!(controller.state(), MotorState::On);

    // Simulate stall (high current draw)
    controller.current_sensor.current_ma = 900; // above 800mA threshold
    controller.update().unwrap();
    assert_eq!(controller.state(), MotorState::Off); // fallback safety transition
    assert_eq!(controller.motor.speed, 0); // motor should be stopped
}
