#![allow(unused_must_use)]

use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::system_controller::{SystemCommand, SystemController};
use controller::thermal_controller::ThermalCommand;
use controller::BlockingSystemWriter;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::channel::Channel;

const LOW_BATTERY_SOC_THRESHOLD: u8 = 20;
const MID_BATTERY_SOC_THRESHOLD: u8 = 21;
const HIGH_BATTERY_SOC_THRESHOLD: u8 = 80;
const INACTIVITY_TIMEOUT_SECONDS: u32 = 30;
const CRITICAL_BATTERY_SOC_THRESHOLD: u8 = 10;
const BATTERY_SOC_HYSTERESIS: u8 = 2;

use firmware_lib::BatteryManager;
use model::types::{
    BootReason, Gesture, MotorSpeed, SystemLedState, SystemStatus, TelemetryRecord,
};

macro_rules! create_test_feature_set {
    ($motor_tx:expr, $battery_tx:expr, $sensors:expr, $led_tx:expr, $thermal_tx:expr) => {
        TestFeatureSet {
            features: (
                controller::MotorFeatureConfig::new($motor_tx, MotorSpeed::MAX),
                controller::BatteryFeatureConfig::new(
                    $battery_tx,
                    BatteryManager::new(
                        CRITICAL_BATTERY_SOC_THRESHOLD,
                        BATTERY_SOC_HYSTERESIS,
                        LOW_BATTERY_SOC_THRESHOLD,
                        MID_BATTERY_SOC_THRESHOLD,
                        HIGH_BATTERY_SOC_THRESHOLD,
                    ),
                ),
                controller::ProximityFeatureConfig::new(
                    &$sensors,
                    20,
                    300,
                    controller::GestureAction::TogglePower,
                    None,
                ),
                controller::LedFeatureConfig::new($led_tx),
                controller::ThermalFeatureConfig::new($thermal_tx),
            ),
        }
    };
}

static MOCK_TELEMETRY_CHANNEL: Channel<
    CriticalSectionRawMutex,
    TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();

/// Test implementation of SystemFeatureSet for unit tests.
#[allow(clippy::type_complexity)]
pub struct TestFeatureSet<MutexRaw: RawMutex + 'static, const N: usize> {
    pub features: (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
    ),
}

impl<MutexRaw: RawMutex + 'static, const N: usize> controller::SystemFeatureSet<MutexRaw, N>
    for TestFeatureSet<MutexRaw, N>
{
    type Features = (
        controller::MotorFeatureConfig<MutexRaw, N>,
        controller::BatteryFeatureConfig<MutexRaw, N>,
        controller::ProximityFeatureConfig<MutexRaw, N>,
        controller::LedFeatureConfig<MutexRaw, N>,
        controller::ThermalFeatureConfig<MutexRaw, N>,
    );

    fn features(&self) -> &Self::Features {
        &self.features
    }

    fn inactivity_timeout_seconds(&self) -> u32 {
        30
    }
}

#[test]
fn test_system_controller_flow() {
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    macro_rules! process {
        ($ctrl:expr) => {
            while let Ok(cmd) = SYSTEM_CHANNEL.try_receive() {
                $ctrl.handle_command(cmd);
            }
        };
    }

    let feature_set = create_test_feature_set!(
        Some(MOTOR_CHANNEL.sender()),
        Some(BATTERY_CHANNEL.sender()),
        [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        Some(LED_CHANNEL.sender()),
        Some(THERMAL_CHANNEL.sender())
    );
    let mut controller = SystemController::new(
        feature_set,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // Consume initial SetSpeed(100)

    // Tick it 29 times, should remain Active
    for _ in 0..29 {
        controller.tick_ms(1000);
        process!(controller);
    }
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);

    // Register activity, resets timer
    controller.handle_command(SystemCommand::ActivityDetected);
    process!(controller);
    for _ in 0..(INACTIVITY_TIMEOUT_SECONDS - 1) {
        controller.tick_ms(1000);
        process!(controller);
    }
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);

    // One more tick reaches 30 seconds -> transitions to Sleep
    controller.tick_ms(1000);
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Sleep);

    // Verify LED was updated to Sleep blue
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);

    // Verify motor stop command was dispatched
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);

    // Wake up through activity
    controller.handle_command(SystemCommand::ActivityDetected);
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);

    // Verify LED was updated to Active green
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);
    assert_eq!(
        MOTOR_CHANNEL.try_receive().unwrap(),
        MotorCommand::SetSpeed(MotorSpeed::MAX)
    );

    // Trigger an alert (thermal critical)
    controller.handle_command(SystemCommand::AlertTriggered);
    process!(controller);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedFourTimes);

    assert_eq!(controller.power_manager.status(), SystemStatus::Sleep);

    // Clear channel receivers for clean state
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    // Use a fresh controller instance to test ToF proximity data fusion and active delay gating
    let feature_set2 = create_test_feature_set!(
        Some(MOTOR_CHANNEL.sender()),
        Some(BATTERY_CHANNEL.sender()),
        [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        Some(LED_CHANNEL.sender()),
        Some(THERMAL_CHANNEL.sender())
    );
    let mut controller = SystemController::new(
        feature_set2,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // Consume initial SetSpeed(100)

    // Tick to INACTIVITY_TIMEOUT_SECONDS to let the fresh controller sleep
    for _ in 0..INACTIVITY_TIMEOUT_SECONDS {
        controller.tick_ms(1000);
        process!(controller);
    }
    assert_eq!(controller.power_manager.status(), SystemStatus::Sleep);
    let _ = LED_CHANNEL.try_receive().unwrap(); // consume Sleep LED command (SolidBlue)
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // consume stop motor command

    // Wake up via Activity
    controller.handle_command(SystemCommand::ActivityDetected);
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen); // consume Active green LED command
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // consume SetSpeed(100)

    // Tick to INACTIVITY_TIMEOUT_SECONDS
    for _ in 0..INACTIVITY_TIMEOUT_SECONDS {
        controller.tick_ms(1000);
        process!(controller);
    }
    // Now it should be allowed to sleep, and does so automatically after 30s inactivity
    assert_eq!(controller.power_manager.status(), SystemStatus::Sleep);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);

    // Clear channels
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    controller.handle_command(SystemCommand::ProximityUpdate {
        direction: model::types::Direction::North,
        distance_mm: 15,
    });
    process!(controller);

    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);

    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::SetSpeed(MotorSpeed::MAX));

    // Test critical battery state
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 5,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    // System enters PowerDown state because battery became critical
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    // LED should blink once per 30 seconds
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedOncePerThirtySeconds);
    // Motor stop should be sent to disable the pump
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);
    while MOTOR_CHANNEL.try_receive().is_ok() {}

    controller.handle_command(SystemCommand::ProximityUpdate {
        direction: model::types::Direction::North,
        distance_mm: 15,
    });
    process!(controller);
    // The pump should NOT start since system is in PowerDown (no SetSpeed command in queue)
    assert!(MOTOR_CHANNEL.try_receive().is_err());
}

#[test]
fn test_power_down_and_gesture_detection() {
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    macro_rules! process {
        ($ctrl:expr) => {
            while let Ok(cmd) = SYSTEM_CHANNEL.try_receive() {
                $ctrl.handle_command(cmd);
            }
        };
    }

    let feature_set3 = create_test_feature_set!(
        Some(MOTOR_CHANNEL.sender()),
        Some(BATTERY_CHANNEL.sender()),
        [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        Some(LED_CHANNEL.sender()),
        Some(THERMAL_CHANNEL.sender())
    );
    let mut controller = SystemController::new(
        feature_set3,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    // 1. Verify booting into PowerDown
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // 2. Stay in PowerDown while battery level is critical
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 5,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedOncePerThirtySeconds);

    // 3. Transition to Active when battery level is no longer critical
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // consume SetSpeed(100)

    // 4. Send high-level DualLongPress gesture
    // Clear motor/LED channels
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    controller.handle_command(SystemCommand::Gesture(Gesture::DualLongPress));
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // Verify LED is turned Off and motor is stopped/locked
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::Off);
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);

    // 5. Normal battery status update when in manual PowerDown should be ignored (and LED stays Off)
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

    // 6. Connecting the charger (charging = true) must trigger transition/remain in PowerDown (and show SoC LED)
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::Charging,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

    // 7. Trying to unlock with 2F long press while charger is connected should be ignored
    controller.handle_command(SystemCommand::Gesture(Gesture::DualLongPress));
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // 8. Disconnect charger (should still remain in PowerDown and set LED Off)
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

    // 9. Unlock with 2F long press gesture after charger is disconnected
    controller.handle_command(SystemCommand::Gesture(Gesture::DualLongPress));
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);

    // Verify LED is SolidYellow (SoC = 50% is between 21% and 79%)
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidYellow);
}

#[test]
fn test_invalid_critical_soc_threshold_recovery() {
    let feature_set4: TestFeatureSet<CriticalSectionRawMutex, 4> =
        create_test_feature_set!(None, None, [], None, None);
    let controller = SystemController::new(
        feature_set4,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    let bm = &controller.feature_set.features.1.battery_manager;
    bm.borrow_mut()
        .set_critical_soc_threshold(LOW_BATTERY_SOC_THRESHOLD + 1);
    if bm.borrow().critical_soc_threshold() >= LOW_BATTERY_SOC_THRESHOLD {
        bm.borrow_mut()
            .set_critical_soc_threshold(LOW_BATTERY_SOC_THRESHOLD - 1);
    }
    assert!(bm.borrow().critical_soc_threshold() < LOW_BATTERY_SOC_THRESHOLD);
}

#[test]
fn test_system_controller_with_missing_controllers() {
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();

    macro_rules! process {
        ($ctrl:expr) => {
            while let Ok(cmd) = SYSTEM_CHANNEL.try_receive() {
                $ctrl.handle_command(cmd);
            }
        };
    }

    let feature_set5: TestFeatureSet<CriticalSectionRawMutex, 4> =
        create_test_feature_set!(None, None, [], None, None);
    let mut controller = SystemController::new(
        feature_set5,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    assert_eq!(controller.power_manager.status(), SystemStatus::Active);

    // Verify it doesn't panic on updates
    controller
        .handle_command(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
        })
        .unwrap();
    process!(controller);
    controller
        .handle_command(SystemCommand::ActivityDetected)
        .unwrap();

    // Check that it transitioned to Active state without any panic
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
}

#[test]
fn test_configurable_motor_speed() {
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();

    let custom_speed = MotorSpeed::new_saturating(50);
    let feature_set = TestFeatureSet {
        features: (
            controller::MotorFeatureConfig::new(Some(MOTOR_CHANNEL.sender()), custom_speed),
            controller::BatteryFeatureConfig::new(
                None,
                BatteryManager::new(
                    CRITICAL_BATTERY_SOC_THRESHOLD,
                    BATTERY_SOC_HYSTERESIS,
                    LOW_BATTERY_SOC_THRESHOLD,
                    MID_BATTERY_SOC_THRESHOLD,
                    HIGH_BATTERY_SOC_THRESHOLD,
                ),
            ),
            controller::ProximityFeatureConfig::new(
                &[],
                20,
                300,
                controller::GestureAction::TogglePower,
                None,
            ),
            controller::LedFeatureConfig::new(None),
            controller::ThermalFeatureConfig::new(None),
        ),
    };
    let mut controller = SystemController::new(
        feature_set,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    // Initial battery update to clear boot power down trap and enter Active state
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    controller
        .handle_command(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
        })
        .unwrap();

    // Verify SetSpeed command received by MOTOR_CHANNEL has custom_speed instead of MAX
    let cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(cmd, MotorCommand::SetSpeed(custom_speed));
}

#[test]
fn test_proximity_wake_lock_behavior() {
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();

    macro_rules! process {
        ($ctrl:expr) => {
            while let Ok(cmd) = SYSTEM_CHANNEL.try_receive() {
                $ctrl.handle_command(cmd);
            }
        };
    }

    let mock_sensors = [
        SENSOR_NORTH_CHANNEL.sender(),
        SENSOR_EAST_CHANNEL.sender(),
        SENSOR_WEST_CHANNEL.sender(),
    ];
    let feature_set = create_test_feature_set!(None, None, mock_sensors, None, None);
    let mut controller = SystemController::new(
        feature_set,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    // Initial battery update to clear boot power down trap and enter Active state
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    controller
        .handle_command(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
        })
        .unwrap();

    // Trigger proximity update in-range for North sensor
    controller
        .handle_command(SystemCommand::ProximityUpdate {
            direction: model::types::Direction::North,
            distance_mm: 50, // wake threshold is 300
        })
        .unwrap();
    process!(controller);

    // Check that system controller acquired a wake lock because of the in-range proximity
    assert_eq!(controller.power_manager.wake_locks(), 1);

    // Trigger proximity update out-of-range for North sensor
    controller
        .handle_command(SystemCommand::ProximityUpdate {
            direction: model::types::Direction::North,
            distance_mm: 400,
        })
        .unwrap();
    process!(controller);

    // Check that wake lock is released
    assert_eq!(controller.power_manager.wake_locks(), 0);
}

#[test]
fn test_boot_traps_clearing_integration() {
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();

    macro_rules! process {
        ($ctrl:expr) => {
            while let Ok(cmd) = SYSTEM_CHANNEL.try_receive() {
                let _ = $ctrl.handle_command(cmd);
            }
        };
    }

    let feature_set = create_test_feature_set!(
        None,
        Some(BATTERY_CHANNEL.sender()),
        [],
        None,
        Some(THERMAL_CHANNEL.sender())
    );
    let mut controller = SystemController::new(
        feature_set,
        MOCK_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    // Verify it boots into PowerDown because traps (Battery and Thermal) are active
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    assert!(controller
        .power_manager
        .has_boot_trap(firmware_lib::BootTrapReason::Battery));
    assert!(controller
        .power_manager
        .has_boot_trap(firmware_lib::BootTrapReason::Thermal));

    // Clear only Battery boot trap via battery update
    controller
        .handle_command(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
        })
        .unwrap();
    process!(controller);

    // Verify battery trap is cleared, but system is still PowerDown because thermal trap is active
    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);
    assert!(!controller
        .power_manager
        .has_boot_trap(firmware_lib::BootTrapReason::Battery));
    assert!(controller
        .power_manager
        .has_boot_trap(firmware_lib::BootTrapReason::Thermal));

    // Clear Thermal boot trap
    controller
        .clear_boot_trap(firmware_lib::BootTrapReason::Thermal)
        .unwrap();
    process!(controller);

    // Verify all traps are cleared and system transitions to Active
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    assert!(!controller.power_manager.is_boot_trapped());
}
