use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use firmware_lib::battery_manager::BatteryManager;
use firmware_lib::power_manager::PowerManager;
use firmware_lib::system::{BatteryUpdateAction, TransitionError};
use firmware_lib::thermal_manager::ThermalManager;
use firmware_lib::BootTrapMask;
use model::types::{BootReason, ChargeState, SystemLedState, SystemStatus, TelemetryRecord};

static TEST_TELEMETRY_CHANNEL: Channel<CriticalSectionRawMutex, TelemetryRecord, 16> =
    Channel::new();

#[test]
fn test_subsystem_managers_initialization() {
    let power = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);
    assert_eq!(power.status(), SystemStatus::PowerDown);
    assert_eq!(power.inactive_ms(), 0);
    assert_eq!(power.active_ms(), 0);
    assert!(power.is_boot_trapped());

    let battery = BatteryManager::new(10, 2, 20, 21, 80);
    assert!(battery.battery_critical());
    assert!(!battery.charger_connected());
    assert_eq!(battery.latest_state_of_charge(), 50);
    assert_eq!(battery.critical_soc_threshold(), 10);
    assert_eq!(battery.soc_hysteresis(), 2);

    let thermal = ThermalManager::new();
    assert!(!thermal.thermal_critical());
}

#[test]
fn test_get_soc_led_state() {
    let mut manager = BatteryManager::new(10, 2, 20, 21, 80);

    // Battery is critical by default
    assert_eq!(
        manager.get_soc_led_state(),
        SystemLedState::BlinksRedOncePerThirtySeconds
    );

    // Make battery non-critical
    manager.set_battery_critical(false);

    // Low battery SoC
    manager.update_battery_status(
        15,
        ChargeState::DoneOrStandbyOrUnplugged,
        SystemStatus::Active,
        false,
    );
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidOrange);

    // Mid battery SoC
    manager.update_battery_status(
        50,
        ChargeState::DoneOrStandbyOrUnplugged,
        SystemStatus::Active,
        false,
    );
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidYellow);

    // High battery SoC
    manager.update_battery_status(
        85,
        ChargeState::DoneOrStandbyOrUnplugged,
        SystemStatus::Active,
        false,
    );
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidGreen);
}

#[test]
fn test_update_battery_status() {
    let mut manager = BatteryManager::new(10, 2, 20, 21, 80);

    // Default is critical
    assert!(manager.battery_critical());

    // Recoverable/NonRecoverable fault always triggers critical battery
    manager.update_battery_status(
        95,
        ChargeState::RecoverableFault,
        SystemStatus::Active,
        false,
    );
    assert!(manager.battery_critical());

    // When charging, critical is cleared even at 5% SoC
    manager.update_battery_status(5, ChargeState::Charging, SystemStatus::Active, false);
    assert!(!manager.battery_critical());

    // Stop charging -> enters critical because SoC (5) < critical_threshold (10)
    manager.update_battery_status(
        5,
        ChargeState::DoneOrStandbyOrUnplugged,
        SystemStatus::Active,
        false,
    );
    assert!(manager.battery_critical());

    // While critical, charging starts -> exits critical
    manager.update_battery_status(5, ChargeState::Charging, SystemStatus::Active, false);
    assert!(!manager.battery_critical());

    // Charge up past critical threshold + hysteresis -> stays non-critical when not charging
    manager.update_battery_status(
        13,
        ChargeState::DoneOrStandbyOrUnplugged,
        SystemStatus::Active,
        false,
    );
    assert!(!manager.battery_critical());
}

#[test]
fn test_tick_ms() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

    // Ticks when NOT active do not increment active timer
    assert!(!manager.tick_ms(500));
    assert_eq!(manager.active_ms(), 0);

    // Activate system
    manager.set_boot_trap_mask(BootTrapMask::new()).unwrap();
    let _ = manager.set_status(SystemStatus::Active, false, false);

    // Tick partial second -> returns false, timer remains 0
    assert!(!manager.tick_ms(500));
    assert_eq!(manager.active_ms(), 0);

    // Tick remaining ms -> crosses boundary, returns true, active timer increments at 1s intervals
    assert!(manager.tick_ms(500));
    assert_eq!(manager.active_ms(), 1000);

    // Transition to Sleep resets accumulator
    let _ = manager.set_status(SystemStatus::Sleep, false, false);
    assert!(!manager.tick_ms(500));
}

#[test]
fn test_interval_ms() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);
    assert_eq!(manager.interval_ms(), 1000);
    manager.set_interval_ms(500);

    manager.set_boot_trap_mask(BootTrapMask::new()).unwrap();
    let _ = manager.set_status(SystemStatus::Active, false, false);

    // Set interval, tick crosses bondary immediately
    assert_eq!(manager.interval_ms(), 500);
    assert!(manager.tick_ms(500));
}

#[test]
fn test_pure_transition_wake() {
    use firmware_lib::system::transition_wake;

    // Wake up is allowed from Sleep
    assert_eq!(
        transition_wake(SystemStatus::Sleep, false, false, false),
        Some(SystemStatus::Active)
    );

    // Cannot wake if battery is critical
    assert_eq!(
        transition_wake(SystemStatus::Sleep, true, false, false),
        None
    );

    // Cannot wake if thermal is critical
    assert_eq!(
        transition_wake(SystemStatus::Sleep, false, true, false),
        None
    );

    // Cannot wake if boot power down is active
    assert_eq!(
        transition_wake(SystemStatus::Sleep, false, false, true),
        None
    );

    // Can wake from PowerDown
    assert_eq!(
        transition_wake(SystemStatus::PowerDown, false, false, false),
        Some(SystemStatus::Active)
    );
}

#[test]
fn test_pure_transition_sleep() {
    use firmware_lib::system::transition_sleep;

    // Active -> Sleep after inactivity timeout
    assert_eq!(
        transition_sleep(SystemStatus::Active, 30, 30, false, false, 0),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep immediately if battery is critical
    assert_eq!(
        transition_sleep(SystemStatus::Active, 5, 30, true, false, 0),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep immediately if thermal is critical
    assert_eq!(
        transition_sleep(SystemStatus::Active, 5, 30, false, true, 0),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep returns None if inactivity timeout is not reached
    assert_eq!(
        transition_sleep(SystemStatus::Active, 29, 30, false, false, 0),
        None
    );

    // Active -> Sleep returns None if wake locks are held
    assert_eq!(
        transition_sleep(SystemStatus::Active, 30, 30, false, false, 1),
        None
    );
}

#[test]
fn test_pure_transition_power_down() {
    use firmware_lib::system::transition_power_down;

    assert_eq!(
        transition_power_down(SystemStatus::Active, 0),
        Some(SystemStatus::PowerDown)
    );
    assert_eq!(transition_power_down(SystemStatus::PowerDown, 0), None);

    // Active -> PowerDown returns None if wake locks are held
    assert_eq!(transition_power_down(SystemStatus::Active, 1), None);
}

#[test]
fn test_power_manager_wake_locks() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

    manager.set_boot_trap_mask(BootTrapMask::new()).unwrap();
    let _ = manager.set_status(SystemStatus::Active, false, false);
    assert_eq!(manager.wake_lock_count(), 0);
    assert_eq!(manager.inactive_ms(), 0);

    // Tick once -> inactive_ms increases
    assert!(manager.tick_ms(1000));
    assert_eq!(manager.inactive_ms(), 1000);

    // Acquire wake lock
    manager.acquire_wake_lock(None);
    assert_eq!(manager.wake_lock_count(), 1);
    assert_eq!(manager.inactive_ms(), 0);

    // Tick with wake lock -> inactive_ms remains 0
    assert!(manager.tick_ms(1000));
    assert_eq!(manager.inactive_ms(), 0);

    // Acquire another wake lock for client 1
    manager.acquire_wake_lock(Some(1));
    assert_eq!(manager.wake_lock_count(), 2);

    // Release client 1 -> still locked by client 0
    manager.release_wake_lock(Some(1));
    assert_eq!(manager.wake_lock_count(), 1);

    // Release last lock -> unlocked
    manager.release_wake_lock(None);
    assert_eq!(manager.wake_lock_count(), 0);

    // Tick -> inactive_ms starts increasing again
    assert!(manager.tick_ms(1000));
    assert_eq!(manager.inactive_ms(), 1000);

    // Reset on wake clears locks
    manager.acquire_wake_lock(None);
    assert_eq!(manager.wake_lock_count(), 1);
    manager.reset_on_wake();
    assert_eq!(manager.wake_lock_count(), 0);
}

#[test]
#[should_panic(expected = "WakeLock: client_id 32 out of bounds!")]
fn test_power_manager_wake_lock_panic_acquire() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);
    manager.acquire_wake_lock(Some(32));
}

#[test]
#[should_panic(expected = "WakeLock: client_id 32 out of bounds!")]
fn test_power_manager_wake_lock_panic_release() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);
    manager.release_wake_lock(Some(32));
}

#[test]
fn test_power_manager_transition_blocked_by_wake_lock() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

    manager.set_boot_trap_mask(BootTrapMask::new()).unwrap();
    assert_eq!(
        manager.set_status(SystemStatus::Active, false, false),
        Ok(Some(SystemStatus::PowerDown))
    );
    manager.acquire_wake_lock(None);
    assert_eq!(manager.wake_lock_count(), 1);

    // Try transitioning to Sleep -> should fail (stay Active)
    assert_eq!(
        manager.set_status(SystemStatus::Sleep, false, false),
        Err(TransitionError::WakeLocksHeld(1))
    );
    assert_eq!(manager.status(), SystemStatus::Active);

    // Try transitioning to PowerDown -> should fail (stay Active)
    assert_eq!(
        manager.set_status(SystemStatus::PowerDown, false, false),
        Err(TransitionError::WakeLocksHeld(1))
    );
    assert_eq!(manager.status(), SystemStatus::Active);

    // Release lock
    manager.release_wake_lock(None);
    assert_eq!(manager.wake_lock_count(), 0);

    // Try transitioning to Sleep -> should succeed
    assert_eq!(
        manager.set_status(SystemStatus::Sleep, false, false),
        Ok(Some(SystemStatus::Active))
    );
    assert_eq!(manager.status(), SystemStatus::Sleep);
}

#[test]
fn test_power_manager_transition_blocked_by_boot_trap() {
    let mut manager = PowerManager::new(TEST_TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

    // Initial state is PowerDown and boot trap is active
    assert_eq!(manager.status(), SystemStatus::PowerDown);
    assert!(manager.is_boot_trapped());

    // Try transitioning to Active -> should fail (stay PowerDown)
    assert_eq!(
        manager.set_status(SystemStatus::Active, false, false),
        Err(TransitionError::BootPowerDownActive)
    );
    assert_eq!(manager.status(), SystemStatus::PowerDown);

    // Clear boot trap
    manager.set_boot_trap_mask(BootTrapMask::new()).unwrap();

    // Try transitioning to Active -> should succeed
    assert_eq!(
        manager.set_status(SystemStatus::Active, false, false),
        Ok(Some(SystemStatus::PowerDown))
    );
    assert_eq!(manager.status(), SystemStatus::Active);
}

#[test]
fn test_update_battery_status_actions() {
    let mut manager = BatteryManager::new(10, 2, 20, 21, 80);

    // 1. In boot trap, healthy battery update (unplugged) should clear the trap
    assert_eq!(
        manager.update_battery_status(
            50,
            ChargeState::DoneOrStandbyOrUnplugged,
            SystemStatus::PowerDown,
            true
        ),
        Some(BatteryUpdateAction::ClearBootTrap)
    );

    // 2. Clear boot trap manually
    manager.set_battery_critical(false);

    // 3. While Active, healthy update with charging = true should GoToPowerDown
    assert_eq!(
        manager.update_battery_status(50, ChargeState::Charging, SystemStatus::Active, false),
        Some(BatteryUpdateAction::GoToPowerDown)
    );

    // 4. While PowerDown, charging status change should ReportSoC
    assert_eq!(
        manager.update_battery_status(
            50,
            ChargeState::DoneOrStandbyOrUnplugged,
            SystemStatus::PowerDown,
            false
        ),
        Some(BatteryUpdateAction::ReportSoC)
    );

    // 5. Subsequent identical update should be None
    assert_eq!(
        manager.update_battery_status(
            50,
            ChargeState::DoneOrStandbyOrUnplugged,
            SystemStatus::PowerDown,
            false
        ),
        None
    );
}
