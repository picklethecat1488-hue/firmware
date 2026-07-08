//! Shared system state and power management utilities.

#![deny(missing_docs)]

use crate::types::{BatteryThresholds, BatteryTransitionResult, BatteryUpdateInfo};
use model::types::SystemStatus;

/// Errors that can occur during system state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    /// Transition blocked because wake locks are still held.
    WakeLocksHeld(u32),
    /// Transition blocked because initial boot power down trap is active.
    BootPowerDownActive,
    /// Transition blocked because battery level is critical.
    BatteryCritical,
    /// Transition blocked because temperature is critical.
    ThermalCritical,
    /// State transition is not allowed under the current system state.
    InvalidTransition,
}

/// Actions to be taken in response to a battery status update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum BatteryUpdateAction {
    /// Go to power down mode immediately.
    GoToPowerDown,
    /// Clear the boot trap.
    ClearBootTrap,
    /// Report the state of charge / update LED.
    ReportSoC,
    /// No action needed.
    NoAction,
}

/// Pure transition function for waking the system.
/// Returns the next status if the transition is valid.
pub fn transition_wake(
    current_status: SystemStatus,
    battery_critical: bool,
    thermal_critical: bool,
    boot_power_down: bool,
) -> Option<SystemStatus> {
    if !battery_critical
        && !thermal_critical
        && current_status != SystemStatus::Active
        && !boot_power_down
    {
        Some(SystemStatus::Active)
    } else {
        None
    }
}

/// Pure transition function for sleeping the system.
/// Returns the next status if the transition is valid.
pub fn transition_sleep(
    current_status: SystemStatus,
    time_in_active_ms: u32,
    inactivity_timeout_ms: u32,
    battery_critical: bool,
    thermal_critical: bool,
    wake_locks: u32,
) -> Option<SystemStatus> {
    if current_status == SystemStatus::Active && wake_locks != 0 {
        return None;
    }
    let can_sleep =
        time_in_active_ms >= inactivity_timeout_ms || battery_critical || thermal_critical;
    if can_sleep
        && current_status != SystemStatus::Sleep
        && current_status != SystemStatus::PowerDown
    {
        Some(SystemStatus::Sleep)
    } else {
        None
    }
}

/// Pure transition function for powering down the system.
/// Returns the next status if the transition is valid.
pub fn transition_power_down(
    current_status: SystemStatus,
    wake_locks: u32,
) -> Option<SystemStatus> {
    if current_status == SystemStatus::Active && wake_locks != 0 {
        return None;
    }
    if current_status != SystemStatus::PowerDown {
        Some(SystemStatus::PowerDown)
    } else {
        None
    }
}

/// Pure transition function for handling battery status updates.
/// Returns the new battery critical state and the next system status.
pub fn transition_battery_update(
    current_status: SystemStatus,
    boot_power_down: bool,
    battery_critical: bool,
    info: BatteryUpdateInfo,
    thresholds: BatteryThresholds,
) -> BatteryTransitionResult {
    let new_critical = if battery_critical {
        info.is_fault
            || (info.state_of_charge < (thresholds.critical_threshold + thresholds.hysteresis)
                && !info.charging)
    } else {
        info.is_fault || (info.state_of_charge < thresholds.critical_threshold && !info.charging)
    };

    let mut next_status = None;
    if new_critical {
        if current_status != SystemStatus::PowerDown {
            next_status = Some(SystemStatus::PowerDown);
        }
    } else {
        let should_exit_power_down =
            current_status == SystemStatus::PowerDown && boot_power_down && !info.charging;
        if should_exit_power_down {
            next_status = Some(SystemStatus::Active);
        } else if current_status == SystemStatus::PowerDown {
            // If charging and already in PowerDown, we don't change state but stay in PowerDown
        } else if info.charging {
            next_status = Some(SystemStatus::PowerDown);
        }
    }

    BatteryTransitionResult {
        new_battery_critical: new_critical,
        next_status,
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
#[defmt::global_logger]
struct HostLogger;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
unsafe impl defmt::Logger for HostLogger {
    fn acquire() {}
    unsafe fn write(_bytes: &[u8]) {}
    unsafe fn flush() {}
    unsafe fn release() {}
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
defmt::timestamp!("{=u64}", 0);
