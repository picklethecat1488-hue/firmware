//! Shared firmware library exposing utility modules.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Gesture detection module.
pub mod gesture_detector;

/// RP2040 panic handler module.
pub mod panic_handler;

/// Reusable hardware bringup shell console infrastructure.
pub mod uart;

/// SWD Real-Time Transfer (RTT) logging backend.
pub mod rtt;

#[doc(hidden)]
pub mod defmt_logger;

/// Shared system state and power management utilities.
pub mod system;

/// Shared types and traits for the library.
pub mod types;

/// Telemetry storage pipeline and task.
pub use controller::telemetry_controller as telemetry;

/// Re-export run_telemetry_task macro.
pub use controller::run_telemetry_task;
