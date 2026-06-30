use super::*;
use peripherals::mock::MockProximitySensor;

#[test]
fn test_sensor_controller_flow() {
    let sensor_n = MockProximitySensor::new(10);
    let sensor_e = MockProximitySensor::new(20);
    let sensor_w = MockProximitySensor::new(30);

    let mut controller = SensorController::new(sensor_n, sensor_e, sensor_w);

    let initial_telemetry = controller.telemetry();
    assert_eq!(initial_telemetry.distance_north_mm, 0);
    assert_eq!(initial_telemetry.distance_east_mm, 0);
    assert_eq!(initial_telemetry.distance_west_mm, 0);

    // Call update to sample ToF measurements
    controller.update().unwrap();

    let updated_telemetry = controller.telemetry();
    assert_eq!(updated_telemetry.distance_north_mm, 10);
    assert_eq!(updated_telemetry.distance_east_mm, 20);
    assert_eq!(updated_telemetry.distance_west_mm, 30);

    // Verify periodic state
    assert!(controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::DisablePeriodic);
    assert!(!controller.is_periodic_enabled());
    controller.handle_command(SensorCommand::EnablePeriodic);
    assert!(controller.is_periodic_enabled());
}
