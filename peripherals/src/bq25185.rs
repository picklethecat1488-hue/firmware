//! Concrete driver implementation for the BQ25185 battery charger using GPIO pins.

#![deny(missing_docs)]

use crate::tracing;
use embedded_hal::digital::InputPin;
use model::interfaces::ChargeStatus;
use model::types::ChargeState;

/// Driver for the BQ25185 battery charger communicating via S1 and S2 status pins.
pub struct Bq25185<P1, P2> {
    s1: P1,
    s2: P2,
}

impl<P1: InputPin, P2: InputPin> Bq25185<P1, P2> {
    /// Creates a new BQ25185 driver using S1 (STAT1/FAULT) and S2 (STAT2/CHG) input pins.
    pub const fn new(s1: P1, s2: P2) -> Self {
        Self { s1, s2 }
    }

    /// Read the current state of the charger from the S1 and S2 pins.
    pub fn get_state(&mut self) -> ChargeState {
        let s1_high = self.s1.is_high().unwrap_or(true);
        let s2_high = self.s2.is_high().unwrap_or(true);
        match (s1_high, s2_high) {
            (true, false) => ChargeState::Charging,
            (true, true) => ChargeState::DoneOrStandbyOrUnplugged,
            (false, true) => ChargeState::RecoverableFault,
            (false, false) => ChargeState::NonRecoverableFault,
        }
    }
}

impl<P1: InputPin, P2: InputPin> ChargeStatus for Bq25185<P1, P2> {
    type Error = core::convert::Infallible;

    /// Checks the current charge state.
    #[tracing::instrument(level = "trace")]
    fn get_charge_state(&mut self) -> Result<ChargeState, Self::Error> {
        Ok(self.get_state())
    }
}
