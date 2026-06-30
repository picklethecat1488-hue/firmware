//! Shared firmware library exposing utility modules.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Gesture detection module.
pub mod gesture_detector;

/// RP2040 panic handler module.
pub mod panic_handler;
