#![allow(unused_must_use)]

use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::system_controller::{
    SystemCommand, SystemController, SystemControllerChannels, LOW_BATTERY_SOC_THRESHOLD,
};
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use model::types::{
    BootReason, Gesture, MotorSpeed, SystemLedState, SystemStatus, TelemetryRecord,
};

static MOCK_TELEMETRY_CHANNEL: Channel<
    CriticalSectionRawMutex,
    TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();

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

    let channels = SystemControllerChannels {
        system_tx: SYSTEM_CHANNEL.sender(),
        motor_tx: Some(MOTOR_CHANNEL.sender()),
        sensor_txs: [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        battery_tx: Some(BATTERY_CHANNEL.sender()),
        thermal_tx: Some(THERMAL_CHANNEL.sender()),
        led_tx: Some(LED_CHANNEL.sender()),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels, BootReason::Unknown);

    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
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
    for _ in 0..(controller::system_controller::INACTIVITY_TIMEOUT_SECONDS - 1) {
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
    let channels2 = SystemControllerChannels {
        system_tx: SYSTEM_CHANNEL.sender(),
        motor_tx: Some(MOTOR_CHANNEL.sender()),
        sensor_txs: [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        battery_tx: Some(BATTERY_CHANNEL.sender()),
        thermal_tx: Some(THERMAL_CHANNEL.sender()),
        led_tx: Some(LED_CHANNEL.sender()),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels2, BootReason::Unknown);

    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    process!(controller);
    assert_eq!(controller.power_manager.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // Consume initial SetSpeed(100)

    // Tick to INACTIVITY_TIMEOUT_SECONDS to let the fresh controller sleep
    for _ in 0..controller::system_controller::INACTIVITY_TIMEOUT_SECONDS {
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
    for _ in 0..controller::system_controller::INACTIVITY_TIMEOUT_SECONDS {
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

    controller.handle_command(SystemCommand::Gesture(Gesture::ProximityDetected));
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

    controller.handle_command(SystemCommand::Gesture(Gesture::ProximityDetected));
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

    let channels3 = SystemControllerChannels {
        system_tx: SYSTEM_CHANNEL.sender(),
        motor_tx: Some(MOTOR_CHANNEL.sender()),
        sensor_txs: [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        battery_tx: Some(BATTERY_CHANNEL.sender()),
        thermal_tx: Some(THERMAL_CHANNEL.sender()),
        led_tx: Some(LED_CHANNEL.sender()),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels3, BootReason::Unknown);

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
    static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    let channels4 = SystemControllerChannels {
        system_tx: SYSTEM_CHANNEL.sender(),
        motor_tx: Some(MOTOR_CHANNEL.sender()),
        sensor_txs: [
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
        ],
        battery_tx: Some(BATTERY_CHANNEL.sender()),
        thermal_tx: Some(THERMAL_CHANNEL.sender()),
        led_tx: Some(LED_CHANNEL.sender()),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let controller = SystemController::new(channels4, BootReason::Unknown);

    assert!(controller.battery_manager.critical_soc_threshold() < LOW_BATTERY_SOC_THRESHOLD);
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

    let channels: SystemControllerChannels<CriticalSectionRawMutex, 4, 0, 64> =
        SystemControllerChannels {
            system_tx: SYSTEM_CHANNEL.sender(),
            motor_tx: None,
            sensor_txs: [],
            battery_tx: None,
            thermal_tx: None,
            led_tx: None,
            telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
        };
    let mut controller = SystemController::new(channels, BootReason::Unknown);

    assert_eq!(controller.power_manager.status(), SystemStatus::PowerDown);

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
