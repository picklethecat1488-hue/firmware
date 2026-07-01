use controller::motor_controller::{MotorCommand, MotorController};
use controller::state_machine::MotorState;
use peripherals::mock::{MockCurrentSensor, MockMotor};

#[test]
fn test_motor_controller_flow() {
    let motor = MockMotor::new();
    let sensor = MockCurrentSensor::new(150); // healthy current: 150mA
    let mut controller = MotorController::new(motor, sensor);

    assert_eq!(controller.state(), MotorState::Off);

    // Turn on the motor using handle_command
    controller.handle_command(MotorCommand::SetSpeed(100), None);
    assert_eq!(controller.state(), MotorState::RampUp);
    assert_eq!(controller.motor.speed, 100);

    // Run update under normal load -> transitions from RampUp to On
    controller.update(None).unwrap();
    assert_eq!(controller.state(), MotorState::On);
    assert_eq!(controller.motor.speed, 100);

    // Simulate dry run (low current draw)
    controller.current_sensor.current_ma = 10; // below 15mA threshold
    controller.update(None).unwrap(); // triggers PowerOff -> state becomes RampDown
    assert_eq!(controller.state(), MotorState::RampDown);

    // Ramping down auto-transitions to Off on next update
    controller.update(None).unwrap();
    assert_eq!(controller.state(), MotorState::Off);
    assert_eq!(controller.motor.speed, 0); // motor should be stopped

    // Restart the motor
    controller.current_sensor.current_ma = 150; // reset to healthy current
    controller.handle_command(MotorCommand::SetSpeed(100), None);
    assert_eq!(controller.state(), MotorState::RampUp);
    assert_eq!(controller.motor.speed, 100);

    // Transition to On
    controller.update(None).unwrap();
    assert_eq!(controller.state(), MotorState::On);

    // Simulate stall (high current draw)
    controller.current_sensor.current_ma = 900; // above 800mA threshold
    controller.update(None).unwrap(); // triggers PowerOff -> state becomes RampDown
    assert_eq!(controller.state(), MotorState::RampDown);

    controller.update(None).unwrap();
    assert_eq!(controller.state(), MotorState::Off); // fallback safety transition complete
    assert_eq!(controller.motor.speed, 0); // motor should be stopped
}

#[test]
fn test_led_controller_flow() {
    futures::executor::block_on(async {
        let mock_led = peripherals::mock::MockLed::new();
        let mut controller = controller::led_controller::LedController::new(mock_led);

        assert_eq!(
            controller.current_state(),
            model::types::SystemLedState::Off
        );

        controller
            .set_pattern(model::types::SystemLedState::SolidGreen)
            .await
            .unwrap();
        assert_eq!(
            controller.current_state(),
            model::types::SystemLedState::SolidGreen
        );
    });
}
