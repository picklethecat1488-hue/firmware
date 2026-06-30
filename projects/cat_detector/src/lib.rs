//! Board configuration library for the Cat Detector project.
//!
//! Defines the single source of truth for pin assignments and helper
//! initialization functions for sharing hardware setup between the main
//! controller and bringup shell binaries.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![deny(missing_docs)]

/// Onboard LED pin (GPIO 25)
pub const LED_PIN: u32 = 25;
/// Pump control pin (uses Pin 25 / Onboard LED for status feedback)
pub const PUMP_PIN: u32 = 25;
/// I2C SDA pin (GPIO 4)
pub const I2C_SDA_PIN: u32 = 4;
/// I2C SCL pin (GPIO 5)
pub const I2C_SCL_PIN: u32 = 5;
/// UART TX pin (GPIO 0)
pub const UART_TX_PIN: u32 = 0;
/// UART RX pin (GPIO 1)
pub const UART_RX_PIN: u32 = 1;

/// Start address of the filesystem storage partition in flash (offset from start of flash).
pub const STORAGE_PARTITION_START: u32 = 0x1C_0000; // 1.75 MB
/// End address of the filesystem storage partition in flash (2.00 MB limit).
pub const STORAGE_PARTITION_END: u32 = 0x20_0000; // 2.00 MB
/// Total QSPI flash memory capacity on the board (2.00 MB).
pub const FLASH_SIZE: usize = 2 * 1024 * 1024;
/// Top address of the stack/SRAM (RP2040 has 264 KB SRAM, ending at 0x2004_0000).
pub const STACK_TOP: u32 = 0x2004_0000;
/// Start address of flash memory mapping (XIP address space).
pub const FLASH_START: u32 = 0x1000_0000;
/// End address of flash memory mapping (FLASH_START + FLASH_SIZE).
pub const FLASH_END: u32 = 0x1020_0000;

#[cfg(all(target_arch = "arm", target_os = "none"))]
mod bsp_target;

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use bsp_target::*;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod bsp_host;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use bsp_host::*;

/// System state and orchestration controller.
pub mod system_controller;

/// Shared command channel for the Motor Controller.
pub static MOTOR_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::motor_controller::MotorCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the System Controller.
pub static SYSTEM_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    crate::system_controller::SystemCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the North Sensor Controller.
pub static SENSOR_NORTH_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::sensor_controller::SensorCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the East Sensor Controller.
pub static SENSOR_EAST_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::sensor_controller::SensorCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the West Sensor Controller.
pub static SENSOR_WEST_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::sensor_controller::SensorCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the Thermal Controller.
pub static THERMAL_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::thermal_controller::ThermalCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the Battery Controller.
pub static BATTERY_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::battery_controller::BatteryCommand,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for the System LED status updates.
pub static LED_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    model::types::SystemLedState,
    4,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for telemetry records.
pub static TELEMETRY_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    model::types::TelemetryRecord,
    16,
> = embassy_sync::channel::Channel::new();

/// Shared command channel for filesystem operations.
pub static FILESYSTEM_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::filesystem_controller::FsRequest,
    16,
> = embassy_sync::channel::Channel::new();

/// Log a telemetry record to the global asynchronous pipeline.
pub fn log_telemetry(record: model::types::TelemetryRecord) {
    let _ = TELEMETRY_CHANNEL.try_send(record);
}

/// Re-export the telemetry module from the shared library
pub use firmware_lib::telemetry;

/// Re-export the run_telemetry_task macro from the shared library
pub use firmware_lib::run_telemetry_task;

/// Re-export the run_filesystem_task macro from the controller crate
pub use controller::run_filesystem_task;

/// Re-export the modular panic handler function
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use firmware_lib::panic_handler::handle_panic;

/// Re-export the modular panic handler initialization
pub use firmware_lib::panic_handler::init as init_panic_handler;

/// Re-export the modular logging helper function
pub use firmware_lib::panic_handler::log_system;

/// Re-export the modular log_info! macro from the panic handler
pub use firmware_lib::log_info;

/// Returns the current system uptime in microseconds since boot (64-bit precision).
pub fn system_time() -> u64 {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        unsafe {
            let timer_high_addr = 0x4005_4008 as *const u32;
            let timer_low_addr = 0x4005_400c as *const u32;
            let mut high = *timer_high_addr;
            let mut low = *timer_low_addr;
            let high2 = *timer_high_addr;
            if high != high2 {
                high = high2;
                low = *timer_low_addr;
            }
            ((high as u64) << 32) | (low as u64)
        }
    }
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    {
        static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        let start = *START.get_or_init(std::time::Instant::now);
        std::time::Instant::now().duration_since(start).as_micros() as u64
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
defmt::timestamp!("{=u64:us}", system_time());
