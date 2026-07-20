use crate::system::{transition_power_down, transition_sleep, transition_wake, TransitionError};
use crate::types::{BootTrapMask, BootTrapReason, Sender};
use embassy_sync::blocking_mutex::raw::RawMutex;
use model::types::{BootReason, Gesture, SystemStatus, TelemetryRecord};

/// Manages system status transitions, wake locks, and sleep timers.
pub struct PowerManager<MutexRaw: RawMutex + 'static, const N: usize> {
    status: SystemStatus,
    inactive_ms: u32,
    active_ms: u32,
    interval_ms: u32,
    tick_ms_accumulator: u32,
    boot_trap_mask: BootTrapMask,
    wake_locks: u32,
    telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl<MutexRaw: RawMutex + 'static, const N: usize> core::fmt::Debug for PowerManager<MutexRaw, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PowerManager")
            .field("status", &self.status)
            .field("inactive_ms", &self.inactive_ms)
            .field("active_ms", &self.active_ms)
            .field("interval_ms", &self.interval_ms)
            .field("tick_ms_accumulator", &self.tick_ms_accumulator)
            .field("boot_trap_mask", &self.boot_trap_mask)
            .field("wake_locks", &self.wake_locks)
            .finish()
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> PartialEq for PowerManager<MutexRaw, N> {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.inactive_ms == other.inactive_ms
            && self.active_ms == other.active_ms
            && self.interval_ms == other.interval_ms
            && self.tick_ms_accumulator == other.tick_ms_accumulator
            && self.boot_trap_mask == other.boot_trap_mask
            && self.wake_locks == other.wake_locks
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> Eq for PowerManager<MutexRaw, N> {}

impl<MutexRaw: RawMutex + 'static, const N: usize> PowerManager<MutexRaw, N> {
    /// Creates a new PowerManager.
    pub fn new(
        telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, N>,
        boot_reason: BootReason,
    ) -> Self {
        let _ = telemetry_tx.try_send(TelemetryRecord::Boot(boot_reason));
        Self {
            status: SystemStatus::PowerDown,
            inactive_ms: 0,
            active_ms: 0,
            interval_ms: 1000,
            tick_ms_accumulator: 0,
            boot_trap_mask: BootTrapMask::from_raw(BootTrapReason::Battery as u32),
            wake_locks: 0,
            telemetry_tx,
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
    /// Returns Ok(Some(prev_status)) if the status changed, Ok(None) if the status was already set to the requested value, or Err(TransitionError) on failure.
    pub fn set_status(
        &mut self,
        status: SystemStatus,
        battery_critical: bool,
        thermal_critical: bool,
    ) -> Result<Option<SystemStatus>, TransitionError> {
        let prev_status = self.status;
        if prev_status != status {
            // Validate using pure transition functions
            match status {
                SystemStatus::Active => {
                    if transition_wake(
                        prev_status,
                        battery_critical,
                        thermal_critical,
                        self.is_boot_trapped(),
                    )
                    .is_none()
                    {
                        if self.is_boot_trapped() {
                            return Err(TransitionError::BootPowerDownActive);
                        } else if battery_critical {
                            return Err(TransitionError::BatteryCritical);
                        } else if thermal_critical {
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
                        battery_critical,
                        thermal_critical,
                        self.wake_locks,
                    )
                    .is_none()
                    {
                        if prev_status == SystemStatus::Active && self.wake_locks != 0 {
                            defmt::warn!(
                                "PowerManager: Cannot transition away from Active state while holding wake locks ({:#X})!",
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
                                "PowerManager: Cannot transition away from Active state while holding wake locks ({:#X})!",
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
            Ok(Some(prev_status))
        } else {
            Ok(None)
        }
    }

    /// Acquires a wake lock, resetting the inactivity timer to 0.
    pub fn acquire_wake_lock(&mut self, client_id: Option<u32>) {
        let id = client_id.unwrap_or(0);
        if id >= 32 {
            panic!("WakeLock: client_id {} out of bounds!", id);
        }
        self.wake_locks |= 1 << id;
        self.inactive_ms = 0;
    }

    /// Releases a wake lock.
    pub fn release_wake_lock(&mut self, client_id: Option<u32>) {
        let id = client_id.unwrap_or(0);
        if id >= 32 {
            panic!("WakeLock: client_id {} out of bounds!", id);
        }
        self.wake_locks &= !(1 << id);
    }

    /// Resets wake locks back to 0.
    pub fn reset_on_wake(&mut self) {
        self.wake_locks = 0;
        self.inactive_ms = 0;
    }

    /// Clears all active wake locks.
    pub fn clear_wake_locks(&mut self) {
        self.wake_locks = 0;
    }

    /// Returns the number of currently active wake locks.
    pub const fn wake_lock_count(&self) -> u32 {
        self.wake_locks.count_ones()
    }

    /// Returns the raw wake lock bitmask.
    pub const fn wake_locks(&self) -> u32 {
        self.wake_locks
    }

    /// Returns the inactivity timer in milliseconds.
    pub const fn inactive_ms(&self) -> u32 {
        self.inactive_ms
    }

    /// Sets the inactivity timer.
    pub fn set_inactive_ms(&mut self, val: u32) {
        self.inactive_ms = val;
    }

    /// Returns the active duration timer in milliseconds.
    pub const fn active_ms(&self) -> u32 {
        self.active_ms
    }

    /// Returns the interval duration in milliseconds.
    pub const fn interval_ms(&self) -> u32 {
        self.interval_ms
    }

    /// Sets the interval duration.
    pub fn set_interval_ms(&mut self, val: u32) {
        self.interval_ms = val;
    }

    /// Checks if any boot traps are still active.
    pub const fn is_boot_trapped(&self) -> bool {
        !self.boot_trap_mask.is_empty()
    }

    /// Checks if a specific boot trap is active.
    pub const fn has_boot_trap(&self, reason: BootTrapReason) -> bool {
        self.boot_trap_mask.has(reason)
    }

    /// Clears a specific boot trap.
    pub fn clear_boot_trap(&mut self, reason: BootTrapReason) {
        self.boot_trap_mask.remove(reason);
    }

    /// Returns the boot trap mask.
    pub const fn boot_trap_mask(&self) -> BootTrapMask {
        self.boot_trap_mask
    }

    /// Sets the boot trap mask, returning an error if it contains invalid bits.
    pub fn set_boot_trap_mask(
        &mut self,
        mask: BootTrapMask,
    ) -> Result<(), crate::types::InvalidBootTrapMask> {
        mask.validate()?;
        self.boot_trap_mask = mask;
        Ok(())
    }

    /// Process a tick of `ms` milliseconds.
    /// Returns true if the 1-second boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        if self.status == SystemStatus::Active {
            self.tick_ms_accumulator = self.tick_ms_accumulator.saturating_add(ms);
            if self.tick_ms_accumulator >= self.interval_ms {
                let seconds = self.tick_ms_accumulator / self.interval_ms;
                self.tick_ms_accumulator %= self.interval_ms;
                let actual_seconds_ms = seconds * self.interval_ms;
                self.active_ms = self.active_ms.saturating_add(actual_seconds_ms);

                if self.wake_locks == 0 {
                    self.inactive_ms = self.inactive_ms.saturating_add(actual_seconds_ms);
                } else {
                    self.inactive_ms = 0;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
