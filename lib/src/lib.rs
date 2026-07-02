//! Shared firmware library exposing utility modules.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Gesture detection module.
pub mod gesture_detector;

/// RP2040 panic handler module.
pub mod panic_handler;

/// Shared system state and power management utilities.
pub mod system;

/// Telemetry storage pipeline and task.
pub use controller::telemetry_controller as telemetry;

/// Re-export run_telemetry_task macro.
pub use controller::run_telemetry_task;
