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
/// Bringup serial command and shell controller.
pub mod shell_controller;
/// System state and orchestration controller.
pub mod system_controller;
/// Telemetry storage pipeline and task.
pub mod telemetry_controller;
/// Thermal monitoring and regulation controller.
pub mod thermal_controller;
/// Controller-specific common types.
pub mod types;

pub use battery_controller::BatteryCommand;
pub use battery_controller::BatteryFeatureConfig;
pub use embedded_cli;
pub use embedded_io;
pub use led_controller::LedFeatureConfig;
pub use motor_controller::MotorCommand;
pub use motor_controller::MotorFeatureConfig;
pub use sensor_controller::ProximityFeatureConfig;
pub use sensor_controller::SensorCommand;
pub use shell_controller::{ShellConfig, ShellDeviceResolver};
pub use system_controller::{
    FeatureList, ProximityEvent, SystemCommand, SystemController, SystemFeature, SystemFeatureSet,
};
pub use thermal_controller::ThermalCommand;
pub use thermal_controller::ThermalFeatureConfig;
pub use types::{
    BatteryStatus, Device, DeviceSupport, FlashPartition, GestureAction, MotorCalState, MotorError,
    MotorSafetyStatus, MotorState, NamedDevice, NamedPartition, ProximityAction, SensorDirection,
    ThermalState,
};

/// Source of truth for all controllers and their channel/message types.
#[macro_export]
macro_rules! define_controllers {
    (
        $(
            $name:ident {
                channel: $channel:ident,
                sender: $sender:ident,
                receiver: $receiver:ident,
                msg: $msg:ty,
            }
        )*
    ) => {
        $(
            /// Channel type for controller communication.
            pub type $channel<MutexRaw, const N: usize> =
                embassy_sync::channel::Channel<MutexRaw, $msg, N>;
            /// Sender type for controller communication.
            pub type $sender<MutexRaw, const N: usize> =
                embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
            /// Receiver type for controller communication.
            pub type $receiver<MutexRaw, const N: usize> =
                embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;
        )*
    };
}

define_controllers! {
    Led {
        channel: LedChannel,
        sender: LedSender,
        receiver: LedReceiver,
        msg: model::types::SystemLedState,
    }
    Battery {
        channel: BatteryChannel,
        sender: BatterySender,
        receiver: BatteryReceiver,
        msg: crate::battery_controller::BatteryCommand,
    }
    Thermal {
        channel: ThermalChannel,
        sender: ThermalSender,
        receiver: ThermalReceiver,
        msg: crate::thermal_controller::ThermalCommand,
    }
    Sensor {
        channel: SensorChannel,
        sender: SensorSender,
        receiver: SensorReceiver,
        msg: crate::sensor_controller::SensorCommand,
    }
    Motor {
        channel: MotorChannel,
        sender: MotorSender,
        receiver: MotorReceiver,
        msg: crate::motor_controller::MotorCommand,
    }
    Filesystem {
        channel: FilesystemChannel,
        sender: FilesystemSender,
        receiver: FilesystemReceiver,
        msg: crate::filesystem_controller::FsRequest,
    }
    System {
        channel: SystemChannel,
        sender: SystemSender,
        receiver: SystemReceiver,
        msg: crate::system_controller::SystemCommand,
    }
    Telemetry {
        channel: TelemetryChannel,
        sender: TelemetrySender,
        receiver: TelemetryReceiver,
        msg: model::telemetry::TelemetryRecord,
    }
}

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
    /// Set motor speed.
    fn set_motor_speed(&mut self, speed: i8) -> Result<(), PeripheralError>;
    /// Stop the motor.
    fn stop(&mut self) -> Result<(), PeripheralError>;
}

impl BlockingMotorWriter for () {
    fn set_motor_speed(&mut self, _: i8) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
    fn stop(&mut self) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

/// Trait for system orchestrator writer operations.
pub trait BlockingSystemWriter {
    /// Resets the inactivity timeout.
    fn record_activity(&mut self) -> Result<(), PeripheralError>;
}

impl BlockingSystemWriter for () {
    fn record_activity(&mut self) -> Result<(), PeripheralError> {
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
                rx: $crate::ThermalReceiver<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    4,
                >,
                telemetry_tx: $crate::TelemetrySender<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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
                rx: $crate::BatteryReceiver<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    4,
                >,
                telemetry_tx: $crate::TelemetrySender<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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
                rx: $crate::MotorReceiver<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    4,
                >,
                telemetry_tx: $crate::TelemetrySender<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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
                rx: $crate::SensorReceiver<$raw_mutex, 4>,
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
                rx: $crate::LedReceiver<$raw_mutex, 4>,
                telemetry_tx: $crate::TelemetrySender<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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
                rx: $crate::FilesystemReceiver<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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
                rx: $crate::TelemetryReceiver<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
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

// Define generic type aliases to be used by implementations
pub use embassy_sync::channel::Channel;
pub use embassy_sync::channel::Receiver;
pub use embassy_sync::channel::Sender;

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    BlockingMotorWriter for MotorSender<MutexRaw, N>
{
    fn set_motor_speed(&mut self, speed: i8) -> Result<(), PeripheralError> {
        let motor_speed =
            model::types::MotorSpeed::new(speed).ok_or(PeripheralError::InvalidConfiguration)?;
        self.try_send(MotorCommand::SetSpeed(motor_speed))
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
    fn stop(&mut self) -> Result<(), PeripheralError> {
        self.try_send(MotorCommand::Stop)
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
}

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    BlockingSystemWriter for SystemSender<MutexRaw, N>
{
    fn record_activity(&mut self) -> Result<(), PeripheralError> {
        self.try_send(SystemCommand::ActivityDetected)
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
}
