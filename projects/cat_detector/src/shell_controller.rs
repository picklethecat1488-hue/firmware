//! Shell controller for processing interactive bringup CLI commands.

use crate as app;
use app::system_controller::SystemCommand;
use app::CliCommand;
use controller::motor_controller::MotorCommand;
use controller::{
    BlockingBatteryReader, BlockingMotorReader, BlockingProximityReader, BlockingThermalReader,
};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use embedded_cli::cli::CliHandle;
use embedded_cli::command::RawCommand;
use embedded_cli::service::CommandProcessor;
use embedded_io::Write as IoWrite;
use model::interfaces::{PowerSensor, ProximitySensor, TemperatureSensor};

/// Controller responsible for processing shell commands.
/// Context pointers to drivers and controllers for direct diagnostics.
pub struct ShellControllerPointers<
    I2c,
    Motor,
    Flash,
    BatteryCtrl,
    ThermalCtrl,
    SensorCtrl,
    MotorCtrl,
    TempSensor,
> {
    /// Pointer to shared I2C bus driver
    pub i2c_ptr: Option<*mut I2c>,
    /// Pointer to physical motor peripheral
    pub motor_ptr: Option<*mut Motor>,
    /// Pointer to physical flash peripheral
    pub flash_ptr: Option<*mut Flash>,
    /// Pointer to battery controller
    pub battery_ctrl_ptr: Option<*mut BatteryCtrl>,
    /// Pointer to thermal controller
    pub thermal_ctrl_ptr: Option<*mut ThermalCtrl>,
    /// Pointer to North proximity sensor controller
    pub sensor_north_ctrl_ptr: Option<*mut SensorCtrl>,
    /// Pointer to East proximity sensor controller
    pub sensor_east_ctrl_ptr: Option<*mut SensorCtrl>,
    /// Pointer to West proximity sensor controller
    pub sensor_west_ctrl_ptr: Option<*mut SensorCtrl>,
    /// Pointer to motor current controller
    pub motor_ctrl_ptr: Option<*mut MotorCtrl>,
    /// Pointer to microcontroller temperature sensor
    pub temp_sensor_ptr: Option<*mut TempSensor>,
}

/// Controller responsible for processing shell commands.
pub struct ShellController<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    I2c: 'static,
    Motor: 'static,
    Flash: 'static,
    BatteryCtrl: 'static,
    ThermalCtrl: 'static,
    SensorCtrl: 'static,
    MotorCtrl: 'static,
    TempSensor: 'static,
> {
    motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    system_tx: Sender<'static, MutexRaw, SystemCommand, N>,
    i2c_ptr: Option<*mut I2c>,
    motor_ptr: Option<*mut Motor>,
    flash_ptr: Option<*mut Flash>,
    battery_ctrl_ptr: Option<*mut BatteryCtrl>,
    thermal_ctrl_ptr: Option<*mut ThermalCtrl>,
    sensor_north_ctrl_ptr: Option<*mut SensorCtrl>,
    sensor_east_ctrl_ptr: Option<*mut SensorCtrl>,
    sensor_west_ctrl_ptr: Option<*mut SensorCtrl>,
    motor_ctrl_ptr: Option<*mut MotorCtrl>,
    temp_sensor_ptr: Option<*mut TempSensor>,
    storage_start: u32,
    storage_end: u32,
}

// Implement Send and Sync manually since ShellController contains raw pointers
unsafe impl<
        MutexRaw: RawMutex + 'static,
        const N: usize,
        I2c: 'static,
        Motor: 'static,
        Flash: 'static,
        BatteryCtrl: 'static,
        ThermalCtrl: 'static,
        SensorCtrl: 'static,
        MotorCtrl: 'static,
        TempSensor: 'static,
    > Send
    for ShellController<
        MutexRaw,
        N,
        I2c,
        Motor,
        Flash,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        TempSensor,
    >
{
}

unsafe impl<
        MutexRaw: RawMutex + 'static,
        const N: usize,
        I2c: 'static,
        Motor: 'static,
        Flash: 'static,
        BatteryCtrl: 'static,
        ThermalCtrl: 'static,
        SensorCtrl: 'static,
        MotorCtrl: 'static,
        TempSensor: 'static,
    > Sync
    for ShellController<
        MutexRaw,
        N,
        I2c,
        Motor,
        Flash,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        TempSensor,
    >
{
}

impl<
        MutexRaw: RawMutex + 'static,
        const N: usize,
        I2c: 'static,
        Motor: 'static,
        Flash: 'static,
        BatteryCtrl: 'static,
        ThermalCtrl: 'static,
        SensorCtrl: 'static,
        MotorCtrl: 'static,
        TempSensor: 'static,
    >
    ShellController<
        MutexRaw,
        N,
        I2c,
        Motor,
        Flash,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        TempSensor,
    >
{
    /// Creates a new ShellController.
    pub fn new(
        motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
        system_tx: Sender<'static, MutexRaw, SystemCommand, N>,
        pointers: ShellControllerPointers<
            I2c,
            Motor,
            Flash,
            BatteryCtrl,
            ThermalCtrl,
            SensorCtrl,
            MotorCtrl,
            TempSensor,
        >,
        storage_start: u32,
        storage_end: u32,
    ) -> Self {
        Self {
            motor_tx,
            system_tx,
            i2c_ptr: pointers.i2c_ptr,
            motor_ptr: pointers.motor_ptr,
            flash_ptr: pointers.flash_ptr,
            battery_ctrl_ptr: pointers.battery_ctrl_ptr,
            thermal_ctrl_ptr: pointers.thermal_ctrl_ptr,
            sensor_north_ctrl_ptr: pointers.sensor_north_ctrl_ptr,
            sensor_east_ctrl_ptr: pointers.sensor_east_ctrl_ptr,
            sensor_west_ctrl_ptr: pointers.sensor_west_ctrl_ptr,
            motor_ctrl_ptr: pointers.motor_ctrl_ptr,
            temp_sensor_ptr: pointers.temp_sensor_ptr,
            storage_start,
            storage_end,
        }
    }
}

impl<
        MutexRaw: RawMutex + 'static,
        const N: usize,
        I2c: 'static,
        Motor: 'static,
        Flash: 'static,
        BatteryCtrl: 'static,
        ThermalCtrl: 'static,
        SensorCtrl: 'static,
        MotorCtrl: 'static,
        TempSensor: 'static,
        W: IoWrite<Error = E>,
        E: embedded_io::Error,
    > CommandProcessor<W, E>
    for ShellController<
        MutexRaw,
        N,
        I2c,
        Motor,
        Flash,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        TempSensor,
    >
where
    I2c: embedded_hal::i2c::I2c,
    Motor: model::interfaces::Motor,
    Flash: embedded_storage::nor_flash::NorFlash,
    BatteryCtrl: BlockingBatteryReader,
    ThermalCtrl: BlockingThermalReader,
    SensorCtrl: BlockingProximityReader,
    MotorCtrl: BlockingMotorReader,
    TempSensor: TemperatureSensor,
{
    fn process<'a>(
        &mut self,
        cli: &mut CliHandle<'_, W, E>,
        raw: RawCommand<'a>,
    ) -> Result<(), embedded_cli::service::ProcessError<'a, E>> {
        let writer = cli.writer();

        // Intercept help commands to print the auto-generated help list and command details
        if let Some(help_req) = embedded_cli::help::HelpRequest::from_command(&raw) {
            match help_req {
                embedded_cli::help::HelpRequest::All => {
                    let _ = <CliCommand as embedded_cli::service::Help>::list_commands(writer);
                }
                embedded_cli::help::HelpRequest::Command(subcommand) => {
                    let mut parent = |_writer: &mut embedded_cli::writer::Writer<'_, W, E>| Ok(());
                    if let Err(embedded_cli::service::HelpError::UnknownCommand) =
                        <CliCommand as embedded_cli::service::Help>::command_help(
                            &mut parent,
                            subcommand,
                            writer,
                        )
                    {
                        let _ = core::writeln!(writer, "\r\nUnknown command");
                    }
                }
            }
            return Ok(());
        }

        let cmd = <CliCommand as embedded_cli::service::FromRaw>::parse(raw)?;

        let res: Result<(), &'static str> = match cmd {
            CliCommand::Motor { speed } => {
                let _ = self.motor_tx.try_send(MotorCommand::SetSpeed(speed));

                if let Some(ctrl_raw) = self.motor_ctrl_ptr {
                    let ctrl = unsafe { &mut *ctrl_raw };
                    if let Some(current) = ctrl.read_current_ma_blocking() {
                        let _ = core::writeln!(writer, "\r\nMotor current: {} mA", current);
                    }
                }
                Ok(())
            }
            CliCommand::Stop => self
                .motor_tx
                .try_send(MotorCommand::Stop)
                .map_err(|_| "Failed to send Motor Stop command"),
            CliCommand::Battery => {
                if let Some(ctrl_raw) = self.battery_ctrl_ptr {
                    let ctrl = unsafe { &*ctrl_raw };
                    if let Some((v, soc)) = ctrl.read_battery_blocking() {
                        let _ = core::writeln!(
                            writer,
                            "\r\nDirect battery reading: {} mV, {}% state of charge",
                            v,
                            soc
                        );
                        Ok(())
                    } else {
                        Err("Direct battery reading failed")
                    }
                } else {
                    Err("Battery controller not available")
                }
            }
            CliCommand::Thermal => {
                if let Some(ctrl_raw) = self.thermal_ctrl_ptr {
                    let ctrl = unsafe { &*ctrl_raw };
                    if let Some(temp) = ctrl.read_temperature_blocking() {
                        let _ = core::writeln!(
                            writer,
                            "\r\nDirect thermal reading (ThermalController): {}.{:03} C",
                            temp / 1000,
                            (temp.abs() % 1000)
                        );
                        Ok(())
                    } else {
                        Err("Direct thermal reading failed")
                    }
                } else {
                    Err("Thermal controller not available")
                }
            }
            CliCommand::Proximity => {
                let d_north = self
                    .sensor_north_ctrl_ptr
                    .and_then(|p| unsafe { &mut *p }.read_distance_blocking());
                let d_east = self
                    .sensor_east_ctrl_ptr
                    .and_then(|p| unsafe { &mut *p }.read_distance_blocking());
                let d_west = self
                    .sensor_west_ctrl_ptr
                    .and_then(|p| unsafe { &mut *p }.read_distance_blocking());

                match (d_north, d_east, d_west) {
                    (Some(dn), Some(de), Some(dw)) => {
                        let _ = core::writeln!(
                            writer,
                            "\r\nDirect proximity readings: North = {} mm, East = {} mm, West = {} mm",
                            dn,
                            de,
                            dw
                        );
                        Ok(())
                    }
                    _ => Err("One or more proximity sensors failed to read"),
                }
            }
            CliCommand::Wake => self
                .system_tx
                .try_send(SystemCommand::Wake)
                .map_err(|_| "Failed to send System Wake command"),
            CliCommand::Sleep => self
                .system_tx
                .try_send(SystemCommand::Sleep)
                .map_err(|_| "Failed to send System Sleep command"),
            CliCommand::Activity => self
                .system_tx
                .try_send(SystemCommand::ActivityDetected)
                .map_err(|_| "Failed to send System Activity command"),
            CliCommand::Crash => {
                panic!("Simulated crash dump flow");
            }
            CliCommand::McuTemp => {
                if let Some(ts_raw) = self.temp_sensor_ptr {
                    let ts = unsafe { &mut *ts_raw };
                    match ts.read_temperature_milli_c() {
                        Ok(temp) => {
                            let _ = core::writeln!(
                                writer,
                                "\r\nDirect system temperature reading (RP2040): {}.{:03} C",
                                temp / 1000,
                                (temp.abs() % 1000)
                            );
                            Ok(())
                        }
                        _ => Err("Direct system temperature reading failed"),
                    }
                } else {
                    Err("RP2040 system temperature sensor not available")
                }
            }
            CliCommand::CalNear { direction } => {
                if let Some(i2c_raw) = self.i2c_ptr {
                    let i2c = unsafe { &mut *i2c_raw };
                    let (addr, name) = match direction {
                        app::SensorDirection::North => (0x30, "North"),
                        app::SensorDirection::East => (0x31, "East"),
                        app::SensorDirection::West => (0x32, "West"),
                    };

                    let d_raw = {
                        let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                        sensor.read_distance_mm().unwrap_or(1000)
                    };

                    let _ = core::writeln!(
                        writer,
                        "\r\nCalibrating cover (near) for {} sensor: Raw distance = {} mm",
                        name,
                        d_raw
                    );

                    if let Some(flash_raw) = self.flash_ptr {
                        let flash_ref = unsafe { &mut *flash_raw };
                        let async_flash =
                            firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                        let mut fs = controller::filesystem_controller::FilesystemController::new(
                            async_flash,
                            self.storage_start..self.storage_end,
                        );

                        let mut buf = [0u8; 128];
                        let mut proximity_cal = match embassy_futures::block_on(
                            fs.read_file("vl53l0x_cal.cbor", &mut buf),
                        ) {
                            Ok(Some(bytes)) => {
                                minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes)
                                    .unwrap_or_default()
                            }
                            _ => model::calibration::Vl53l0xCalibration::default(),
                        };

                        match direction {
                            app::SensorDirection::North => proximity_cal.north_near = d_raw,
                            app::SensorDirection::East => proximity_cal.east_near = d_raw,
                            app::SensorDirection::West => proximity_cal.west_near = d_raw,
                        }

                        let mut write_buf = [0u8; 128];
                        let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                        let mut encoder = minicbor::Encoder::new(cursor);
                        encoder.encode(proximity_cal).unwrap();
                        let len = encoder.into_writer().position();

                        match embassy_futures::block_on(
                            fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                        ) {
                            Ok(_) => {
                                let _ = core::writeln!(
                                    writer,
                                    "Saved cover calibration for {} to flash.",
                                    name
                                );
                                Ok(())
                            }
                            Err(_) => Err("Error saving calibration to flash"),
                        }
                    } else {
                        Err("Flash controller not available")
                    }
                } else {
                    Err("I2C controller not available")
                }
            }
            CliCommand::CalFar { direction } => {
                if let Some(i2c_raw) = self.i2c_ptr {
                    let i2c = unsafe { &mut *i2c_raw };
                    let (addr, name) = match direction {
                        app::SensorDirection::North => (0x30, "North"),
                        app::SensorDirection::East => (0x31, "East"),
                        app::SensorDirection::West => (0x32, "West"),
                    };

                    let d_raw = {
                        let mut sensor = peripherals::vl53l0x::Vl53l0x::new(i2c, addr);
                        sensor.read_distance_mm().unwrap_or(1000)
                    };

                    let _ = core::writeln!(
                        writer,
                        "\r\nCalibrating 100mm (far) for {} sensor: Raw distance = {} mm",
                        name,
                        d_raw
                    );

                    if let Some(flash_raw) = self.flash_ptr {
                        let flash_ref = unsafe { &mut *flash_raw };
                        let async_flash =
                            firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                        let mut fs = controller::filesystem_controller::FilesystemController::new(
                            async_flash,
                            self.storage_start..self.storage_end,
                        );

                        let mut buf = [0u8; 128];
                        let mut proximity_cal = match embassy_futures::block_on(
                            fs.read_file("vl53l0x_cal.cbor", &mut buf),
                        ) {
                            Ok(Some(bytes)) => {
                                minicbor::decode::<model::calibration::Vl53l0xCalibration>(bytes)
                                    .unwrap_or_default()
                            }
                            _ => model::calibration::Vl53l0xCalibration::default(),
                        };

                        match direction {
                            app::SensorDirection::North => proximity_cal.north_100 = d_raw,
                            app::SensorDirection::East => proximity_cal.east_100 = d_raw,
                            app::SensorDirection::West => proximity_cal.west_100 = d_raw,
                        }

                        let mut write_buf = [0u8; 128];
                        let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                        let mut encoder = minicbor::Encoder::new(cursor);
                        encoder.encode(proximity_cal).unwrap();
                        let len = encoder.into_writer().position();

                        match embassy_futures::block_on(
                            fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                        ) {
                            Ok(_) => {
                                let _ = core::writeln!(
                                    writer,
                                    "Saved 100mm calibration for {} to flash.",
                                    name
                                );
                                Ok(())
                            }
                            Err(_) => Err("Error saving calibration to flash"),
                        }
                    } else {
                        Err("Flash controller not available")
                    }
                } else {
                    Err("I2C controller not available")
                }
            }
            CliCommand::CalMotor { state } => {
                if let Some(motor_raw) = self.motor_ptr {
                    let motor = unsafe { &mut *motor_raw };
                    let _ = core::writeln!(writer, "\r\nStarting motor for calibration...");
                    let _ = motor.set_speed(100);

                    let _ = core::writeln!(writer, "Waiting 1 second for motor to ramp up...");
                    embassy_futures::block_on(embassy_time::Timer::after(
                        embassy_time::Duration::from_millis(1000),
                    ));

                    if let Some(i2c_raw) = self.i2c_ptr {
                        let i2c = unsafe { &mut *i2c_raw };
                        let mut current_sensor = peripherals::ina219::Ina219::new(i2c);
                        if let Err(e) = current_sensor.init() {
                            let _ = core::writeln!(
                                writer,
                                "Warning: Failed to initialize INA219: {:?}",
                                e
                            );
                        }

                        let mut sum = 0;
                        for _ in 0..5 {
                            sum += current_sensor.read_current_ma().unwrap_or(0);
                            embassy_futures::block_on(embassy_time::Timer::after(
                                embassy_time::Duration::from_millis(100),
                            ));
                        }
                        let current = sum / 5;

                        let name = match state {
                            app::MotorCalState::Empty => "Empty",
                            app::MotorCalState::Water100ml => "100ml",
                            app::MotorCalState::Full => "Full",
                        };

                        let _ = core::writeln!(
                            writer,
                            "Stopping motor and recording measured current for {} state: {} mA",
                            name,
                            current
                        );
                        let _ = motor.stop();

                        if let Some(flash_raw) = self.flash_ptr {
                            let flash_ref = unsafe { &mut *flash_raw };
                            let async_flash =
                                firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                            let mut fs =
                                controller::filesystem_controller::FilesystemController::new(
                                    async_flash,
                                    self.storage_start..self.storage_end,
                                );

                            let mut buf = [0u8; 128];
                            let mut cal = match embassy_futures::block_on(
                                fs.read_file("motor_cal.cbor", &mut buf),
                            ) {
                                Ok(Some(bytes)) => {
                                    minicbor::decode::<model::calibration::MotorCalibration>(bytes)
                                        .unwrap_or_default()
                                }
                                _ => model::calibration::MotorCalibration::default(),
                            };

                            match state {
                                app::MotorCalState::Empty => cal.empty_current_ma = current,
                                app::MotorCalState::Water100ml => {
                                    cal.water_100ml_current_ma = current
                                }
                                app::MotorCalState::Full => cal.full_current_ma = current,
                            }

                            let mut write_buf = [0u8; 128];
                            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                            let mut encoder = minicbor::Encoder::new(cursor);
                            encoder.encode(cal).unwrap();
                            let len = encoder.into_writer().position();

                            match embassy_futures::block_on(
                                fs.write_file("motor_cal.cbor", &write_buf[..len]),
                            ) {
                                Ok(_) => {
                                    let _ = core::writeln!(
                                        writer,
                                        "Saved motor {} calibration to flash.",
                                        name
                                    );
                                    Ok(())
                                }
                                Err(_) => Err("Error saving calibration to flash"),
                            }
                        } else {
                            Err("Flash controller not available")
                        }
                    } else {
                        Err("I2C controller not available")
                    }
                } else {
                    Err("Motor controller not available")
                }
            }
        };

        match res {
            Ok(()) => {
                let _ = core::writeln!(writer, "Command succeeded");
            }
            Err(err) => {
                let _ = core::writeln!(writer, "Command failed: {}", err);
            }
        }
        Ok(())
    }
}
