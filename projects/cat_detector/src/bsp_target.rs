//! Target Board Support Package (BSP) for Raspberry Pi Pico.
//!
//! Provides hardware-specific peripheral wrappers, pin initialization,
//! and lookup mappings for bare-metal deployment.

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]

use embassy_rp::gpio::{Flex, Pin, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::uart::{Config as UartConfig, Uart};
use embassy_rp::Peripherals;

/// Helper structure containing all pre-initialized board interfaces.
pub struct Board<'d> {
    /// Blocking UART0 instance for interactive terminal shell
    pub uart: Uart<'d, embassy_rp::peripherals::UART0, embassy_rp::uart::Blocking>,
    /// Blocking I2C0 instance for sensor communications
    pub i2c: I2c<'d, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    /// The onboard flash peripheral
    pub flash: embassy_rp::peripherals::FLASH,
    /// Lookup array containing Flex instances for dynamic GPIO diagnostics
    pub gpio_pins: [Option<Flex<'d>>; 30],
}

impl<'d> Board<'d> {
    /// Initialize all hardware components and return the Board interface.
    ///
    /// # Arguments
    /// * `p` - The RP2040 peripheral set.
    pub fn init(p: Peripherals) -> Self {
        // 1. Perform I2C bus unstuck on I2C0 (GP4 SDA, GP5 SCL) using raw registers
        // to avoid taking ownership of Pin types before constructing I2c.
        unsafe {
            const SIO_BASE: u32 = 0xd000_0000;
            const SIO_GPIO_OUT_SET: *mut u32 = (SIO_BASE + 0x14) as *mut u32;
            const SIO_GPIO_OUT_CLR: *mut u32 = (SIO_BASE + 0x18) as *mut u32;
            const SIO_GPIO_OE_SET: *mut u32 = (SIO_BASE + 0x24) as *mut u32;
            const SIO_GPIO_IN: *const u32 = (SIO_BASE + 0x04) as *const u32;

            const IO_BANK0_BASE: u32 = 0x4001_4000;
            const IO_BANK0_GPIO4_CTRL: *mut u32 = (IO_BANK0_BASE + 0x24) as *mut u32;
            const IO_BANK0_GPIO5_CTRL: *mut u32 = (IO_BANK0_BASE + 0x2c) as *mut u32;

            const PADS_BANK0_BASE: u32 = 0x4001_c000;
            const PADS_BANK0_GPIO4: *mut u32 = (PADS_BANK0_BASE + 0x14) as *mut u32;
            const PADS_BANK0_GPIO5: *mut u32 = (PADS_BANK0_BASE + 0x18) as *mut u32;

            // Set pin functions to SIO (GPIO function is 5 on RP2040)
            core::ptr::write_volatile(IO_BANK0_GPIO5_CTRL, 5);
            core::ptr::write_volatile(IO_BANK0_GPIO4_CTRL, 5);

            // Enable pull-ups on SCL/SDA pads
            core::ptr::write_volatile(PADS_BANK0_GPIO5, 0x5a);
            core::ptr::write_volatile(PADS_BANK0_GPIO4, 0x5a);

            // Set SCL (GP5) as output high
            core::ptr::write_volatile(SIO_GPIO_OUT_SET, 1 << 5);
            core::ptr::write_volatile(SIO_GPIO_OE_SET, 1 << 5);

            // Toggle SCL up to 9 times or until SDA releases (goes high)
            for _ in 0..9 {
                let sda_val = core::ptr::read_volatile(SIO_GPIO_IN);
                if (sda_val & (1 << 4)) != 0 {
                    break;
                }
                // Drive SCL low
                core::ptr::write_volatile(SIO_GPIO_OUT_CLR, 1 << 5);
                cortex_m::asm::delay(200);
                // Drive SCL high
                core::ptr::write_volatile(SIO_GPIO_OUT_SET, 1 << 5);
                cortex_m::asm::delay(200);
            }
        }

        let uart = Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, UartConfig::default());
        let mut i2c = I2c::new_blocking(p.I2C0, p.PIN_5, p.PIN_4, I2cConfig::default());
        let mut gpio_pins: [Option<Flex<'d>>; 30] = [
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

        // 2. Assert XSHUT (active low) on all ToF sensors (GP2, GP3, GP6)
        if let Some(ref mut pin) = gpio_pins[crate::TOF_NORTH_XSHUT_PIN as usize] {
            pin.set_as_output();
            pin.set_low();
        }
        if let Some(ref mut pin) = gpio_pins[crate::TOF_EAST_XSHUT_PIN as usize] {
            pin.set_as_output();
            pin.set_low();
        }
        if let Some(ref mut pin) = gpio_pins[crate::TOF_WEST_XSHUT_PIN as usize] {
            pin.set_as_output();
            pin.set_low();
        }

        // 3. Configure Fuel Gauge Alert pin (GP10) as input with pull-up (active-low, open-drain)
        if let Some(ref mut pin) = gpio_pins[crate::FUEL_GAUGE_INT_PIN as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Up);
        }

        // 4. Configure ToF Sensor Interrupt pins (GP7, GP8, GP9) as inputs with pull-ups (active-low, open-drain)
        if let Some(ref mut pin) = gpio_pins[crate::TOF_NORTH_INT_PIN as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Up);
        }
        if let Some(ref mut pin) = gpio_pins[crate::TOF_EAST_INT_PIN as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Up);
        }
        if let Some(ref mut pin) = gpio_pins[crate::TOF_WEST_INT_PIN as usize] {
            pin.set_as_input();
            pin.set_pull(Pull::Up);
        }

        // Wait for sensors to register reset state
        cortex_m::asm::delay(20_000);

        // Bring North sensor out of shutdown (GP2 high) and assign address 0x30
        if let Some(ref mut pin) = gpio_pins[crate::TOF_NORTH_XSHUT_PIN as usize] {
            pin.set_high();
            cortex_m::asm::delay(20_000); // Wait for sensor to boot
            let mut sensor = peripherals::vl53l0x::Vl53l0x::new(&mut i2c, 0x29);
            let _ = sensor.set_address(0x30);
        }

        // Bring East sensor out of shutdown (GP3 high) and assign address 0x31
        if let Some(ref mut pin) = gpio_pins[crate::TOF_EAST_XSHUT_PIN as usize] {
            pin.set_high();
            cortex_m::asm::delay(20_000); // Wait for sensor to boot
            let mut sensor = peripherals::vl53l0x::Vl53l0x::new(&mut i2c, 0x29);
            let _ = sensor.set_address(0x31);
        }

        // Bring West sensor out of shutdown (GP6 high) and assign address 0x32
        if let Some(ref mut pin) = gpio_pins[crate::TOF_WEST_XSHUT_PIN as usize] {
            pin.set_high();
            cortex_m::asm::delay(20_000); // Wait for sensor to boot
            let mut sensor = peripherals::vl53l0x::Vl53l0x::new(&mut i2c, 0x29);
            let _ = sensor.set_address(0x32);
        }

        Self {
            uart,
            i2c,
            flash: p.FLASH,
            gpio_pins,
        }
    }
}
