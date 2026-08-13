//! Model crate containing target-agnostic state machines and models.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![allow(async_fn_in_trait)]

/// Calibration types.
pub mod calibration;
/// Gesture detection models.
pub mod gesture;
/// Hardware peripheral interfaces.
pub mod interfaces;
/// Telemetry types and serialization.
pub mod telemetry;
/// Domain types.
pub mod types;
