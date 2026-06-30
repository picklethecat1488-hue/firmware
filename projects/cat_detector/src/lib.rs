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

/// Shared command channel for the Sensor Controller.
pub static SENSOR_CHANNEL: embassy_sync::channel::Channel<
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
