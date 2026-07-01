use cat_detector::system_controller::{SystemCommand, SystemController};
use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use model::types::{SystemLedState, SystemStatus};

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

    let mut controller = SystemController::new(
        MOTOR_CHANNEL.sender(),
        SENSOR_NORTH_CHANNEL.sender(),
        SENSOR_EAST_CHANNEL.sender(),
        SENSOR_WEST_CHANNEL.sender(),
        BATTERY_CHANNEL.sender(),
        THERMAL_CHANNEL.sender(),
        LED_CHANNEL.sender(),
    );

    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen

    // Tick it 29 times, should remain Active
    for _ in 0..29 {
        controller.tick();
    }
    assert_eq!(controller.status(), SystemStatus::Active);

    // Register activity, resets timer
    controller.handle_command(SystemCommand::ActivityDetected);
    for _ in 0..29 {
        controller.tick();
    }
    assert_eq!(controller.status(), SystemStatus::Active);

    // One more tick reaches 30 seconds -> transitions to Sleep
    controller.tick();
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
    let mut controller = SystemController::new(
        MOTOR_CHANNEL.sender(),
        SENSOR_NORTH_CHANNEL.sender(),
        SENSOR_EAST_CHANNEL.sender(),
        SENSOR_WEST_CHANNEL.sender(),
        BATTERY_CHANNEL.sender(),
        THERMAL_CHANNEL.sender(),
        LED_CHANNEL.sender(),
    );

    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // Send a battery update showing battery is ok -> transitions to Active
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let _ = LED_CHANNEL.try_receive().unwrap(); // Consume initial SolidGreen

    // Tick to 30s to let the fresh controller sleep
    for _ in 0..30 {
        controller.tick();
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

    // Tick to 30s
    for _ in 0..30 {
        controller.tick();
    }
    // Now it should be allowed to sleep, and does so automatically after 30s inactivity
    assert_eq!(controller.status(), SystemStatus::Sleep);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidBlue);

    // Clear channels
    while MOTOR_CHANNEL.try_receive().is_ok() {}
    while LED_CHANNEL.try_receive().is_ok() {}

    // Send SensorUpdate showing a cat detected on North ToF (distance_mm = 150 < 300)
    controller.handle_command(SystemCommand::SensorUpdate {
        sensor_id: 0,
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

    // Send another SensorUpdate showing a cat detected
    controller.handle_command(SystemCommand::SensorUpdate {
        sensor_id: 0,
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

    let mut controller = SystemController::new(
        MOTOR_CHANNEL.sender(),
        SENSOR_NORTH_CHANNEL.sender(),
        SENSOR_EAST_CHANNEL.sender(),
        SENSOR_WEST_CHANNEL.sender(),
        BATTERY_CHANNEL.sender(),
        THERMAL_CHANNEL.sender(),
        LED_CHANNEL.sender(),
    );

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
        state_of_charge: 15,
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

    // 5. Normal battery status update when in manual PowerDown should be ignored
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::DoneOrStandbyOrUnplugged,
    });
    assert_eq!(controller.status(), SystemStatus::PowerDown);

    // 6. Connecting the charger (charging = true) must trigger exit from PowerDown
    controller.handle_command(SystemCommand::BatteryUpdate {
        state_of_charge: 50,
        charger_state: model::types::ChargeState::Charging,
    });
    assert_eq!(controller.status(), SystemStatus::Active);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state, SystemLedState::SolidYellow);
}
