//! Shared system state and power management utilities.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use model::types::{Gesture, ProximityTelemetry, SystemLedState, SystemStatus, TelemetryRecord};

/// Result of a battery update state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryTransitionResult {
    /// The new battery critical flag value.
    pub new_battery_critical: bool,
    /// The next system status if a transition occurred.
    pub next_status: Option<SystemStatus>,
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
        && current_status != SystemStatus::PowerDown
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
    time_in_active: u32,
    inactivity_timeout_seconds: u32,
    battery_critical: bool,
    thermal_critical: bool,
) -> Option<SystemStatus> {
    let can_sleep =
        time_in_active >= inactivity_timeout_seconds || battery_critical || thermal_critical;
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
pub fn transition_power_down(current_status: SystemStatus) -> Option<SystemStatus> {
    if current_status != SystemStatus::PowerDown {
        Some(SystemStatus::PowerDown)
    } else {
        None
    }
}

/// Context info containing state-of-charge measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryUpdateInfo {
    /// Percentage integer (0-100)
    pub state_of_charge: u8,
    /// Is the charger connected?
    pub charging: bool,
    /// Is there a fault?
    pub is_fault: bool,
}

/// Threshold values for battery safety transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryThresholds {
    /// Critical SOC percentage limit
    pub critical_threshold: u8,
    /// Recovery hysteresis value
    pub hysteresis: u8,
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
    inactivity_seconds: u32,
    time_in_active: u32,
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
    telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> core::fmt::Debug
    for SystemStateManager<MutexRaw, N>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SystemStateManager")
            .field("status", &self.status)
            .field("inactivity_seconds", &self.inactivity_seconds)
            .field("time_in_active", &self.time_in_active)
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
            .finish()
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> Clone for SystemStateManager<MutexRaw, N> {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            inactivity_seconds: self.inactivity_seconds,
            time_in_active: self.time_in_active,
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
            telemetry_tx: self.telemetry_tx,
        }
    }
}

// Manual PartialEq implementation is required because embassy_sync::channel::Sender
// does not implement PartialEq, and comparing telemetry senders is not necessary
// to determine if the FSM state of two SystemStateManagers is equal.
impl<MutexRaw: RawMutex + 'static, const N: usize> PartialEq for SystemStateManager<MutexRaw, N> {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.inactivity_seconds == other.inactivity_seconds
            && self.time_in_active == other.time_in_active
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
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> Eq for SystemStateManager<MutexRaw, N> {}

impl<MutexRaw: RawMutex + 'static, const N: usize> SystemStateManager<MutexRaw, N> {
    /// Creates a new SystemStateManager.
    pub fn new(
        critical_soc_threshold: u8,
        soc_hysteresis: u8,
        low_soc_threshold: u8,
        mid_soc_threshold: u8,
        high_soc_threshold: u8,
        telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
    ) -> Self {
        Self {
            status: SystemStatus::PowerDown,
            inactivity_seconds: 0,
            time_in_active: 0,
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
            telemetry_tx,
        }
    }

    /// Logs a telemetry record.
    pub fn log_telemetry(&self, record: TelemetryRecord) {
        let _ = self.telemetry_tx.try_send(record);
    }

    /// Log proximity telemetry.
    pub fn log_proximity_telemetry(&self, distance_mm: u16, threshold_mm: u16) {
        let prox = if distance_mm < threshold_mm {
            ProximityTelemetry::InRange(distance_mm)
        } else {
            ProximityTelemetry::OutRange(distance_mm)
        };
        self.log_telemetry(TelemetryRecord::Proximity(prox));
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
    pub fn set_status(&mut self, status: SystemStatus) {
        self.status = status;
        self.log_telemetry(TelemetryRecord::System(status));
    }

    /// Returns the inactivity timer in seconds.
    pub const fn inactivity_seconds(&self) -> u32 {
        self.inactivity_seconds
    }

    /// Sets the inactivity timer in seconds.
    pub fn set_inactivity_seconds(&mut self, val: u32) {
        self.inactivity_seconds = val;
    }

    /// Returns the time spent in active state in seconds.
    pub const fn time_in_active(&self) -> u32 {
        self.time_in_active
    }

    /// Sets the time spent in active state in seconds.
    pub fn set_time_in_active(&mut self, val: u32) {
        self.time_in_active = val;
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
    /// Returns true if battery entered or exited critical state, or charging status changed.
    pub fn update_battery_status(
        &mut self,
        state_of_charge: u8,
        charging: bool,
        is_fault: bool,
    ) -> bool {
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

        let old_critical = self.battery_critical;
        self.battery_critical = res.new_battery_critical;

        old_critical != self.battery_critical
    }

    /// Process a tick of `ms` milliseconds.
    /// Returns true if the 1-second boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        if self.status == SystemStatus::Active {
            self.tick_ms_accumulator += ms;
            if self.tick_ms_accumulator >= 1000 {
                self.tick_ms_accumulator -= 1000;
                self.time_in_active += 1;
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
        self.inactivity_seconds = 0;
        self.time_in_active = 0;
    }
}
