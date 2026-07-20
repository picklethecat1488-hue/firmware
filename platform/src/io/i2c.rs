//! Shared I2C blocking access wrapper structures.

#![cfg(all(target_arch = "arm", target_os = "none"))]

use core::cell::RefCell;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

/// A wrapper structure containing the initialized I2C0 peripheral on target.
pub struct SafeI2c(
    pub  Option<
        embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    >,
);

#[derive(Clone, Copy)]
/// A unit struct wrapper that implements `embedded_hal::i2c::I2c` by dynamically locking a Shared I2C Mutex.
pub struct SharedI2cWrapper<'a> {
    mutex: &'a Mutex<CriticalSectionRawMutex, RefCell<SafeI2c>>,
}

impl<'a> SharedI2cWrapper<'a> {
    /// Creates a new SharedI2cWrapper wrapping a Mutex.
    pub const fn new(mutex: &'a Mutex<CriticalSectionRawMutex, RefCell<SafeI2c>>) -> Self {
        Self { mutex }
    }
}

impl<'a> embedded_hal::i2c::ErrorType for SharedI2cWrapper<'a> {
    type Error = embassy_rp::i2c::Error;
}

impl<'a> embedded_hal::i2c::I2c for SharedI2cWrapper<'a> {
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.read(address, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write(address, write)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write_read(address, write, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.transaction(address, operations)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }
}
