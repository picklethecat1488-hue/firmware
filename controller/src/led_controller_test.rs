use super::*;
use peripherals::mock::MockLed;

#[test]
fn test_led_controller_flow() {
    futures::executor::block_on(async {
        let mock_led = MockLed::new();
        let mut controller = LedController::new(mock_led);

        assert_eq!(controller.current_state(), SystemLedState::Off);

        controller.set_pattern(SystemLedState::SolidGreen).await.unwrap();
        assert_eq!(controller.current_state(), SystemLedState::SolidGreen);
    });
}
