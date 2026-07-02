use cat_detector::system_controller::{SystemCommand, SystemController};
use controller::battery_controller::{BatteryCommand, BatteryController};
use controller::led_controller::LedController;
use controller::motor_controller::{MotorCommand, MotorController};
use controller::sensor_controller::{SensorCommand, SensorController};
use controller::thermal_controller::{ThermalCommand, ThermalController};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use model::types::{ChargeState, SystemLedState, SystemStatus};
use peripherals::mock::{
    DummyCurrentSensor, MockBattery, MockCharger, MockLed, MockMotor, MockProximitySensor,
};

// Mock wrappers for interrupt pins
struct MockPin;
impl controller::battery_controller::BatteryAlertPin for MockPin {
    async fn wait_for_alert(&mut self) {}
}
impl controller::sensor_controller::DataReadyPin for MockPin {
    async fn wait_for_data_ready(&mut self) {}
}

#[test]
fn test_system_integration_flow() {
    futures::executor::block_on(async {
        // Channels
        static SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
        static MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
        static SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
            Channel::new();
        static SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
            Channel::new();
        static SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
            Channel::new();
        static BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> =
            Channel::new();
        static THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> =
            Channel::new();
        static LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();
        static TELEMETRY_CHANNEL: Channel<
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            16,
        > = Channel::new();

        // Mock peripherals
        let mock_motor = MockMotor::new();
        let mock_led = MockLed::new();
        let mock_tof_north = MockProximitySensor::new(1000);

        // Wrap MockBattery/MockCharger/MockTemp in Mutex
        let mock_battery = Mutex::new(MockBattery::new(3700, 25000));
        let mock_charger = Mutex::new(MockCharger::new(ChargeState::DoneOrStandbyOrUnplugged));
        let mock_temp = Mutex::new(MockBattery::new(3700, 25000)); // MockBattery implements TemperatureSensor

        // Controllers
        let mut motor_ctrl = MotorController::new(mock_motor, DummyCurrentSensor);
        let mut led_ctrl = LedController::new(mock_led);
        let mut system_ctrl = SystemController::new(
            MOTOR_CHANNEL.sender(),
            SENSOR_NORTH_CHANNEL.sender(),
            SENSOR_EAST_CHANNEL.sender(),
            SENSOR_WEST_CHANNEL.sender(),
            BATTERY_CHANNEL.sender(),
            THERMAL_CHANNEL.sender(),
            LED_CHANNEL.sender(),
        );

        // Set system controller thresholds
        system_ctrl.critical_soc_threshold = 10;
        system_ctrl.soc_hysteresis = 2;

        let mut battery_ctrl = BatteryController::new_with_system_and_alert(
            &mock_battery,
            &mock_charger,
            SYSTEM_CHANNEL.sender(),
            |soc, state| SystemCommand::BatteryUpdate {
                state_of_charge: soc,
                charger_state: state,
            },
            MockPin,
        );

        let mut thermal_ctrl = ThermalController::new_with_shutdown(
            &mock_temp,
            SYSTEM_CHANNEL.sender(),
            SystemCommand::AlertTriggered,
        );
        thermal_ctrl.set_overheating_temp_milli_c(45000);
        thermal_ctrl.set_critical_temp_milli_c(60000);
        thermal_ctrl.set_hysteresis_temp_milli_c(2000);

        let mut sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
            0,
            mock_tof_north,
            SYSTEM_CHANNEL.sender(),
            |id, dist| SystemCommand::SensorUpdate {
                sensor_id: id,
                distance_mm: dist,
            },
            MockPin,
        );

        // Verify initial state is PowerDown
        assert_eq!(system_ctrl.status(), SystemStatus::PowerDown);

        // 1. Simulate battery status report: SoC = 50% -> triggers system wake-up to Active
        {
            let mut bat = mock_battery.lock().await;
            bat.state_of_charge = 50;
        }
        battery_ctrl
            .update(Some(&TELEMETRY_CHANNEL.sender()))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        system_ctrl.handle_command(cmd);
        assert_eq!(system_ctrl.status(), SystemStatus::Active);

        // Check that commands were sent to LED channel
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidGreen));

        // Let's run led_ctrl and motor_ctrl update logic manually to verify state transition
        led_ctrl
            .set_pattern(SystemLedState::SolidGreen)
            .await
            .unwrap();
        assert_eq!(led_ctrl.current_state(), SystemLedState::SolidGreen);

        // 2. Simulate object detection: North sensor reads 150mm
        sensor_ctrl_north.sensor_mut().distance_mm = 150;
        sensor_ctrl_north.update().unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        system_ctrl.handle_command(cmd);

        // Proximity detected -> motor starts
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::SetSpeed(100)));
        motor_ctrl.handle_command(MotorCommand::SetSpeed(100), None);
        assert_eq!(motor_ctrl.motor.speed, 100);

        // 3. Simulate critical low battery: SoC drops to 5%
        {
            let mut bat = mock_battery.lock().await;
            bat.state_of_charge = 5;
        }
        battery_ctrl
            .update(Some(&TELEMETRY_CHANNEL.sender()))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        system_ctrl.handle_command(cmd);

        // Should transition to PowerDown, stop motor, and blink red
        assert_eq!(system_ctrl.status(), SystemStatus::PowerDown);
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));
        assert_eq!(
            LED_CHANNEL.try_receive(),
            Ok(SystemLedState::BlinksRedOncePerThirtySeconds)
        );
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

        motor_ctrl.handle_command(MotorCommand::Stop, None);
        assert_eq!(motor_ctrl.motor.speed, 0);

        // 4. Simulate battery hysteresis recovery: SoC rising to 11% (not charging)
        // Since critical_soc_threshold = 10, soc_hysteresis = 2, recovery must be >= 12% to transition back to normal.
        // Let's set SoC to 11% (recovery check should fail, remaining in critical)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 11,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        };
        system_ctrl.handle_command(cmd);
        // Should remain critical (PowerDown) because 11 < 10 + 2 (12)
        assert_eq!(system_ctrl.status(), SystemStatus::PowerDown);
        assert_eq!(
            LED_CHANNEL.try_receive(),
            Ok(SystemLedState::BlinksRedOncePerThirtySeconds)
        );

        // Now set SoC to 13% and state to Charging
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 13,
            charger_state: ChargeState::Charging,
        };
        system_ctrl.handle_command(cmd);
        // Exits PowerDown to Active
        assert_eq!(system_ctrl.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        // 5. Simulate thermal critical: Temp reaches 61°C (61000 mC)
        {
            let mut temp_sensor = mock_temp.lock().await;
            temp_sensor.temperature_milli_c = 61000;
        }
        thermal_ctrl
            .update(Some(&TELEMETRY_CHANNEL.sender()))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        println!("Received command: {:?}", cmd);
        system_ctrl.handle_command(cmd);

        // Critical temperature triggers safety shutdown -> Sleep state
        assert_eq!(system_ctrl.status(), SystemStatus::Sleep);
        assert_eq!(
            LED_CHANNEL.try_receive(),
            Ok(SystemLedState::BlinksRedFourTimes)
        );
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidBlue));
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));

        // 6. Simulate Sleep -> Active -> PowerDown -> Active (Charging) -> Sleep transition
        // Simulate cool down
        system_ctrl.thermal_critical = false;

        // Wake up from Sleep to Active
        system_ctrl.handle_command(SystemCommand::ActivityDetected);
        assert_eq!(system_ctrl.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidGreen));

        // Simulate long press (East & West distance < 20mm for 5s)
        system_ctrl.distance_east = 15;
        system_ctrl.distance_west = 15;
        system_ctrl.update_gesture(0);
        system_ctrl.update_gesture(2_000_000);
        system_ctrl.update_gesture(5_000_000);
        assert_eq!(system_ctrl.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));

        // Connect charger (exits PowerDown to Active)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 50,
            charger_state: ChargeState::Charging,
        };
        system_ctrl.handle_command(cmd);
        assert_eq!(system_ctrl.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        // Release buttons and simulate cat walking away
        sensor_ctrl_north.sensor_mut().distance_mm = 1000;
        sensor_ctrl_north.update().unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        system_ctrl.handle_command(cmd);

        system_ctrl.distance_east = 1000;
        system_ctrl.distance_west = 1000;
        system_ctrl.update_gesture(6_000_000);

        // Drain motor channel for a clean state
        while MOTOR_CHANNEL.try_receive().is_ok() {}

        // Inactivity for timeout triggers Sleep
        for _ in 0..cat_detector::system_controller::INACTIVITY_TIMEOUT_SECONDS {
            system_ctrl.tick();
        }
        assert_eq!(system_ctrl.status(), SystemStatus::Sleep);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidBlue));
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));
    });
}
