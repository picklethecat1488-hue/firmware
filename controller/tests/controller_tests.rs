use controller::fountain_controller::FountainController;
use model::state_machine::FountainState;
use peripherals::mock::{MockPump, MockWaterSensor};

#[test]
fn test_fountain_controller_flow() {
    let pump = MockPump::new();
    let sensor = MockWaterSensor::new(false);
    let mut controller = FountainController::new(pump, sensor);

    assert_eq!(controller.state(), FountainState::Idle);

    controller.sensor.water_present = true;
    controller.update().unwrap();
    assert_eq!(controller.state(), FountainState::Pumping);
    assert_eq!(controller.pump.speed, 100);

    controller.sensor.water_present = false;
    controller.update().unwrap();
    assert_eq!(controller.state(), FountainState::LowWaterWarning);
    assert_eq!(controller.pump.speed, 0);
}
