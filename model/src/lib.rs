//! Model crate containing target-agnostic state machines and models.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Domain types.
pub mod types;
/// Hardware peripheral interfaces.
pub mod interfaces;
