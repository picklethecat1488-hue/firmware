//! Domain models representing controller states and status telemetry.

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

/// Telemetry status of the battery system.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BatteryStatus {
    /// Voltage (mV), temperature (mC), battery state, and active wake locks mask.
    #[n(0)]
    VolTempState(#[n(0)] u32, #[n(1)] i32, #[n(2)] BatteryState, #[n(3)] u32),
}

impl Default for BatteryStatus {
    fn default() -> Self {
        Self::VolTempState(0, 0, BatteryState::default(), 0)
    }
}

/// Enumeration of battery states.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BatteryState {
    /// Battery voltage is normal.
    #[default]
    #[n(0)]
    Ok,
    /// Battery voltage is low.
    #[n(1)]
    Low,
    /// Battery is charging.
    #[n(2)]
    Charging,
    /// Battery is critically low (system runs, but pump is disabled).
    #[n(3)]
    Critical,
}

/// Telemetry status of the motor (pump) system.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum MotorStatus {
    /// Motor is braked/stopped.
    #[default]
    #[n(0)]
    Brake,
    /// Motor is running at the specified speed (0-100).
    #[n(1)]
    Running(#[n(0)] u8),
}

/// Telemetry status of the thermal monitoring system.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ThermalStatus {
    /// Temperature (mC) and overheating status.
    #[n(0)]
    TempOverheating(#[n(0)] i32, #[n(1)] bool),
}

impl Default for ThermalStatus {
    fn default() -> Self {
        Self::TempOverheating(0, false)
    }
}

/// Operating mode of the system (Active, Sleep, or PowerDown).
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SystemStatus {
    /// System is powered down for safe transport/boot constraints.
    #[default]
    #[n(0)]
    PowerDown,
    /// System is fully awake and processing sensors/motor.
    #[n(1)]
    Active,
    /// System is in low-power sleep state.
    #[n(2)]
    Sleep,
}

/// Telemetry data from the battery fuel gauge.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum FuelGaugeTelemetry {
    /// Battery cell voltage (mV) and state of charge percentage (0-100).
    #[n(0)]
    VolSoc(#[n(0)] u32, #[n(1)] u8),
}

impl Default for FuelGaugeTelemetry {
    fn default() -> Self {
        Self::VolSoc(0, 0)
    }
}

/// Telemetry data from the proximity (ToF) sensors.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ProximityTelemetry {
    /// Target is detected within active range (direction, value in mm).
    #[n(0)]
    InRange(#[n(0)] Direction, #[n(1)] u16),
    /// Target is out of range (direction, value in mm).
    #[n(1)]
    OutRange(#[n(0)] Direction, #[n(1)] u16),
}

impl Default for ProximityTelemetry {
    fn default() -> Self {
        Self::OutRange(Direction::North, 1000)
    }
}

/// State patterns of the indicator system LEDs representing operating status.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SystemLedState {
    /// LED is powered off.
    #[default]
    #[n(0)]
    Off,
    /// Solid green indicating Active/Normal status.
    #[n(1)]
    SolidGreen,
    /// Solid blue indicating low-power Sleep status.
    #[n(2)]
    SolidBlue,
    /// Solid yellow indicating battery is charging.
    #[n(3)]
    SolidYellow,
    /// Solid orange indicating battery level is low.
    #[n(4)]
    SolidOrange,
    /// Four quick red blinks indicating a critical system alert.
    #[n(5)]
    BlinksRedFourTimes,
    /// One red blink once every 30 seconds indicating critical battery low.
    #[n(6)]
    BlinksRedOncePerThirtySeconds,
}

/// Gestures representing proximity sensor states (North, East, West) in mm.
#[derive(Clone, Copy, PartialEq, Eq, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum Gesture {
    /// Proximity readings from North, East, and West sensors in mm (north, east, west).
    #[n(0)]
    Proximity(#[n(0)] u16, #[n(1)] u16, #[n(2)] u16),
    /// A completed dual-sensor long press gesture.
    #[n(1)]
    DualLongPress,
    /// Proximity detection (any sensor < 300 mm).
    #[n(2)]
    ProximityDetected,
    /// Proximity not detected (all sensors >= 300 mm).
    #[n(3)]
    ProximityNotDetected,
}

/// Telemetry data from the flash storage/filesystem.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cbor(array)]
pub struct FlashEraseTelemetry {
    /// Erased sector index (offset / 4096).
    #[n(0)]
    pub sector: u32,
    /// Erase duration in milliseconds.
    #[n(1)]
    pub duration_ms: u32,
    /// Total erases since boot.
    #[n(2)]
    pub erase_count: u32,
}

/// State of the battery charger.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ChargeState {
    /// S1=HIGH, S2=LOW: Normal Charging.
    #[n(0)]
    Charging,
    /// S1=HIGH, S2=HIGH: Charging Done, Standby, or Unplugged.
    #[default]
    #[n(1)]
    DoneOrStandbyOrUnplugged,
    /// S1=LOW, S2=HIGH: Recoverable Fault.
    #[n(2)]
    RecoverableFault,
    /// S1=LOW, S2=LOW: Non-Recoverable Fault.
    #[n(3)]
    NonRecoverableFault,
}

/// Proximity sensor direction.
#[derive(Clone, Copy, PartialEq, Eq, Hash, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[repr(usize)]
pub enum Direction {
    /// North direction.
    #[n(0)]
    North = 0,
    /// East direction.
    #[n(1)]
    East = 1,
    /// West direction.
    #[n(2)]
    West = 2,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::North),
            1 => Ok(Self::East),
            2 => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl From<Direction> for u8 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => 0,
            Direction::East => 1,
            Direction::West => 2,
        }
    }
}

/// Peripheral errors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum PeripheralError {
    /// An expected device ID/WHO_AM_I register value did not match.
    #[n(0)]
    DeviceNotFound,
    /// The parameters passed to a driver configuration function were invalid.
    #[n(1)]
    InvalidConfiguration,
    /// Peripheral function not implemented.
    #[n(2)]
    NotImplemented,
    /// Peripheral function not implemented.
    #[n(3)]
    DeviceNotAvailable,
    /// Unknown error.
    #[n(4)]
    Unknown,
    /// Pin error.
    #[n(5)]
    PinError,
    /// I2C Bus Error.
    #[n(100)]
    I2CBusError,
    /// I2C bus Collision.
    #[n(101)]
    I2CArbitrationLoss,
    /// I2C buffer overrun.
    #[n(102)]
    I2COverrun,
    /// I2C NACK: Address.
    #[n(103)]
    I2CNackAddress,
    /// I2C NACK: Data.
    #[n(104)]
    I2CNackData,
    /// I2C NACK: Unknown.
    #[n(105)]
    I2CNackUnknown,
    /// I2C Error: Other.
    #[n(106)]
    I2COther,
    /// i2C Error: Unknown.
    #[n(107)]
    I2CUnknown,
}

/// The reason why the device booted.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BootReason {
    /// Cold boot or standard power cycle.
    #[n(0)]
    PowerOn,
    /// Booted due to system watchdog reset.
    #[n(1)]
    Watchdog,
    /// Booted due to software reset.
    #[n(2)]
    SoftwareReset,
    /// Boot reason is unknown or unclassified.
    #[default]
    #[n(3)]
    Unknown,
}

dummy_debug!(BatteryStatus);
dummy_debug!(BatteryState);
dummy_debug!(MotorStatus);
dummy_debug!(ThermalStatus);
dummy_debug!(SystemStatus);
dummy_debug!(FuelGaugeTelemetry);
dummy_debug!(ProximityTelemetry);
dummy_debug!(SystemLedState);
dummy_debug!(Gesture);
dummy_debug!(FlashEraseTelemetry);
dummy_debug!(ChargeState);
dummy_debug!(Direction);
dummy_debug!(PeripheralError);
dummy_debug!(BootReason);

/// One-way commands to control the global system state and notify it of events.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum SystemCommand {
    /// Notify system of activity, resetting inactivity timer and waking up if asleep.
    ActivityDetected,
    /// Thermal safety or motor stall alert occurred.
    AlertTriggered,
    /// Battery level updates from the fuel gauge.
    BatteryUpdate {
        /// Battery capacity percentage (0-100).
        state_of_charge: u8,
        /// Charger state.
        charger_state: ChargeState,
    },
    /// High-level gesture detected.
    Gesture(Gesture),
    /// The system status/power state changed.
    StateChanged {
        /// The previous system status.
        from: SystemStatus,
        /// The new system status.
        to: SystemStatus,
    },
}

dummy_debug!(SystemCommand);

pub use crate::telemetry::TelemetryRecord;
