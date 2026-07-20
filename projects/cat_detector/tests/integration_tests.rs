use controller::battery_controller::{BatteryCommand, BatteryController};
use controller::led_controller::LedController;
use controller::motor_controller::{MotorCommand, MotorController};
use controller::sensor_controller::{SensorCommand, SensorController};
use controller::thermal_controller::{ThermalCommand, ThermalController};
use controller::{BlockingSystemWriter, SystemCommand, SystemController};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use model::interfaces::NoTick;
use model::types::{BootReason, ChargeState, Direction, MotorSpeed, SystemLedState, SystemStatus};
use peripherals::mock::{
    DummyCurrentSensor, MockBattery, MockCharger, MockLed, MockMotor, MockProximitySensor,
};
use platform::GestureDetector;

// Mock wrappers for interrupt pins
struct MockPin;
impl controller::battery_controller::BatteryAlertPin for MockPin {
    async fn wait_for_alert(&mut self) {
        embassy_time::Timer::after_millis(50).await;
    }
}
impl controller::sensor_controller::DataReadyPin for MockPin {
    async fn wait_for_data_ready(&mut self) {
        embassy_time::Timer::after_millis(50).await;
    }
}

struct TestFlash {
    data: [u8; 1024 * 64],
}

impl TestFlash {
    fn new() -> Self {
        Self {
            data: [0xFF; 1024 * 64],
        }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for TestFlash {
    type Error = core::convert::Infallible;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for TestFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for TestFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 4096;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.data[from as usize..to as usize].fill(0xFF);
        Ok(())
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for TestFlash {}

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
        static THERMAL_ACTION_CHANNEL: Channel<
            CriticalSectionRawMutex,
            controller::types::ThermalUpdateAction,
            4,
        > = Channel::new();
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
                    platform::BatteryManager::new(
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

        let mut thermal_ctrl = ThermalController::new_with_shutdown_and_trap(
            &mock_temp,
            THERMAL_ACTION_CHANNEL.sender(),
        );
        thermal_ctrl.set_overheating_temp_milli_c(45000);
        thermal_ctrl.set_critical_temp_milli_c(60000);
        thermal_ctrl.set_hysteresis_temp_milli_c(2000);

        let mut sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
            controller::types::SensorMetadata {
                direction: model::types::Direction::North,
            },
            mock_tof_north,
            SYSTEM_CHANNEL.sender(),
            MockPin,
            300,
        );

        // Helpers to run system command processing and tick processing while draining StateChanged messages
        let process_system = |ctrl: &mut SystemController<_, _, _>, cmd: SystemCommand| {
            let _ = ctrl.handle_command(cmd);
            while let Ok(q) = SYSTEM_CHANNEL.try_receive() {
                let _ = ctrl.handle_command(q);
            }
            while BATTERY_CHANNEL.try_receive().is_ok() {}
            while THERMAL_CHANNEL.try_receive().is_ok() {}
        };

        let tick_system = |ctrl: &mut SystemController<_, _, _>, ms: u32| {
            ctrl.tick_ms(ms);
            while let Ok(q) = SYSTEM_CHANNEL.try_receive() {
                let _ = ctrl.handle_command(q);
            }
            while BATTERY_CHANNEL.try_receive().is_ok() {}
            while THERMAL_CHANNEL.try_receive().is_ok() {}
        };

        // Verify initial state is PowerDown
        assert_eq!(system_ctrl.power_manager.status(), SystemStatus::PowerDown);

        // Clear thermal trap to allow waking up
        let _ = system_ctrl.clear_boot_trap(platform::BootTrapReason::Thermal);

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
        let action = THERMAL_ACTION_CHANNEL.receive().await;
        let _ = system_ctrl.handle_thermal_action(action);
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
static RUN_SYSTEM_CHANNEL: Channel<CriticalSectionRawMutex, SystemCommand, 4> = Channel::new();
static RUN_MOTOR_CHANNEL: Channel<CriticalSectionRawMutex, MotorCommand, 4> = Channel::new();
static RUN_SENSOR_NORTH_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> =
    Channel::new();
static RUN_SENSOR_EAST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
static RUN_SENSOR_WEST_CHANNEL: Channel<CriticalSectionRawMutex, SensorCommand, 4> = Channel::new();
static RUN_BATTERY_CHANNEL: Channel<CriticalSectionRawMutex, BatteryCommand, 4> = Channel::new();
static RUN_THERMAL_CHANNEL: Channel<CriticalSectionRawMutex, ThermalCommand, 4> = Channel::new();
static RUN_LED_CHANNEL: Channel<CriticalSectionRawMutex, SystemLedState, 4> = Channel::new();
static RUN_TELEMETRY_CHANNEL: Channel<
    CriticalSectionRawMutex,
    model::telemetry::TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();
static RUN_TELEMETRY_CONSUMER_CHANNEL: Channel<
    CriticalSectionRawMutex,
    model::telemetry::TelemetryRecord,
    { controller::telemetry_controller::CHANNEL_CAPACITY },
> = Channel::new();
static RUN_FILESYSTEM_CHANNEL: Channel<
    CriticalSectionRawMutex,
    controller::filesystem_controller::FsRequest,
    16,
> = Channel::new();
static RUN_GESTURE_CHANNEL: Channel<CriticalSectionRawMutex, model::types::Gesture, 4> =
    Channel::new();
static RUN_THERMAL_ACTION_CHANNEL: Channel<
    CriticalSectionRawMutex,
    controller::types::ThermalUpdateAction,
    4,
> = Channel::new();

#[embassy_executor::task]
async fn test_control_task() {
    embassy_time::Timer::after_millis(50).await;
    RUN_SYSTEM_CHANNEL
        .send(SystemCommand::BatteryUpdate {
            state_of_charge: 85,
            charger_state: ChargeState::DoneOrStandbyOrUnplugged,
        })
        .await;

    let mut success = false;
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(2) {
        if let Ok(model::telemetry::TelemetryRecord::System(SystemStatus::Active)) =
            RUN_TELEMETRY_CHANNEL.try_receive()
        {
            success = true;
            break;
        }
        embassy_time::Timer::after_millis(10).await;
    }

    if success {
        std::process::exit(0);
    } else {
        eprintln!("Failed to receive expected telemetry record!");
        std::process::exit(1);
    }
}

#[test]
fn test_spawn_controllers_embassy_routing() {
    // 1. Mock Peripherals (allocated with 'static lifetime)
    let mock_motor = MockMotor::new();
    let mock_led = MockLed::new();
    let mock_tof_north = MockProximitySensor::new(1000);
    let mock_tof_east = MockProximitySensor::new(1000);
    let mock_tof_west = MockProximitySensor::new(1000);
    let mock_battery = Box::leak(Box::new(Mutex::new(MockBattery::new(3700, 25000))));
    let mock_charger = Box::leak(Box::new(Mutex::new(MockCharger::new(
        ChargeState::DoneOrStandbyOrUnplugged,
    ))));
    let mock_temp = Box::leak(Box::new(Mutex::new(MockBattery::new(3700, 25000))));

    // 2. Controllers
    let motor_ctrl = MotorController::new(NoTick::new(mock_motor), DummyCurrentSensor);
    let led_ctrl = LedController::new(mock_led);
    let battery_ctrl = BatteryController::new_with_system_and_alert(
        mock_battery,
        mock_charger,
        RUN_SYSTEM_CHANNEL.sender(),
        MockPin,
    );
    let thermal_ctrl = ThermalController::new_with_shutdown_and_trap(
        mock_temp,
        RUN_THERMAL_ACTION_CHANNEL.sender(),
    );
    let sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::North,
        },
        mock_tof_north,
        RUN_SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );
    let sensor_ctrl_east = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::East,
        },
        mock_tof_east,
        RUN_SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );
    let sensor_ctrl_west = SensorController::new_with_fusion_and_interrupt(
        controller::types::SensorMetadata {
            direction: Direction::West,
        },
        mock_tof_west,
        RUN_SYSTEM_CHANNEL.sender(),
        MockPin,
        300,
    );

    let feature_set = cat_detector::CatDetectorFeatureSet {
        features: (
            controller::MotorFeatureConfig::new(
                Some(RUN_MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            controller::BatteryFeatureConfig::new(
                Some(RUN_BATTERY_CHANNEL.sender()),
                platform::BatteryManager::new(
                    cat_detector::CRITICAL_BATTERY_SOC_THRESHOLD,
                    cat_detector::BATTERY_SOC_HYSTERESIS,
                    cat_detector::LOW_BATTERY_SOC_THRESHOLD,
                    cat_detector::MID_BATTERY_SOC_THRESHOLD,
                    cat_detector::HIGH_BATTERY_SOC_THRESHOLD,
                ),
            ),
            controller::ProximityFeatureConfig::new(
                &[
                    RUN_SENSOR_NORTH_CHANNEL.sender(),
                    RUN_SENSOR_EAST_CHANNEL.sender(),
                    RUN_SENSOR_WEST_CHANNEL.sender(),
                ],
                cat_detector::DEFAULT_PRESS_THRESHOLD_MM,
                cat_detector::DEFAULT_WAKE_THRESHOLD_MM,
                controller::GestureAction::TogglePower,
                Some(RUN_TELEMETRY_CHANNEL.sender()),
            ),
            controller::LedFeatureConfig::new(Some(RUN_LED_CHANNEL.sender())),
            controller::ThermalFeatureConfig::new(Some(RUN_THERMAL_CHANNEL.sender())),
        ),
    };
    let system_ctrl = SystemController::new(
        feature_set,
        RUN_TELEMETRY_CHANNEL.sender(),
        BootReason::Unknown,
    );

    let fs_buf = Box::leak(vec![0u8; 4096].into_boxed_slice());
    let fs_controller = controller::filesystem_controller::FilesystemController::new(
        controller::filesystem_controller::ProfilingFlash::new(TestFlash::new()),
        0..1024 * 64,
        fs_buf,
    );

    let client =
        controller::filesystem_controller::FilesystemClient::new(RUN_FILESYSTEM_CHANNEL.sender());
    let telemetry_ctrl = Box::leak(Box::new(
        controller::telemetry_controller::TelemetryController::new(client),
    ));

    // 3. Run Embassy Executor
    use embassy_executor::Executor;
    let executor = Box::leak(Box::new(Executor::new()));

    executor.run(|spawner: embassy_executor::Spawner| {
        // Spawn all controllers
        controller::spawn_controllers! {
            spawner,
            telemetry: RUN_TELEMETRY_CHANNEL,
            controllers: {
                Thermal(thermal_ctrl, RUN_THERMAL_CHANNEL), generics: (peripherals::mock::MockBattery),
                Battery(battery_ctrl, RUN_BATTERY_CHANNEL), generics: (peripherals::mock::MockBattery, peripherals::mock::MockCharger, MockPin, SystemCommand),
                Motor(motor_ctrl, RUN_MOTOR_CHANNEL), generics: (model::interfaces::NoTick<MockMotor>, DummyCurrentSensor),
                Sensor(sensor_ctrl_north, RUN_SENSOR_NORTH_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Sensor(sensor_ctrl_east, RUN_SENSOR_EAST_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Sensor(sensor_ctrl_west, RUN_SENSOR_WEST_CHANNEL), generics: (MockProximitySensor, MockPin, SystemCommand),
                Led(led_ctrl, RUN_LED_CHANNEL), generics: (MockLed),
                System(system_ctrl, RUN_SYSTEM_CHANNEL, RUN_GESTURE_CHANNEL, RUN_THERMAL_ACTION_CHANNEL), generics: (controller::SystemController<CriticalSectionRawMutex, cat_detector::CatDetectorFeatureSet<CriticalSectionRawMutex, 4>, 4, 64>),
                Filesystem(fs_controller, RUN_FILESYSTEM_CHANNEL), generics: (controller::filesystem_controller::ProfilingFlash<TestFlash>),
                Telemetry(telemetry_ctrl, RUN_TELEMETRY_CONSUMER_CHANNEL), generics: ({ cat_detector::MAX_RECORDS }, { controller::telemetry_controller::CHANNEL_CAPACITY }),
            }
        }

        // Spawn the control task
        spawner.spawn(test_control_task()).unwrap();
    });
}

#[test]
fn test_filesystem_utilization_limit() {
    // Total partition size
    let partition_size =
        (cat_detector::STORAGE_PARTITION_END - cat_detector::STORAGE_PARTITION_START) as usize;

    // Non-telemetry space budget
    let vl53l0x_cal_size = 168;
    let motor_cal_size = 168;
    let crash_idx_size = 44;
    let crash_logs_size = cat_detector::MAX_CRASH_LOGS as usize * 1240; // 1240 bytes per crash log
    let dir_file_size = 2088; // estimated maximum .dir size
    let telemetry_metadata_size = 52;
    let non_telemetry_space = vl53l0x_cal_size
        + motor_cal_size
        + crash_idx_size
        + crash_logs_size
        + dir_file_size
        + telemetry_metadata_size;

    // Telemetry space budget
    // Each chunk occupies CHUNK_FILE_SIZE bytes plus sequential_storage map metadata overhead (~40 bytes)
    let telemetry_chunk_overhead = 40;
    let telemetry_space =
        cat_detector::NUM_CHUNKS * (model::telemetry::CHUNK_FILE_SIZE + telemetry_chunk_overhead);

    let total_budget = non_telemetry_space + telemetry_space;
    let utilization_ratio = total_budget as f64 / partition_size as f64;

    // Assert that we have at least 25% headroom (utilization <= 75%)
    // This leaves plenty of empty pages for sequential_storage GC to prevent choking.
    assert!(
        utilization_ratio <= 0.75,
        "Filesystem capacity utilization is too high! Estimated active storage usage is {} bytes out of {} bytes partition size ({:.2}%). GC choking will occur unless utilization is <= 75.00%. Please reduce NUM_CHUNKS.",
        total_budget,
        partition_size,
        utilization_ratio * 100.0
    );
}
