//! Controller crate containing project-agnostic orchestrators.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Battery status and telemetry controller.
pub mod battery_controller;
/// Flat filesystem and storage controller.
pub mod filesystem_controller;
/// LED controller to drive indicator RGB LEDs.
pub mod led_controller;
/// Motor status and telemetry controller.
pub mod motor_controller;
/// Sensor controller for Time-of-Flight sensors.
pub mod sensor_controller;
/// Telemetry storage pipeline and task.
pub mod telemetry_controller;
/// Thermal monitoring and regulation controller.
pub mod thermal_controller;

pub use battery_controller::BatteryCommand;
pub use motor_controller::MotorCommand;
pub use sensor_controller::SensorCommand;
pub use thermal_controller::ThermalCommand;

use model::types::PeripheralError;

/// Trait for reading battery status blocking-ly.
pub trait BlockingBatteryReader {
    /// Read voltage (mV) and state of charge (%).
    fn read_battery_blocking(&self) -> Result<(u32, u8), PeripheralError>;
}

/// Trait for reading temperature blocking-ly.
pub trait BlockingThermalReader {
    /// Read temperature in milli-Celsius.
    fn read_temperature_blocking(&self) -> Result<i32, PeripheralError>;
}

/// Trait for reading proximity distance blocking-ly.
pub trait BlockingProximityReader {
    /// Read distance in millimeters.
    fn read_distance_blocking(&mut self) -> Result<u16, PeripheralError>;
}

impl BlockingBatteryReader for () {
    fn read_battery_blocking(&self) -> Result<(u32, u8), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

impl BlockingThermalReader for () {
    fn read_temperature_blocking(&self) -> Result<i32, PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

impl BlockingProximityReader for () {
    fn read_distance_blocking(&mut self) -> Result<u16, PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

/// Trait for reading motor current/torque blocking-ly.
pub trait BlockingMotorReader {
    /// Read motor current in mA.
    fn read_current_ma_blocking(&mut self) -> Result<i32, PeripheralError>;
}

impl BlockingMotorReader for () {
    fn read_current_ma_blocking(&mut self) -> Result<i32, PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

/// Trait for controlling motor speed.
pub trait BlockingMotorWriter {
    /// Read motor current in mA.
    fn set_motor_speed(&mut self, speed: u8) -> Result<(), PeripheralError>;
}

impl BlockingMotorWriter for () {
    fn set_motor_speed(&mut self, _: u8) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

/// A macro to define and spawn the Thermal Controller task.
///
/// Generates the task definition generic over the battery driver type,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_thermal_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $telemetry_tx:expr,
        $battery_type:ty,
        $cmd_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::thermal_controller::ThermalController<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $battery_type,
                    $cmd_type,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::thermal_controller::ThermalCommand,
                    4,
                >,
                telemetry_tx: embassy_sync::channel::Sender<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::telemetry::TelemetryRecord,
                    { $crate::telemetry_controller::CHANNEL_CAPACITY },
                >,
            ) {
                controller.run(rx, telemetry_tx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx, $telemetry_tx))
            .unwrap();
    };
}

/// A macro to define and spawn the Battery Controller task.
///
/// Generates the task definition generic over the battery driver type,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_battery_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $telemetry_tx:expr,
        $battery_type:ty,
        $charger_type:ty,
        $pin_type:ty,
        $cmd_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::battery_controller::BatteryController<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $battery_type,
                    $charger_type,
                    $pin_type,
                    $cmd_type,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::battery_controller::BatteryCommand,
                    4,
                >,
                telemetry_tx: embassy_sync::channel::Sender<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::telemetry::TelemetryRecord,
                    { $crate::telemetry_controller::CHANNEL_CAPACITY },
                >,
            ) {
                controller.run(rx, telemetry_tx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx, $telemetry_tx))
            .unwrap();
    };
}

/// A macro to define and spawn the Motor Controller task.
///
/// Generates the task definition generic over the motor and current sensor types,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_motor_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $telemetry_tx:expr,
        $motor_type:ty,
        $current_sensor_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::motor_controller::MotorController<
                    $motor_type,
                    $current_sensor_type,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::motor_controller::MotorCommand,
                    4,
                >,
                telemetry_tx: embassy_sync::channel::Sender<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::telemetry::TelemetryRecord,
                    { $crate::telemetry_controller::CHANNEL_CAPACITY },
                >,
            ) {
                controller.run(rx, telemetry_tx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx, $telemetry_tx))
            .unwrap();
    };
}

/// A macro to define and spawn the Sensor Controller task.
///
/// Generates the task definition generic over the proximity sensor,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_sensor_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $sensor_type:ty,
        $raw_mutex:ty,
        $pin_type:ty,
        $cmd_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::sensor_controller::SensorController<
                    'static,
                    $sensor_type,
                    $raw_mutex,
                    $pin_type,
                    $cmd_type,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    $raw_mutex,
                    $crate::sensor_controller::SensorCommand,
                    4,
                >,
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
            .unwrap();
    };
}

/// A macro to define and spawn the LED Controller task.
///
/// Generates the task definition generic over the LED driver,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_led_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $telemetry_tx:expr,
        $driver_type:ty,
        $raw_mutex:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::led_controller::LedController<$driver_type>,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    $raw_mutex,
                    model::types::SystemLedState,
                    4,
                >,
                telemetry_tx: embassy_sync::channel::Sender<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::telemetry::TelemetryRecord,
                    { $crate::telemetry_controller::CHANNEL_CAPACITY },
                >,
            ) {
                controller.run(rx, telemetry_tx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx, $telemetry_tx))
            .unwrap();
    };
}

/// A macro to define and spawn the Filesystem Controller task.
///
/// Generates the task definition generic over the flash type,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_filesystem_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $flash_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                fs: $crate::filesystem_controller::FilesystemController<$flash_type>,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::filesystem_controller::FsRequest,
                    16,
                >,
            ) {
                $crate::filesystem_controller::run_filesystem_task(fs, rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
            .unwrap();
    };
}

/// A macro to define and spawn the Telemetry Controller task.
///
/// Generates the task definition generic over the max record count and buffer size,
/// then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_telemetry_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $max_records:expr
    ) => {
        $crate::run_telemetry_task!($spawner, $task_module, $controller, $rx, $max_records, 16);
    };
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr,
        $max_records:expr,
        $channel_size:expr
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                mut controller: &'static mut $crate::telemetry_controller::TelemetryController<
                    $max_records,
                    { model::telemetry::BUFFER_SIZE },
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::telemetry::TelemetryRecord,
                    $channel_size,
                >,
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
            .unwrap();
    };
}
