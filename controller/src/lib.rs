//! Controller crate containing project-agnostic orchestrators.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Target-safe maximum duration (1 year) to prevent time-queue addition overflows in embassy-time.
pub const OVERFLOW_SAFE_MAX_DURATION: embassy_time::Duration =
    embassy_time::Duration::from_secs(3600 * 24 * 365);

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
/// System feature trait and tuples list dispatcher.
pub mod system_feature;
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
pub use system_controller::{ProximityEvent, SystemCommand, SystemController, SystemFeatureSet};
pub use system_feature::{FeatureList, Periodic, PeriodicInterval, SystemFeature};
pub use thermal_controller::ThermalCommand;
pub use thermal_controller::ThermalFeatureConfig;
pub use types::{
    BatteryStatus, Device, DeviceSupport, FlashPartition, GestureAction, MotorCalState, MotorError,
    MotorSafetyStatus, MotorState, NamedDevice, NamedPartition, ProximityAction, SensorDirection,
    ThermalState, ThermalUpdateAction,
};

/// Consolidated tracing facade module from firmware_lib.
pub use firmware_lib::tracing;

/// Source of truth macro for generating all controller types, channels, and task running helper macros.
#[macro_export]
macro_rules! define_controllers {
    // Rule 1: Task with telemetry_tx
    (
        $name:ident {
            channel: $channel:ident,
            sender: $sender:ident,
            receiver: $receiver:ident,
            msg: $msg:ty,
            task: $run_macro:ident {
                generics: ($($gen:tt)*),
                controller: [$($controller_ty:tt)*],
                rx: [$($rx_ty:tt)*],
                telemetry_tx: [$($telemetry_tx_ty:tt)*],
                call: |$c:ident, $r:ident, $t:ident| $body:expr
            }
        }
        $($rest:tt)*
    ) => {
        /// Channel type for controller communication.
        pub type $channel<MutexRaw, const N: usize> =
            embassy_sync::channel::Channel<MutexRaw, $msg, N>;
        /// Sender type for controller communication.
        pub type $sender<MutexRaw, const N: usize> =
            embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
        /// Receiver type for controller communication.
        pub type $receiver<MutexRaw, const N: usize> =
            embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;

        /// Task runner macro for the controller.
        #[macro_export]
        macro_rules! $run_macro {
            (
                $spawner:expr,
                $task_module:ident,
                $controller:expr,
                $rx:expr,
                $telemetry_tx:expr,
                $($gen)*
            ) => {
                #[allow(non_snake_case)]
                mod $task_module {
                    use super::*;
                    #[cfg(feature = "tracing")]
                    use $crate::tracing::tracing_defmt;

                    #[ $crate::tracing::instrument(name = stringify!($task_module), level = "info", skip($c, $r, $t)) ]
                    #[embassy_executor::task]
                    #[allow(unreachable_code)]
                    pub async fn task(
                        mut $c: $($controller_ty)*,
                        $r: $($rx_ty)*,
                        $t: $($telemetry_tx_ty)*
                    ) {
                        $body;
                    }
                }

                $spawner
                    .spawn($task_module::task($controller, $rx, $telemetry_tx))
                    .unwrap();
            };
        }

        $crate::define_controllers! { $($rest)* }
    };

    // Rule 2: Task without telemetry_tx
    (
        $name:ident {
            channel: $channel:ident,
            sender: $sender:ident,
            receiver: $receiver:ident,
            msg: $msg:ty,
            task: $run_macro:ident {
                generics: ($($gen:tt)*),
                controller: [$($controller_ty:tt)*],
                rx: [$($rx_ty:tt)*],
                call: |$c:ident, $r:ident| $body:expr
            }
        }
        $($rest:tt)*
    ) => {
        /// Channel type for controller communication.
        pub type $channel<MutexRaw, const N: usize> =
            embassy_sync::channel::Channel<MutexRaw, $msg, N>;
        /// Sender type for controller communication.
        pub type $sender<MutexRaw, const N: usize> =
            embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
        /// Receiver type for controller communication.
        pub type $receiver<MutexRaw, const N: usize> =
            embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;

        /// Task runner macro for the controller.
        #[macro_export]
        macro_rules! $run_macro {
            (
                $spawner:expr,
                $task_module:ident,
                $controller:expr,
                $rx:expr,
                $($gen)*
            ) => {
                #[allow(non_snake_case)]
                mod $task_module {
                    use super::*;
                    #[cfg(feature = "tracing")]
                    use $crate::tracing::tracing_defmt;

                    #[ $crate::tracing::instrument(name = stringify!($task_module), level = "info", skip($c, $r)) ]
                    #[embassy_executor::task]
                    #[allow(unreachable_code)]
                    pub async fn task(
                        mut $c: $($controller_ty)*,
                        $r: $($rx_ty)*
                    ) {
                        $body;
                    }
                }

                $spawner
                    .spawn($task_module::task($controller, $rx))
                    .unwrap();
            };
        }

        $crate::define_controllers! { $($rest)* }
    };

    // Rule 3: No task at all (e.g. System)
    (
        $name:ident {
            channel: $channel:ident,
            sender: $sender:ident,
            receiver: $receiver:ident,
            msg: $msg:ty $(,)?
        }
        $($rest:tt)*
    ) => {
        /// Channel type for controller communication.
        pub type $channel<MutexRaw, const N: usize> =
            embassy_sync::channel::Channel<MutexRaw, $msg, N>;
        /// Sender type for controller communication.
        pub type $sender<MutexRaw, const N: usize> =
            embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
        /// Receiver type for controller communication.
        pub type $receiver<MutexRaw, const N: usize> =
            embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;

        $crate::define_controllers! { $($rest)* }
    };

    // Rule 4: Task with two receivers and dynamic controller type on invocation (e.g. System)
    (
        $name:ident {
            channel: $channel:ident,
            sender: $sender:ident,
            receiver: $receiver:ident,
            msg: $msg:ty,
            task: $run_macro:ident {
                generics: ($($gen:tt)*),
                controller: [$c:ident],
                rx: [$($rx_ty:tt)*],
                rx2: [$($rx2_ty:tt)*],
                call: |$c_bind:ident, $r:ident, $r2:ident| $body:expr
            }
        }
        $($rest:tt)*
    ) => {
        /// Channel type for controller communication.
        pub type $channel<MutexRaw, const N: usize> =
            embassy_sync::channel::Channel<MutexRaw, $msg, N>;
        /// Sender type for controller communication.
        pub type $sender<MutexRaw, const N: usize> =
            embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
        /// Receiver type for controller communication.
        pub type $receiver<MutexRaw, const N: usize> =
            embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;

        /// Task runner macro for the controller.
        #[macro_export]
        macro_rules! $run_macro {
            (
                $spawner:expr,
                $task_module:ident,
                $controller:expr,
                $controller_type:ty,
                $rx:expr,
                $rx2:expr
            ) => {
                #[allow(non_snake_case)]
                mod $task_module {
                    use super::*;
                    #[cfg(feature = "tracing")]
                    use $crate::tracing::tracing_defmt;

                    #[ $crate::tracing::instrument(name = stringify!($task_module), level = "info", skip($c, $r, $r2)) ]
                    #[embassy_executor::task]
                    #[allow(unreachable_code)]
                    pub async fn task(
                        mut $c: $controller_type,
                        $r: $($rx_ty)*,
                        $r2: $($rx2_ty)*
                    ) {
                        $body;
                    }
                }

                $spawner
                    .spawn($task_module::task($controller, $rx, $rx2))
                    .unwrap();
            };
        }

        $crate::define_controllers! { $($rest)* }
    };

    // Rule 5: Task with three receivers and dynamic controller type on invocation (e.g. System with thermal_rx)
    (
        $name:ident {
            channel: $channel:ident,
            sender: $sender:ident,
            receiver: $receiver:ident,
            msg: $msg:ty,
            task: $run_macro:ident {
                generics: ($($gen:tt)*),
                controller: [$c:ident],
                rx: [$($rx_ty:tt)*],
                rx2: [$($rx2_ty:tt)*],
                rx3: [$($rx3_ty:tt)*],
                call: |$c_bind:ident, $r:ident, $r2:ident, $r3:ident| $body:expr
            }
        }
        $($rest:tt)*
    ) => {
        /// Channel type for controller communication.
        pub type $channel<MutexRaw, const N: usize> =
            embassy_sync::channel::Channel<MutexRaw, $msg, N>;
        /// Sender type for controller communication.
        pub type $sender<MutexRaw, const N: usize> =
            embassy_sync::channel::Sender<'static, MutexRaw, $msg, N>;
        /// Receiver type for controller communication.
        pub type $receiver<MutexRaw, const N: usize> =
            embassy_sync::channel::Receiver<'static, MutexRaw, $msg, N>;

        /// Task runner macro for the controller.
        #[macro_export]
        macro_rules! $run_macro {
            (
                $spawner:expr,
                $task_module:ident,
                $controller:expr,
                $controller_type:ty,
                $rx:expr,
                $rx2:expr,
                $rx3:expr
            ) => {
                #[allow(non_snake_case)]
                mod $task_module {
                    use super::*;
                    #[cfg(feature = "tracing")]
                    use $crate::tracing::tracing_defmt;

                    #[ $crate::tracing::instrument(name = stringify!($task_module), level = "info", skip($c, $r, $r2, $r3)) ]
                    #[embassy_executor::task]
                    #[allow(unreachable_code)]
                    pub async fn task(
                        mut $c: $controller_type,
                        $r: $($rx_ty)*,
                        $r2: $($rx2_ty)*,
                        $r3: $($rx3_ty)*
                    ) {
                        $body;
                    }
                }

                $spawner
                    .spawn($task_module::task($controller, $rx, $rx2, $rx3))
                    .unwrap();
            };
        }

        $crate::define_controllers! { $($rest)* }
    };

    // Base case: empty
    () => {};
}

define_controllers! {
    Led {
        channel: LedChannel,
        sender: LedSender,
        receiver: LedReceiver,
        msg: model::types::SystemLedState,
        task: run_led_task {
            generics: ($driver_type:ty),
            controller: [$crate::led_controller::LedController<$driver_type>],
            rx: [$crate::LedReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            telemetry_tx: [$crate::TelemetrySender<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                { $crate::telemetry_controller::CHANNEL_CAPACITY },
            >],
            call: |controller, rx, telemetry_tx| controller.run(rx, telemetry_tx).await
        }
    }
    Battery {
        channel: BatteryChannel,
        sender: BatterySender,
        receiver: BatteryReceiver,
        msg: crate::battery_controller::BatteryCommand,
        task: run_battery_task {
            generics: ($battery_type:ty, $charger_type:ty, $pin_type:ty, $cmd_type:ty),
            controller: [$crate::battery_controller::BatteryController<
                'static,
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                $battery_type,
                $charger_type,
                $pin_type,
                $cmd_type,
            >],
            rx: [$crate::BatteryReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            telemetry_tx: [$crate::TelemetrySender<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                { $crate::telemetry_controller::CHANNEL_CAPACITY },
            >],
            call: |controller, rx, telemetry_tx| controller.run(rx, telemetry_tx).await
        }
    }
    Thermal {
        channel: ThermalChannel,
        sender: ThermalSender,
        receiver: ThermalReceiver,
        msg: crate::thermal_controller::ThermalCommand,
        task: run_thermal_task {
            generics: ($battery_type:ty),
            controller: [$crate::thermal_controller::ThermalController<
                'static,
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                $battery_type,
            >],
            rx: [$crate::ThermalReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            telemetry_tx: [$crate::TelemetrySender<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                { $crate::telemetry_controller::CHANNEL_CAPACITY },
            >],
            call: |controller, rx, telemetry_tx| controller.run(rx, telemetry_tx).await
        }
    }
    Sensor {
        channel: SensorChannel,
        sender: SensorSender,
        receiver: SensorReceiver,
        msg: crate::sensor_controller::SensorCommand,
        task: run_sensor_task {
            generics: ($sensor_type:ty, $pin_type:ty, $cmd_type:ty),
            controller: [$crate::sensor_controller::SensorController<
                'static,
                $sensor_type,
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                $pin_type,
                $cmd_type,
            >],
            rx: [$crate::SensorReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            call: |controller, rx| controller.run(rx).await
        }
    }
    Motor {
        channel: MotorChannel,
        sender: MotorSender,
        receiver: MotorReceiver,
        msg: crate::motor_controller::MotorCommand,
        task: run_motor_task {
            generics: ($motor_type:ty, $current_sensor_type:ty),
            controller: [$crate::motor_controller::MotorController<
                $motor_type,
                $current_sensor_type,
            >],
            rx: [$crate::MotorReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            telemetry_tx: [$crate::TelemetrySender<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                { $crate::telemetry_controller::CHANNEL_CAPACITY },
            >],
            call: |controller, rx, telemetry_tx| controller.run(rx, telemetry_tx).await
        }
    }
    Filesystem {
        channel: FilesystemChannel,
        sender: FilesystemSender,
        receiver: FilesystemReceiver,
        msg: crate::filesystem_controller::FsRequest,
        task: run_filesystem_task {
            generics: ($flash_type:ty),
            controller: [$crate::filesystem_controller::FilesystemController<$flash_type>],
            rx: [$crate::FilesystemReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 16>],
            call: |controller, rx| controller.run(rx).await
        }
    }
    System {
        channel: SystemChannel,
        sender: SystemSender,
        receiver: SystemReceiver,
        msg: crate::system_controller::SystemCommand,
        task: run_system_task {
            generics: ($controller_type:ty),
            controller: [controller],
            rx: [$crate::SystemReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            rx2: [firmware_lib::gesture_detector::GestureReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>],
            rx3: [embassy_sync::channel::Receiver<'static, embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $crate::ThermalUpdateAction, 4>],
            call: |controller, system_rx, gesture_rx, thermal_rx| controller.run(system_rx, gesture_rx, thermal_rx).await
        }
    }
    Telemetry {
        channel: TelemetryChannel,
        sender: TelemetrySender,
        receiver: TelemetryReceiver,
        msg: model::telemetry::TelemetryRecord,
        task: run_telemetry_task {
            generics: ($max_records:expr, $channel_size:expr),
            controller: [&'static mut $crate::telemetry_controller::TelemetryController<
                $max_records,
                { model::telemetry::BUFFER_SIZE },
            >],
            rx: [$crate::TelemetryReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $channel_size>],
            call: |controller, rx| controller.run(rx).await
        }
    }
}

/// A dummy telemetry channel used when telemetry is disabled or omitted.
pub static DUMMY_TELEMETRY_CHANNEL: TelemetryChannel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    { telemetry_controller::CHANNEL_CAPACITY },
> = TelemetryChannel::new();

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
#[allow(async_fn_in_trait)]
pub trait BlockingSystemWriter {
    /// Resets the inactivity timeout.
    fn record_activity(&mut self) -> Result<(), PeripheralError>;

    /// Clears a specific boot trap.
    fn clear_boot_trap(
        &mut self,
        _reason: firmware_lib::BootTrapReason,
    ) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }

    /// Checks if the system is trapped in boot.
    fn is_boot_trapped(&self) -> Result<bool, PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

impl BlockingSystemWriter for () {
    fn record_activity(&mut self) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
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

/// Macro to spawn any permutation of controllers concurrently on the provided spawner.
///
/// Automatically handles telemetry wiring and extracts channel receivers.
#[macro_export]
macro_rules! spawn_controllers {
    // With explicit telemetry channel
    (
        $spawner:expr,
        telemetry: $telemetry:expr,
        controllers: {
            $(
                $name:ident ( $controller:expr, $rx:ident $(, $extra_rx:ident)* )
                , generics: ($($gen:tt)*)
            ),* $(,)?
        }
    ) => {
        // Assert that in production ARM builds, telemetry is not routed to the dummy channel
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let telemetry_ptr = &$telemetry as *const _;
            let dummy_ptr = &$crate::DUMMY_TELEMETRY_CHANNEL as *const _;
            if core::ptr::eq(telemetry_ptr, dummy_ptr) {
                panic!("Production firmware cannot be run with disabled/dummy telemetry!");
            }
        }

        $(
            $crate::spawn_single_controller!(
                $spawner,
                $name,
                $controller,
                $rx,
                $telemetry,
                ($( $extra_rx ),*),
                ($($gen)*)
            );
        )*
    };

    // Without explicit telemetry channel (defaults to DUMMY_TELEMETRY_CHANNEL)
    (
        $spawner:expr,
        controllers: {
            $(
                $name:ident ( $controller:expr, $rx:ident $(, $extra_rx:ident)* )
                , generics: ($($gen:tt)*)
            ),* $(,)?
        }
    ) => {
        $crate::spawn_controllers!(
            $spawner,
            telemetry: $crate::DUMMY_TELEMETRY_CHANNEL,
            controllers: {
                $(
                    $name ( $controller, $rx $(, $extra_rx)* )
                    , generics: ($($gen)*)
                ),*
            }
        );
    };
}

/// Helper macro to delegate the spawning of a single controller to its respective task macro.
#[macro_export]
#[doc(hidden)]
macro_rules! spawn_single_controller {
    // Led
    ($spawner:expr, Led, $controller:expr, $rx:ident, $telemetry:expr, (), ($driver_type:ty)) => {
        $crate::run_led_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $telemetry.sender(),
            $driver_type
        );
    };
    // Battery
    ($spawner:expr, Battery, $controller:expr, $rx:ident, $telemetry:expr, (), ($battery_type:ty, $charger_type:ty, $pin_type:ty, $cmd_type:ty)) => {
        $crate::run_battery_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $telemetry.sender(),
            $battery_type,
            $charger_type,
            $pin_type,
            $cmd_type
        );
    };
    // Thermal
    ($spawner:expr, Thermal, $controller:expr, $rx:ident, $telemetry:expr, (), ($battery_type:ty)) => {
        $crate::run_thermal_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $telemetry.sender(),
            $battery_type
        );
    };
    // Sensor
    ($spawner:expr, Sensor, $controller:expr, $rx:ident, $telemetry:expr, (), ($sensor_type:ty, $pin_type:ty, $cmd_type:ty)) => {
        $crate::run_sensor_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $sensor_type,
            $pin_type,
            $cmd_type
        );
    };
    // Motor
    ($spawner:expr, Motor, $controller:expr, $rx:ident, $telemetry:expr, (), ($motor_type:ty, $current_sensor_type:ty)) => {
        $crate::run_motor_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $telemetry.sender(),
            $motor_type,
            $current_sensor_type
        );
    };
    // Filesystem
    ($spawner:expr, Filesystem, $controller:expr, $rx:ident, $telemetry:expr, (), ($flash_type:ty)) => {
        $crate::run_filesystem_task!($spawner, $rx, $controller, $rx.receiver(), $flash_type);
    };
    // Telemetry
    ($spawner:expr, Telemetry, $controller:expr, $rx:ident, $telemetry:expr, (), ($max_records:expr, $channel_size:expr)) => {
        $crate::run_telemetry_task!(
            $spawner,
            $rx,
            $controller,
            $rx.receiver(),
            $max_records,
            $channel_size
        );
    };
    // System
    ($spawner:expr, System, $controller:expr, $rx:ident, $telemetry:expr, ($gesture_rx:ident, $thermal_action_rx:ident), ($controller_type:ty)) => {
        $crate::run_system_task!(
            $spawner,
            $rx,
            $controller,
            $controller_type,
            $rx.receiver(),
            $gesture_rx.receiver(),
            $thermal_action_rx.receiver()
        );
    };
}
