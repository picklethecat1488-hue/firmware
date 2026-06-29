//! Target Board Support Package (BSP) for Raspberry Pi Pico.
//!
//! Provides hardware-specific peripheral wrappers, pin initialization,
//! and lookup mappings for bare-metal deployment.

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]

use embassy_rp::gpio::{Flex, Pin};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::uart::{Config as UartConfig, Uart};
use embassy_rp::Peripherals;

/// Helper structure containing all pre-initialized board interfaces.
pub struct Board<'d> {
    /// Blocking UART0 instance for interactive terminal shell
    pub uart: Uart<'d, embassy_rp::peripherals::UART0, embassy_rp::uart::Blocking>,
    /// Blocking I2C0 instance for sensor communications
    pub i2c: I2c<'d, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    /// Lookup array containing Flex instances for dynamic GPIO diagnostics
    pub gpio_pins: [Option<Flex<'d>>; 30],
}

impl<'d> Board<'d> {
    /// Initialize all hardware components and return the Board interface.
    ///
    /// # Arguments
    /// * `p` - The RP2040 peripheral set.
    pub fn init(p: Peripherals) -> Self {
        let uart = Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, UartConfig::default());
        let i2c = I2c::new_blocking(p.I2C0, p.PIN_5, p.PIN_4, I2cConfig::default());
        let gpio_pins: [Option<Flex<'d>>; 30] = [
            None, // 0 - UART TX
            None, // 1 - UART RX
            Some(Flex::new(p.PIN_2.degrade())),
            Some(Flex::new(p.PIN_3.degrade())),
            None, // 4 - I2C SDA
            None, // 5 - I2C SCL
            Some(Flex::new(p.PIN_6.degrade())),
            Some(Flex::new(p.PIN_7.degrade())),
            Some(Flex::new(p.PIN_8.degrade())),
            Some(Flex::new(p.PIN_9.degrade())),
            Some(Flex::new(p.PIN_10.degrade())),
            Some(Flex::new(p.PIN_11.degrade())),
            Some(Flex::new(p.PIN_12.degrade())),
            Some(Flex::new(p.PIN_13.degrade())),
            Some(Flex::new(p.PIN_14.degrade())),
            Some(Flex::new(p.PIN_15.degrade())),
            Some(Flex::new(p.PIN_16.degrade())),
            Some(Flex::new(p.PIN_17.degrade())),
            Some(Flex::new(p.PIN_18.degrade())),
            Some(Flex::new(p.PIN_19.degrade())),
            Some(Flex::new(p.PIN_20.degrade())),
            Some(Flex::new(p.PIN_21.degrade())),
            Some(Flex::new(p.PIN_22.degrade())),
            Some(Flex::new(p.PIN_23.degrade())),
            Some(Flex::new(p.PIN_24.degrade())),
            Some(Flex::new(p.PIN_25.degrade())), // Onboard LED / Pump pin
            Some(Flex::new(p.PIN_26.degrade())),
            Some(Flex::new(p.PIN_27.degrade())),
            Some(Flex::new(p.PIN_28.degrade())),
            Some(Flex::new(p.PIN_29.degrade())),
        ];
        Self {
            uart,
            i2c,
            gpio_pins,
        }
    }
}
