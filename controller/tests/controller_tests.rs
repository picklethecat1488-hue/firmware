use controller::motor_controller::{MotorCommand, MotorController};
use controller::MotorState;
use model::calibration::Calibration;
use model::interfaces::{Motor, NoTick, Tickable};
use model::types::MotorSpeed;
use peripheral::mock::{MockCurrentSensor, MockMotor, MockSpeedSensor};

#[test]
fn test_motor_controller_flow() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(150); // healthy current: 150mA
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        assert_eq!(controller.state(), MotorState::Off);

        // Apply motor calibration so that it can be started
        use model::calibration::Calibration;
        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });

        // Turn on the motor using handle_command
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(controller.motor.speed, 0);
        // Speed ramping disabled: speed is applied immediately on the first tick
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.speed, 100);

        // Simulate dry run (low current draw)
        controller.current_sensor.current_ma = 10; // below 15mA threshold
        controller.bypass_startup_blanking();
        controller.update(None).await.unwrap(); // triggers PowerOff -> state becomes Off
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.motor.speed, 0); // motor should be stopped

        // Restart the motor
        controller.current_sensor.current_ma = 150; // reset to healthy current
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.speed, 100);

        // Simulate stall (high current draw persisting for >= 3 samples)
        controller.current_sensor.current_ma = 900; // above 800mA threshold
        controller.bypass_startup_blanking();
        // Sample 1: throttles duty cycle, does not stop yet
        controller.update(None).await.unwrap();
        assert_eq!(controller.state(), MotorState::On);
        // Sample 2: throttles duty cycle further, does not stop yet
        controller.update(None).await.unwrap();
        assert_eq!(controller.state(), MotorState::On);
        // Sample 3: persistent overcurrent triggers emergency stall shutdown
        controller.update(None).await.unwrap();
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.motor.speed, 0); // motor should be stopped
    });
}

#[test]
fn test_led_controller_flow() {
    futures::executor::block_on(async {
        let mock_led = peripheral::mock::MockLed::new();
        let mut controller = controller::led_controller::LedController::new(mock_led);

        assert_eq!(
            controller.current_state(),
            model::types::SystemLedState::Off
        );

        controller
            .set_pattern(model::types::SystemLedState::SolidGreen)
            .await
            .unwrap();
        assert_eq!(
            controller.current_state(),
            model::types::SystemLedState::SolidGreen
        );
    });
}

#[test]
fn test_motor_controller_sad_cases() {
    futures::executor::block_on(async {
        let mut motor = MockMotor::new();
        motor.should_fail = true; // Make motor fail
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);
        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });

        // Try starting motor. Since motor is failing, update() or handle_command() should report errors
        let telemetry_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            { controller::telemetry_controller::CHANNEL_CAPACITY },
        >::new()));
        let telemetry_tx = telemetry_channel.sender();
        let telemetry_rx = telemetry_channel.receiver();

        let mut client =
            controller::telemetry_controller::MotorTelemetryClient::new(Some(telemetry_tx));
        // Set speed sets target speed. Error is triggered when we tick the motor controller.
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), Some(&mut client))
            .await;

        // Consume the initial MotorStatus report
        let rec1 = telemetry_rx.try_receive().unwrap();
        assert!(matches!(rec1, model::telemetry::TelemetryRecord::Motor(_)));

        let res = controller.tick_motor().await;
        assert!(res.is_err());
        client.report_error(res.unwrap_err());

        // Check if error is received on the telemetry channel
        let rec = telemetry_rx.try_receive().unwrap();
        assert!(matches!(
            rec,
            model::telemetry::TelemetryRecord::PeripheralError(_)
        ));

        // Now make current sensor fail
        let motor2 = MockMotor::new();
        let mut sensor2 = MockCurrentSensor::new(150);
        sensor2.should_fail = true; // Make current sensor fail
        let speed_sensor2 = MockSpeedSensor::new(3000);
        let mut controller2 = MotorController::new(NoTick::new(motor2), sensor2, speed_sensor2);
        controller2.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });
        controller2
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None)
            .await; // start motor first (no failure on motor)

        let mut client2 =
            controller::telemetry_controller::MotorTelemetryClient::new(Some(telemetry_tx));
        // Call update, which reads current. It should fail and return Err
        let res = controller2.update(Some(&mut client2)).await;
        assert!(res.is_err());
    });
}

#[test]
fn test_led_controller_sad_cases() {
    futures::executor::block_on(async {
        let mut mock_led = peripheral::mock::MockLed::new();
        mock_led.should_fail = true; // Make LED driver fail
        let mut controller = controller::led_controller::LedController::new(mock_led);

        // Setting pattern should fail
        let res = controller
            .set_pattern(model::types::SystemLedState::SolidGreen)
            .await;
        assert!(res.is_err());

        // Test the task loop error reporting
        let command_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            controller::led_controller::LedCommand,
            4,
        >::new()));
        let command_tx = command_channel.sender();
        let command_rx = command_channel.receiver();

        let telemetry_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            { controller::telemetry_controller::CHANNEL_CAPACITY },
        >::new()));
        let telemetry_tx = telemetry_channel.sender();
        let telemetry_rx = telemetry_channel.receiver();

        // Run the controller's main loop and push a command
        command_tx
            .try_send(controller::led_controller::LedCommand::State(
                model::types::SystemLedState::SolidGreen,
            ))
            .unwrap();

        let run_fut = controller.run(command_rx, telemetry_tx);
        let check_fut = async {
            // First message is the initial state logged at startup (Off)
            let rec = telemetry_rx.receive().await;
            assert_eq!(
                rec,
                model::telemetry::TelemetryRecord::Led(model::types::SystemLedState::Off)
            );

            // Second message is the state transition to SolidGreen
            let rec2 = telemetry_rx.receive().await;
            assert_eq!(
                rec2,
                model::telemetry::TelemetryRecord::Led(model::types::SystemLedState::SolidGreen)
            );

            // Third message is the PeripheralError from the failed write
            let rec3 = telemetry_rx.receive().await;
            assert!(matches!(
                rec3,
                model::telemetry::TelemetryRecord::PeripheralError(_)
            ));
        };

        embassy_futures::select::select(run_fut, check_fut).await;
    });
}

#[test]
fn test_battery_controller_sad_cases() {
    futures::executor::block_on(async {
        use controller::battery_controller::BatteryController;
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use embassy_sync::mutex::Mutex;
        use model::types::ChargeState;
        use peripheral::mock::{MockBattery, MockCharger};

        let mut battery = MockBattery::new(3700, 25000);
        battery.should_fail = true; // Make battery fail
        let battery_mutex = Mutex::<CriticalSectionRawMutex, _>::new(battery);

        let charger = MockCharger::new(ChargeState::DoneOrStandbyOrUnplugged);
        let charger_mutex = Mutex::<CriticalSectionRawMutex, _>::new(charger);

        let system_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            CriticalSectionRawMutex,
            controller::system_controller::SystemCommand,
            16,
        >::new()));
        let system_tx = system_channel.sender();

        let mut controller =
            BatteryController::new_with_system(&battery_mutex, &charger_mutex, system_tx);

        let telemetry_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            { controller::telemetry_controller::CHANNEL_CAPACITY },
        >::new()));
        let telemetry_tx = telemetry_channel.sender();
        let telemetry_rx = telemetry_channel.receiver();

        let mut telemetry_client =
            controller::telemetry_controller::BatteryTelemetryClient::new(Some(telemetry_tx));
        // Calling update should return Err and report to telemetry
        let res = controller.update(Some(&mut telemetry_client)).await;
        assert!(res.is_err());

        // The first message is BatteryStatus (with Critical state and 0 mV)
        let rec1 = telemetry_rx.try_receive().unwrap();
        assert!(matches!(
            rec1,
            model::telemetry::TelemetryRecord::Battery(model::types::BatteryStatus::VolTempState(
                0,
                _,
                model::types::BatteryState::Critical,
                _
            ))
        ));

        // The second message is FuelGaugeTelemetry (0 mV, 0%)
        let rec2 = telemetry_rx.try_receive().unwrap();
        assert!(matches!(
            rec2,
            model::telemetry::TelemetryRecord::FuelGauge(model::types::FuelGaugeTelemetry::VolSoc(
                0, 0
            ))
        ));

        // The third message is ChargeState
        let rec3 = telemetry_rx.try_receive().unwrap();
        assert!(matches!(
            rec3,
            model::telemetry::TelemetryRecord::ChargerState(_)
        ));

        // The fourth message is the PeripheralError from the failed read
        let rec4 = telemetry_rx.try_receive().unwrap();
        assert!(matches!(
            rec4,
            model::telemetry::TelemetryRecord::PeripheralError(_)
        ));
    });
}

struct MockTickableMotor {
    speed: MotorSpeed,
    tick_count: usize,
    should_fail_tick: bool,
}

impl MockTickableMotor {
    fn new() -> Self {
        Self {
            speed: MotorSpeed::ZERO,
            tick_count: 0,
            should_fail_tick: false,
        }
    }
}

impl Motor for MockTickableMotor {
    type Error = ();

    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        self.speed = speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = MotorSpeed::ZERO;
        Ok(())
    }
}

impl Tickable for MockTickableMotor {
    type Error = ();

    async fn tick(&mut self) -> Result<(), Self::Error> {
        if self.should_fail_tick {
            Err(())
        } else {
            self.tick_count += 1;
            Ok(())
        }
    }
}

#[test]
fn test_motor_controller_tickable_vs_notick() {
    futures::executor::block_on(async {
        use model::calibration::Calibration;

        // 1. Test stateful Tickable motor
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // Calibrate and start the motor so active_speed will ramp to non-zero
        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(0, 0, 0, 1000),
            max_rpm: Some(3000),
            rpm_limit: Some(3000),
        });
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;

        assert_eq!(controller.motor.tick_count, 0);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.motor.tick_count, 1);
        // Test failing tick: tick until next commutation step executes
        controller.motor.should_fail_tick = true;
        let mut tick_res = Ok(());
        for _ in 0..=controller::motor_controller::STARTUP_COMMUTATION_STEP_TICKS {
            tick_res = controller.tick_motor().await;
            if tick_res.is_err() {
                break;
            }
        }
        assert!(tick_res.is_err());
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.motor.speed, MotorSpeed::ZERO);
        assert!(controller.tick_motor().await.is_ok());

        // 2. Test NoTick motor wrapper
        let notick_motor = NoTick::new(MockTickableMotor::new());
        let sensor2 = MockCurrentSensor::new(150);
        let speed_sensor2 = MockSpeedSensor::new(3000);
        let mut controller2 = MotorController::new(notick_motor, sensor2, speed_sensor2);

        // Calibrate and start the motor so active_speed will ramp to non-zero
        controller2.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(0, 0, 0, 1000),
            max_rpm: Some(3000),
            rpm_limit: Some(3000),
        });
        controller2
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;

        // Ticking shouldn't increase inner tick count or return error even if inner should fail,
        // because NoTick::tick() is a no-op that always returns Ok(()) without calling inner tick.
        controller2.motor.should_fail_tick = true;
        assert!(controller2.tick_motor().await.is_ok());
        assert_eq!(controller2.motor.tick_count, 0);
    });
}

#[test]
fn test_motor_controller_rpm_command() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        // Set calibration with max RPM = 3000
        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });

        // Handle SetSpeedRpm(1500) -> sets speed to 50%
        controller
            .handle_command(
                controller::motor_controller::MotorCommand::SetSpeedRpm(1500),
                None,
            )
            .await;
        assert_eq!(controller.min_current_ma(), 80); // verify calibration is active

        // Let's call tick_motor to propagate active_speed (needs 50 ticks to ramp to 50%)
        for _ in 0..50 {
            controller.tick_motor().await.unwrap();
        }
        assert_eq!(controller.motor.speed, 50);

        // Handle SetSpeedRpm(-3000) (signed/reverse direction)
        controller
            .handle_command(
                controller::motor_controller::MotorCommand::SetSpeedRpm(-3000),
                None,
            )
            .await;
        for _ in 0..150 {
            controller.tick_motor().await.unwrap();
        }
        assert_eq!(controller.motor.speed, -100);

        // Handle SetSpeedRpm(4000) (clamped to 100)
        controller
            .handle_command(
                controller::motor_controller::MotorCommand::SetSpeedRpm(4000),
                None,
            )
            .await;
        for _ in 0..200 {
            controller.tick_motor().await.unwrap();
        }
        assert_eq!(controller.motor.speed, 100);
    });
}

#[test]
fn test_motor_limits_checking() {
    use controller::motor_controller::MotorLimits;
    use controller::MotorSafetyStatus;

    let limits = MotorLimits {
        min_current_ma: 80,
        max_current_ma: 800,
        max_rpm: 3000,
        rpm_limit: 3000,
    };

    // Safe
    assert_eq!(limits.check_limits(2500, 150), MotorSafetyStatus::Ok);
    // RPM exceeded
    assert_eq!(
        limits.check_limits(3500, 150),
        MotorSafetyStatus::RpmExceeded(3500)
    );
    // Dry run
    assert_eq!(limits.check_limits(2500, 50), MotorSafetyStatus::DryRun(50));
    // Stall
    assert_eq!(
        limits.check_limits(2500, 900),
        MotorSafetyStatus::Stall(900)
    );
}

#[test]
fn test_motor_controller_hall_sensor_rpm_and_zero_rpm_stall() {
    futures::executor::block_on(async {
        use controller::MotorReader;

        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(2850);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });

        // Start motor
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);

        // Tick motor so speed sensor is polled
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.current_rpm(), 2850);
        assert_eq!(controller.measured_rpm(), 2850);

        // Read motor RPM via MotorReader trait
        let rpm = controller.read_motor_rpm().await.unwrap();
        assert_eq!(rpm, 2850);

        // While Hall sensor support is temporarily disabled, zero RPM does not trigger a stall shutdown [WORM-UNDULATING-SERPULA-688]
        controller.speed_sensor.rpm = 0;
        controller.bypass_startup_blanking();
        let _ = controller.update(None).await;

        // Motor controller should remain running (MotorState::On)
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_motor_controller_parameterized_max_current_and_default() {
    futures::executor::block_on(async {
        use controller::motor_controller::DEFAULT_MAX_CURRENT_MA;

        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);

        // 1. Default constructor must retain canonical 2500 mA default
        let default_ctrl = MotorController::new(NoTick::new(motor), sensor, speed_sensor);
        assert_eq!(default_ctrl.max_current_ma(), DEFAULT_MAX_CURRENT_MA);
        assert_eq!(DEFAULT_MAX_CURRENT_MA, 2500);

        // 2. Parameterized constructor sets custom limit (e.g. 2000 mA from app.toml)
        let motor2 = MockMotor::new();
        let sensor2 = MockCurrentSensor::new(150);
        let speed_sensor2 = MockSpeedSensor::new(3000);
        let mut custom_ctrl = MotorController::new_with_max_current(
            NoTick::new(motor2),
            sensor2,
            speed_sensor2,
            2000,
        );
        assert_eq!(custom_ctrl.max_current_ma(), 2000);

        // 3. set_current_limits cannot exceed configured hardware limit (2000 mA)
        custom_ctrl.set_current_limits(20, 3000);
        assert_eq!(custom_ctrl.max_current_ma(), 2000);
        assert_eq!(custom_ctrl.min_current_ma(), 20);

        // 4. Calibration overload cannot exceed configured hardware limit (2000 mA)
        custom_ctrl.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(80, 0, 0, 2800),
            max_rpm: Some(3000),
            rpm_limit: Some(0),
        });
        assert_eq!(custom_ctrl.max_current_ma(), 2000);
    });
}

#[test]
fn test_worm_peaty_bobbit_630_motor_tick_driver_fault_stops_ticking() {
    futures::executor::block_on(async {
        use model::calibration::Calibration;

        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // Calibrate and start the motor at speed 100
        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(0, 0, 0, 1000),
            max_rpm: Some(3000),
            rpm_limit: Some(3000),
        });
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;

        // Verify initial tick succeeds and motor begins ramping up
        assert_eq!(controller.motor.tick_count, 0);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.motor.tick_count, 1);
        assert_eq!(controller.state(), MotorState::On);

        // Simulate driver fault assertion (e.g. TMC6300 DIAG pin asserted)
        controller.motor.should_fail_tick = true;

        // tick_motor() must return an error and immediately transition controller to Off when next step executes
        let mut tick_res = Ok(());
        for _ in 0..=controller::motor_controller::STARTUP_COMMUTATION_STEP_TICKS {
            tick_res = controller.tick_motor().await;
            if tick_res.is_err() {
                break;
            }
        }
        assert!(tick_res.is_err());
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.motor.speed, MotorSpeed::ZERO);
        assert_eq!(controller.current_rpm(), 0);

        // Subsequent tick_motor() calls must NOT tick the failed motor again (zero log spam)
        let count_before = controller.motor.tick_count;
        for _ in 0..50 {
            assert!(controller.tick_motor().await.is_ok());
        }
        assert_eq!(controller.motor.tick_count, count_before);
        assert_eq!(controller.state(), MotorState::Off);
    });
}

#[test]
fn test_worm_setosa_tubeworm_177_commutation_step_scaling_and_soft_start_ramp() {
    futures::executor::block_on(async {
        use controller::motor_controller::{
            calculate_commutation_step_ticks, STARTUP_COMMUTATION_STEP_TICKS,
        };

        // Invariant 1: Speed scaling for target step intervals
        // At speed 100: 1 tick (10ms) per step (100 Hz step rate)
        assert_eq!(calculate_commutation_step_ticks(100), 1);
        // At speed 50: 2 ticks (20ms) per step (50 Hz step rate)
        assert_eq!(calculate_commutation_step_ticks(50), 2);
        // At speed 20: 5 ticks (50ms) per step (20 Hz step rate)
        assert_eq!(calculate_commutation_step_ticks(20), 5);
        // At speed 10: 10 ticks (100ms) per step (10 Hz step rate)
        assert_eq!(calculate_commutation_step_ticks(10), 10);
        // At speed 1: 100 ticks (1000ms) per step (1 Hz step rate)
        assert_eq!(calculate_commutation_step_ticks(1), 100);

        // Invariant 2: Startup soft-start ramp and stepping intervals
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        controller.set_calibration(&model::calibration::MotorCalibration {
            current_ma: model::calibration::FourPointCalibration::new(0, 0, 0, 1000),
            max_rpm: Some(3000),
            rpm_limit: Some(3000),
        });

        // Command speed 10 (target interval = 10 ticks)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(10).unwrap()), None)
            .await;

        assert_eq!(controller.target_commutation_step_ticks(), 10);
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS
        );

        // Initial tick executes Step 0 immediately
        assert_eq!(controller.motor.tick_count, 0);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.motor.tick_count, 1);

        // Subsequent ticks hold the step without stepping
        for _ in 1..STARTUP_COMMUTATION_STEP_TICKS {
            assert!(controller.tick_motor().await.is_ok());
            assert_eq!(
                controller.motor.tick_count, 1,
                "Must hold step during startup alignment interval"
            );
        }

        // Tick after interval expires advances to Step 1
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(
            controller.motor.tick_count, 2,
            "Must advance step when startup interval expires"
        );

        // Now test soft-start acceleration ramp on speed 100
        controller.handle_command(MotorCommand::Stop, None).await;
        assert_eq!(controller.commutation_interval_ticks(), 0);

        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;
        assert_eq!(controller.target_commutation_step_ticks(), 1);

        // First step executes immediately
        let initial_ticks = controller.motor.tick_count;
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.motor.tick_count, initial_ticks + 1);

        // Over the next steps, interval gradually decrements toward 1
        let mut previous_interval = controller.commutation_interval_ticks();
        while controller.commutation_interval_ticks() > 1 {
            assert!(controller.tick_motor().await.is_ok());
            let current = controller.commutation_interval_ticks();
            assert!(
                current <= previous_interval,
                "Interval must decrease during ramp"
            );
            previous_interval = current;
        }
        assert_eq!(controller.commutation_interval_ticks(), 1);
    });
}

#[test]
fn test_worm_segmented_sipunculid_504_read_rpm_samples_speed_sensor() {
    futures::executor::block_on(async {
        use controller::motor_controller::SendSignalU32Ptr;

        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(2500);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // Start motor without pre-existing calibration
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;

        // Command ReadRpm via channel signal pattern
        let signal = embassy_sync::signal::Signal::new();
        let cmd = MotorCommand::ReadRpm(SendSignalU32Ptr(&signal as *const _));
        controller.handle_command(cmd, None).await;

        let rpm = signal.wait().await;
        assert_eq!(rpm, Ok(2500u32));
        assert_eq!(controller.measured_rpm(), 2500);
    });
}

#[test]
fn test_cr_7f321327_motor_driver_tick_interval_sync() {
    futures::executor::block_on(async {
        use controller::motor_controller::MOTOR_TICK_INTERVAL;
        use peripheral::tmc6300::DEFAULT_TICK_INTERVAL;

        // 1. Controller compile-time constant is synchronized with motor driver default (333us)
        assert_eq!(MOTOR_TICK_INTERVAL, DEFAULT_TICK_INTERVAL);
        assert_eq!(
            MOTOR_TICK_INTERVAL,
            embassy_time::Duration::from_micros(333)
        );

        // 2. MotorController tick_interval() reflects the driver's configured tick interval
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(2500);
        let controller = MotorController::new(motor, sensor, speed_sensor);

        // MockTickableMotor defaults to standard tick interval via Tickable trait default
        assert_eq!(
            controller.tick_interval(),
            embassy_time::Duration::from_millis(10)
        );

        // 3. With TMC6300 PIO driver mock, tick_interval() matches TMC6300 (10ms)
        struct MockInputPin;
        impl embedded_hal::digital::ErrorType for MockInputPin {
            type Error = core::convert::Infallible;
        }
        impl embedded_hal::digital::InputPin for MockInputPin {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(false)
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(true)
            }
        }

        let tmc_driver = peripheral::tmc6300::Tmc6300Pio::new(MockInputPin);
        let sensor2 = MockCurrentSensor::new(150);
        let speed_sensor2 = MockSpeedSensor::new(2500);
        let mut controller_tmc = MotorController::new(tmc_driver, sensor2, speed_sensor2);
        assert_eq!(
            controller_tmc.tick_interval(),
            embassy_time::Duration::from_micros(333)
        );
        assert_eq!(controller_tmc.tick_interval(), MOTOR_TICK_INTERVAL);

        // Dynamically adjusting the driver updates controller's active tick interval
        controller_tmc.motor.set_tick_interval_us(250);
        assert_eq!(
            controller_tmc.tick_interval(),
            embassy_time::Duration::from_micros(250)
        );
    });
}

#[test]
fn test_worm_proboscid_roundworm_8_startup_commutation_and_alignment() {
    futures::executor::block_on(async {
        use controller::motor_controller::{MOTOR_TICK_INTERVAL, STARTUP_COMMUTATION_STEP_TICKS};

        // 1. Tick interval matches MOTOR_TICK_INTERVAL (333 µs)
        assert_eq!(
            MOTOR_TICK_INTERVAL,
            embassy_time::Duration::from_micros(333)
        );

        // 2. Startup commutation step duration (10 ticks * 333µs = 3330µs for initial rotor lock-in)
        let startup_period_us =
            STARTUP_COMMUTATION_STEP_TICKS * (MOTOR_TICK_INTERVAL.as_micros() as u32);
        assert!(
            startup_period_us >= 1000,
            "Startup commutation step duration must be >= 1ms (got {}us)",
            startup_period_us
        );
        assert_eq!(startup_period_us, 3330);

        // 3. Controller startup alignment: holding Step 0 for full startup duration
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;

        // Startup blanking must be 50 ticks (500 ms at 10 ms)
        assert_eq!(controller.config().startup_blanking_ticks, 50);

        // Initial tick executes Step 0 for alignment
        assert_eq!(controller.motor.tick_count, 0);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(
            controller.motor.tick_count, 1,
            "Initial tick must energize Step 0"
        );

        // Step 0 must be held for the remaining ticks of the startup period
        for tick in 1..STARTUP_COMMUTATION_STEP_TICKS {
            assert!(controller.tick_motor().await.is_ok());
            assert_eq!(
                controller.motor.tick_count, 1,
                "Must hold Step 0 during startup alignment period at tick {}",
                tick
            );
        }

        // The tick after Step 0 advances to Step 1 and decrements interval
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(
            controller.motor.tick_count, 2,
            "Must advance to Step 1 after startup alignment period expires"
        );
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS - 1
        );

        // Next ticks hold Step 1
        for _ in 1..(STARTUP_COMMUTATION_STEP_TICKS - 1) {
            assert!(controller.tick_motor().await.is_ok());
            assert_eq!(controller.motor.tick_count, 2);
        }
        // Next tick advances to Step 2 and decrements interval
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.motor.tick_count, 3);
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS - 2
        );
    });
}

#[test]
fn test_worm_glowing_slug_613_motor_speed_transitions_and_startup_ramp() {
    futures::executor::block_on(async {
        use controller::motor_controller::STARTUP_COMMUTATION_STEP_TICKS;

        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(150);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // 1. Initial start from Off engages soft-start ramp starting at STARTUP_COMMUTATION_STEP_TICKS
        assert_eq!(controller.state(), MotorState::Off);
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS
        );
        assert_eq!(controller.target_commutation_step_ticks(), 2); // 100 / 50 = 2

        // Ramp down to target speed 50 (interval = 2)
        while controller.commutation_interval_ticks() > 2 {
            assert!(controller.tick_motor().await.is_ok());
        }
        assert_eq!(controller.commutation_interval_ticks(), 2);

        // 2. Transitioning to speed 60 while running must NOT reset commutation_interval_ticks to STARTUP_COMMUTATION_STEP_TICKS
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(60).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(
            controller.commutation_interval_ticks(),
            2,
            "Speed transition while running must preserve current commutation interval"
        );
        assert_eq!(controller.target_commutation_step_ticks(), 1); // 100 / 60 = 1

        // Ramp down to target speed 60 (interval = 1)
        while controller.commutation_interval_ticks() > 1 {
            assert!(controller.tick_motor().await.is_ok());
        }
        assert_eq!(controller.commutation_interval_ticks(), 1);

        // 3. Transitioning back to speed 50 while running must ramp up interval smoothly
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(
            controller.commutation_interval_ticks(),
            1,
            "Speed transition must not jump back to startup ticks"
        );
        assert_eq!(controller.target_commutation_step_ticks(), 2);

        while controller.commutation_interval_ticks() < 2 {
            assert!(controller.tick_motor().await.is_ok());
        }
        assert_eq!(controller.commutation_interval_ticks(), 2);

        // 4. Stopping transitions to Off, resetting interval to 0
        controller.handle_command(MotorCommand::Stop, None).await;
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.commutation_interval_ticks(), 0);

        // 5. Subsequent start from Off properly re-engages the soft-start ramp from STARTUP_COMMUTATION_STEP_TICKS
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS
        );
    });
}

#[test]
fn test_worm_segmented_mealworm_41_startup_blanking_and_torque_boost() {
    futures::executor::block_on(async {
        use controller::motor_controller::{MOTOR_STARTUP_BLANKING_TICKS, MOTOR_TICK_INTERVAL};

        // 1. Startup blanking duration must be <= 500ms
        let blanking_duration = MOTOR_TICK_INTERVAL * MOTOR_STARTUP_BLANKING_TICKS;
        assert!(
            blanking_duration <= embassy_time::Duration::from_millis(500),
            "Startup blanking interval must be <= 500ms (got {:?})",
            blanking_duration
        );
        assert_eq!(MOTOR_STARTUP_BLANKING_TICKS, 50);

        // 2. Continuous 100% duty cycle torque and soft-start commutation ramp:
        // Speed 30 commands 30% of maximum theoretical speed (2,500 RPM).
        // Motor driver applies 100% duty cycle for holding torque from standstill.
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(200);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(30).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert!(controller.in_startup_ramp());

        // Driver is commanded to speed 30 (which operates at 100% duty cycle)
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(
            controller.motor.speed,
            MotorSpeed::new(30).unwrap(),
            "Driver speed must apply commanded speed directly"
        );

        // While in startup ramp, commutation interval steps down to target interval (3 ticks)
        while controller.in_startup_ramp() {
            assert!(controller.tick_motor().await.is_ok());
        }

        assert_eq!(
            controller.motor.speed,
            MotorSpeed::new(30).unwrap(),
            "Driver speed remains at commanded speed 30 after ramp completes"
        );
        assert!(!controller.in_startup_ramp());
    });
}

#[test]
fn test_worm_composted_digger_386_speed_30_breakaway_torque_and_gradual_slowdown() {
    futures::executor::block_on(async {
        use controller::motor_controller::{DEFAULT_MAX_RPM, STARTUP_COMMUTATION_STEP_TICKS};
        use model::types::MotorConfig;

        // 1. Initialize controller with target motor configuration
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(200);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);
        let config = MotorConfig {
            startup_blanking_ticks: 50,
            default_max_current_ma: 2500,
            startup_commutation_step_ticks: 10,
            commutation_speed_divisor: 100,
            min_commutation_step_ticks: 1,
            max_commutation_step_ticks: 100,
        };
        controller.set_config(config);

        // Verify default physical max RPM for freewheeling motor
        assert_eq!(DEFAULT_MAX_RPM, 5000);
        assert_eq!(controller.limits().max_rpm, 5000);

        // 2. Command speed 30: speed represents 30% of maximum theoretical speed (5,000 RPM)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(30).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(
            controller.motor_telemetry().speed,
            MotorSpeed::new(30).unwrap()
        );
        assert!(controller.in_startup_ramp());

        // Target commutation step interval for speed 30 is 3 ticks (833 RPM at 10ms, or 1666 RPM at 333us)
        assert_eq!(
            controller.target_commutation_step_ticks(),
            3,
            "Target interval for speed 30 must be 3 ticks to prevent cogging resonance"
        );

        // Initial soft-start commutation interval starts at STARTUP_COMMUTATION_STEP_TICKS (10 ticks)
        assert_eq!(
            controller.commutation_interval_ticks(),
            STARTUP_COMMUTATION_STEP_TICKS
        );
        assert_eq!(STARTUP_COMMUTATION_STEP_TICKS, 10);

        // Driver speed applies directly (speed configures commutation frequency, not PWM duty cycle)
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.estimated_rpm(), 1500);
        assert_eq!(
            controller.motor.speed,
            MotorSpeed::new(30).unwrap(),
            "Motor driver speed must match commanded speed directly without artificial duty cycle modulation"
        );

        // 3. Tick through soft-start commutation ramp:
        // commutation interval smoothly ramps from 15 down to 3 while motor continuously advances steps
        let mut last_interval = controller.commutation_interval_ticks();
        let mut step_count = 0;
        let mut prev_motor_ticks = controller.motor.tick_count;

        for _ in 0..150 {
            assert!(controller.tick_motor().await.is_ok());

            // Check if motor commutated
            if controller.motor.tick_count > prev_motor_ticks {
                step_count += controller.motor.tick_count - prev_motor_ticks;
                prev_motor_ticks = controller.motor.tick_count;
            }

            let cur_interval = controller.commutation_interval_ticks();
            assert!(
                cur_interval <= last_interval,
                "Commutation interval must monotonically decrease during soft-start ramp"
            );
            assert!(
                cur_interval >= 3,
                "Commutation interval must not drop below target interval (3)"
            );
            last_interval = cur_interval;
        }

        // Once settled at target interval (3 ticks) and commanded speed (30), verify state
        assert_eq!(controller.commutation_interval_ticks(), 3);
        assert_eq!(controller.motor.speed, MotorSpeed::new(30).unwrap());
        assert!(!controller.in_startup_ramp());
        assert!(
            step_count >= 15,
            "Motor must continuously advance commutation steps in forward rotation (advanced {} steps)",
            step_count
        );

        // 4. Normal runtime speed changes apply directly and update target commutation rate
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(
            controller.motor_telemetry().speed,
            MotorSpeed::new(50).unwrap()
        );
        assert_eq!(controller.target_commutation_step_ticks(), 2);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.estimated_rpm(), 2500);
        assert_eq!(controller.motor.speed, MotorSpeed::new(50).unwrap());
    });
}

#[test]
fn test_worm_undulating_serpula_688_freewheeling_motor_without_hall_sensor_spins_without_stall() {
    futures::executor::block_on(async {
        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(200);
        // Speed sensor reports 0 RPM (Hall sensor detached or freewheeling motor)
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // 1. Command motor speed 30
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(30).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.estimated_rpm(), 1500);

        // 2. Bypass startup blanking to simulate post-blanking running state
        controller.bypass_startup_blanking();
        let _ = controller.update(None).await;

        // Motor must NOT trip a false zero-RPM stall
        assert_eq!(controller.state(), MotorState::On);

        // 3. Stop motor
        controller.handle_command(MotorCommand::Stop, None).await;
        assert_eq!(controller.state(), MotorState::Off);

        // 4. Command motor speed 100
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert!(controller.tick_motor().await.is_ok());
        assert_eq!(controller.estimated_rpm(), 5000);

        // 5. Bypass startup blanking and update
        controller.bypass_startup_blanking();
        let _ = controller.update(None).await;
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_motor_speed_proportional_duty_cycle_prevents_low_speed_overheating() {
    futures::executor::block_on(async {
        use peripheral::tmc6300::Tmc6300Pio;

        struct MockInputPin;
        impl embedded_hal::digital::ErrorType for MockInputPin {
            type Error = core::convert::Infallible;
        }
        impl embedded_hal::digital::InputPin for MockInputPin {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(false)
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(true)
            }
        }

        let tmc = Tmc6300Pio::new(MockInputPin);
        let sensor = MockCurrentSensor::new(100);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(tmc, sensor, speed_sensor);

        // 1. Command low speed (1%): duty cycle must be 1 (not forced 100) to prevent coil overheating
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(1).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(controller.target_commutation_step_ticks(), 100);
        assert_eq!(
            controller.commutation_interval_ticks(),
            100,
            "Low speed startup must begin at gentle 100-tick interval (50 RPM)"
        );
        let tick1: Result<(), model::types::PeripheralError> = controller.tick_motor().await;
        assert!(tick1.is_ok());
        assert_eq!(controller.motor.duty_cycle(), 1);
        assert_eq!(controller.motor.effective_on_count(), 7); // minimum safe pulse width 520ns

        // 2. Command moderate speed (50%): active duty cycle is preserved at 1 (not reset to bare minimum)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        let tick2: Result<(), model::types::PeripheralError> = controller.tick_motor().await;
        assert!(tick2.is_ok());
        assert_eq!(
            controller.motor.duty_cycle(),
            1,
            "Speed change on running motor must preserve active duty cycle"
        );

        // 3. Regulate duty cycle to 50%
        controller.motor.set_duty_cycle(50).unwrap();
        assert_eq!(controller.motor.duty_cycle(), 50);
        assert_eq!(controller.motor.effective_on_count(), 14);

        // 4. Command full speed (100%): active duty cycle is preserved at 50%
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(100).unwrap()), None)
            .await;
        let tick3: Result<(), model::types::PeripheralError> = controller.tick_motor().await;
        assert!(tick3.is_ok());
        assert_eq!(
            controller.motor.duty_cycle(),
            50,
            "Active duty cycle must be preserved across speed transition to 100%"
        );

        // 5. Stop motor: duty cycle must clear to 0
        controller.handle_command(MotorCommand::Stop, None).await;
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.motor.duty_cycle(), 0);
    });
}

#[test]
fn test_worm_flexible_woodlouse_713_speed_reset_preserves_active_duty_cycle() {
    futures::executor::block_on(async {
        use peripheral::tmc6300::Tmc6300Pio;

        struct MockInputPin;
        impl embedded_hal::digital::ErrorType for MockInputPin {
            type Error = core::convert::Infallible;
        }
        impl embedded_hal::digital::InputPin for MockInputPin {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(false)
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(true)
            }
        }

        let tmc = Tmc6300Pio::new(MockInputPin);
        let sensor = MockCurrentSensor::new(100);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(tmc, sensor, speed_sensor);

        // 1. Start motor at 1% speed
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(1).unwrap()), None)
            .await;
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 1);

        // 2. Closed-loop or explicit regulation elevates duty cycle to 45%
        controller.motor.set_duty_cycle(45).unwrap();
        assert_eq!(controller.motor.duty_cycle(), 45);

        // 3. Command speed 2: active duty cycle must be preserved at 45% (not reset to bare minimum 2%)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(2).unwrap()), None)
            .await;
        controller.tick_motor().await.unwrap();
        assert_eq!(
            controller.motor.duty_cycle(),
            45,
            "Resetting speed on running motor must not wipe out regulated duty cycle"
        );

        // 4. Command speed 10: active duty cycle must still be preserved
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(10).unwrap()), None)
            .await;
        controller.tick_motor().await.unwrap();
        assert_eq!(
            controller.motor.duty_cycle(),
            45,
            "Speed change to 10% must preserve active duty cycle"
        );

        // 5. Stopping the motor clears duty cycle to 0
        controller.handle_command(MotorCommand::Stop, None).await;
        assert_eq!(controller.motor.duty_cycle(), 0);
        assert_eq!(controller.state(), MotorState::Off);
    });
}

#[test]
fn test_motor_closed_loop_current_regulation_boosts_duty_under_500ma() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(400); // Below 500 mA floor
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        // Configure limits with 800 mA ceiling
        controller.set_current_limits(15, 800);

        // Start motor at 50% speed (initial duty cycle = 50)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 50);

        // Update 1: current is 400 mA (< 500 mA floor), duty cycle should boost by 5%
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 55);
        assert_eq!(controller.consecutive_overcurrent_samples(), 0);
        assert_eq!(controller.state(), MotorState::On);

        // Update 2: still below floor, duty cycle boosts to 60%
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 60);

        // Rapidly advance duty cycle near max
        let _ = controller.motor.set_duty_cycle(98);
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 100);

        // Clamps at 100%, does not overflow
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 100);
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_motor_closed_loop_current_regulation_throttles_duty_above_800ma() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(850); // Above 800 mA ceiling
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        controller.set_current_limits(15, 800);

        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 50);

        // Update 1: current > ceiling, duty cycle throttled by 10% (50 -> 40)
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 40);
        assert_eq!(controller.consecutive_overcurrent_samples(), 1);
        assert_eq!(controller.state(), MotorState::On);

        // Update 2: persists > ceiling, duty cycle throttled to 30%
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 30);
        assert_eq!(controller.consecutive_overcurrent_samples(), 2);
        assert_eq!(controller.state(), MotorState::On);

        // Now simulate current dropping into normal operating band (500 mA)
        controller.current_sensor.current_ma = 500;
        controller.update(None).await.unwrap();
        // Overcurrent counter resets to 0, duty cycle stays at 30 in deadband
        assert_eq!(controller.consecutive_overcurrent_samples(), 0);
        assert_eq!(controller.motor.duty_cycle(), 30);
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_motor_stall_trips_only_after_persistent_overcurrent() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        let sensor = MockCurrentSensor::new(850); // Above 800 mA ceiling
        let speed_sensor = MockSpeedSensor::new(3000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        controller.set_current_limits(15, 800);

        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        controller.tick_motor().await.unwrap();
        controller.bypass_startup_blanking();

        // Sample 1: overcurrent sample 1 -> throttled, not stopped
        controller.update(None).await.unwrap();
        assert_eq!(controller.consecutive_overcurrent_samples(), 1);
        assert_eq!(controller.state(), MotorState::On);

        // Sample 2: overcurrent sample 2 -> throttled, not stopped
        controller.update(None).await.unwrap();
        assert_eq!(controller.consecutive_overcurrent_samples(), 2);
        assert_eq!(controller.state(), MotorState::On);

        // Sample 3: persistent overcurrent (sample 3) -> trips stall shutdown!
        controller.update(None).await.unwrap();
        assert_eq!(controller.state(), MotorState::Off);
        assert_eq!(controller.consecutive_overcurrent_samples(), 0);
        assert_eq!(controller.motor.speed, 0);
    });
}

#[test]
fn test_worm_supple_segment_373_tight_500ma_regulation_prevents_overheating() {
    futures::executor::block_on(async {
        let motor = MockMotor::new();
        // Simulate 740 mA current draw (the exact current observed in WORM-SUPPLE-SEGMENT-373 before DIAG trip)
        let sensor = MockCurrentSensor::new(740);
        let speed_sensor = MockSpeedSensor::new(2000);
        let mut controller = MotorController::new(NoTick::new(motor), sensor, speed_sensor);

        controller.set_current_limits(15, 800);

        // Command speed 25 (as in WORM-SUPPLE-SEGMENT-373)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(25).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        controller.tick_motor().await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 25);

        // In the prior code, 740 mA was below the 800 mA ceiling, so duty cycle was never throttled,
        // running continuously at ~760 mA for 240s and tripping thermal DIAG shutdown.
        // With tight regulation (ceiling 550 mA), 740 mA > 550 mA actively throttles duty cycle down.
        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 20); // 25 -> 20 (-5%)
        assert_eq!(controller.consecutive_overcurrent_samples(), 0); // Not a stall fault (>800mA)

        controller.update(None).await.unwrap();
        assert_eq!(controller.motor.duty_cycle(), 15); // 20 -> 15 (-5%)

        // Now current stabilizes in target operating band (450..=550 mA)
        controller.current_sensor.current_ma = 490;
        controller.update(None).await.unwrap();
        // Duty cycle stays at 15% in normal operating band
        assert_eq!(controller.motor.duty_cycle(), 15);
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_worm_subterranean_earthworm_231_start_at_50_speed_begins_with_breakaway_alignment() {
    futures::executor::block_on(async {
        use model::types::MotorConfig;

        let motor = MockTickableMotor::new();
        let sensor = MockCurrentSensor::new(200);
        let speed_sensor = MockSpeedSensor::new(0);
        let mut controller = MotorController::new(motor, sensor, speed_sensor);

        // Configure target board motor parameters from board.toml
        let config = MotorConfig {
            startup_blanking_ticks: 50,
            default_max_current_ma: 800,
            startup_commutation_step_ticks: 100,
            commutation_speed_divisor: 200,
            min_commutation_step_ticks: 2,
            max_commutation_step_ticks: 200,
        };
        controller.set_config(config);

        // Command speed 50% from standstill (as in WORM-SUBTERRANEAN-EARTHWORM-231)
        controller
            .handle_command(MotorCommand::SetSpeed(MotorSpeed::new(50).unwrap()), None)
            .await;
        assert_eq!(controller.state(), MotorState::On);
        assert_eq!(controller.motor_telemetry().speed.get(), 50);

        // Breakaway alignment must start at 100 ticks (50 RPM) rather than jumping to 500 RPM,
        // allowing stationary rotor to align with stator field without pole-slipping.
        assert_eq!(
            controller.commutation_interval_ticks(),
            100,
            "Initial start at speed 50 must begin at startup breakaway interval (100 ticks)"
        );
        // Target interval is 4 ticks (200 / 50 = 4 ticks = 1,250 RPM)
        assert_eq!(controller.target_commutation_step_ticks(), 4);
        assert!(controller.in_startup_ramp());

        // Step through soft-start ramp until reaching target interval
        let mut steps = 0;
        while controller.in_startup_ramp() && steps < 10_000 {
            assert!(controller.tick_motor().await.is_ok());
            steps += 1;
        }

        assert_eq!(controller.commutation_interval_ticks(), 4);
        assert!(!controller.in_startup_ramp());
        assert_eq!(controller.state(), MotorState::On);
    });
}

#[test]
fn test_worm_silken_tubeworm_979_and_round_polychaete_84_smooth_speed_scaling_prevents_stall() {
    use model::types::MotorConfig;

    let config = MotorConfig {
        startup_blanking_ticks: 50,
        default_max_current_ma: 800,
        startup_commutation_step_ticks: 100,
        commutation_speed_divisor: 200,
        min_commutation_step_ticks: 2,
        max_commutation_step_ticks: 200,
    };

    // Verify speed 35 (WORM-SILKEN-TUBEWORM-979) scales to 5 ticks (1,000 RPM)
    assert_eq!(config.calculate_commutation_step_ticks(35), 5);

    // Verify speed 50 (WORM-ROUND-POLYCHAETE-84) scales to 4 ticks (1,250 RPM)
    assert_eq!(config.calculate_commutation_step_ticks(50), 4);

    // Verify speed 51 steps smoothly to 3 ticks (1,666 RPM) without jumping to 1 tick (5,000 RPM)
    assert_eq!(config.calculate_commutation_step_ticks(51), 3);

    // Verify speed 100 scales to 2 ticks (2,500 RPM)
    assert_eq!(config.calculate_commutation_step_ticks(100), 2);

    // Verify that across all speeds 1..=100:
    // 1. Ticks is never less than min_commutation_step_ticks (2 ticks)
    // 2. Ticks is monotonically non-increasing
    let mut prev_ticks = u32::MAX;
    for speed in 1..=100 {
        let ticks = config.calculate_commutation_step_ticks(speed);
        assert!(
            ticks >= 2,
            "Ticks at speed {speed} must be >= 2 (got {ticks}) to prevent 5,000 RPM stall"
        );
        assert!(
            ticks <= prev_ticks,
            "Ticks at speed {speed} ({ticks}) must be <= previous ticks ({prev_ticks})"
        );
        prev_ticks = ticks;
    }
}
