//! Integration tests for Carrier Board 2.0 definitions in the app crate.

use cat_detector::carrier_board::{
    hello_world, run_bringup_self_test, BOARD_NAME, BOARD_REVISION, CANONICAL_COMPONENTS,
    EXT_NAND_SIZE_BYTES, FLASH_SIZE_BYTES, I2C0_ADDR_FUEL_GAUGE, I2C0_ADDR_LP5009,
    I2C1_ADDR_IQS7222A, MCU_ARCH, SRAM_SIZE_BYTES,
};

#[test]
fn test_carrier_board_constants() {
    assert_eq!(BOARD_NAME, "Carrier Board 2.0");
    assert_eq!(BOARD_REVISION, "Rev 2.0");
    assert!(MCU_ARCH.contains("MCX N947"));
    assert_eq!(FLASH_SIZE_BYTES, 2 * 1024 * 1024);
    assert_eq!(SRAM_SIZE_BYTES, 512 * 1024);
    assert_eq!(EXT_NAND_SIZE_BYTES, 128 * 1024 * 1024);
}

#[test]
fn test_carrier_board_i2c_addresses() {
    assert_eq!(I2C0_ADDR_FUEL_GAUGE, 0x36);
    assert_eq!(I2C0_ADDR_LP5009, 0x14);
    assert_eq!(I2C1_ADDR_IQS7222A, 0x44);
}

#[test]
fn test_carrier_board_canonical_components() {
    assert_eq!(CANONICAL_COMPONENTS.len(), 9);
    let u1 = CANONICAL_COMPONENTS.iter().find(|c| c.designator == "U1");
    assert!(u1.is_some());
    assert!(u1.unwrap().part_number.contains("MCXN947"));

    let u8 = CANONICAL_COMPONENTS.iter().find(|c| c.designator == "U8");
    assert!(u8.is_some());
    assert!(u8.unwrap().part_number.contains("W25N01GV"));
}

#[test]
fn test_carrier_board_hello_world() {
    let msg = hello_world();
    assert!(msg.contains("Hello, World!"));
    assert!(msg.contains("Carrier Board 2.0"));
}

#[test]
fn test_carrier_board_self_test() {
    assert!(run_bringup_self_test());
}
