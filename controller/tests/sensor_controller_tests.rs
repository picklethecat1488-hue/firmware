use controller::sensor_controller::{SensorCommand, SensorController};
use controller::types::SensorMetadata;
use model::types::Direction;
use peripheral::mock::MockProximitySensor;

#[test]
fn test_sensor_controller_flow() {
    futures::executor::block_on(async {
        let sensor = MockProximitySensor::new(10);
        let mut controller = SensorController::new(
            SensorMetadata {
                direction: Direction::North,
            },
            sensor,
            300,
        );

        assert_eq!(
            controller.latest_distance(),
            model::types::SensorReading::Invalid
        );
        assert_eq!(
            controller.telemetry(),
            model::types::SensorTelemetry::Status(
                Direction::North,
                model::types::SensorReading::Invalid
            )
        );
        assert_eq!(controller.direction(), Direction::North);

        // Call update to sample ToF measurements
        controller.update().await.unwrap();

        assert_eq!(
            controller.latest_distance(),
            model::types::SensorReading::Proximity(10)
        );
        assert_eq!(
            controller.telemetry(),
            model::types::SensorTelemetry::Status(
                Direction::North,
                model::types::SensorReading::Proximity(10)
            )
        );

        // Verify periodic state
        assert!(!controller.is_periodic_enabled());
        controller
            .handle_command(SensorCommand::SetInterval(
                controller::PeriodicInterval::UpdateMs(1000),
            ))
            .await;
        assert!(controller.is_periodic_enabled());
        controller
            .handle_command(SensorCommand::SetInterval(
                controller::PeriodicInterval::None,
            ))
            .await;
        assert!(!controller.is_periodic_enabled());
    });
}

#[test]
fn test_sensor_controller_sad_cases() {
    futures::executor::block_on(async {
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
        let res = controller.update().await;
        assert!(res.is_err());
    });
}
