use controller::battery_controller::{BatteryCommand, BatteryController};
use controller::led_controller::LedController;
use controller::motor_controller::{MotorCommand, MotorController};
use controller::sensor_controller::{SensorCommand, SensorController};
use controller::thermal_controller::{ThermalCommand, ThermalController};
use controller::{SystemCommand, SystemController};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use firmware_lib::GestureDetector;
use model::interfaces::NoTick;
use model::types::{BootReason, ChargeState, Direction, MotorSpeed, SystemLedState, SystemStatus};
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
            { controller::telemetry_controller::CHANNEL_CAPACITY },
        > = Channel::new();

        let mut telemetry_records = Vec::new();
        let mut drain_telemetry = || {
            while let Ok(rec) = TELEMETRY_CHANNEL.try_receive() {
                telemetry_records.push(rec);
            }
        };

        // Mock peripherals
        let mock_motor = MockMotor::new();
        let mock_led = MockLed::new();
        let mock_tof_north = MockProximitySensor::new(1000);

        // Wrap MockBattery/MockCharger/MockTemp in Mutex
        let mock_battery = Mutex::new(MockBattery::new(3700, 25000));
        let mock_charger = Mutex::new(MockCharger::new(ChargeState::DoneOrStandbyOrUnplugged));
        let mock_temp = Mutex::new(MockBattery::new(3700, 25000)); // MockBattery implements TemperatureSensor

        // Controllers
        let mut motor_ctrl = MotorController::new(NoTick::new(mock_motor), DummyCurrentSensor);
        use model::calibration::{Calibration, CalibrationType};
        motor_ctrl.set_calibration(CalibrationType::MotorCal {
            current_limits: model::calibration::TwoPointCalibration::new(80, 800),
            max_rpm: 3000,
            rpm_limit: 0,
        });
        let mut led_ctrl = LedController::new(mock_led);
        let feature_set = cat_detector::CatDetectorFeatureSet {
            features: (
                controller::MotorFeatureConfig::new(
                    Some(MOTOR_CHANNEL.sender()),
                    model::types::MotorSpeed::MAX,
                ),
                controller::BatteryFeatureConfig::new(
                    Some(BATTERY_CHANNEL.sender()),
                    firmware_lib::BatteryManager::new(
                        cat_detector::CRITICAL_BATTERY_SOC_THRESHOLD,
                        cat_detector::BATTERY_SOC_HYSTERESIS,
                        cat_detector::LOW_BATTERY_SOC_THRESHOLD,
                        cat_detector::MID_BATTERY_SOC_THRESHOLD,
                        cat_detector::HIGH_BATTERY_SOC_THRESHOLD,
                    ),
                ),
                controller::ProximityFeatureConfig::new(
                    &[
                        SENSOR_NORTH_CHANNEL.sender(),
                        SENSOR_EAST_CHANNEL.sender(),
                        SENSOR_WEST_CHANNEL.sender(),
                    ],
                    cat_detector::DEFAULT_PRESS_THRESHOLD_MM,
                    cat_detector::DEFAULT_WAKE_THRESHOLD_MM,
                    controller::GestureAction::TogglePower,
                    Some(TELEMETRY_CHANNEL.sender()),
                ),
                controller::LedFeatureConfig::new(Some(LED_CHANNEL.sender())),
                controller::ThermalFeatureConfig::new(Some(THERMAL_CHANNEL.sender())),
            ),
        };
        let mut system_ctrl =
            SystemController::new(feature_set, TELEMETRY_CHANNEL.sender(), BootReason::Unknown);

        // Set system controller thresholds
        system_ctrl
            .feature_set
            .features
            .1
            .battery_manager
            .borrow_mut()
            .set_critical_soc_threshold(cat_detector::CRITICAL_BATTERY_SOC_THRESHOLD);
        system_ctrl
            .feature_set
            .features
            .1
            .battery_manager
            .borrow_mut()
            .set_soc_hysteresis(cat_detector::BATTERY_SOC_HYSTERESIS);

        let mut battery_ctrl = BatteryController::new_with_system_and_alert(
            &mock_battery,
            &mock_charger,
            SYSTEM_CHANNEL.sender(),
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
            |_id, dist| SystemCommand::ProximityUpdate {
                direction: model::types::Direction::North,
                distance_mm: dist,
            },
            MockPin,
            300,
        );

        // Helpers to run system command processing and tick processing while draining StateChanged messages
        let process_system = |ctrl: &mut SystemController<_, _, _>, cmd: SystemCommand| {
            let _ = ctrl.handle_command(cmd);
            while let Ok(q) = SYSTEM_CHANNEL.try_receive() {
                let _ = ctrl.handle_command(q);
            }
        };

        let tick_system = |ctrl: &mut SystemController<_, _, _>, ms: u32| {
            ctrl.tick_ms(ms);
            while let Ok(q) = SYSTEM_CHANNEL.try_receive() {
                let _ = ctrl.handle_command(q);
            }
        };

        // Verify initial state is PowerDown
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);

        // 1. Simulate battery status report: SoC = 85% -> triggers system wake-up to Active
        {
            let mut bat = mock_battery.lock().await;
            bat.state_of_charge = 85;
        }
        let mut battery_client1 = controller::telemetry_controller::BatteryTelemetryClient::new(
            Some(TELEMETRY_CHANNEL.sender()),
        );
        battery_ctrl
            .update(Some(&mut battery_client1))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Active);

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
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();

        // Proximity detected -> motor starts
        assert_eq!(
            MOTOR_CHANNEL.try_receive(),
            Ok(MotorCommand::SetSpeed(MotorSpeed::MAX))
        );
        motor_ctrl.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None);
        for _ in 0..100 {
            motor_ctrl.tick_motor().unwrap();
        }
        assert_eq!(motor_ctrl.motor.speed, 100);

        // 3. Simulate critical low battery: SoC drops to 5%
        {
            let mut bat = mock_battery.lock().await;
            bat.state_of_charge = 5;
        }
        let mut battery_client2 = controller::telemetry_controller::BatteryTelemetryClient::new(
            Some(TELEMETRY_CHANNEL.sender()),
        );
        battery_ctrl
            .update(Some(&mut battery_client2))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();

        // Should transition to PowerDown, stop motor, and blink red
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));
        assert_eq!(
            LED_CHANNEL.try_receive(),
            Ok(SystemLedState::BlinksRedOncePerThirtySeconds)
        );

        motor_ctrl.handle_command(MotorCommand::Stop, None);
        assert_eq!(motor_ctrl.motor.speed, 0);

        // 4. Simulate battery hysteresis recovery: SoC rising to 11% (not charging)
        // Since critical_soc_threshold = 10, soc_hysteresis = 2, recovery must be >= 12% to transition back to normal.
        // Let's set SoC to 11% (recovery check should fail, remaining in critical)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 11,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        };
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        // Should remain critical (PowerDown) because 11 < 10 + 2 (12)
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert!(LED_CHANNEL.try_receive().is_err());

        // Now set SoC to 13% and state to Charging (should enter PowerDown and show Orange)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 13,
            charger_state: ChargeState::Charging,
        };
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidOrange));

        // Disconnect charger and set SoC to 50% (should remain in PowerDown and set LED Off)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 50,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        };
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

        {
            let gd = &system_ctrl.feature_set.features.2.gesture_detector;
            gd.borrow_mut().register_distance(Direction::East, 15);
            gd.borrow_mut().register_distance(Direction::West, 15);
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 0);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 2_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 5_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        {
            let gd = &system_ctrl.feature_set.features.2.gesture_detector;
            gd.borrow_mut().register_distance(Direction::East, 1000);
            gd.borrow_mut().register_distance(Direction::West, 1000);
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 1000), 6_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        drain_telemetry();

        assert_eq!(
            MOTOR_CHANNEL.try_receive(),
            Ok(MotorCommand::SetSpeed(MotorSpeed::MAX))
        );
        motor_ctrl.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None);
        for _ in 0..100 {
            motor_ctrl.tick_motor().unwrap();
        }
        assert_eq!(motor_ctrl.motor.speed, 100);

        // 5. Simulate thermal critical: Temp reaches 61°C (61000 mC)
        {
            let mut temp_sensor = mock_temp.lock().await;
            temp_sensor.temperature_milli_c = 61000;
        }
        let mut thermal_client = controller::telemetry_controller::ThermalTelemetryClient::new(
            Some(TELEMETRY_CHANNEL.sender()),
        );
        thermal_ctrl
            .update(Some(&mut thermal_client))
            .await
            .unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        println!("Received command: {:?}", cmd);
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();

        // Critical temperature triggers safety shutdown -> Sleep state
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Sleep);
        assert_eq!(
            LED_CHANNEL.try_receive(),
            Ok(SystemLedState::BlinksRedFourTimes)
        );
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));
        system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .reset();

        // 6. Simulate Sleep -> Active -> PowerDown -> Active (Charging) -> Sleep transition
        // 3. Simulated Proximity detection to exit Sleep
        system_ctrl
            .feature_set
            .features
            .4
            .thermal_manager
            .borrow_mut()
            .set_thermal_critical(false);

        // Wake up from Sleep to Active
        process_system(&mut system_ctrl, SystemCommand::ActivityDetected);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        // Drain motor channel for a clean state before simulated long press
        while MOTOR_CHANNEL.try_receive().is_ok() {}

        {
            let gd = &system_ctrl.feature_set.features.2.gesture_detector;
            gd.borrow_mut().register_distance(Direction::East, 15);
            gd.borrow_mut().register_distance(Direction::West, 15);
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 0);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 2_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 5_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));
        system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .reset();

        // Connect charger (should remain/enter PowerDown and show SoC LED)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 50,
            charger_state: ChargeState::Charging,
        };
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        // Disconnect charger (should still remain in PowerDown and LED off)
        let cmd = SystemCommand::BatteryUpdate {
            state_of_charge: 50,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        };
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::Off));

        // Unlock with 2F long press gesture after charger is disconnected
        {
            let gd = &system_ctrl.feature_set.features.2.gesture_detector;
            gd.borrow_mut().register_distance(Direction::East, 15);
            gd.borrow_mut().register_distance(Direction::West, 15);
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 6_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 8_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        let g = system_ctrl
            .feature_set
            .features
            .2
            .gesture_detector
            .borrow_mut()
            .update((Direction::West, 15), 11_000_000);
        if let Some(g) = g {
            process_system(&mut system_ctrl, SystemCommand::Gesture(g));
        }
        drain_telemetry();
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Active);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidYellow));

        // Release buttons and simulate cat walking away
        {
            let gd = &system_ctrl.feature_set.features.2.gesture_detector;
            gd.borrow_mut().register_distance(Direction::East, 1000);
            gd.borrow_mut().register_distance(Direction::West, 1000);
        }

        sensor_ctrl_north.sensor_mut().distance_mm = 1000;
        sensor_ctrl_north.update().unwrap();
        let cmd = SYSTEM_CHANNEL.receive().await;
        process_system(&mut system_ctrl, cmd);
        drain_telemetry();

        // Drain motor channel for a clean state
        while MOTOR_CHANNEL.try_receive().is_ok() {}

        // Inactivity for timeout triggers Sleep
        for _ in 0..cat_detector::INACTIVITY_TIMEOUT_SECONDS {
            tick_system(&mut system_ctrl, 1000);
            drain_telemetry();
        }
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::Sleep);
        assert_eq!(LED_CHANNEL.try_receive(), Ok(SystemLedState::SolidBlue));
        assert_eq!(MOTOR_CHANNEL.try_receive(), Ok(MotorCommand::Stop));

        // Verify that no telemetry writes were dropped and all expected events are captured
        assert!(!telemetry_records.is_empty());
        assert!(telemetry_records.len() >= 15);

        // Filter and inspect state transitions
        let system_states: Vec<SystemStatus> = telemetry_records
            .iter()
            .filter_map(|rec| {
                if let model::telemetry::TelemetryRecord::System(status) = rec {
                    Some(*status)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(
            system_states,
            vec![
                SystemStatus::Active,    // wake up from SoC = 85%
                SystemStatus::PowerDown, // SoC drops to 5%
                SystemStatus::Active,    // unlock gesture
                SystemStatus::Sleep,     // thermal critical
                SystemStatus::Active,    // activity wake
                SystemStatus::PowerDown, // long press to lock
                SystemStatus::Active,    // unlock after charging
                SystemStatus::Sleep,     // inactivity timeout
            ]
        );
    });
}
