#![allow(
    unused_mut,
    unused_variables,
    unused_imports,
    clippy::match_single_binding,
    clippy::assertions_on_constants
)]

use embassy_sync::blocking_mutex::raw::RawMutex;

// Re-export controller submodules so crate:: paths resolve
pub use controller::battery_controller;
pub use controller::filesystem_controller;
pub use controller::motor_controller;
pub use controller::sensor_controller;
pub use controller::system_controller;
pub use controller::thermal_controller;

// Mock tracing module for #[crate::tracing::controller_context]
pub mod tracing {
    pub use platform::tracing::controller_context;
}

// Include the generated mock structures, clients, and aliases
include!(concat!(env!("OUT_DIR"), "/generated_test_mocks.rs"));

// Include the generated run loops
include!(concat!(env!("OUT_DIR"), "/generated_runloops.rs"));

#[test]
fn test_boilerplate_compiles() {
    // This is a static compilation check test. If the test builds, compilation verification succeeded.
}
