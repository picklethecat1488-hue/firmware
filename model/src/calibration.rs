//! Calibration data structures.

#![deny(missing_docs)]

/// Time-of-Flight (ToF) offset calibration values for VL53L0X.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cbor(array)]
pub struct Vl53l0xCalibration {
    /// Calibration offset for North sensor cover.
    #[n(0)]
    pub north_near: u16,
    /// Calibration offset for North sensor 100mm reading.
    #[n(1)]
    pub north_100: u16,
    /// Calibration offset for East sensor cover.
    #[n(2)]
    pub east_near: u16,
    /// Calibration offset for East sensor 100mm reading.
    #[n(3)]
    pub east_100: u16,
    /// Calibration offset for West sensor cover.
    #[n(4)]
    pub west_near: u16,
    /// Calibration offset for West sensor 100mm reading.
    #[n(5)]
    pub west_100: u16,
}

/// Motor calibration data structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, minicbor::Encode, minicbor::Decode)]
#[cbor(array)]
pub struct MotorCalibration {
    /// Average current in mA when the water bowl is empty.
    #[n(0)]
    pub empty_current_ma: i32,
    /// Average current in mA with 100ml of water in the bowl.
    #[n(1)]
    pub water_100ml_current_ma: i32,
    /// Average current in mA when the bowl is full.
    #[n(2)]
    pub full_current_ma: i32,
    /// Physical maximum RPM at 100% duty cycle.
    #[n(3)]
    pub max_rpm: Option<u32>,
    /// Safety RPM limit.
    #[n(4)]
    pub rpm_limit: Option<u32>,
}

/// Enum representing different types of calibration parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationType {
    /// Calibration for proximity sensors, specifying the cover (0mm) raw value and the 100mm raw value.
    ProximityCal(u16, u16),
    /// Calibration for motor current/load values, physical maximum RPM, and RPM safety limit.
    MotorCal(i32, i32, u32, u32),
}

/// Trait representing a peripheral or controller that can be calibrated.
pub trait Calibration {
    /// Sets the calibration parameters. By default, this does nothing (no-op).
    fn set_calibration(&mut self, _calibration: CalibrationType) {}
}
