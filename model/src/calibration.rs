//! Calibration data structures.

#![deny(missing_docs)]

/// A general two-point calibration structure mapping raw readings at two reference points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
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
    /// Maps `low` to 0, and `high` to `scale` (e.g. 100).
    pub fn map(&self, raw: u16, scale: u32) -> u16 {
        if self.high > self.low {
            if raw <= self.low {
                0
            } else {
                (((raw - self.low) as u32 * scale) / (self.high - self.low) as u32) as u16
            }
        } else {
            raw
        }
    }
}

/// A generic four-point calibration structure mapping low, mid, high, and overload reference states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Time-of-Flight (ToF) offset calibration values for VL53L0X.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cbor(array)]
pub struct Vl53l0xCalibration {
    /// Calibration for each sensor direction.
    #[n(0)]
    pub sensors: [TwoPointCalibration<u16>; 3],
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
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

/// Enum representing different types of calibration parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationType {
    /// Calibration for proximity sensors, specifying the cover (0mm) raw value and the 100mm raw value.
    ProximityCal(TwoPointCalibration<u16>),
    /// Calibration for motor current/load values, physical maximum RPM, and RPM safety limit.
    MotorCal {
        /// Current limit range (min/max).
        current_limits: TwoPointCalibration<i32>,
        /// Physical maximum RPM at 100% duty cycle.
        max_rpm: u32,
        /// Maximum RPM limit for safety cut-off.
        rpm_limit: u32,
    },
}

/// Trait representing a peripheral or controller that can be calibrated.
pub trait Calibration {
    /// Sets the calibration parameters. By default, this does nothing (no-op).
    fn set_calibration(&mut self, _calibration: CalibrationType) {}
}
