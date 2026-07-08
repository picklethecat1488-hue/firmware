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

/// ToF Sensor 1 (North) XSHUT pin (GPIO 2)
pub const TOF_NORTH_XSHUT_PIN: u32 = 2;
/// ToF Sensor 2 (East) XSHUT pin (GPIO 3)
pub const TOF_EAST_XSHUT_PIN: u32 = 3;
/// ToF Sensor 3 (West) XSHUT pin (GPIO 6)
pub const TOF_WEST_XSHUT_PIN: u32 = 6;

/// ToF Sensor 1 (North) Interrupt pin (GPIO 7)
pub const TOF_NORTH_INT_PIN: u32 = 7;
/// ToF Sensor 2 (East) Interrupt pin (GPIO 8)
pub const TOF_EAST_INT_PIN: u32 = 8;
/// ToF Sensor 3 (West) Interrupt pin (GPIO 9)
pub const TOF_WEST_INT_PIN: u32 = 9;

/// Fuel Gauge Interrupt/Alert pin (GPIO 10)
pub const FUEL_GAUGE_INT_PIN: u32 = 10;

/// The default proximity threshold in millimeters under which target presence is detected.
pub const DEFAULT_PROXIMITY_THRESHOLD_MM: u16 = 300;

/// Charger Status 1 (S1 / STAT1 / FAULT) pin (GPIO 12)
pub const CHARGER_S1_PIN: u32 = 12;
/// Charger Status 2 (S2 / STAT2 / CHG) pin (GPIO 13)
pub const CHARGER_S2_PIN: u32 = 13;

/// Start address of the filesystem storage partition in flash (offset from start of flash).
pub const STORAGE_PARTITION_START: u32 = 0x1C_0000; // 1.75 MB
/// End address of the filesystem storage partition in flash (2.00 MB limit).
pub const STORAGE_PARTITION_END: u32 = 0x20_0000; // 2.00 MB
/// Total QSPI flash memory capacity on the board (2.00 MB).
pub const FLASH_SIZE: usize = 2 * 1024 * 1024;
/// Top address of the stack/SRAM (RP2040 has 264 KB SRAM, ending at 0x2004_0000).
pub const STACK_TOP: u32 = 0x2004_2000;
/// Start address of flash memory mapping (XIP address space).
pub const FLASH_START: u32 = 0x1000_0000;
/// End address of flash memory mapping (FLASH_START + FLASH_SIZE).
pub const FLASH_END: u32 = 0x1020_0000;
/// Flash page write size in bytes.
pub const FLASH_WRITE_SIZE: usize = 1;
/// Flash erase block size in bytes.
pub const FLASH_ERASE_SIZE: usize = 4096;

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

/// Bringup serial command and shell controller.
pub mod shell_controller;

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
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = embassy_sync::channel::Channel::new();

/// Shared command channel for filesystem operations.
pub static FILESYSTEM_CHANNEL: embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    controller::filesystem_controller::FsRequest,
    16,
> = embassy_sync::channel::Channel::new();

/// Re-export the telemetry module from the shared library
pub use firmware_lib::telemetry;

/// Re-export the run_telemetry_task macro from the shared library
pub use firmware_lib::run_telemetry_task;

/// Re-export the run_filesystem_task macro from the controller crate
pub use controller::run_filesystem_task;

/// Re-export the modular panic handler function
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use firmware_lib::panic_handler::handle_panic_with_sizes;

/// Re-export the modular panic handler initialization
pub use firmware_lib::panic_handler::init as init_panic_handler;

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

/// Represents the physical directions of ToF proximity sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorDirection {
    /// North sensor
    North,
    /// East sensor
    East,
    /// West sensor
    West,
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for SensorDirection {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "north" => Ok(SensorDirection::North),
            "east" => Ok(SensorDirection::East),
            "west" => Ok(SensorDirection::West),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'north', 'east', or 'west'",
            }),
        }
    }
}

/// Derived command enum representing all supported user commands.
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum CliCommand {
    /// Motor speed control (motor <speed>)
    Motor {
        /// Speed value (0-100)
        speed: u8,
    },
    /// Stop the motor
    Stop,
    /// Query battery voltage and status
    Battery,
    /// Query thermal sensor and status
    Thermal,
    /// Query proximity (ToF) sensors
    Proximity,
    /// Wake the system to Active state
    Wake,
    /// Put the system to Sleep state
    Sleep,
    /// Simulate activity event
    Activity,
    /// Trigger a panic to test the crash dump / panic flow
    Crash,
    /// Calibrate ToF sensors with target held at the cover (0mm)
    #[command(name = "cal_near")]
    CalNear {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
    /// Calibrate ToF sensors with target held at 100mm
    #[command(name = "cal_far")]
    CalFar {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
    /// Calibrate motor current levels (cal_motor <empty|100ml|full>)
    #[command(name = "cal_motor")]
    CalMotor {
        /// Calibration state ('empty', '100ml', or 'full')
        state: MotorCalState,
    },
    /// Read the RP2040 system temperature
    #[command(name = "mcu_temp")]
    McuTemp,
    /// Format/erase the filesystem partition
    Format,
}

/// Represents the motor calibration target state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorCalState {
    /// Empty water bowl
    Empty,
    /// Bowl with 100ml of water
    Water100ml,
    /// Full water bowl
    Full,
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for MotorCalState {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "empty" => Ok(MotorCalState::Empty),
            "100ml" => Ok(MotorCalState::Water100ml),
            "full" => Ok(MotorCalState::Full),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'empty', '100ml', or 'full'",
            }),
        }
    }
}

/// Embedded project metadata for autodetect functionality.
#[used]
#[no_mangle]
#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".rodata.project_metadata"
)]
pub static PROJECT_METADATA: firmware_lib::types::ProjectMetadata =
    firmware_lib::types::ProjectMetadata {
        magic: *b"PROJMET\0",
        version: 1,
        chip: {
            let mut buf = [0u8; 32];
            let bytes = b"rp2040";
            let mut i = 0;
            while i < bytes.len() {
                buf[i] = bytes[i];
                i += 1;
            }
            buf
        },
        partition_address: 0x10000000 + STORAGE_PARTITION_START,
        partition_size: (STORAGE_PARTITION_END - STORAGE_PARTITION_START),
        flash_write_size: FLASH_WRITE_SIZE as u32,
        flash_erase_size: FLASH_ERASE_SIZE as u32,
        stack_scan_limit: firmware_lib::types::STACK_SCAN_LIMIT,
    };

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// A wrapper structure containing the initialized I2C0 peripheral on target.
pub struct SafeI2c(
    pub  Option<
        embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
    >,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Thread-safe Mutex wrapping the active I2C peripheral for shared access between tasks.
pub static SHARED_I2C: embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<SafeI2c>,
> = embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(SafeI2c(None)));

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[derive(Clone, Copy)]
/// A unit struct wrapper that implements `embedded_hal::i2c::I2c` by dynamically locking `SHARED_I2C`.
pub struct SharedI2cWrapper;

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl embedded_hal::i2c::ErrorType for SharedI2cWrapper {
    type Error = embassy_rp::i2c::Error;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl embedded_hal::i2c::I2c for SharedI2cWrapper {
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        SHARED_I2C.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.read(address, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        SHARED_I2C.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write(address, write)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        SHARED_I2C.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.write_read(address, write, read)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        SHARED_I2C.lock(|cell| {
            let mut guard = cell.borrow_mut();
            if let Some(ref mut i2c) = guard.0 {
                i2c.transaction(address, operations)
            } else {
                Err(embassy_rp::i2c::Error::Abort(
                    embassy_rp::i2c::AbortReason::Other(0),
                ))
            }
        })
    }
}
