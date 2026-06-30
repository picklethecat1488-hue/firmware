use super::*;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

#[test]
fn test_system_controller_flow() {
    static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
    static SENSOR_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
    static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
    static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
    static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();

    let mut controller = SystemController::new(
        MOTOR_CHANNEL.sender(),
        SENSOR_CHANNEL.sender(),
        BATTERY_CHANNEL.sender(),
        THERMAL_CHANNEL.sender(),
        LED_CHANNEL.sender(),
    );

    assert_eq!(controller.status(), SystemStatus::Active);

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
    assert_eq!(led_state.r, 0);
    assert_eq!(led_state.g, 0);
    assert_eq!(led_state.b, 64);

    // Verify motor stop command was dispatched
    let motor_cmd = MOTOR_CHANNEL.try_receive().unwrap();
    assert_eq!(motor_cmd, MotorCommand::Stop);

    // Wake up through activity
    controller.handle_command(SystemCommand::ActivityDetected);
    assert_eq!(controller.status(), SystemStatus::Active);

    // Verify LED was updated to Active green
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state.r, 0);
    assert_eq!(led_state.g, 128);
    assert_eq!(led_state.b, 0);

    // Trigger an alert
    controller.handle_command(SystemCommand::AlertTriggered);
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state.r, 255);
    assert_eq!(led_state.g, 0);
    assert_eq!(led_state.b, 0);

    // Trigger a battery charging update
    controller.handle_command(SystemCommand::BatteryUpdate { state_of_charge: 50, charging: true });
    let led_state = LED_CHANNEL.try_receive().unwrap();
    assert_eq!(led_state.r, 128);
    assert_eq!(led_state.g, 128);
    assert_eq!(led_state.b, 0);
}
