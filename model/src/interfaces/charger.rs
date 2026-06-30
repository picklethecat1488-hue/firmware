//! Generic battery charger interface.

#![deny(missing_docs)]

/// Trait representing a battery charger.
pub trait Charger {
    /// Error type returned by the physical hardware.
    type Error;

    /// Enables or disables battery charging.
    fn set_charging_enabled(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Returns whether the charging power source (VBUS) is connected.
    fn is_charging_input_present(&mut self) -> Result<bool, Self::Error>;
}
