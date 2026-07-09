use controller::battery_controller::FromBatteryUpdate;
use controller::motor_controller::{MotorCommand, MotorController, MotorState};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TestCmd(pub u8);

impl FromBatteryUpdate for TestCmd {
    fn from_battery_update(state_of_charge: u8, _charger_state: model::types::ChargeState) -> Self {
        TestCmd(state_of_charge)
    }
}
use model::calibration::Calibration;
use model::interfaces::{Motor, NoTick, Tickable};
use model::types::MotorSpeed;
use peripherals::mock::{MockCurrentSensor, MockMotor};

#[test]
fn test_motor_controller_flow() {
    let motor = MockMotor::new();
    let sensor = MockCurrentSensor::new(150); // healthy current: 150mA
    let mut controller = MotorController::new(NoTick::new(motor), sensor);

    assert_eq!(controller.state(), MotorState::Off);

    // Apply motor calibration so that it can be started
    use model::calibration::{Calibration, CalibrationType};
    controller.set_calibration(CalibrationType::MotorCal(80, 800));

    // Turn on the motor using handle_command
    controller.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None);
    assert_eq!(controller.state(), MotorState::On);
    assert_eq!(controller.motor.speed, 0);
    // Let's call tick_motor() once:
    controller.tick_motor().unwrap();
    assert_eq!(controller.motor.speed, 1);

    // Tick to complete the ramp
    for _ in 0..99 {
        controller.tick_motor().unwrap();
    }
    assert_eq!(controller.motor.speed, 100);

    // Simulate dry run (low current draw)
    controller.current_sensor.current_ma = 10; // below 15mA threshold
    controller.update(None).unwrap(); // triggers PowerOff -> state becomes Off
    assert_eq!(controller.state(), MotorState::Off);
    assert_eq!(controller.motor.speed, 0); // motor should be stopped

    // Restart the motor
    controller.current_sensor.current_ma = 150; // reset to healthy current
    controller.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None);
    assert_eq!(controller.state(), MotorState::On);
    controller.tick_motor().unwrap();
    assert_eq!(controller.motor.speed, 1);
    for _ in 0..99 {
        controller.tick_motor().unwrap();
    }
    assert_eq!(controller.motor.speed, 100);

    // Simulate stall (high current draw)
    controller.current_sensor.current_ma = 900; // above 800mA threshold
    controller.update(None).unwrap(); // triggers PowerOff -> state becomes Off
    assert_eq!(controller.state(), MotorState::Off);
    assert_eq!(controller.motor.speed, 0); // motor should be stopped
}

#[test]
fn test_led_controller_flow() {
    futures::executor::block_on(async {
        let mock_led = peripherals::mock::MockLed::new();
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
    let mut motor = MockMotor::new();
    motor.should_fail = true; // Make motor fail
    let sensor = MockCurrentSensor::new(150);
    let mut controller = MotorController::new(NoTick::new(motor), sensor);
    controller.set_calibration(model::calibration::CalibrationType::MotorCal(80, 800));

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
    controller.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), Some(&mut client));

    // Consume the initial MotorStatus report
    let rec1 = telemetry_rx.try_receive().unwrap();
    assert!(matches!(rec1, model::telemetry::TelemetryRecord::Motor(_)));

    let res = controller.tick_motor();
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
    let mut controller2 = MotorController::new(NoTick::new(motor2), sensor2);
    controller2.set_calibration(model::calibration::CalibrationType::MotorCal(80, 800));
    controller2.handle_command(MotorCommand::SetSpeed(MotorSpeed::MAX), None); // start motor first (no failure on motor)

    let mut client2 =
        controller::telemetry_controller::MotorTelemetryClient::new(Some(telemetry_tx));
    // Call update, which reads current. It should fail and return Err
    let res = controller2.update(Some(&mut client2));
    assert!(res.is_err());
}

#[test]
fn test_led_controller_sad_cases() {
    futures::executor::block_on(async {
        let mut mock_led = peripherals::mock::MockLed::new();
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
            model::types::SystemLedState,
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
            .try_send(model::types::SystemLedState::SolidGreen)
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
        use peripherals::mock::{MockBattery, MockCharger};

        let mut battery = MockBattery::new(3700, 25000);
        battery.should_fail = true; // Make battery fail
        let battery_mutex = Mutex::<CriticalSectionRawMutex, _>::new(battery);

        let charger = MockCharger::new(ChargeState::DoneOrStandbyOrUnplugged);
        let charger_mutex = Mutex::<CriticalSectionRawMutex, _>::new(charger);

        let system_channel = Box::leak(Box::new(embassy_sync::channel::Channel::<
            CriticalSectionRawMutex,
            TestCmd,
            4,
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

    fn tick(&mut self) -> Result<(), Self::Error> {
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
    // 1. Test stateful Tickable motor
    let motor = MockTickableMotor::new();
    let sensor = MockCurrentSensor::new(150);
    let mut controller = MotorController::new(motor, sensor);

    assert_eq!(controller.motor.tick_count, 0);
    assert!(controller.tick_motor().is_ok());
    assert_eq!(controller.motor.tick_count, 1);

    // Test failing tick
    controller.motor.should_fail_tick = true;
    assert!(controller.tick_motor().is_err());

    // 2. Test NoTick motor wrapper
    let notick_motor = NoTick::new(MockTickableMotor::new());
    let sensor2 = MockCurrentSensor::new(150);
    let mut controller2 = MotorController::new(notick_motor, sensor2);

    // Ticking shouldn't increase inner tick count or return error even if inner should fail,
    // because NoTick::tick() is a no-op that always returns Ok(()) without calling inner tick.
    controller2.motor.should_fail_tick = true;
    assert!(controller2.tick_motor().is_ok());
    assert_eq!(controller2.motor.tick_count, 0);
}
