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

/// Mock I2C device.
pub struct MockI2c {
    /// Currently active address on the bus.
    pub active_address: u8,
}

impl embedded_hal_async::i2c::ErrorType for MockI2c {
    type Error = Infallible;
}

impl embedded_hal_async::i2c::I2c for MockI2c {
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        if address == self.active_address {
            for b in read.iter_mut() {
                *b = 0;
            }
        }
        Ok(())
    }

    async fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
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
    fn resolve_i2c(&self, _name: Option<&str>) -> Result<&mut Self::I2c, &'static str> {
        Ok(unsafe { &mut *self.i2c.as_ptr() })
    }
}
