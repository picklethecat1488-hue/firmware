//! Controller crate containing project-agnostic orchestrators.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Target-safe maximum duration (1 year) to prevent time-queue addition overflows in embassy-time.
pub const OVERFLOW_SAFE_MAX_DURATION: embassy_time::Duration =
    embassy_time::Duration::from_secs(3600 * 24 * 365);

// Include the generated controllers, channels, and spawn macros
include!(concat!(env!("OUT_DIR"), "/generated_controllers.rs"));

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
pub use shell_controller::ShellDeviceResolver;
pub use system_controller::{ProximityEvent, SystemCommand, SystemController, SystemFeatureSet};
pub use system_feature::{FeatureList, Periodic, PeriodicInterval, SystemFeature};
pub use thermal_controller::ThermalCommand;
pub use thermal_controller::ThermalFeatureConfig;
pub use types::{
    BatteryStatus, Device, DeviceSupport, FlashPartition, GestureAction, MapFilesystem,
    MotorCalState, MotorError, MotorSafetyStatus, MotorState, NamedDevice, NamedPartition,
    PartitionKind, ProximityAction, QueueFilesystem, ResolvedPartition, SensorDirection,
    ThermalState, ThermalUpdateAction,
};

/// Consolidated tracing facade module from platform.
pub use platform::tracing;

/// Re-export CriticalSectionRawMutex for use by generated macros and receiver type configurations.
pub use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

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
                $name:ident ( $controller:expr, $rx:ident $(, $extra_rx:expr)* )
                $( register: $register:expr )?
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
                $(, register: $register)?
            );
        )*
    };

    // Without explicit telemetry channel (defaults to DUMMY_TELEMETRY_CHANNEL)
    (
        $spawner:expr,
        controllers: {
            $(
                $name:ident ( $controller:expr, $rx:ident $(, $extra_rx:expr)* )
                $( register: $register:expr )?
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
                    $( register: $register )?
                    , generics: ($($gen)*)
                ),*
            }
        );
    };
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

    /// Configure alerts on the battery.
    fn configure_alerts(&self, _v_min_mv: u32, _v_max_mv: u32) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }

    /// Check and clear alerts on the battery.
    fn check_and_clear_alerts(&self) -> Result<(bool, bool), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }

    /// Read the current physical state of the alert pin (true = low/asserted, false = high/deasserted).
    fn read_alert_pin(&self) -> Result<bool, PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

/// Trait for reading temperature blocking-ly.
pub trait BlockingThermalReader {
    /// Read temperature in milli-Celsius.
    fn read_temperature_blocking(&self) -> Result<i32, PeripheralError>;
}

/// Trait for reading proximity distance blocking-ly.
pub trait BlockingProximityReader:
    model::calibration::ApplyCalibration<
    Input = model::types::SensorReading,
    Output = model::types::SensorReading,
    Error = &'static str,
>
{
    /// Read distance in millimeters.
    fn read_distance_blocking(&mut self) -> Result<model::types::SensorReading, PeripheralError>;

    /// Read raw distance in millimeters (ignoring calibration mapping).
    fn read_raw_distance_blocking(
        &mut self,
    ) -> Result<model::types::SensorReading, PeripheralError> {
        self.read_distance_blocking()
    }

    /// Get the latest cached proximity distance in millimeters.
    fn latest_distance(&self) -> model::types::SensorReading {
        model::types::SensorReading::Invalid
    }

    /// Send a command to the sensor controller background task.
    fn send_command(
        &mut self,
        _cmd: crate::sensor_controller::SensorCommand,
    ) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }

    /// Update the reader's internal calibration parameters.
    fn update_calibration(
        &mut self,
        _cal: &model::calibration::Vl53l0xCalibration,
    ) -> Result<(), PeripheralError> {
        Ok(())
    }
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
    fn read_distance_blocking(&mut self) -> Result<model::types::SensorReading, PeripheralError> {
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
    /// Set motor speed using target RPM.
    fn set_motor_speed_rpm(&mut self, rpm: i32) -> Result<(), PeripheralError>;
    /// Stop the motor.
    fn stop_motor_blocking(&mut self) -> Result<(), PeripheralError>;
    /// Update the writer's internal calibration parameters.
    fn update_calibration(
        &mut self,
        _cal: &model::calibration::MotorCalibration,
    ) -> Result<(), PeripheralError> {
        Ok(())
    }
}

impl BlockingMotorWriter for () {
    fn set_motor_speed(&mut self, _: i8) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
    fn set_motor_speed_rpm(&mut self, _: i32) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
    fn stop_motor_blocking(&mut self) -> Result<(), PeripheralError> {
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
        _reason: platform::BootTrapReason,
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
    fn set_motor_speed_rpm(&mut self, rpm: i32) -> Result<(), PeripheralError> {
        self.try_send(MotorCommand::SetSpeedRpm(rpm))
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
    fn stop_motor_blocking(&mut self) -> Result<(), PeripheralError> {
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

/// Trait for controlling LED color state pattern.
pub trait BlockingLedWriter {
    /// Set the current LED state pattern.
    fn set_pattern_blocking(
        &mut self,
        pattern: model::types::SystemLedState,
    ) -> Result<(), PeripheralError>;
}

impl BlockingLedWriter for () {
    fn set_pattern_blocking(
        &mut self,
        _pattern: model::types::SystemLedState,
    ) -> Result<(), PeripheralError> {
        Err(PeripheralError::NotImplemented)
    }
}

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    BlockingLedWriter for LedSender<MutexRaw, N>
{
    fn set_pattern_blocking(
        &mut self,
        pattern: model::types::SystemLedState,
    ) -> Result<(), PeripheralError> {
        self.try_send(pattern)
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
}
