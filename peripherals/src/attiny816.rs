//! Concrete driver implementation for the ATtiny816 custom LED driver over I2C.

#![deny(missing_docs)]

use crate::tracing;
use crate::I2cToPeripheralError;
use embedded_hal::i2c::I2c;
use model::interfaces::{LedDriver, Probeable};
use model::types::PeripheralError;

macro_rules! log_warn {
    ($fmt:literal $(, $arg:expr)*) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::warn!($fmt, "ATtiny816" $(, $arg)*);
    };
}

const BASE_NEOPIXEL: u8 = 0x0E;

struct Command;
impl Command {
    const SET_PIN: u8 = 0x01;
    const SET_BUF_LEN: u8 = 0x03;
    const WRITE_BUF: u8 = 0x04;
    const SHOW: u8 = 0x05;
}

/// Driver for the ATtiny816 custom NeoPixel LED driver over I2C.
pub struct Attiny816<I> {
    i2c: I,
    address: u8,
}

impl<I: I2c> Attiny816<I> {
    /// Creates a new ATtiny816 LED driver instance with default address (0x60).
    pub const fn new(i2c: I) -> Self {
        Self { i2c, address: 0x60 }
    }

    /// Initializes the NeoPixel driver on pin 14 with a buffer of 1 pixel (3 bytes).
    pub fn init(&mut self) -> Result<(), PeripheralError> {
        let res = (|| {
            // 1. Set Output Pin to 14
            self.i2c
                .write(self.address, &[BASE_NEOPIXEL, Command::SET_PIN, 14])
                .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
            // 2. Set Buffer Length (3 bytes for 1 RGB NeoPixel)
            self.i2c
                .write(self.address, &[BASE_NEOPIXEL, Command::SET_BUF_LEN, 0, 3])
                .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to locate or initialize LED driver at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }

    /// Sets the color of the connected NeoPixel LED.
    /// Writes the GRB values to offset 0 and sends the show command.
    pub fn set_led_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), PeripheralError> {
        let res = (|| {
            // 3. Write data to buffer (offset 0, standard GRB sequence)
            self.i2c
                .write(
                    self.address,
                    &[BASE_NEOPIXEL, Command::WRITE_BUF, 0, 0, g, r, b],
                )
                .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
            // 4. Send show command
            self.i2c
                .write(self.address, &[BASE_NEOPIXEL, Command::SHOW])
                .map_err(|e| e.to_i2c_error(self.address as u16, BASE_NEOPIXEL as u16))?;
            Ok(())
        })();
        if let Err(ref _e) = res {
            log_warn!(
                "{}: Failed to set LED color at address 0x{:02x}: {:?}",
                self.address,
                defmt::Debug2Format(_e)
            );
        }
        res
    }
}

impl<I: I2c> LedDriver for Attiny816<I> {
    type Error = PeripheralError;

    #[tracing::instrument(level = "trace")]
    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        self.set_led_color(r, g, b)
    }
}

struct StatusModule;
impl StatusModule {
    const BASE: u8 = 0x00;
    const HW_ID: u8 = 0x01;
    const SWRST: u8 = 0x7F;
    const SWRST_VAL: u8 = 0xFF;
}

impl<I: I2c> Probeable for Attiny816<I> {
    type Error = PeripheralError;

    fn read_chip_id(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(
                self.address,
                &[StatusModule::BASE, StatusModule::HW_ID],
                &mut buf,
            )
            .map_err(|e| e.to_i2c_error(self.address as u16, StatusModule::HW_ID as u16))?;
        let id = buf[0] as u16;
        if id == 0x86 {
            Ok(id)
        } else {
            Err(PeripheralError::DeviceNotFound(id))
        }
    }

    fn reset(&mut self) -> Result<(), Self::Error> {
        self.i2c
            .write(
                self.address,
                &[
                    StatusModule::BASE,
                    StatusModule::SWRST,
                    StatusModule::SWRST_VAL,
                ],
            )
            .map_err(|e| e.to_i2c_error(self.address as u16, StatusModule::SWRST as u16))?;
        Ok(())
    }
}
