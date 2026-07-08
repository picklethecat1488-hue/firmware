//! Battery manager submodule.

use crate::system::{transition_battery_update, BatteryUpdateAction};
use crate::types::{BatteryThresholds, BatteryUpdateInfo};
use model::types::{SystemLedState, SystemStatus};

/// Manages battery thresholds, state of charge, and battery critical detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryManager {
    battery_critical: bool,
    charger_connected: bool,
    latest_state_of_charge: u8,
    first_battery_update: bool,
    critical_soc_threshold: u8,
    soc_hysteresis: u8,
    low_soc_threshold: u8,
    mid_soc_threshold: u8,
    high_soc_threshold: u8,
}

impl BatteryManager {
    /// Creates a new BatteryManager.
    pub fn new(
        critical_soc_threshold: u8,
        soc_hysteresis: u8,
        low_soc_threshold: u8,
        mid_soc_threshold: u8,
        high_soc_threshold: u8,
    ) -> Self {
        Self {
            battery_critical: true,
            charger_connected: false,
            latest_state_of_charge: 50,
            first_battery_update: true,
            critical_soc_threshold,
            soc_hysteresis,
            low_soc_threshold,
            mid_soc_threshold,
            high_soc_threshold,
        }
    }

    /// Returns the battery critical status.
    pub const fn battery_critical(&self) -> bool {
        self.battery_critical
    }

    /// Sets the battery critical status.
    pub fn set_battery_critical(&mut self, val: bool) {
        self.battery_critical = val;
    }

    /// Returns the charger connection status.
    pub const fn charger_connected(&self) -> bool {
        self.charger_connected
    }

    /// Returns the latest state of charge.
    pub const fn latest_state_of_charge(&self) -> u8 {
        self.latest_state_of_charge
    }

    /// Returns critical SoC threshold.
    pub const fn critical_soc_threshold(&self) -> u8 {
        self.critical_soc_threshold
    }

    /// Sets critical SoC threshold.
    pub fn set_critical_soc_threshold(&mut self, val: u8) {
        self.critical_soc_threshold = val;
    }

    /// Returns SoC hysteresis.
    pub const fn soc_hysteresis(&self) -> u8 {
        self.soc_hysteresis
    }

    /// Sets SoC hysteresis.
    pub fn set_soc_hysteresis(&mut self, val: u8) {
        self.soc_hysteresis = val;
    }

    /// Maps the battery SoC to the correct LED state.
    pub const fn get_soc_led_state(&self) -> SystemLedState {
        if self.battery_critical {
            SystemLedState::BlinksRedOncePerThirtySeconds
        } else if self.latest_state_of_charge <= self.low_soc_threshold {
            SystemLedState::SolidOrange
        } else if self.latest_state_of_charge >= self.mid_soc_threshold
            && self.latest_state_of_charge < self.high_soc_threshold
        {
            SystemLedState::SolidYellow
        } else {
            SystemLedState::SolidGreen
        }
    }

    /// Handles battery status updates and updates the internal critical flag.
    /// Returns the action to take in response to the update.
    pub fn update_battery_status(
        &mut self,
        state_of_charge: u8,
        charging: bool,
        is_fault: bool,
        system_status: SystemStatus,
        boot_power_down: bool,
    ) -> BatteryUpdateAction {
        let old_led_state = self.get_soc_led_state();
        let old_charger_connected = self.charger_connected;
        let old_critical = self.battery_critical;

        self.charger_connected = charging;
        self.latest_state_of_charge = state_of_charge;

        let info = BatteryUpdateInfo {
            state_of_charge,
            charging,
            is_fault,
        };
        let thresholds = BatteryThresholds {
            critical_threshold: self.critical_soc_threshold,
            hysteresis: self.soc_hysteresis,
        };

        let res = transition_battery_update(
            system_status,
            boot_power_down,
            self.battery_critical,
            info,
            thresholds,
        );

        self.battery_critical = res.new_battery_critical;

        let is_first = self.first_battery_update;
        self.first_battery_update = false;

        let changed = old_critical != self.battery_critical
            || old_charger_connected != self.charger_connected
            || old_led_state != self.get_soc_led_state()
            || is_first;

        if self.battery_critical {
            if system_status != SystemStatus::PowerDown {
                BatteryUpdateAction::GoToPowerDown
            } else if changed {
                BatteryUpdateAction::ReportSoC
            } else {
                BatteryUpdateAction::NoAction
            }
        } else if system_status == SystemStatus::PowerDown {
            if boot_power_down && !self.charger_connected {
                BatteryUpdateAction::ClearBootTrap
            } else if changed {
                BatteryUpdateAction::ReportSoC
            } else {
                BatteryUpdateAction::NoAction
            }
        } else if self.charger_connected {
            BatteryUpdateAction::GoToPowerDown
        } else if system_status == SystemStatus::Active && changed {
            BatteryUpdateAction::ReportSoC
        } else {
            BatteryUpdateAction::NoAction
        }
    }
}
