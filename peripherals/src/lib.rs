//! Peripherals crate containing platform-agnostic generic driver wrappers.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Generic battery peripheral traits.
pub mod battery;
/// Generic and platform-specific pump peripherals.
pub mod pump;
/// Generic and platform-specific water sensor peripherals.
pub mod water_sensor;

/// Mock implementations of peripherals for host-based testing.
#[cfg(any(test, feature = "mock"))]
pub mod mock;
