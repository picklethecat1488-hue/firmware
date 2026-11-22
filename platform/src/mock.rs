#![cfg(any(test, not(all(target_arch = "arm", target_os = "none"))))]
//! Mock structures for testing and diagnostics.

use crate::i2c::I2cResolver;
use core::cell::RefCell;
use core::convert::Infallible;
use embedded_io::{ErrorType, Write};

/// Mock writer that collects written bytes in an internal buffer.
pub struct MockWriter {
    /// Internal buffer containing the written bytes.
    pub buf: heapless::Vec<u8, 2048>,
}

impl Default for MockWriter {
    fn default() -> Self {
        Self {
            buf: heapless::Vec::new(),
        }
    }
}

impl ErrorType for MockWriter {
    type Error = Infallible;
}

impl Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let _ = self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Mock I2C transaction error.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MockI2cError {
    /// Target peripheral did not acknowledge its 7-bit address.
    NoAcknowledge,
}

impl embedded_hal_async::i2c::Error for MockI2cError {
    fn kind(&self) -> embedded_hal_async::i2c::ErrorKind {
        match self {
            Self::NoAcknowledge => embedded_hal_async::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_async::i2c::NoAcknowledgeSource::Address,
            ),
        }
    }
}

/// Mock I2C device backed by an 8-bit register array plus sparse 16-bit register storage at `active_address`.
pub struct MockI2c {
    /// Currently active 7-bit address on the bus.
    pub active_address: u8,
    /// Simulated 256-byte register file for 8-bit register offsets.
    pub registers: [u8; 256],
    /// Simulated sparse register file for 16-bit big-endian register offsets.
    pub registers_16: heapless::LinearMap<u16, u8, 64>,
    /// Whether this mock operates in 16-bit register-addressing mode (`2`-byte register prefix).
    pub use_16bit_reg_addr: bool,
    /// Raw bytes sent during the most recent `write` or `write_read` address phase.
    pub last_tx_bytes: heapless::Vec<u8, 16>,
}

impl MockI2c {
    /// Creates a new `MockI2c` responding at `active_address` with zeroed 8-bit registers.
    pub const fn new(active_address: u8) -> Self {
        Self {
            active_address,
            registers: [0u8; 256],
            registers_16: heapless::LinearMap::new(),
            use_16bit_reg_addr: false,
            last_tx_bytes: heapless::Vec::new(),
        }
    }
}

impl embedded_hal_async::i2c::ErrorType for MockI2c {
    type Error = MockI2cError;
}

impl embedded_hal_async::i2c::I2c for MockI2c {
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        if address != self.active_address {
            return Err(MockI2cError::NoAcknowledge);
        }
        for (idx, b) in read.iter_mut().enumerate() {
            *b = self.registers[idx & 0xFF];
        }
        Ok(())
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        if address != self.active_address {
            return Err(MockI2cError::NoAcknowledge);
        }
        self.last_tx_bytes.clear();
        let _ = self.last_tx_bytes.extend_from_slice(write);
        if self.use_16bit_reg_addr {
            if write.len() >= 2 {
                let base_reg = u16::from_be_bytes([write[0], write[1]]);
                for (offset, &byte) in write[2..].iter().enumerate() {
                    let _ = self
                        .registers_16
                        .insert(base_reg.wrapping_add(offset as u16), byte);
                }
            }
        } else if let Some((&reg, payload)) = write.split_first() {
            for (offset, &byte) in payload.iter().enumerate() {
                let idx = reg.wrapping_add(offset as u8) as usize;
                self.registers[idx] = byte;
            }
        }
        Ok(())
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        if address != self.active_address {
            return Err(MockI2cError::NoAcknowledge);
        }
        self.last_tx_bytes.clear();
        let _ = self.last_tx_bytes.extend_from_slice(write);
        if self.use_16bit_reg_addr || write.len() >= 2 {
            let base_reg = if write.len() >= 2 {
                u16::from_be_bytes([write[0], write[1]])
            } else {
                write.first().copied().unwrap_or(0) as u16
            };
            for (offset, b) in read.iter_mut().enumerate() {
                let key = base_reg.wrapping_add(offset as u16);
                *b = self.registers_16.get(&key).copied().unwrap_or(0);
            }
        } else {
            let base_reg = write.first().copied().unwrap_or(0);
            for (offset, b) in read.iter_mut().enumerate() {
                let idx = base_reg.wrapping_add(offset as u8) as usize;
                *b = self.registers[idx];
            }
        }
        Ok(())
    }

    async fn transaction(
        &mut self,
        address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        if address != self.active_address {
            return Err(MockI2cError::NoAcknowledge);
        }
        Ok(())
    }
}

/// Mock I2C Resolver.
pub struct MockI2cResolver {
    /// RefCell wrapping the mock I2C peripheral.
    pub i2c: RefCell<MockI2c>,
}

impl I2cResolver for MockI2cResolver {
    type I2c = MockI2c;
    fn resolve_i2c(&self, name: Option<&str>) -> Result<&mut Self::I2c, &'static str> {
        if let Some(bus_name) = name {
            if bus_name != "i2c0" && bus_name != "default" {
                return Err("Unknown I2C bus");
            }
        }
        Ok(unsafe { &mut *self.i2c.as_ptr() })
    }
}
