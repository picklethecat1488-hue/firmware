//! Generic fuel gauge interface.

#![deny(missing_docs)]

/// Trait representing a battery fuel gauge capable of reading state of charge.
pub trait FuelGauge {
    /// Error type returned by the physical hardware.
    type Error;

    /// Reads the current battery voltage in millivolts (mV).
    fn read_voltage_mv(&mut self) -> Result<u32, Self::Error>;

    /// Reads the current state of charge as a percentage (0-100).
    fn read_state_of_charge(&mut self) -> Result<u8, Self::Error>;

    /// Configure voltage and state of charge alerts.
    fn configure_alerts(
        &mut self,
        _voltage_min_mv: u32,
        _voltage_max_mv: u32,
        _soc_threshold_pct: u8,
        _enable_soc_change_alert: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Check and clear active alerts.
    /// Returns a tuple of (has_voltage_alert, has_soc_alert).
    fn check_and_clear_alerts(&mut self) -> Result<(bool, bool), Self::Error> {
        Ok((false, false))
    }
}
