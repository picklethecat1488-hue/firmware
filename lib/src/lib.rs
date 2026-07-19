//! Shared firmware library exposing utility modules.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// CLI-related helpers and structures.
pub mod cli;

#[doc(hidden)]
pub use embedded_cli;

/// RP2040 panic handler module.
pub mod panic_handler;

/// NOR Flash driver adapters.
pub mod flash;

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

/// Core and execution liveness monitoring module.
pub mod core_monitor;

/// Gesture detection library.
pub mod gesture_detector;

/// Async future demultiplexing helper utilities.
pub mod select;

pub use battery_manager::BatteryManager;
pub use flash::BlockingAsyncFlash;
pub use gesture_detector::{GestureDetector, ProximityEvent, ProximityGestureDetector};
pub use periodic_timer::PeriodicTimer;
pub use power_manager::PowerManager;
pub use system::{transition_thermal_update, BatteryUpdateAction, TransitionError};
pub use thermal_manager::ThermalManager;
pub use types::{
    BootTrapMask, BootTrapReason, InvalidBootTrapMask, ThermalTransitionResult, ThermalUpdateAction,
};

/// Compile-time CBOR serialization helpers.
pub mod cbor;

/// Shared directory index and key management utilities.
pub mod directory;

/// Consolidated conditional tracing module.
pub mod tracing;
