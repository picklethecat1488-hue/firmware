//! Controller crate containing project-agnostic orchestrators.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// Battery status and telemetry controller.
pub mod battery_controller;
/// Flat filesystem and storage controller.
pub mod filesystem_controller;
/// Gesture detection module.
pub mod gesture_detector;
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

pub use battery_controller::BatteryCommand;
pub use embedded_cli;
pub use embedded_io;
pub use motor_controller::MotorCommand;
pub use sensor_controller::SensorCommand;
pub use system_controller::SystemCommand;
pub use thermal_controller::ThermalCommand;

use model::types::PeripheralError;

/// Represents a partition on a flash peripheral.
#[derive(Debug, PartialEq, Eq)]
pub struct FlashPartition<F> {
    /// Pointer to the underlying flash hardware driver.
    pub flash_ptr: *mut F,
    /// Start address of the partition.
    pub start_address: u32,
    /// End address of the partition (exclusive).
    pub end_address: u32,
}

impl<F> Clone for FlashPartition<F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<F> Copy for FlashPartition<F> {}

// Implement Send/Sync since it contains a raw pointer
unsafe impl<F> Send for FlashPartition<F> {}
unsafe impl<F> Sync for FlashPartition<F> {}

/// Binds a device name to a physical peripheral pointer.
#[derive(Debug, PartialEq, Eq)]
pub struct NamedDevice<D> {
    /// Friendly name (e.g., "left", "right", "mcu", "external")
    pub name: &'static str,
    /// Raw pointer to the peripheral driver.
    pub device: *mut D,
}

impl<D> Clone for NamedDevice<D> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<D> Copy for NamedDevice<D> {}

// Implement Send/Sync since it contains a raw pointer
unsafe impl<D> Send for NamedDevice<D> {}
unsafe impl<D> Sync for NamedDevice<D> {}

/// Binds a partition name to a flash partition.
#[derive(Debug, PartialEq, Eq)]
pub struct NamedPartition<F> {
    /// Friendly name (e.g., "logs", "config", "calibration")
    pub name: &'static str,
    /// The associated flash partition details.
    pub partition: FlashPartition<F>,
}

impl<F> Clone for NamedPartition<F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<F> Copy for NamedPartition<F> {}

// Implement Send/Sync since it contains raw pointers/types
unsafe impl<F> Send for NamedPartition<F> {}
unsafe impl<F> Sync for NamedPartition<F> {}

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

/// Helper macro to declare static embassy channels with CriticalSectionRawMutex.
///
/// Example:
/// ```rust
/// # use controller::declare_channels;
/// declare_channels! {
///     pub static MOTOR_CHANNEL: controller::motor_controller::MotorCommand, capacity = 4;
///     pub static SYSTEM_CHANNEL: controller::system_controller::SystemCommand, capacity = 4;
/// }
/// ```
#[macro_export]
macro_rules! declare_channels {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis static $name:ident : $ty:ty, capacity = $cap:expr;
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis static $name: $crate::Channel<$ty, $cap> = $crate::Channel::new();
        )*
    };
}

/// Type alias for an Embassy channel using CriticalSectionRawMutex.
pub type Channel<T, const N: usize> = embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    T,
    N,
>;

/// Type alias for an Embassy channel Sender using CriticalSectionRawMutex.
pub type Sender<T, const N: usize> = embassy_sync::channel::Sender<
    'static,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    T,
    N,
>;

/// Type alias for an Embassy channel Receiver using CriticalSectionRawMutex.
pub type Receiver<T, const N: usize> = embassy_sync::channel::Receiver<
    'static,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    T,
    N,
>;

/// A dummy/no-op I2C driver.
#[derive(Debug, Clone, Copy, Default)]
pub struct DummyI2c;

impl embedded_hal::i2c::ErrorType for DummyI2c {
    type Error = core::convert::Infallible;
}

impl embedded_hal::i2c::I2c for DummyI2c {
    fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write(&mut self, _address: u8, _write: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op motor driver.
#[derive(Debug, Clone, Copy, Default)]
pub struct DummyMotor;

impl model::interfaces::Motor for DummyMotor {
    type Error = core::convert::Infallible;
    fn set_speed(&mut self, _speed: model::types::MotorSpeed) -> Result<(), Self::Error> {
        Ok(())
    }
    fn stop(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op flash driver.
#[derive(Debug, Clone, Copy, Default)]
pub struct DummyFlash;

impl embedded_storage::nor_flash::ErrorType for DummyFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage::nor_flash::ReadNorFlash for DummyFlash {
    const READ_SIZE: usize = 1;
    fn read(&mut self, _offset: u32, _bytes: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn capacity(&self) -> usize {
        0
    }
}

impl embedded_storage::nor_flash::NorFlash for DummyFlash {
    const WRITE_SIZE: usize = 1;
    const ERASE_SIZE: usize = 4096;
    fn write(&mut self, _offset: u32, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn erase(&mut self, _from: u32, _to: u32) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A dummy/no-op temperature sensor.
#[derive(Debug, Clone, Copy, Default)]
pub struct DummyTempSensor;

impl model::interfaces::TemperatureSensor for DummyTempSensor {
    type Error = core::convert::Infallible;
    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        Ok(0)
    }
}

impl<MutexRaw: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const N: usize>
    BlockingMotorWriter for embassy_sync::channel::Sender<'static, MutexRaw, MotorCommand, N>
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
    BlockingSystemWriter for embassy_sync::channel::Sender<'static, MutexRaw, SystemCommand, N>
{
    fn record_activity(&mut self) -> Result<(), PeripheralError> {
        self.try_send(SystemCommand::ActivityDetected)
            .map_err(|_| PeripheralError::DeviceNotAvailable)
    }
}
