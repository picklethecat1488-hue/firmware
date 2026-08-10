//! Calibration data structures.

#![deny(missing_docs)]

/// A general two-point calibration structure mapping raw readings at two reference points.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cbor(array)]
pub struct TwoPointCalibration<T> {
    /// Reading at the lower reference point (e.g. near / minimum).
    #[n(0)]
    pub low: T,
    /// Reading at the upper reference point (e.g. far / maximum).
    #[n(1)]
    pub high: T,
}

impl<T> TwoPointCalibration<T> {
    /// Create a new two-point calibration.
    pub const fn new(low: T, high: T) -> Self {
        Self { low, high }
    }
}

impl TwoPointCalibration<u16> {
    /// Interpolate or map a raw reading using the two-point calibration.
    /// Maps `low` to 0, and `high` to 100.
    pub fn map(&self, raw: u16) -> u16 {
        let min_r = 0;
        let max_r = 100;

        if self.high > self.low {
            if raw <= self.low {
                min_r
            } else {
                let range = (max_r - min_r) as u32;
                min_r + (((raw - self.low) as u32 * range) / (self.high - self.low) as u32) as u16
            }
        } else if self.low > self.high {
            if raw >= self.low {
                min_r
            } else if raw <= self.high {
                max_r
            } else {
                let range = (max_r - min_r) as u32;
                min_r + (((self.low - raw) as u32 * range) / (self.low - self.high) as u32) as u16
            }
        } else {
            raw
        }
    }
}

/// A generic four-point calibration structure mapping low, mid, high, and overload reference states.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cbor(array)]
pub struct FourPointCalibration<T> {
    /// Raw reading at the low reference point (e.g. empty).
    #[n(0)]
    pub low: T,
    /// Raw reading at the mid reference point (e.g. partial / 100ml).
    #[n(1)]
    pub mid: T,
    /// Raw reading at the high reference point (e.g. full).
    #[n(2)]
    pub high: T,
    /// Raw reading at the overload/stall reference point.
    #[n(3)]
    pub overload: T,
}

impl<T> FourPointCalibration<T> {
    /// Create a new four-point calibration.
    pub const fn new(low: T, mid: T, high: T, overload: T) -> Self {
        Self {
            low,
            mid,
            high,
            overload,
        }
    }
}

/// Reference points for a four-point calibration.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum FourPointRef {
    /// Low reference point (e.g., empty).
    Low,
    /// Mid reference point (e.g., 100ml / partial).
    Mid,
    /// High reference point (e.g., full).
    High,
    /// Overload reference point (e.g., stall).
    Overload,
}

impl<T> core::ops::Index<FourPointRef> for FourPointCalibration<T> {
    type Output = T;

    fn index(&self, index: FourPointRef) -> &Self::Output {
        match index {
            FourPointRef::Low => &self.low,
            FourPointRef::Mid => &self.mid,
            FourPointRef::High => &self.high,
            FourPointRef::Overload => &self.overload,
        }
    }
}

impl<T> core::ops::IndexMut<FourPointRef> for FourPointCalibration<T> {
    fn index_mut(&mut self, index: FourPointRef) -> &mut Self::Output {
        match index {
            FourPointRef::Low => &mut self.low,
            FourPointRef::Mid => &mut self.mid,
            FourPointRef::High => &mut self.high,
            FourPointRef::Overload => &mut self.overload,
        }
    }
}

/// Maximum number of proximity sensors.
pub const MAX_PROXIMITY_SENSORS: usize = 4;

/// Time-of-Flight (ToF) offset calibration values for VL53L0X.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cbor(array)]
pub struct Vl53l0xCalibration {
    /// Calibration for each sensor direction.
    #[n(0)]
    pub sensors: [TwoPointCalibration<u16>; MAX_PROXIMITY_SENSORS],
    /// Crosstalk compensation rates in milli-MCPS for each direction (default is 0/none).
    #[n(1)]
    pub xtalk_m_mcps: [u16; MAX_PROXIMITY_SENSORS],
}

impl Vl53l0xCalibration {
    /// Calibration filename in flash.
    pub const CALIBRATION_FILE_NAME: &'static str = "vl53l0x_cal.cbor";

    /// Applies the calibration for a specific direction to a reading.
    pub fn map_calibrated(
        &self,
        direction: crate::types::Direction,
        reading: crate::types::SensorReading,
    ) -> Result<crate::types::SensorReading, &'static str> {
        match reading {
            crate::types::SensorReading::Proximity(distance) => {
                let cal = self.sensors[direction as usize];
                if cal.low > 0 || cal.high > 0 {
                    Ok(crate::types::SensorReading::Proximity(cal.map(distance)))
                } else {
                    Ok(reading)
                }
            }
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}

impl core::ops::Index<crate::types::Direction> for Vl53l0xCalibration {
    type Output = TwoPointCalibration<u16>;

    fn index(&self, index: crate::types::Direction) -> &Self::Output {
        &self.sensors[index as usize]
    }
}

impl core::ops::IndexMut<crate::types::Direction> for Vl53l0xCalibration {
    fn index_mut(&mut self, index: crate::types::Direction) -> &mut Self::Output {
        &mut self.sensors[index as usize]
    }
}

/// Motor calibration data structure.
#[derive(Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cbor(array)]
pub struct MotorCalibration {
    /// Current calibration at four reference points (empty, 100ml, full, overload).
    #[n(0)]
    pub current_ma: FourPointCalibration<i32>,
    /// Physical maximum RPM at 100% duty cycle.
    #[n(1)]
    pub max_rpm: Option<u32>,
    /// Safety RPM limit.
    #[n(2)]
    pub rpm_limit: Option<u32>,
}

impl MotorCalibration {
    /// Calibration filename in flash.
    pub const CALIBRATION_FILE_NAME: &'static str = "motor_cal.cbor";

    /// Gets the calculated dry run/minimum current limit.
    pub fn dry_run_limit(&self) -> i32 {
        (self.current_ma.low + self.current_ma.mid) / 2
    }

    /// Gets the calculated stall/maximum current limit.
    /// Returns the average of the full bowl current and measured overload current if calibrated,
    /// otherwise falls back to a default safety limit of 800 mA.
    pub fn stall_limit(&self) -> i32 {
        if self.current_ma.overload > 0 {
            (self.current_ma.high + self.current_ma.overload) / 2
        } else {
            800
        }
    }
}

/// Trait representing a peripheral or controller that can be calibrated.
pub trait Calibration {
    /// Filename used to store calibration data in flash.
    const CALIBRATION_FILE_NAME: &'static str;

    /// Associated type representing the full file structure stored in flash.
    type Store: for<'b> minicbor::Decode<'b, ()> + minicbor::Encode<()> + Default;

    /// Sets the calibration parameters from the full store structure. By default, this does nothing (no-op).
    fn set_calibration(&mut self, _store: &Self::Store) {}

    /// Gets the current calibration parameters. By default, this returns the default store.
    fn get_calibration(&self) -> Self::Store {
        Self::Store::default()
    }
}

/// Trait representing a calibrated peripheral/controller that can apply its active calibration to a raw reading.
pub trait ApplyCalibration {
    /// Input reading type.
    type Input;
    /// Output reading type.
    type Output;
    /// Error type.
    type Error;

    /// Maps a raw reading to a calibrated reading.
    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error>;
}

impl ApplyCalibration for () {
    type Input = crate::types::SensorReading;
    type Output = crate::types::SensorReading;
    type Error = &'static str;

    fn apply_calibration(&self, reading: Self::Input) -> Result<Self::Output, Self::Error> {
        match reading {
            crate::types::SensorReading::Proximity(_) => Ok(reading),
            _ => Err("Non-proximity reading cannot be calibrated"),
        }
    }
}
