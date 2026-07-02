//! Generic battery charge status interface.

#![deny(missing_docs)]

/// Trait representing battery charge status.
pub trait ChargeStatus {
    /// Error type returned by the physical hardware.
    type Error;

    /// Returns the current charge state.
    fn get_charge_state(&mut self) -> Result<crate::types::ChargeState, Self::Error>;
}
