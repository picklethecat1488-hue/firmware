//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use embassy_sync::mutex::Mutex;
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
use peripherals::mock::MockBattery;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    app::CatDetectorFeatureSet,
    cat_detector as app,
    controller::{
        battery_controller::BatteryController, led_controller::LedController,
        motor_controller::MotorController, sensor_controller::SensorController,
        telemetry_controller::TelemetryController, thermal_controller::ThermalController,
        BatteryFeatureConfig, GestureAction, LedFeatureConfig, MotorFeatureConfig,
        ProximityFeatureConfig, SystemController, ThermalFeatureConfig,
    },
    embassy_executor::Spawner,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    embassy_sync::mutex::Mutex,
    firmware_lib::BatteryManager,
    peripherals::l9110s::L9110s,
    peripherals::mock::{DummyProximitySensor, MockLed},
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct AlertPinWrapper(embassy_rp::gpio::Flex<'static>);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl controller::battery_controller::BatteryAlertPin for AlertPinWrapper {
    async fn wait_for_alert(&mut self) {
        self.0.wait_for_low().await;
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct ProximityPinWrapper(embassy_rp::gpio::Flex<'static>);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl controller::sensor_controller::DataReadyPin for ProximityPinWrapper {
    async fn wait_for_data_ready(&mut self) {
        self.0.wait_for_falling_edge().await;
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic_with_sizes::<
        { app::FLASH_SIZE },
        { app::STACK_TOP },
        { app::FLASH_START },
        { app::FLASH_END },
        { app::FLASH_WRITE_SIZE },
        { app::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Statically allocated mutex holding the physical MAX17048 fuel gauge on target.
static SHARED_BATTERY: Mutex<
    CriticalSectionRawMutex,
    peripherals::max17048::Max17048<firmware_lib::i2c::SharedI2cWrapper<'static>>,
> = Mutex::new(peripherals::max17048::Max17048::new(
    firmware_lib::i2c::SharedI2cWrapper::new(&app::SHARED_I2C),
));

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
#[allow(dead_code)]
/// Statically allocated mutex holding the MockBattery for non-ARM targets.
static SHARED_BATTERY: Mutex<CriticalSectionRawMutex, MockBattery> =
    Mutex::new(MockBattery::new(3700, 25000));

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct SafeRp2040TempSensor(Option<app::Rp2040TempSensor>);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl model::interfaces::TemperatureSensor for SafeRp2040TempSensor {
    type Error = ();

    fn read_temperature_milli_c(&mut self) -> Result<i32, Self::Error> {
        if let Some(ref mut sensor) = self.0 {
            sensor.read_temperature_milli_c().map_err(|_| ())
        } else {
            Ok(25000)
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static SHARED_TEMP_SENSOR: Mutex<CriticalSectionRawMutex, SafeRp2040TempSensor> =
    Mutex::new(SafeRp2040TempSensor(None));

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct SafeBq25185(
    Option<
        peripherals::bq25185::Bq25185<
            embassy_rp::gpio::Flex<'static>,
            embassy_rp::gpio::Flex<'static>,
        >,
    >,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl model::interfaces::ChargeStatus for SafeBq25185 {
    type Error = ();

    fn get_charge_state(&mut self) -> Result<model::types::ChargeState, Self::Error> {
        if let Some(ref mut chg) = self.0 {
            Ok(chg.get_state())
        } else {
            Ok(model::types::ChargeState::DoneOrStandbyOrUnplugged)
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static SHARED_CHARGER: Mutex<CriticalSectionRawMutex, SafeBq25185> = Mutex::new(SafeBq25185(None));

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure hardware stack guard using Cortex-M MPU
    app::configure_mpu_stack_guard();

    // Initialize board peripherals using the unified board configuration
    let mut board = app::Board::init(p);

    // Route defmt logs to RTT
    firmware_lib::defmt_logger::DefmtLogger::set_writer(
        &firmware_lib::defmt_logger::DEFAULT_RTT_WRITER,
    );
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    defmt::info!("Booting Cat Detector App...");

    // Initialize the modular panic handler
    static mut PANIC_FLASH: Option<
        embassy_rp::flash::Flash<
            'static,
            embassy_rp::peripherals::FLASH,
            embassy_rp::flash::Blocking,
            { app::FLASH_SIZE },
        >,
    > = None;
    // Declare statically to avoid stack allocation and stack overflow
    static mut FS_BUF: [u8; 4096] = [0u8; 4096];

    let panic_flash = unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(board.flash));
        PANIC_FLASH.as_mut().unwrap()
    };
    // Obtain separate static mut references for the panic handler and filesystem controller.
    // This is safe because the panic handler only runs after the application halts/panics.
    let fs_buf_panic = unsafe { &mut *core::ptr::addr_of_mut!(FS_BUF) };
    let fs_buf_controller = unsafe { &mut *core::ptr::addr_of_mut!(FS_BUF) };

    app::init_panic_handler(
        panic_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf_panic,
    );

    // Initialize the FilesystemController using stolen FLASH peripheral (safe because panic handler only reads/writes on panic)
    let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
    let raw_flash = embassy_rp::flash::Flash::<
        _,
        embassy_rp::flash::Blocking,
        { app::FLASH_SIZE },
    >::new_blocking(fs_flash);
    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(raw_flash);
    let profiling_flash = controller::filesystem_controller::ProfilingFlash::new(async_flash);
    let mut fs_controller = controller::filesystem_controller::FilesystemController::new(
        profiling_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf_controller,
    );
    fs_controller.set_telemetry(app::TELEMETRY_CHANNEL.sender());

    // Verify and repair/reformat the filesystem if it is corrupted
    let _ = fs_controller.verify_and_repair().await;

    // Extract the motor control pins from the board configuration array
    let motor_pin_ia = board.gpio_pins[app::PUMP_PIN_IA as usize]
        .take()
        .expect("Motor pin IA must be available");
    let motor_pin_ib = board.gpio_pins[app::PUMP_PIN_IB as usize]
        .take()
        .expect("Motor pin IB must be available");

    let motor = L9110s::new(motor_pin_ia, motor_pin_ib);

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    let current_sensor = {
        let mut sensor = peripherals::ina219::Ina219::new(
            firmware_lib::i2c::SharedI2cWrapper::new(&app::SHARED_I2C),
        );
        let _ = sensor.init();
        sensor
    };
    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    let current_sensor = peripherals::mock::DummyCurrentSensor;

    let mut controller = MotorController::new(motor, current_sensor);

    // Initialize simulated proximity sensors for North, East, West ToFs
    let tof_north = DummyProximitySensor::new(100);
    let tof_east = DummyProximitySensor::new(150);
    let tof_west = DummyProximitySensor::new(200);

    let pin_north = board.gpio_pins[app::TOF_NORTH_INT_PIN as usize]
        .take()
        .expect("North ToF interrupt pin must be available");
    let pin_east = board.gpio_pins[app::TOF_EAST_INT_PIN as usize]
        .take()
        .expect("East ToF interrupt pin must be available");
    let pin_west = board.gpio_pins[app::TOF_WEST_INT_PIN as usize]
        .take()
        .expect("West ToF interrupt pin must be available");

    // Read calibration file from flash
    let mut cal_buf = [0u8; 128];
    let proximity_cal = match fs_controller
        .read_file("vl53l0x_cal.cbor", &mut cal_buf)
        .await
    {
        Ok(Some(bytes)) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("vl53l0x calibration: {=[u8]:cbor}", bytes);
            minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes).unwrap_or_default()
        }
        _ => model::calibration::Vl53l0xCalibration::default(),
    };

    let mut motor_cal_buf = [0u8; 128];
    let motor_cal = match fs_controller
        .read_file("motor_cal.cbor", &mut motor_cal_buf)
        .await
    {
        Ok(Some(bytes)) => {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("motor calibration: {=[u8]:cbor}", bytes);
            minicbor::decode::<model::calibration::MotorCalibration>(bytes).ok()
        }
        _ => None,
    };

    use model::calibration::Calibration;
    use model::calibration::CalibrationType;

    if let Some(cal) = motor_cal {
        let min_ma = cal.dry_run_limit();
        let max_ma = cal.stall_limit();
        let max_rpm = cal.max_rpm.unwrap_or(0);
        let rpm_limit = cal.rpm_limit.unwrap_or(0);
        controller.set_calibration(CalibrationType::MotorCal {
            current_limits: model::calibration::TwoPointCalibration::new(min_ma, max_ma),
            max_rpm,
            rpm_limit,
        });
    }

    let mut sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
        0,
        tof_north,
        app::SYSTEM_CHANNEL.sender(),
        |_id, dist| app::SystemCommand::ProximityUpdate {
            direction: model::types::Direction::North,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_north),
        app::DEFAULT_WAKE_THRESHOLD_MM,
    );
    sensor_ctrl_north.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::North],
    ));

    let mut sensor_ctrl_east = SensorController::new_with_fusion_and_interrupt(
        1,
        tof_east,
        app::SYSTEM_CHANNEL.sender(),
        |_id, dist| app::SystemCommand::ProximityUpdate {
            direction: model::types::Direction::East,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_east),
        app::DEFAULT_WAKE_THRESHOLD_MM,
    );
    sensor_ctrl_east.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::East],
    ));

    let mut sensor_ctrl_west = SensorController::new_with_fusion_and_interrupt(
        2,
        tof_west,
        app::SYSTEM_CHANNEL.sender(),
        |_id, dist| app::SystemCommand::ProximityUpdate {
            direction: model::types::Direction::West,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_west),
        app::DEFAULT_WAKE_THRESHOLD_MM,
    );
    sensor_ctrl_west.set_calibration(CalibrationType::ProximityCal(
        proximity_cal[model::types::Direction::West],
    ));

    // Initialize the real Rp2040TempSensor in SHARED_TEMP_SENSOR
    {
        let mut sensor = SHARED_TEMP_SENSOR.lock().await;
        sensor.0 = board.temp_sensor.take();
    }

    // Initialize the real Bq25185 in SHARED_CHARGER
    {
        let mut chg = SHARED_CHARGER.lock().await;
        chg.0 = board.charger.take();
    }

    // Initialize the shared I2C bus on target
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    {
        app::SHARED_I2C.lock(|cell| {
            cell.borrow_mut().0 = Some(board.i2c);
        });
    }

    let thermal_ctrl = ThermalController::new_with_shutdown(
        &SHARED_TEMP_SENSOR,
        app::SYSTEM_CHANNEL.sender(),
        app::SystemCommand::AlertTriggered,
    );

    let fg_alert_pin = board.gpio_pins[app::FUEL_GAUGE_INT_PIN as usize]
        .take()
        .expect("Fuel gauge alert pin must be available");
    let alert_wrapper = AlertPinWrapper(fg_alert_pin);

    let power_ctrl = BatteryController::new_with_system_and_alert(
        &SHARED_BATTERY,
        &SHARED_CHARGER,
        app::SYSTEM_CHANNEL.sender(),
        alert_wrapper,
    );

    // Initialize simulated LED driver and its controller
    let led_driver = MockLed::new();
    let led_ctrl = LedController::new(led_driver);

    // Initialize SystemController to coordinate all loops
    let feature_set = app::CatDetectorFeatureSet {
        features: (
            MotorFeatureConfig::new(
                Some(app::MOTOR_CHANNEL.sender()),
                model::types::MotorSpeed::MAX,
            ),
            BatteryFeatureConfig::new(
                Some(app::BATTERY_CHANNEL.sender()),
                BatteryManager::new(
                    app::CRITICAL_BATTERY_SOC_THRESHOLD,
                    app::BATTERY_SOC_HYSTERESIS,
                    app::LOW_BATTERY_SOC_THRESHOLD,
                    app::MID_BATTERY_SOC_THRESHOLD,
                    app::HIGH_BATTERY_SOC_THRESHOLD,
                ),
            ),
            ProximityFeatureConfig::new(
                &[
                    app::SENSOR_NORTH_CHANNEL.sender(),
                    app::SENSOR_EAST_CHANNEL.sender(),
                    app::SENSOR_WEST_CHANNEL.sender(),
                ],
                app::DEFAULT_PRESS_THRESHOLD_MM,
                app::DEFAULT_WAKE_THRESHOLD_MM,
                GestureAction::TogglePower,
                Some(app::TELEMETRY_CHANNEL.sender()),
            ),
            LedFeatureConfig::new(Some(app::LED_CHANNEL.sender())),
            ThermalFeatureConfig::new(Some(app::THERMAL_CHANNEL.sender())),
        ),
    };
    let boot_reason = app::get_boot_reason();

    let system_ctrl =
        SystemController::new(feature_set, app::TELEMETRY_CHANNEL.sender(), boot_reason);

    // Spawn controllers selectively and concurrently using separate macros
    controller::run_thermal_task!(
        spawner,
        thermal_task,
        thermal_ctrl,
        app::THERMAL_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        SafeRp2040TempSensor,
        app::SystemCommand
    );

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    controller::run_battery_task!(
        spawner,
        power_task,
        power_ctrl,
        app::BATTERY_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        peripherals::max17048::Max17048<firmware_lib::i2c::SharedI2cWrapper<'static>>,
        SafeBq25185,
        AlertPinWrapper,
        app::SystemCommand
    );

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    controller::run_battery_task!(
        spawner,
        power_task,
        power_ctrl,
        app::BATTERY_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        MockBattery,
        SafeBq25185,
        AlertPinWrapper,
        app::SystemCommand
    );

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    controller::run_motor_task!(
        spawner,
        motor_task,
        controller,
        app::MOTOR_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        L9110s<embassy_rp::gpio::Flex<'static>, embassy_rp::gpio::Flex<'static>>,
        peripherals::ina219::Ina219<firmware_lib::i2c::SharedI2cWrapper<'static>>
    );

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    controller::run_motor_task!(
        spawner,
        motor_task,
        controller,
        app::MOTOR_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        L9110s<embassy_rp::gpio::Flex<'static>, embassy_rp::gpio::Flex<'static>>,
        DummyCurrentSensor
    );

    // Spawn the three proximity sensor tasks
    controller::run_sensor_task!(
        spawner,
        sensor_north_task,
        sensor_ctrl_north,
        app::SENSOR_NORTH_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        app::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_east_task,
        sensor_ctrl_east,
        app::SENSOR_EAST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        app::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_west_task,
        sensor_ctrl_west,
        app::SENSOR_WEST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        app::SystemCommand
    );

    // Spawn the LED controller task
    controller::run_led_task!(
        spawner,
        led_task,
        led_ctrl,
        app::LED_CHANNEL.receiver(),
        app::TELEMETRY_CHANNEL.sender(),
        MockLed,
        CriticalSectionRawMutex
    );

    controller::run_system_task!(
        spawner,
        system_task,
        system_ctrl,
        SystemController<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            CatDetectorFeatureSet<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                4,
            >,
        >,
        app::SYSTEM_CHANNEL.receiver(),
        app::GESTURE_CHANNEL.receiver()
    );

    app::run_filesystem_task!(
        spawner,
        filesystem_task,
        fs_controller,
        app::FILESYSTEM_CHANNEL.receiver(),
        controller::filesystem_controller::ProfilingFlash<
            firmware_lib::panic_handler::BlockingAsyncFlash<
                embassy_rp::flash::Flash<
                    'static,
                    embassy_rp::peripherals::FLASH,
                    embassy_rp::flash::Blocking,
                    { app::FLASH_SIZE },
                >,
            >,
        >
    );

    // Declare statically to avoid stack overflow on the main thread MSP stack
    static mut TELEMETRY_CTRL: Option<
        TelemetryController<1024, { model::telemetry::BUFFER_SIZE }>,
    > = None;

    let client =
        controller::filesystem_controller::FilesystemClient::new(app::FILESYSTEM_CHANNEL.sender());

    let telemetry_ctrl = unsafe {
        TELEMETRY_CTRL = Some(TelemetryController::new(client));
        TELEMETRY_CTRL.as_mut().unwrap()
    };

    app::run_telemetry_task!(
        spawner,
        telemetry_task,
        telemetry_ctrl,
        app::TELEMETRY_CHANNEL.receiver(),
        1024,
        { controller::telemetry_controller::CHANNEL_CAPACITY }
    );
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
