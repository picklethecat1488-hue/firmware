use super::*;
use peripherals::mock::MockProximitySensor;

#[test]
fn test_sensor_controller_flow() {
    let sensor = MockProximitySensor::new(10);
    let mut controller = SensorController::new(0, sensor);

    assert_eq!(controller.latest_distance(), 1000);
    assert_eq!(
        controller.telemetry(),
        model::types::ProximityTelemetry::OutRange(1000)
    );
    assert_eq!(controller.sensor_id(), 0);

    // Call update to sample ToF measurements
    controller.update().unwrap();

    assert_eq!(controller.latest_distance(), 10);
    assert_eq!(
        controller.telemetry(),
        model::types::ProximityTelemetry::InRange(10)
    );

    // Verify periodic state
    assert!(controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::DisablePeriodic);
    assert!(!controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::EnablePeriodic);
    assert!(controller.is_periodic_enabled());
}
