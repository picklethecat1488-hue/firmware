use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use firmware_lib::system::SystemStateManager;
use model::types::{SystemLedState, SystemStatus, TelemetryRecord};

static TEST_TELEMETRY_CHANNEL: Channel<CriticalSectionRawMutex, TelemetryRecord, 16> =
    Channel::new();

#[test]
fn test_system_state_manager_initialization() {
    let manager = SystemStateManager::new(10, 2, 20, 21, 80, TEST_TELEMETRY_CHANNEL.sender());

    assert_eq!(manager.status(), SystemStatus::PowerDown);
    assert_eq!(manager.inactivity_seconds(), 0);
    assert_eq!(manager.time_in_active(), 0);
    assert!(manager.battery_critical());
    assert!(!manager.thermal_critical());
    assert!(!manager.charger_connected());
    assert_eq!(manager.latest_state_of_charge(), 50);
    assert!(manager.boot_power_down());
    assert_eq!(manager.critical_soc_threshold(), 10);
    assert_eq!(manager.soc_hysteresis(), 2);
}

#[test]
fn test_get_soc_led_state() {
    let mut manager = SystemStateManager::new(10, 2, 20, 21, 80, TEST_TELEMETRY_CHANNEL.sender());

    // Battery is critical by default
    assert_eq!(
        manager.get_soc_led_state(),
        SystemLedState::BlinksRedOncePerThirtySeconds
    );

    // Make battery non-critical
    manager.set_battery_critical(false);

    // Low battery SoC
    manager.update_battery_status(15, false, false);
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidOrange);

    // Mid battery SoC
    manager.update_battery_status(50, false, false);
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidYellow);

    // High battery SoC
    manager.update_battery_status(85, false, false);
    assert_eq!(manager.get_soc_led_state(), SystemLedState::SolidGreen);
}

#[test]
fn test_update_battery_status() {
    let mut manager = SystemStateManager::new(10, 2, 20, 21, 80, TEST_TELEMETRY_CHANNEL.sender());

    // Default is critical
    assert!(manager.battery_critical());

    // Recoverable/NonRecoverable fault always triggers critical battery
    manager.update_battery_status(95, false, true);
    assert!(manager.battery_critical());

    // When charging, critical is cleared even at 5% SoC
    manager.update_battery_status(5, true, false);
    assert!(!manager.battery_critical());

    // Stop charging -> enters critical because SoC (5) < critical_threshold (10)
    manager.update_battery_status(5, false, false);
    assert!(manager.battery_critical());

    // While critical, charging starts -> exits critical
    manager.update_battery_status(5, true, false);
    assert!(!manager.battery_critical());

    // Charge up past critical threshold + hysteresis -> stays non-critical when not charging
    manager.update_battery_status(13, false, false);
    assert!(!manager.battery_critical());
}

#[test]
fn test_tick_ms() {
    let mut manager = SystemStateManager::new(10, 2, 20, 21, 80, TEST_TELEMETRY_CHANNEL.sender());

    // Ticks when NOT active do not increment active timer
    assert!(!manager.tick_ms(500));
    assert_eq!(manager.time_in_active(), 0);

    // Activate system
    manager.set_status(SystemStatus::Active);

    // Tick partial second -> returns false, timer remains 0
    assert!(!manager.tick_ms(500));
    assert_eq!(manager.time_in_active(), 0);

    // Tick remaining ms -> crosses boundary, returns true, timer increments
    assert!(manager.tick_ms(500));
    assert_eq!(manager.time_in_active(), 1);

    // Transition to Sleep resets accumulator
    manager.set_status(SystemStatus::Sleep);
    assert!(!manager.tick_ms(500));
}

#[test]
fn test_pure_transition_wake() {
    use firmware_lib::system::transition_wake;

    // Normal Wake: Sleep -> Active
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

    // Cannot wake if already PowerDown
    assert_eq!(
        transition_wake(SystemStatus::PowerDown, false, false, false),
        None
    );
}

#[test]
fn test_pure_transition_sleep() {
    use firmware_lib::system::transition_sleep;

    // Active -> Sleep after inactivity timeout
    assert_eq!(
        transition_sleep(SystemStatus::Active, 30, 30, false, false),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep immediately if battery is critical
    assert_eq!(
        transition_sleep(SystemStatus::Active, 5, 30, true, false),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep immediately if thermal is critical
    assert_eq!(
        transition_sleep(SystemStatus::Active, 5, 30, false, true),
        Some(SystemStatus::Sleep)
    );

    // Active -> Sleep returns None if inactivity timeout is not reached
    assert_eq!(
        transition_sleep(SystemStatus::Active, 29, 30, false, false),
        None
    );
}

#[test]
fn test_pure_transition_power_down() {
    use firmware_lib::system::transition_power_down;

    assert_eq!(
        transition_power_down(SystemStatus::Active),
        Some(SystemStatus::PowerDown)
    );
    assert_eq!(transition_power_down(SystemStatus::PowerDown), None);
}
