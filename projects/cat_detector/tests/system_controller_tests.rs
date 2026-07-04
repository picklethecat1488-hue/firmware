use cat_detector::system_controller::{
    SystemCommand, SystemController, SystemControllerChannels, LOW_BATTERY_SOC_THRESHOLD,
};
use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use model::types::{SystemLedState, SystemStatus, TelemetryRecord};

static MOCK_TELEMETRY_CHANNEL: Channel<CriticalSectionRawMutex, TelemetryRecord, 16> =
    Channel::new();

#[test]
fn test_system_controller_flow() {
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    let channels = SystemControllerChannels {
        motor_tx: MOTOR_CHANNEL.sender(),
        sensor_north_tx: SENSOR_NORTH_CHANNEL.sender(),
        sensor_east_tx: SENSOR_EAST_CHANNEL.sender(),
        sensor_west_tx: SENSOR_WEST_CHANNEL.sender(),
        battery_tx: BATTERY_CHANNEL.sender(),
        thermal_tx: THERMAL_CHANNEL.sender(),
        led_tx: LED_CHANNEL.sender(),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels, 300);

    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen

    // Tick it 29 times, should remain Active
    for _ in 0..29 {
        controller.tick_ms(1000);
    }
    assert_eq!(controller.status(), SystemStatus::Active);

    // Register activity, resets timer
    controller.handle_command(SystemCommand::ActivityDetected);
    for _ in 0..(cat_detector::system_controller::INACTIVITY_TIMEOUT_SECONDS - 1) {
        controller.tick_ms(1000);
    }
    assert_eq!(controller.status(), SystemStatus::Active);

    // One more tick reaches 30 seconds -> transitions to Sleep
    controller.tick_ms(1000);
    assert_eq!(controller.status(), SystemStatus::Sleep);

    // Verify LED was updated to Sleep blue
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);

    // Verify motor stop command was dispatched
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);

    // Wake up through activity
    controller.handle_command(SystemCommand::ActivityDetected);
    assert_eq!(controller.status(), SystemStatus::Active);

    // Verify LED was updated to Active green
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);

    // Trigger an alert (thermal critical)
    controller.handle_command(SystemCommand::AlertTriggered);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedFourTimes);

    // Since alert was triggered, it forces immediate sleep (LED turns blue)
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);
    assert_eq!(controller.status(), SystemStatus::Sleep);

    // Clear channel receivers for clean state
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    // Use a fresh controller instance to test ToF proximity data fusion and active delay gating
    let channels2 = SystemControllerChannels {
        motor_tx: MOTOR_CHANNEL.sender(),
        sensor_north_tx: SENSOR_NORTH_CHANNEL.sender(),
        sensor_east_tx: SENSOR_EAST_CHANNEL.sender(),
        sensor_west_tx: SENSOR_WEST_CHANNEL.sender(),
        battery_tx: BATTERY_CHANNEL.sender(),
        thermal_tx: THERMAL_CHANNEL.sender(),
        led_tx: LED_CHANNEL.sender(),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels2, 300);

    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen

    // Tick to INACTIVITY_TIMEOUT_SECONDS to let the fresh controller sleep
    for _ in 0..cat_detector::system_controller::INACTIVITY_TIMEOUT_SECONDS {
        controller.tick_ms(1000);
    }
    assert_eq!(controller.status(), SystemStatus::Sleep);
    let _ = LED_CHANNEL.try_receive().unwrap(); // consume Sleep LED command (SolidBlue)
    let _ = MOTOR_CHANNEL.try_receive().unwrap(); // consume stop motor command

    // Wake up via Activity
    controller.handle_command(SystemCommand::ActivityDetected);
    assert_eq!(controller.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen); // consume Active green LED command

    // Try to transition to sleep immediately (ignored because time_in_active = 0 < 30)
    controller.handle_command(SystemCommand::Sleep);
    assert_eq!(controller.status(), SystemStatus::Active);

    // Tick to INACTIVITY_TIMEOUT_SECONDS
    for _ in 0..cat_detector::system_controller::INACTIVITY_TIMEOUT_SECONDS {
        controller.tick_ms(1000);
    }
    // Now it should be allowed to sleep, and does so automatically after 30s inactivity
    assert_eq!(controller.status(), SystemStatus::Sleep);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);

    // Clear channels
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    controller.handle_command(SystemCommand::SensorUpdate {
        direction: model::types::Direction::North,
        distance_mm: 150,
    });

    assert_eq!(controller.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);

    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::SetSpeed(100));

    // Test critical battery state
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 5,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    // System enters PowerDown state because battery became critical
    assert_eq!(controller.status(), SystemStatus::PowerDown);
    // LED should blink once per 30 seconds
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedOncePerThirtySeconds);
    // Motor stop should be sent to disable the pump
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);

    controller.handle_command(SystemCommand::SensorUpdate {
        direction: model::types::Direction::North,
        distance_mm: 150,
    });
    // The pump should NOT start since system is in PowerDown (no SetSpeed command in queue)
    assert!(MOTOR_CHANNEL.try_receive().is_err());
}

#[test]
fn test_power_down_and_gesture_detection() {
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    let channels3 = SystemControllerChannels {
        motor_tx: MOTOR_CHANNEL.sender(),
        sensor_north_tx: SENSOR_NORTH_CHANNEL.sender(),
        sensor_east_tx: SENSOR_EAST_CHANNEL.sender(),
        sensor_west_tx: SENSOR_WEST_CHANNEL.sender(),
        battery_tx: BATTERY_CHANNEL.sender(),
        thermal_tx: THERMAL_CHANNEL.sender(),
        led_tx: LED_CHANNEL.sender(),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels3, 300);

    // 1. Verify booting into PowerDown
    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // 2. Stay in PowerDown while battery level is critical
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 5,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::PowerDown);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::BlinksRedOncePerThirtySeconds);

    // 3. Transition to Active when battery level is no longer critical
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 85,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidGreen);

    // 4. Simulate simultaneous press on East & West ToF sensors (distance < 20mm)
    controller.distance_east = 15;
    controller.distance_west = 15;

    // Clear motor/LED channels
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    // Start gesture at t = 0
    controller.update_gesture(0);
    assert_eq!(controller.status(), SystemStatus::Active);
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::SetSpeed(100));

    // Tick gesture to 2 seconds -> should not power down yet
    controller.update_gesture(2_000_000);
    assert_eq!(controller.status(), SystemStatus::Active);
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::SetSpeed(100));

    // Tick gesture to 5 seconds -> total 5 seconds simultaneous press -> triggers PowerDown
    controller.update_gesture(5_000_000);
    assert_eq!(controller.status(), SystemStatus::PowerDown);

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
    assert_eq!(controller.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

    // 6. Connecting the charger (charging = true) must trigger transition/remain in PowerDown (and show SoC LED)
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::Charging,
    });
    assert_eq!(controller.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

    // 7. Trying to unlock with 2F long press while charger is connected should be ignored
    controller.distance_east = 15;
    controller.distance_west = 15;
    controller.update_gesture(6_000_000);
    controller.update_gesture(8_000_000);
    controller.update_gesture(11_000_000);
    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // 8. Disconnect charger (should still remain in PowerDown and set LED Off)
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::PowerDown);
    assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

    // 9. Unlock with 2F long press gesture after charger is disconnected
    controller.distance_east = 15;
    controller.distance_west = 15;
    controller.update_gesture(12_000_000);
    controller.update_gesture(14_000_000);
    controller.update_gesture(17_000_000);
    assert_eq!(controller.status(), SystemStatus::Active);

    // Verify LED is SolidYellow (SoC = 50% is between 21% and 79%)
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidYellow);
}

#[test]
fn test_invalid_critical_soc_threshold_recovery() {
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
        Channel::new();
    static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    let channels4 = SystemControllerChannels {
        motor_tx: MOTOR_CHANNEL.sender(),
        sensor_north_tx: SENSOR_NORTH_CHANNEL.sender(),
        sensor_east_tx: SENSOR_EAST_CHANNEL.sender(),
        sensor_west_tx: SENSOR_WEST_CHANNEL.sender(),
        battery_tx: BATTERY_CHANNEL.sender(),
        thermal_tx: THERMAL_CHANNEL.sender(),
        led_tx: LED_CHANNEL.sender(),
        telemetry_tx: MOCK_TELEMETRY_CHANNEL.sender(),
    };
    let mut controller = SystemController::new(channels4, 300);

    // Set critical threshold to a value greater than LOW_BATTERY_SOC_THRESHOLD (20)
    controller.set_critical_soc_threshold(25);

    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(
        controller.critical_soc_threshold(),
        LOW_BATTERY_SOC_THRESHOLD - 1
    )
}
