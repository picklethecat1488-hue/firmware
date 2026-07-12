use controller::sensor_controller::{SensorCommand, SensorController};
use controller::types::SensorMetadata;
use model::types::{Direction, ProximityTelemetry};
use peripherals::mock::MockProximitySensor;

#[test]
fn test_sensor_controller_flow() {
    let sensor = MockProximitySensor::new(10);
    let mut controller = SensorController::new(
        SensorMetadata {
            direction: Direction::North,
        },
        sensor,
        300,
    );

    assert_eq!(controller.latest_distance(), 1000);
    assert_eq!(
        controller.telemetry(),
        ProximityTelemetry::OutRange(Direction::North, 1000)
    );
    assert_eq!(controller.direction(), Direction::North);

    // Call update to sample ToF measurements
    controller.update().unwrap();

    assert_eq!(controller.latest_distance(), 10);
    assert_eq!(
        controller.telemetry(),
        ProximityTelemetry::InRange(Direction::North, 10)
    );

    // Verify periodic state
    assert!(controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::DisablePeriodic);
    assert!(!controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::EnablePeriodic);
    assert!(controller.is_periodic_enabled());
}

#[test]
fn test_sensor_controller_sad_cases() {
    let mut sensor = MockProximitySensor::new(10);
    sensor.should_fail = true; // Make sensor fail
    let mut controller = SensorController::new(
        SensorMetadata {
            direction: Direction::North,
        },
        sensor,
        300,
    );

    // Call update, which should fail and return Err
    let res = controller.update();
    assert!(res.is_err());
}
