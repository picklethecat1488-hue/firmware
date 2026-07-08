//! Thermal manager submodule.

/// Manages temperature alerts and thermal critical status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThermalManager {
    thermal_critical: bool,
}

impl ThermalManager {
    /// Creates a new ThermalManager.
    pub fn new() -> Self {
        Self {
            thermal_critical: false,
        }
    }

    /// Returns the thermal critical status.
    pub const fn thermal_critical(&self) -> bool {
        self.thermal_critical
    }

    /// Sets the thermal critical status.
    pub fn set_thermal_critical(&mut self, val: bool) {
        self.thermal_critical = val;
    }
}

impl Default for ThermalManager {
    fn default() -> Self {
        Self::new()
    }
}
