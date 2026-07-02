//! Cat Detector target application for Raspberry Pi Pico.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    cat_detector::system_controller::SystemController,
    controller::battery_controller::BatteryController,
    controller::led_controller::LedController,
    controller::motor_controller::MotorController,
    controller::sensor_controller::SensorController,
    controller::telemetry_controller::TelemetryController,
    controller::thermal_controller::ThermalController,
    defmt_rtt as _,
    embassy_executor::Spawner,
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    embassy_sync::mutex::Mutex,
    peripherals::mock::{DummyProximitySensor, MockBattery, MockLed},
    peripherals::motor::GpioMotor,
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
        self.0.wait_for_low().await;
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cat_detector::handle_panic_with_sizes::<
        { cat_detector::FLASH_SIZE },
        { cat_detector::STACK_TOP },
        { cat_detector::FLASH_START },
        { cat_detector::FLASH_END },
        { cat_detector::FLASH_WRITE_SIZE },
        { cat_detector::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
// Define raw statically allocated Mutex for thread-safe/multi-core peripheral sharing
static SHARED_BATTERY: Mutex<CriticalSectionRawMutex, MockBattery> =
    Mutex::new(MockBattery::new(3700, 25000));

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct SafeRp2040TempSensor(Option<cat_detector::Rp2040TempSensor>);

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
    cat_detector::log_info!("Initializing hardware for cat detector...");
    let p = embassy_rp::init(Default::default());

    // Initialize board peripherals using the unified board configuration
    let mut board = cat_detector::Board::init(p);

    // Register system time function for the panic handler
    cat_detector::set_time_fn(cat_detector::system_time);

    // Initialize the modular panic handler
    static mut PANIC_FLASH: Option<
        embassy_rp::flash::Flash<
            'static,
            embassy_rp::peripherals::FLASH,
            embassy_rp::flash::Blocking,
            { cat_detector::FLASH_SIZE },
        >,
    > = None;
    let panic_flash = unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(board.flash));
        PANIC_FLASH.as_mut().unwrap()
    };
    cat_detector::init_panic_handler(
        panic_flash,
        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
    );

    // Initialize the FilesystemController using stolen FLASH peripheral (safe because panic handler only reads/writes on panic)
    let fs_flash = unsafe { embassy_rp::peripherals::FLASH::steal() };
    let raw_flash = embassy_rp::flash::Flash::<
        _,
        embassy_rp::flash::Blocking,
        { cat_detector::FLASH_SIZE },
    >::new_blocking(fs_flash);
    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(raw_flash);
    let profiling_flash = controller::filesystem_controller::ProfilingFlash::new(async_flash);
    let mut fs_controller = controller::filesystem_controller::FilesystemController::new(
        profiling_flash,
        cat_detector::STORAGE_PARTITION_START..cat_detector::STORAGE_PARTITION_END,
    );
    fs_controller.set_telemetry(cat_detector::TELEMETRY_CHANNEL.sender());

    // Extract the motor control pin from the board configuration array
    let motor_pin = board.gpio_pins[cat_detector::LED_PIN as usize]
        .take()
        .expect("Motor pin must be available");

    let motor = GpioMotor::new(motor_pin);

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    let current_sensor = {
        let mut sensor = peripherals::ina219::Ina219::new(board.i2c);
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

    let pin_north = board.gpio_pins[cat_detector::TOF_NORTH_INT_PIN as usize]
        .take()
        .expect("North ToF interrupt pin must be available");
    let pin_east = board.gpio_pins[cat_detector::TOF_EAST_INT_PIN as usize]
        .take()
        .expect("East ToF interrupt pin must be available");
    let pin_west = board.gpio_pins[cat_detector::TOF_WEST_INT_PIN as usize]
        .take()
        .expect("West ToF interrupt pin must be available");

    // Read calibration file from flash
    let mut cal_buf = [0u8; 128];
    let proximity_cal = match fs_controller
        .read_file("vl53l0x_cal.cbor", &mut cal_buf)
        .await
    {
        Ok(Some(bytes)) => {
            minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes).unwrap_or_default()
        }
        _ => model::calibration::Vl53l0xCalibration::default(),
    };

    let mut motor_cal_buf = [0u8; 128];
    let motor_cal = match fs_controller
        .read_file("motor_cal.cbor", &mut motor_cal_buf)
        .await
    {
        Ok(Some(bytes)) => minicbor::decode::<model::calibration::MotorCalibration>(bytes).ok(),
        _ => None,
    };

    use model::calibration::Calibration;
    use model::calibration::CalibrationType;

    if let Some(cal) = motor_cal {
        let min_ma = (cal.empty_current_ma + cal.water_100ml_current_ma) / 2;
        let max_ma = 800;
        controller.set_calibration(CalibrationType::MotorCal(min_ma, max_ma));
    }

    let mut sensor_ctrl_north = SensorController::new_with_fusion_and_interrupt(
        0,
        tof_north,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |_id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            direction: model::types::Direction::North,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_north),
        cat_detector::DEFAULT_PROXIMITY_THRESHOLD_MM,
    );
    sensor_ctrl_north.set_calibration(CalibrationType::ProximityCal(
        proximity_cal.north_near,
        proximity_cal.north_100,
    ));

    let mut sensor_ctrl_east = SensorController::new_with_fusion_and_interrupt(
        1,
        tof_east,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |_id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            direction: model::types::Direction::East,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_east),
        cat_detector::DEFAULT_PROXIMITY_THRESHOLD_MM,
    );
    sensor_ctrl_east.set_calibration(CalibrationType::ProximityCal(
        proximity_cal.east_near,
        proximity_cal.east_100,
    ));

    let mut sensor_ctrl_west = SensorController::new_with_fusion_and_interrupt(
        2,
        tof_west,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |_id, dist| cat_detector::system_controller::SystemCommand::SensorUpdate {
            direction: model::types::Direction::West,
            distance_mm: dist,
        },
        ProximityPinWrapper(pin_west),
        cat_detector::DEFAULT_PROXIMITY_THRESHOLD_MM,
    );
    sensor_ctrl_west.set_calibration(CalibrationType::ProximityCal(
        proximity_cal.west_near,
        proximity_cal.west_100,
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

    let thermal_ctrl = ThermalController::new_with_shutdown(
        &SHARED_TEMP_SENSOR,
        cat_detector::SYSTEM_CHANNEL.sender(),
        cat_detector::system_controller::SystemCommand::Sleep,
    );

    let fg_alert_pin = board.gpio_pins[cat_detector::FUEL_GAUGE_INT_PIN as usize]
        .take()
        .expect("Fuel gauge alert pin must be available");
    let alert_wrapper = AlertPinWrapper(fg_alert_pin);

    let power_ctrl = BatteryController::new_with_system_and_alert(
        &SHARED_BATTERY,
        &SHARED_CHARGER,
        cat_detector::SYSTEM_CHANNEL.sender(),
        |soc, state| cat_detector::system_controller::SystemCommand::BatteryUpdate {
            state_of_charge: soc,
            charger_state: state,
        },
        alert_wrapper,
    );

    // Initialize simulated LED driver and its controller
    let led_driver = MockLed::new();
    let led_ctrl = LedController::new(led_driver);

    // Initialize SystemController to coordinate all loops
    let system_ctrl = SystemController::new(
        cat_detector::MOTOR_CHANNEL.sender(),
        cat_detector::SENSOR_NORTH_CHANNEL.sender(),
        cat_detector::SENSOR_EAST_CHANNEL.sender(),
        cat_detector::SENSOR_WEST_CHANNEL.sender(),
        cat_detector::BATTERY_CHANNEL.sender(),
        cat_detector::THERMAL_CHANNEL.sender(),
        cat_detector::LED_CHANNEL.sender(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        cat_detector::DEFAULT_PROXIMITY_THRESHOLD_MM,
    );

    // Spawn controllers selectively and concurrently using separate macros
    controller::run_thermal_task!(
        spawner,
        thermal_task,
        thermal_ctrl,
        cat_detector::THERMAL_CHANNEL.receiver(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        SafeRp2040TempSensor,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_battery_task!(
        spawner,
        power_task,
        power_ctrl,
        cat_detector::BATTERY_CHANNEL.receiver(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        MockBattery,
        SafeBq25185,
        AlertPinWrapper,
        cat_detector::system_controller::SystemCommand
    );

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    controller::run_motor_task!(
        spawner,
        motor_task,
        controller,
        cat_detector::MOTOR_CHANNEL.receiver(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        GpioMotor<embassy_rp::gpio::Flex<'static>>,
        peripherals::ina219::Ina219<
            embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
        >
    );

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    controller::run_motor_task!(
        spawner,
        motor_task,
        controller,
        cat_detector::MOTOR_CHANNEL.receiver(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        GpioMotor<embassy_rp::gpio::Flex<'static>>,
        DummyCurrentSensor
    );

    // Spawn the three proximity sensor tasks
    controller::run_sensor_task!(
        spawner,
        sensor_north_task,
        sensor_ctrl_north,
        cat_detector::SENSOR_NORTH_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_east_task,
        sensor_ctrl_east,
        cat_detector::SENSOR_EAST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        cat_detector::system_controller::SystemCommand
    );

    controller::run_sensor_task!(
        spawner,
        sensor_west_task,
        sensor_ctrl_west,
        cat_detector::SENSOR_WEST_CHANNEL.receiver(),
        DummyProximitySensor,
        CriticalSectionRawMutex,
        ProximityPinWrapper,
        cat_detector::system_controller::SystemCommand
    );

    // Spawn the LED controller task
    controller::run_led_task!(
        spawner,
        led_task,
        led_ctrl,
        cat_detector::LED_CHANNEL.receiver(),
        cat_detector::TELEMETRY_CHANNEL.sender(),
        MockLed,
        CriticalSectionRawMutex
    );

    cat_detector::run_system_task!(
        spawner,
        system_task,
        system_ctrl,
        cat_detector::SYSTEM_CHANNEL.receiver()
    );

    cat_detector::run_filesystem_task!(
        spawner,
        filesystem_task,
        fs_controller,
        cat_detector::FILESYSTEM_CHANNEL.receiver(),
        controller::filesystem_controller::ProfilingFlash<
            firmware_lib::panic_handler::BlockingAsyncFlash<
                embassy_rp::flash::Flash<
                    'static,
                    embassy_rp::peripherals::FLASH,
                    embassy_rp::flash::Blocking,
                    { cat_detector::FLASH_SIZE },
                >,
            >,
        >
    );

    let client = controller::filesystem_controller::FilesystemClient::new(
        cat_detector::FILESYSTEM_CHANNEL.sender(),
    );
    let telemetry_ctrl =
        TelemetryController::<45, { 12 + 45 * 20 + 128 }>::new(client, cat_detector::system_time);
    cat_detector::run_telemetry_task!(
        spawner,
        telemetry_task,
        telemetry_ctrl,
        cat_detector::TELEMETRY_CHANNEL.receiver(),
        45
    );
}

/// Host main entry point for testing and compilation verification.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
