//! Peripherals crate containing platform-agnostic generic driver wrappers.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Generic and platform-specific motor peripherals.
pub mod motor;

/// Concrete driver implementation for the ATtiny816 custom LED driver.
pub mod attiny816;
/// Concrete driver implementation for the BQ25185 battery charger.
pub mod bq25185;
/// Concrete driver implementation for the INA219 current monitor.
pub mod ina219;
/// Concrete driver implementation for the L9110S motor driver.
pub mod l9110s;
/// Concrete driver implementation for the MAX17048 fuel gauge.
pub mod max17048;
/// Concrete driver implementation for the VL53L0X proximity sensor.
pub mod vl53l0x;

/// Mock implementations of peripherals for host-based testing.
#[cfg(any(test, feature = "mock"))]
pub mod mock;
