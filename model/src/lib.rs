//! Model crate containing target-agnostic state machines and models.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Calibration types.
pub mod calibration;
/// Hardware peripheral interfaces.
pub mod interfaces;
/// Telemetry types and serialization.
pub mod telemetry;
/// Domain types.
pub mod types;
