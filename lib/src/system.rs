//! Shared system state and power management utilities.

#![deny(missing_docs)]

use crate::types::{BatteryThresholds, BatteryTransitionResult, BatteryUpdateInfo};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use model::types::{
    BootReason, Gesture, SystemCommand, SystemLedState, SystemStatus, TelemetryRecord,
};

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

/// Generic container for the system's power state, timers, and critical statuses.
pub struct SystemStateManager<MutexRaw: RawMutex + 'static, const N: usize> {
    status: SystemStatus,
    inactive_ms: u32,
    active_ms: u32,
    interval_ms: u32,
    tick_ms_accumulator: u32,
    battery_critical: bool,
    thermal_critical: bool,
    charger_connected: bool,
    latest_state_of_charge: u8,
    boot_power_down: bool,
    critical_soc_threshold: u8,
    soc_hysteresis: u8,
    low_soc_threshold: u8,
    mid_soc_threshold: u8,
    high_soc_threshold: u8,
    wake_locks: u32,
    telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
    system_tx: Option<Sender<'static, MutexRaw, SystemCommand, 4>>,
    first_battery_update: bool,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> core::fmt::Debug
    for SystemStateManager<MutexRaw, N>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SystemStateManager")
            .field("status", &self.status)
            .field("inactive_ms", &self.inactive_ms)
            .field("active_ms", &self.active_ms)
            .field("interval_ms", &self.interval_ms)
            .field("tick_ms_accumulator", &self.tick_ms_accumulator)
            .field("battery_critical", &self.battery_critical)
            .field("thermal_critical", &self.thermal_critical)
            .field("charger_connected", &self.charger_connected)
            .field("latest_state_of_charge", &self.latest_state_of_charge)
            .field("boot_power_down", &self.boot_power_down)
            .field("critical_soc_threshold", &self.critical_soc_threshold)
            .field("soc_hysteresis", &self.soc_hysteresis)
            .field("low_soc_threshold", &self.low_soc_threshold)
            .field("mid_soc_threshold", &self.mid_soc_threshold)
            .field("high_soc_threshold", &self.high_soc_threshold)
            .field("wake_locks", &self.wake_locks)
            .finish()
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> Clone for SystemStateManager<MutexRaw, N> {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            inactive_ms: self.inactive_ms,
            active_ms: self.active_ms,
            interval_ms: self.interval_ms,
            tick_ms_accumulator: self.tick_ms_accumulator,
            battery_critical: self.battery_critical,
            thermal_critical: self.thermal_critical,
            charger_connected: self.charger_connected,
            latest_state_of_charge: self.latest_state_of_charge,
            boot_power_down: self.boot_power_down,
            critical_soc_threshold: self.critical_soc_threshold,
            soc_hysteresis: self.soc_hysteresis,
            low_soc_threshold: self.low_soc_threshold,
            mid_soc_threshold: self.mid_soc_threshold,
            high_soc_threshold: self.high_soc_threshold,
            wake_locks: self.wake_locks,
            telemetry_tx: self.telemetry_tx,
            system_tx: self.system_tx,
            first_battery_update: self.first_battery_update,
        }
    }
}

// Manual PartialEq implementation is required because embassy_sync::channel::Sender
// does not implement PartialEq, and comparing telemetry senders is not necessary
// to determine if the FSM state of two SystemStateManagers is equal.
impl<MutexRaw: RawMutex + 'static, const N: usize> PartialEq for SystemStateManager<MutexRaw, N> {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.inactive_ms == other.inactive_ms
            && self.active_ms == other.active_ms
            && self.interval_ms == other.interval_ms
            && self.tick_ms_accumulator == other.tick_ms_accumulator
            && self.battery_critical == other.battery_critical
            && self.thermal_critical == other.thermal_critical
            && self.charger_connected == other.charger_connected
            && self.latest_state_of_charge == other.latest_state_of_charge
            && self.boot_power_down == other.boot_power_down
            && self.critical_soc_threshold == other.critical_soc_threshold
            && self.soc_hysteresis == other.soc_hysteresis
            && self.low_soc_threshold == other.low_soc_threshold
            && self.mid_soc_threshold == other.mid_soc_threshold
            && self.high_soc_threshold == other.high_soc_threshold
            && self.wake_locks == other.wake_locks
            && self.first_battery_update == other.first_battery_update
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> Eq for SystemStateManager<MutexRaw, N> {}

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

impl<MutexRaw: RawMutex + 'static, const N: usize> SystemStateManager<MutexRaw, N> {
    /// Creates a new SystemStateManager.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        critical_soc_threshold: u8,
        soc_hysteresis: u8,
        low_soc_threshold: u8,
        mid_soc_threshold: u8,
        high_soc_threshold: u8,
        telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
        system_tx: Option<Sender<'static, MutexRaw, SystemCommand, 4>>,
        boot_reason: BootReason,
    ) -> Self {
        let _ = telemetry_tx.try_send(TelemetryRecord::Boot(boot_reason));
        Self {
            status: SystemStatus::PowerDown,
            inactive_ms: 0,
            active_ms: 0,
            interval_ms: 1000,
            tick_ms_accumulator: 0,
            battery_critical: true,
            thermal_critical: false,
            charger_connected: false,
            latest_state_of_charge: 50,
            boot_power_down: true,
            critical_soc_threshold,
            soc_hysteresis,
            low_soc_threshold,
            mid_soc_threshold,
            high_soc_threshold,
            wake_locks: 0,
            telemetry_tx,
            system_tx,
            first_battery_update: true,
        }
    }

    /// Logs a telemetry record.
    pub fn log_telemetry(&self, record: TelemetryRecord) {
        let _ = self.telemetry_tx.try_send(record);
    }

    /// Log gesture telemetry.
    pub fn log_gesture_telemetry(&self, gesture: Gesture) {
        self.log_telemetry(TelemetryRecord::Gesture(gesture));
    }

    /// Returns the current system status.
    pub const fn status(&self) -> SystemStatus {
        self.status
    }

    /// Sets the current system status.
    pub fn set_status(&mut self, status: SystemStatus) -> Result<(), TransitionError> {
        let prev_status = self.status;
        if prev_status != status {
            // Validate using pure transition functions
            match status {
                SystemStatus::Active => {
                    if transition_wake(
                        prev_status,
                        self.battery_critical,
                        self.thermal_critical,
                        self.boot_power_down,
                    )
                    .is_none()
                    {
                        if self.boot_power_down {
                            return Err(TransitionError::BootPowerDownActive);
                        } else if self.battery_critical {
                            return Err(TransitionError::BatteryCritical);
                        } else if self.thermal_critical {
                            return Err(TransitionError::ThermalCritical);
                        } else {
                            return Err(TransitionError::InvalidTransition);
                        }
                    }
                }
                SystemStatus::Sleep => {
                    if transition_sleep(
                        prev_status,
                        1, // Force can_sleep logic to true since caller requests it
                        0,
                        self.battery_critical,
                        self.thermal_critical,
                        self.wake_locks,
                    )
                    .is_none()
                    {
                        if prev_status == SystemStatus::Active && self.wake_locks != 0 {
                            defmt::warn!(
                                "SystemStateManager: Cannot transition away from Active state while holding wake locks ({:#X})!",
                                self.wake_locks
                            );
                            return Err(TransitionError::WakeLocksHeld(self.wake_locks));
                        }
                        return Err(TransitionError::InvalidTransition);
                    }
                }
                SystemStatus::PowerDown => {
                    if transition_power_down(prev_status, self.wake_locks).is_none() {
                        if prev_status == SystemStatus::Active && self.wake_locks != 0 {
                            defmt::warn!(
                                "SystemStateManager: Cannot transition away from Active state while holding wake locks ({:#X})!",
                                self.wake_locks
                            );
                            return Err(TransitionError::WakeLocksHeld(self.wake_locks));
                        }
                        return Err(TransitionError::InvalidTransition);
                    }
                }
            }
            self.status = status;
            self.log_telemetry(TelemetryRecord::System(status));
            if let Some(ref tx) = self.system_tx {
                let _ = tx.try_send(SystemCommand::StateChanged {
                    from: prev_status,
                    to: status,
                });
            }
        }
        Ok(())
    }

    /// Acquires a wake lock, resetting the inactivity timer to 0.
    pub fn acquire_wake_lock(&mut self, client_id: Option<u32>) {
        let id = client_id.unwrap_or(0);
        if id >= 32 {
            panic!("WakeLock: client_id {} out of bounds!", id);
        }
        let mask = 1u32 << id;
        if (self.wake_locks & mask) != 0 {
            defmt::warn!("WakeLock: client {} double-acquired!", id);
        } else {
            self.wake_locks |= mask;
        }
        self.inactive_ms = 0;
    }

    /// Releases a wake lock.
    pub fn release_wake_lock(&mut self, client_id: Option<u32>) {
        let id = client_id.unwrap_or(0);
        if id >= 32 {
            panic!("WakeLock: client_id {} out of bounds!", id);
        }
        let mask = 1u32 << id;
        if (self.wake_locks & mask) == 0 {
            defmt::warn!("WakeLock: client {} double-released!", id);
        } else {
            self.wake_locks &= !mask;
        }
    }

    /// Returns the number of active wake locks.
    pub const fn wake_lock_count(&self) -> u32 {
        self.wake_locks.count_ones()
    }

    /// Returns the raw active wake locks bitmask.
    pub const fn wake_locks(&self) -> u32 {
        self.wake_locks
    }

    /// Returns the inactivity timer in seconds.
    pub const fn inactive_ms(&self) -> u32 {
        self.inactive_ms
    }

    /// Sets the inactivity timer in seconds.
    pub fn set_inactive_ms(&mut self, val: u32) {
        self.inactive_ms = val;
    }

    /// Returns the time spent in active state in seconds.
    pub const fn active_ms(&self) -> u32 {
        self.active_ms
    }

    /// Returns the time spent in active state in seconds.
    pub const fn interval_ms(&self) -> u32 {
        self.interval_ms
    }

    /// Sets the inactivity timer in seconds.
    pub fn set_interval_ms(&mut self, val: u32) {
        self.interval_ms = val;
    }

    /// Returns if the battery is critical.
    pub const fn battery_critical(&self) -> bool {
        self.battery_critical
    }

    /// Sets if the battery is critical.
    pub fn set_battery_critical(&mut self, val: bool) {
        self.battery_critical = val;
    }

    /// Returns if the thermal state is critical.
    pub const fn thermal_critical(&self) -> bool {
        self.thermal_critical
    }

    /// Sets if the thermal state is critical.
    pub fn set_thermal_critical(&mut self, val: bool) {
        self.thermal_critical = val;
    }

    /// Returns if the charger is connected.
    pub const fn charger_connected(&self) -> bool {
        self.charger_connected
    }

    /// Sets if the charger is connected.
    pub fn set_charger_connected(&mut self, val: bool) {
        self.charger_connected = val;
    }

    /// Returns the latest reported state of charge.
    pub const fn latest_state_of_charge(&self) -> u8 {
        self.latest_state_of_charge
    }

    /// Returns if the system booted in power down.
    pub const fn boot_power_down(&self) -> bool {
        self.boot_power_down
    }

    /// Sets if the system booted in power down.
    pub fn set_boot_power_down(&mut self, val: bool) {
        self.boot_power_down = val;
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
            self.status,
            self.boot_power_down,
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
            if self.status != SystemStatus::PowerDown {
                BatteryUpdateAction::GoToPowerDown
            } else if changed {
                BatteryUpdateAction::ReportSoC
            } else {
                BatteryUpdateAction::NoAction
            }
        } else if self.status == SystemStatus::PowerDown {
            if self.boot_power_down && !self.charger_connected {
                BatteryUpdateAction::ClearBootTrap
            } else if changed {
                BatteryUpdateAction::ReportSoC
            } else {
                BatteryUpdateAction::NoAction
            }
        } else if self.charger_connected {
            BatteryUpdateAction::GoToPowerDown
        } else if self.status == SystemStatus::Active && changed {
            BatteryUpdateAction::ReportSoC
        } else {
            BatteryUpdateAction::NoAction
        }
    }

    /// Process a tick of `ms` milliseconds.
    /// Returns true if the 1-second boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        if self.status == SystemStatus::Active {
            self.tick_ms_accumulator = self.tick_ms_accumulator.saturating_add(ms);
            if self.tick_ms_accumulator >= self.interval_ms {
                self.tick_ms_accumulator =
                    self.tick_ms_accumulator.saturating_sub(self.interval_ms);
                self.active_ms = self.active_ms.saturating_add(self.interval_ms);
                if self.wake_locks > 0 {
                    self.inactive_ms = 0;
                } else {
                    self.inactive_ms = self.inactive_ms.saturating_add(self.interval_ms);
                }
                return true;
            }
        } else {
            self.tick_ms_accumulator = 0;
        }
        false
    }

    /// Resets the boot power-down flag and active/inactivity timers on system wakeup.
    pub fn reset_on_wake(&mut self) {
        self.boot_power_down = false;
        self.inactive_ms = 0;
        self.active_ms = 0;
        self.wake_locks = 0;
    }

    /// Clears all active wake locks.
    pub fn clear_wake_locks(&mut self) {
        self.wake_locks = 0;
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
