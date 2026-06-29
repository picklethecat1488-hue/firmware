use cat_detector::{Board, MockFlex, LED_PIN, PUMP_PIN};
use embedded_hal::digital::OutputPin;

#[test]
fn test_board_pin_constants() {
    assert_eq!(LED_PIN, 25);
    assert_eq!(PUMP_PIN, 25);
}

#[test]
fn test_mock_flex_toggling() {
    let mut pin = MockFlex::new();
    assert!(!pin.is_high());

    pin.set_high();
    assert!(pin.is_high());

    pin.set_low();
    assert!(!pin.is_high());
}

#[test]
fn test_embedded_hal_output_trait_compatibility() {
    let mut pin = MockFlex::new();

    let res_high = OutputPin::set_high(&mut pin);
    assert!(res_high.is_ok());
    assert!(pin.is_high());

    let res_low = OutputPin::set_low(&mut pin);
    assert!(res_low.is_ok());
    assert!(!pin.is_high());
}

#[test]
fn test_mock_board_initialization() {
    let mut board = Board::init();

    assert!(board.gpio_pins[LED_PIN as usize].is_some());

    let mut led_pin = board.gpio_pins[LED_PIN as usize].take().unwrap();
    assert!(!led_pin.is_high());

    led_pin.set_high();
    assert!(led_pin.is_high());
}
