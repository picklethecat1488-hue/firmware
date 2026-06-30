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
/// Fountain state machine.
pub mod state_machine;
/// Thermal monitoring and regulation controller.
pub mod thermal_controller;

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
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
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
        $battery_type:ty
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::battery_controller::BatteryController<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $battery_type,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::battery_controller::BatteryCommand,
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
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
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
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
            .unwrap();
    };
}
