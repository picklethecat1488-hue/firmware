use cat_detector::{Board, MockFlex, PUMP_PIN_IA, PUMP_PIN_IB};
use embedded_hal::digital::OutputPin;

#[test]
fn test_board_pin_constants() {
    assert_eq!(PUMP_PIN_IA, 14);
    assert_eq!(PUMP_PIN_IB, 15);
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

    assert!(board.gpio_pins[PUMP_PIN_IA as usize].is_some());
    assert!(board.gpio_pins[PUMP_PIN_IB as usize].is_some());

    let mut pump_ia = board.gpio_pins[PUMP_PIN_IA as usize].take().unwrap();
    assert!(!pump_ia.is_high());

    pump_ia.set_high();
    assert!(pump_ia.is_high());
}
