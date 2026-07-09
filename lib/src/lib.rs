//! Shared firmware library exposing utility modules.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Gesture detection module.
pub mod gesture_detector;

/// RP2040 panic handler module.
pub mod panic_handler;

/// SWD Real-Time Transfer (RTT) logging backend.
pub mod rtt;

#[doc(hidden)]
pub mod defmt_logger;

/// Shared system state and power management utilities.
pub mod system;

/// Battery manager module.
pub mod battery_manager;

/// Power manager module.
pub mod power_manager;

/// Thermal manager module.
pub mod thermal_manager;

/// Periodic timer utility.
pub mod periodic_timer;

/// Shared I2C blocking access wrapper structures.
pub mod i2c;

/// Shared types and traits for the library.
pub mod types;

/// Heartbeat and execution liveness monitoring module.
pub mod heartbeat_monitor;

/// Telemetry storage pipeline and task.
pub use controller::telemetry_controller as telemetry;

/// Re-export run_telemetry_task macro.
pub use controller::run_telemetry_task;

pub use battery_manager::BatteryManager;
pub use gesture_detector::ProximityEvent;
pub use periodic_timer::PeriodicTimer;
pub use power_manager::PowerManager;
pub use system::{BatteryUpdateAction, TransitionError};
pub use thermal_manager::ThermalManager;
