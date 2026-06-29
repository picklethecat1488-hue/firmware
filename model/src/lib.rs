//! Model crate containing target-agnostic state machines and models.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Pure state machine domain model.
pub mod state_machine;
/// Domain status models for telemetry and state tracking.
pub mod status;
