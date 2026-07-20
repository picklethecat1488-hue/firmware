//! Platform abstraction library exposing system, I/O, telemetry, and power services.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// CLI-related helpers and structures.
#[path = "services/cli.rs"]
pub mod cli;

#[doc(hidden)]
pub use embedded_cli;

/// RP2040 panic handler module.
#[path = "system/panic.rs"]
pub mod panic_handler;

/// NOR Flash driver adapters.
#[path = "io/flash.rs"]
pub mod flash;

/// SWD Real-Time Transfer (RTT) logging backend.
#[path = "system/rtt.rs"]
pub mod rtt;

#[doc(hidden)]
#[path = "telemetry/logger.rs"]
pub mod defmt_logger;

/// Shared system state and power management utilities.
#[path = "system/scheduler.rs"]
pub mod system;

/// Battery manager module.
#[path = "power/battery.rs"]
pub mod battery_manager;

/// Power manager module.
#[path = "power/power.rs"]
pub mod power_manager;

/// Thermal manager module.
#[path = "power/thermal.rs"]
pub mod thermal_manager;

/// Periodic timer utility.
#[path = "io/timer.rs"]
pub mod periodic_timer;

/// Shared I2C blocking access wrapper structures.
#[path = "io/i2c.rs"]
pub mod i2c;

/// Shared types and traits for the library.
#[path = "system/types.rs"]
pub mod types;

/// Core and execution liveness monitoring module.
#[path = "system/monitor.rs"]
pub mod core_monitor;

/// Gesture detection library.
#[path = "services/gesture.rs"]
pub mod gesture_detector;

/// Async future demultiplexing helper utilities.
#[path = "system/select.rs"]
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
#[path = "telemetry/cbor.rs"]
pub mod cbor;

/// Shared directory index and key management utilities.
#[path = "services/directory.rs"]
pub mod directory;

/// Consolidated conditional tracing module.
#[path = "telemetry/tracing.rs"]
pub mod tracing;
