//! Shell controller for processing interactive bringup CLI commands.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use crate as app;
use app::system_controller::SystemCommand;
use controller::motor_controller::MotorCommand;
use controller::{
    BlockingBatteryReader, BlockingMotorReader, BlockingMotorWriter, BlockingProximityReader,
    BlockingThermalReader,
};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use embedded_cli::cli::CliHandle;
use embedded_cli::command::RawCommand;
use embedded_cli::service::CommandProcessor;
use embedded_io::Write as IoWrite;

/// Represents the physical directions of ToF proximity sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorDirection {
    /// North sensor
    North,
    /// East sensor
    East,
    /// West sensor
    West,
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for SensorDirection {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "north" => Ok(SensorDirection::North),
            "east" => Ok(SensorDirection::East),
            "west" => Ok(SensorDirection::West),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'north', 'east', or 'west'",
            }),
        }
    }
}

/// Derived command enum representing all supported user commands.
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum CliCommand {
    /// Motor speed control (motor <speed>)
    #[command(name = "motor")]
    Motor {
        /// Target speed percentage (-100 to 100)
        speed: i8,
    },
    /// Stop the motor
    Stop,
    /// Query battery voltage and status
    Battery,
    /// Query thermal sensor and status
    Thermal,
    /// Query proximity (ToF) sensors
    Proximity,

    /// Simulate activity event
    Activity,
    /// Trigger a panic to test the crash dump / panic flow
    Crash,
    /// Calibrate ToF sensors with target held at the cover (0mm)
    #[command(name = "cal_near")]
    CalNear {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
    /// Calibrate ToF sensors with target held at 100mm
    #[command(name = "cal_far")]
    CalFar {
        /// Sensor direction ('north', 'east', or 'west')
        direction: SensorDirection,
    },
    /// Calibrate motor current levels (cal_motor <empty|100ml|full> [max_rpm] [rpm_limit])
    #[command(name = "cal_motor")]
    CalMotor {
        /// Calibration state ('empty', '100ml', or 'full')
        state: MotorCalState,
        /// Optional physical maximum RPM at 100% duty cycle
        max_rpm: Option<u32>,
        /// Optional maximum RPM safety limit to configure
        rpm_limit: Option<u32>,
    },
    /// Read the RP2040 system temperature
    #[command(name = "mcu_temp")]
    McuTemp,
    /// Format/erase the filesystem partition
    Format,
}

/// Represents the motor calibration target state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorCalState {
    /// Empty water bowl
    Empty,
    /// Bowl with 100ml of water
    Water100ml,
    /// Full water bowl
    Full,
}

impl<'a> embedded_cli::arguments::FromArgument<'a> for MotorCalState {
    fn from_arg(arg: &'a str) -> Result<Self, embedded_cli::arguments::FromArgumentError<'a>> {
        match arg {
            "empty" => Ok(MotorCalState::Empty),
            "100ml" => Ok(MotorCalState::Water100ml),
            "full" => Ok(MotorCalState::Full),
            _ => Err(embedded_cli::arguments::FromArgumentError {
                value: arg,
                expected: "one of 'empty', '100ml', or 'full'",
            }),
        }
    }
}
use model::interfaces::{Motor, PowerSensor, ProximitySensor, TemperatureSensor};
use model::types::MotorSpeed;

/// Controller responsible for processing shell commands.
/// Context pointers to drivers and controllers for direct diagnostics.
/// Configuration trait for the ShellController.
/// Encapsulates the target-specific raw mutex and peripheral types.
pub trait ShellConfig {
    /// Type of the raw mutex for task channels.
    type MutexRaw: RawMutex + 'static;
    /// Type of the shared I2C bus driver.
    type I2c: embedded_hal::i2c::I2c + 'static;
    /// Type of the physical motor peripheral.
    type Motor: model::interfaces::Motor + 'static;
    /// Type of the physical flash peripheral.
    type Flash: embedded_storage::nor_flash::NorFlash + 'static;
    /// Type of the battery controller.
    type BatteryCtrl: BlockingBatteryReader + 'static;
    /// Type of the thermal controller.
    type ThermalCtrl: BlockingThermalReader + 'static;
    /// Type of the proximity sensor controller.
    type SensorCtrl: BlockingProximityReader + 'static;
    /// Type of the motor controller.
    type MotorCtrl: BlockingMotorReader + BlockingMotorWriter + 'static;
    /// Type of the system temperature sensor.
    type TempSensor: TemperatureSensor + 'static;
}

/// A generic implementation of ShellConfig that enables type inference.
#[allow(clippy::type_complexity)]
pub struct ShellConfigImpl<
    MutexRaw,
    I2c,
    Motor,
    Flash,
    BatteryCtrl,
    ThermalCtrl,
    SensorCtrl,
    MotorCtrl,
    TempSensor,
>(
    core::marker::PhantomData<(
        MutexRaw,
        I2c,
        Motor,
        Flash,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        TempSensor,
    )>,
);

impl<
        MutexRaw: RawMutex + 'static,
        I2c: embedded_hal::i2c::I2c + 'static,
        Motor: model::interfaces::Motor + 'static,
        Flash: embedded_storage::nor_flash::NorFlash + 'static,
        BatteryCtrl: BlockingBatteryReader + 'static,
        ThermalCtrl: BlockingThermalReader + 'static,
        SensorCtrl: BlockingProximityReader + 'static,
        MotorCtrl: BlockingMotorReader + BlockingMotorWriter + 'static,
        TempSensor: TemperatureSensor + 'static,
    > ShellConfig
    for ShellConfigImpl<
        MutexRaw,
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
    type MutexRaw = MutexRaw;
    type I2c = I2c;
    type Motor = Motor;
    type Flash = Flash;
    type BatteryCtrl = BatteryCtrl;
    type ThermalCtrl = ThermalCtrl;
    type SensorCtrl = SensorCtrl;
    type MotorCtrl = MotorCtrl;
    type TempSensor = TempSensor;
}

/// Controller responsible for processing shell commands.
/// Context pointers to drivers and controllers for direct diagnostics.
pub struct ShellControllerPointers<C: ShellConfig> {
    /// Pointer to shared I2C bus driver
    pub i2c_ptr: Option<*mut C::I2c>,
    /// Pointer to physical motor peripheral
    pub motor_ptr: Option<*mut C::Motor>,
    /// Pointer to physical flash peripheral
    pub flash_ptr: Option<*mut C::Flash>,
    /// Pointer to battery controller
    pub battery_ctrl_ptr: Option<*mut C::BatteryCtrl>,
    /// Pointer to thermal controller
    pub thermal_ctrl_ptr: Option<*mut C::ThermalCtrl>,
    /// Pointer to North proximity sensor controller
    pub sensor_north_ctrl_ptr: Option<*mut C::SensorCtrl>,
    /// Pointer to East proximity sensor controller
    pub sensor_east_ctrl_ptr: Option<*mut C::SensorCtrl>,
    /// Pointer to West proximity sensor controller
    pub sensor_west_ctrl_ptr: Option<*mut C::SensorCtrl>,
    /// Pointer to motor current controller
    pub motor_ctrl_ptr: Option<*mut C::MotorCtrl>,
    /// Pointer to microcontroller temperature sensor
    pub temp_sensor_ptr: Option<*mut C::TempSensor>,
}

/// Controller responsible for processing shell commands.
pub struct ShellController<C: ShellConfig, const N: usize> {
    motor_tx: Sender<'static, C::MutexRaw, MotorCommand, N>,
    system_tx: Sender<'static, C::MutexRaw, SystemCommand, N>,
    i2c_ptr: Option<*mut C::I2c>,
    motor_ptr: Option<*mut C::Motor>,
    flash_ptr: Option<*mut C::Flash>,
    battery_ctrl_ptr: Option<*mut C::BatteryCtrl>,
    thermal_ctrl_ptr: Option<*mut C::ThermalCtrl>,
    sensor_north_ctrl_ptr: Option<*mut C::SensorCtrl>,
    sensor_east_ctrl_ptr: Option<*mut C::SensorCtrl>,
    sensor_west_ctrl_ptr: Option<*mut C::SensorCtrl>,
    motor_ctrl_ptr: Option<*mut C::MotorCtrl>,
    temp_sensor_ptr: Option<*mut C::TempSensor>,
    storage_start: u32,
    storage_end: u32,
}

// Implement Send and Sync manually since ShellController contains raw pointers
unsafe impl<C: ShellConfig, const N: usize> Send for ShellController<C, N> {}
unsafe impl<C: ShellConfig, const N: usize> Sync for ShellController<C, N> {}

impl<C: ShellConfig, const N: usize> ShellController<C, N> {
    /// Creates a new ShellController.
    pub fn new(
        motor_tx: Sender<'static, C::MutexRaw, MotorCommand, N>,
        system_tx: Sender<'static, C::MutexRaw, SystemCommand, N>,
        pointers: ShellControllerPointers<C>,
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

impl<C: ShellConfig, const N: usize, W: IoWrite<Error = E>, E: embedded_io::Error>
    CommandProcessor<W, E> for ShellController<C, N>
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

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "ShellController: received command {:?}",
            defmt::Debug2Format(&cmd)
        );

        let res: Result<(), &'static str> = match cmd {
            CliCommand::Motor { speed } => self
                .motor_ctrl_ptr
                .ok_or("Motor controller not available")
                .and_then(|p| {
                    let ctrl = unsafe { &mut *p };
                    ctrl.set_motor_speed(speed)
                        .map_err(|_| "Failed to set motor speed")?;
                    ctrl.read_current_ma_blocking()
                        .map_err(|_| "Failed to read motor current")
                })
                .map(|current| {
                    let _ = core::writeln!(writer, "\r\nMotor current: {} mA", current);
                }),
            CliCommand::Stop => self
                .motor_tx
                .try_send(MotorCommand::Stop)
                .map_err(|_| "Failed to send Motor Stop command"),
            CliCommand::Battery => self
                .battery_ctrl_ptr
                .ok_or("Battery controller not available")
                .and_then(|ctrl_raw| {
                    let ctrl = unsafe { &*ctrl_raw };
                    ctrl.read_battery_blocking()
                        .map_err(|_| "Direct battery reading failed")
                })
                .map(|(v, soc)| {
                    let _ = core::writeln!(
                        writer,
                        "\r\nDirect battery reading: {} mV, {}% state of charge",
                        v,
                        soc
                    );
                }),
            CliCommand::Thermal => self
                .thermal_ctrl_ptr
                .ok_or("Thermal controller not available")
                .and_then(|ctrl_raw| {
                    let ctrl = unsafe { &*ctrl_raw };
                    ctrl.read_temperature_blocking()
                        .map_err(|_| "Direct thermal reading failed")
                })
                .map(|temp| {
                    let _ = core::writeln!(
                        writer,
                        "\r\nDirect thermal reading (ThermalController): {}.{:03} C",
                        temp / 1000,
                        (temp.abs() % 1000)
                    );
                }),
            CliCommand::Proximity => {
                let read_sensor = |ptr_opt: Option<*mut C::SensorCtrl>| {
                    ptr_opt
                        .ok_or("Proximity sensor pointer not available")
                        .and_then(|p| {
                            unsafe { &mut *p }
                                .read_distance_blocking()
                                .map_err(|_| "Proximity sensor failed to read")
                        })
                };
                read_sensor(self.sensor_north_ctrl_ptr)
                    .and_then(|dn| {
                        read_sensor(self.sensor_east_ctrl_ptr).map(|de| (dn, de))
                    })
                    .and_then(|(dn, de)| {
                        read_sensor(self.sensor_west_ctrl_ptr).map(|dw| (dn, de, dw))
                    })
                    .map(|(dn, de, dw)| {
                        let _ = core::writeln!(
                            writer,
                            "\r\nDirect proximity readings: North = {} mm, East = {} mm, West = {} mm",
                            dn,
                            de,
                            dw
                        );
                    })
                    .map_err(|_| "One or more proximity sensors failed to read")
            }

            CliCommand::Activity => self
                .system_tx
                .try_send(SystemCommand::ActivityDetected)
                .map_err(|_| "Failed to send System Activity command"),
            CliCommand::Crash => {
                panic!("Simulated crash dump flow");
            }
            CliCommand::McuTemp => self
                .temp_sensor_ptr
                .ok_or("RP2040 system temperature sensor not available")
                .and_then(|ts_raw| {
                    unsafe { &mut *ts_raw }
                        .read_temperature_milli_c()
                        .map_err(|_| "Direct system temperature reading failed")
                })
                .map(|temp| {
                    let _ = core::writeln!(
                        writer,
                        "\r\nDirect system temperature reading (RP2040): {}.{:03} C",
                        temp / 1000,
                        (temp.abs() % 1000)
                    );
                }),
            CliCommand::Format => {
                if let Some(flash_raw) = self.flash_ptr {
                    static mut SHELL_FS_BUF_3: [u8; 2048] = [0u8; 2048];
                    let flash_ref = unsafe { &mut *flash_raw };
                    let async_flash = firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                    let fs_buf = unsafe { &mut SHELL_FS_BUF_3 };
                    let mut fs = controller::filesystem_controller::FilesystemController::new(
                        async_flash,
                        self.storage_start..self.storage_end,
                        fs_buf,
                    );

                    let _ = core::writeln!(writer, "\r\nFormatting filesystem...");
                    let res = embassy_futures::block_on(fs.format());
                    match res {
                        Ok(()) => {
                            let _ = core::writeln!(
                                writer,
                                "Formatting successful! Rebooting target system..."
                            );
                            #[cfg(all(target_arch = "arm", target_os = "none"))]
                            cortex_m::peripheral::SCB::sys_reset();
                            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                            Ok(())
                        }
                        Err(()) => Err("Formatting failed!"),
                    }
                } else {
                    Err("Flash peripheral not available")
                }
            }
            CliCommand::CalNear { direction } => {
                if let Some(i2c_raw) = self.i2c_ptr {
                    let i2c = unsafe { &mut *i2c_raw };
                    let (addr, name) = match direction {
                        SensorDirection::North => (0x30, "North"),
                        SensorDirection::East => (0x31, "East"),
                        SensorDirection::West => (0x32, "West"),
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
                        static mut SHELL_FS_BUF_1: [u8; 2048] = [0u8; 2048];
                        let flash_ref = unsafe { &mut *flash_raw };
                        let async_flash =
                            firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                        let fs_buf = unsafe { &mut SHELL_FS_BUF_1 };
                        let mut fs = controller::filesystem_controller::FilesystemController::new(
                            async_flash,
                            self.storage_start..self.storage_end,
                            fs_buf,
                        );

                        let mut buf = [0u8; 128];
                        let mut proximity_cal =
                            embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                                .ok()
                                .flatten()
                                .and_then(|bytes| {
                                    minicbor::decode::<model::calibration::Vl53l0xCalibration>(
                                        bytes,
                                    )
                                    .ok()
                                })
                                .unwrap_or_default();

                        match direction {
                            SensorDirection::North => proximity_cal.north_near = d_raw,
                            SensorDirection::East => proximity_cal.east_near = d_raw,
                            SensorDirection::West => proximity_cal.west_near = d_raw,
                        }

                        let mut write_buf = [0u8; 128];
                        let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                        let mut encoder = minicbor::Encoder::new(cursor);
                        encoder.encode(proximity_cal).unwrap();
                        let len = encoder.into_writer().position();

                        embassy_futures::block_on(
                            fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                        )
                        .map(|_| {
                            let _ = core::writeln!(
                                writer,
                                "Saved cover calibration for {} to flash.",
                                name
                            );
                        })
                        .map_err(|_| "Error saving calibration to flash")
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
                        SensorDirection::North => (0x30, "North"),
                        SensorDirection::East => (0x31, "East"),
                        SensorDirection::West => (0x32, "West"),
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
                        static mut SHELL_FS_BUF_2: [u8; 2048] = [0u8; 2048];
                        let flash_ref = unsafe { &mut *flash_raw };
                        let async_flash =
                            firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                        let fs_buf = unsafe { &mut SHELL_FS_BUF_2 };
                        let mut fs = controller::filesystem_controller::FilesystemController::new(
                            async_flash,
                            self.storage_start..self.storage_end,
                            fs_buf,
                        );

                        let mut buf = [0u8; 128];
                        let mut proximity_cal =
                            embassy_futures::block_on(fs.read_file("vl53l0x_cal.cbor", &mut buf))
                                .ok()
                                .flatten()
                                .and_then(|bytes| {
                                    minicbor::decode::<model::calibration::Vl53l0xCalibration>(
                                        bytes,
                                    )
                                    .ok()
                                })
                                .unwrap_or_default();

                        match direction {
                            SensorDirection::North => proximity_cal.north_100 = d_raw,
                            SensorDirection::East => proximity_cal.east_100 = d_raw,
                            SensorDirection::West => proximity_cal.west_100 = d_raw,
                        }

                        let mut write_buf = [0u8; 128];
                        let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                        let mut encoder = minicbor::Encoder::new(cursor);
                        encoder.encode(proximity_cal).unwrap();
                        let len = encoder.into_writer().position();

                        embassy_futures::block_on(
                            fs.write_file("vl53l0x_cal.cbor", &write_buf[..len]),
                        )
                        .map(|_| {
                            let _ = core::writeln!(
                                writer,
                                "Saved 100mm calibration for {} to flash.",
                                name
                            );
                        })
                        .map_err(|_| "Error saving calibration to flash")
                    } else {
                        Err("Flash controller not available")
                    }
                } else {
                    Err("I2C controller not available")
                }
            }
            CliCommand::CalMotor {
                state,
                max_rpm,
                rpm_limit,
            } => {
                if let Some(motor_raw) = self.motor_ptr {
                    let motor = unsafe { &mut *motor_raw };
                    let _ = core::writeln!(writer, "\r\nStarting motor for calibration...");
                    let _ = motor.set_speed(MotorSpeed::MAX);

                    let _ = core::writeln!(writer, "Waiting 1 second for motor to ramp up...");
                    embassy_time::block_for(embassy_time::Duration::from_millis(1000));

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
                            embassy_time::block_for(embassy_time::Duration::from_millis(100));
                        }
                        let current = sum / 5;

                        let name = match state {
                            MotorCalState::Empty => "Empty",
                            MotorCalState::Water100ml => "100ml",
                            MotorCalState::Full => "Full",
                        };

                        let _ = core::writeln!(
                            writer,
                            "Stopping motor and recording measured current for {} state: {} mA",
                            name,
                            current
                        );
                        let _ = motor.stop();

                        if let Some(flash_raw) = self.flash_ptr {
                            static mut SHELL_FS_BUF_3: [u8; 2048] = [0u8; 2048];
                            let flash_ref = unsafe { &mut *flash_raw };
                            let async_flash =
                                firmware_lib::panic_handler::BlockingAsyncFlash(flash_ref);
                            let fs_buf = unsafe { &mut SHELL_FS_BUF_3 };
                            let mut fs =
                                controller::filesystem_controller::FilesystemController::new(
                                    async_flash,
                                    self.storage_start..self.storage_end,
                                    fs_buf,
                                );

                            let mut buf = [0u8; 128];
                            let cal =
                                embassy_futures::block_on(fs.read_file("motor_cal.cbor", &mut buf))
                                    .ok()
                                    .flatten()
                                    .and_then(|bytes| {
                                        minicbor::decode::<model::calibration::MotorCalibration>(
                                            bytes,
                                        )
                                        .ok()
                                    })
                                    .unwrap_or_default();

                            let mut cal = cal;
                            match state {
                                MotorCalState::Empty => cal.empty_current_ma = current,
                                MotorCalState::Water100ml => cal.water_100ml_current_ma = current,
                                MotorCalState::Full => cal.full_current_ma = current,
                            }
                            if let Some(max_rpm_val) = max_rpm {
                                cal.max_rpm = Some(max_rpm_val);
                                let _ = core::writeln!(
                                    writer,
                                    "Configuring physical maximum RPM to {} RPM.",
                                    max_rpm_val
                                );
                            }
                            if let Some(limit_val) = rpm_limit {
                                cal.rpm_limit = Some(limit_val);
                                let _ = core::writeln!(
                                    writer,
                                    "Configuring maximum RPM safety limit to {} RPM.",
                                    limit_val
                                );
                            }

                            let mut write_buf = [0u8; 128];
                            let cursor = minicbor::encode::write::Cursor::new(&mut write_buf[..]);
                            let mut encoder = minicbor::Encoder::new(cursor);
                            encoder.encode(cal).unwrap();
                            let len = encoder.into_writer().position();

                            embassy_futures::block_on(
                                fs.write_file("motor_cal.cbor", &write_buf[..len]),
                            )
                            .map(|_| {
                                let _ = core::writeln!(
                                    writer,
                                    "Saved motor {} calibration to flash.",
                                    name
                                );
                            })
                            .map_err(|_| "Error saving calibration to flash")
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
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!("ShellController: command execution succeeded");
            }
            Err(err) => {
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("ShellController: command execution failed: {}", err);
                let _ = core::writeln!(writer, "Command failed: {}", err);
            }
        }
        Ok(())
    }
}
