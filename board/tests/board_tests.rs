use board::{Board, MockFlex};
use embedded_hal::digital::{InputPin, OutputPin};

#[test]
fn test_board_pin_constants() {
    assert_eq!(Board::PUMP_UH_PIN, 16);
    assert_eq!(Board::PUMP_UL_PIN, 17);
    assert_eq!(Board::PUMP_VH_PIN, 18);
    assert_eq!(Board::PUMP_VL_PIN, 19);
    assert_eq!(Board::PUMP_WH_PIN, 20);
    assert_eq!(Board::PUMP_WL_PIN, 21);
    assert_eq!(Board::PUMP_DIAG_PIN, 22);
    assert_eq!(Board::SPEED_SENSE_PIN, 26);
}

#[test]
fn test_mock_flex_toggling() {
    let mut pin = MockFlex::new();
    assert!(!pin.is_set());
    assert!(!pin.is_high().unwrap());

    pin.set_high();
    assert!(pin.is_set());
    assert!(pin.is_high().unwrap());

    pin.set_low();
    assert!(!pin.is_set());
    assert!(!pin.is_high().unwrap());
}

#[test]
fn test_embedded_hal_output_trait_compatibility() {
    let mut pin = MockFlex::new();

    let res_high = OutputPin::set_high(&mut pin);
    assert!(res_high.is_ok());
    assert!(pin.is_set());
    assert!(pin.is_high().unwrap());

    let res_low = OutputPin::set_low(&mut pin);
    assert!(res_low.is_ok());
    assert!(!pin.is_set());
    assert!(!pin.is_high().unwrap());
}

#[test]
fn test_mock_board_initialization() {
    let mut board = Board::init();

    assert!(board.gpio_pins[Board::PUMP_UH_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_UL_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_VH_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_VL_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_WH_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_WL_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::PUMP_DIAG_PIN as usize].is_some());
    assert!(board.gpio_pins[Board::SPEED_SENSE_PIN as usize].is_some());

    let mut pump_uh = board.gpio_pins[Board::PUMP_UH_PIN as usize].take().unwrap();
    assert!(!pump_uh.is_set());
    assert!(!pump_uh.is_high().unwrap());

    pump_uh.set_high();
    assert!(pump_uh.is_set());
    assert!(pump_uh.is_high().unwrap());

    let peripherals = board.take_peripherals();
    assert_eq!(peripherals.speed_sensor.pulses_per_rev(), 3);
}
