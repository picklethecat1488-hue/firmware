//! Common types used across the controllers.

use model::types::{Direction, SystemLedState};

macro_rules! dummy_debug {
    ($ty:ident) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        impl core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(stringify!($ty))
            }
        }
    };
}

/// Actions that can be mapped from gestures.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum GestureAction {
    /// No action.
    None,
    /// Toggle system power state (Active <-> PowerDown).
    TogglePower,
}

/// Action returned by the proximity feature update.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ProximityAction {
    /// No action.
    None,
    /// Acquire system wake lock.
    AcquireWakeLock,
    /// Release system wake lock.
    ReleaseWakeLock,
    /// Wake system if asleep.
    WakeSystem,
}

/// Battery status summary passed to features and stored on the system controller.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct BatteryStatus {
    /// True if the battery level is critically low.
    pub battery_critical: bool,
    /// True if the charger is connected and charging.
    pub charger_connected: bool,
    /// The mapped LED state for the current state of charge.
    pub soc_led_state: SystemLedState,
}

/// Devices that can be power-managed by the system.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum Device {
    /// The motor.
    Motor,
    /// Proximity/gesture sensors.
    Sensors,
    /// Status indicator LED.
    Led,
    /// Battery / Fuel gauge.
    Battery,
    /// Thermal monitoring.
    Thermal,
}

/// Device activity support status in the current system state.
#[derive(Clone, Copy)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct DeviceSupport {
    /// True if motor is supported.
    pub motor: bool,
    /// True if battery monitoring is supported.
    pub battery: bool,
    /// True if proximity sensors are supported.
    pub proximity: bool,
    /// True if led is supported.
    pub led: bool,
    /// True if thermal monitoring is supported.
    pub thermal: bool,
}

/// Represents a partition on a flash peripheral.
#[derive(PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
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
#[derive(PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
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
#[derive(PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
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

/// Metadata associated with a proximity sensor.
#[derive(Clone, Copy)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct SensorMetadata {
    /// The physical direction the sensor is facing.
    pub direction: Direction,
}

/// Represents the physical directions of ToF proximity sensors.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SensorDirection {
    /// North sensor
    North,
    /// East sensor
    East,
    /// West sensor
    West,
}

/// Current thermal status of the system.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ThermalState {
    /// System temperature is normal.
    Normal,
    /// System is overheating.
    Overheating,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl core::fmt::Debug for ThermalState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ThermalState")
    }
}

/// The operating states of the motor.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MotorState {
    /// The motor is powered off.
    #[default]
    Off,
    /// The motor is running continuously at target speed.
    On,
}

/// Status representing safety check results.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MotorSafetyStatus {
    /// All limits are within safe operating parameters.
    Ok,
    /// The motor RPM exceeded the safety limit.
    RpmExceeded(u32),
    /// Low load / dry run detected.
    DryRun(i32),
    /// Motor stall detected (high current).
    Stall(i32),
}

/// Errors returned by the motor controller loop.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MotorError<ME, CE> {
    /// Error originating from the motor driver.
    Motor(ME),
    /// Error originating from the current sensor driver.
    CurrentSensor(CE),
}

/// Represents the motor calibration target state.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MotorCalState {
    /// Empty calibration state
    Empty,
    /// Low calibration state
    Low,
    /// High calibration state
    High,
    /// Overload calibration state
    Overload,
}

impl From<MotorCalState> for model::calibration::FourPointRef {
    fn from(state: MotorCalState) -> Self {
        match state {
            MotorCalState::Empty => model::calibration::FourPointRef::Low,
            MotorCalState::Low => model::calibration::FourPointRef::Low,
            MotorCalState::High => model::calibration::FourPointRef::High,
            MotorCalState::Overload => model::calibration::FourPointRef::Overload,
        }
    }
}

dummy_debug!(GestureAction);
dummy_debug!(ProximityAction);
dummy_debug!(BatteryStatus);
dummy_debug!(Device);
dummy_debug!(DeviceSupport);
dummy_debug!(SensorMetadata);
dummy_debug!(SensorDirection);
dummy_debug!(MotorState);
dummy_debug!(MotorSafetyStatus);
dummy_debug!(MotorCalState);

pub use firmware_lib::ThermalUpdateAction;
