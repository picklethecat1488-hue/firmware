//! Shell controller for routing interactive bringup CLI commands.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use crate::battery_controller::{process_battery_command, BatteryCliCommand};
use crate::filesystem_controller::{process_filesystem_command, FilesystemCliCommand};
use crate::motor_controller::{
    process_motor_command, MotorCalState, MotorCliCommand, MotorCommand,
};
use crate::sensor_controller::{process_sensor_command, SensorCliCommand, SensorDirection};
use crate::system_controller::{process_system_command, SystemCliCommand, SystemCommand};
use crate::thermal_controller::{process_thermal_command, ThermalCliCommand};
use crate::{
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
use model::interfaces::TemperatureSensor;

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

/// The top-level combined CLI command set.
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum ShellCliCommand {
    /// Query battery voltage and status
    Battery,
    /// Query thermal sensor and status
    Thermal,
    /// Read the MCU system temperature
    #[command(name = "mcu_temp")]
    McuTemp,
    /// Motor speed control (motor <speed>)
    #[command(name = "motor")]
    Motor {
        /// Target speed percentage (-100 to 100)
        speed: i8,
    },
    /// Stop the motor
    Stop,
    /// Calibrate motor current levels (cal_motor <empty|100ml|full|overload> [max_rpm] [rpm_limit])
    #[command(name = "cal_motor")]
    CalMotor {
        /// Calibration state ('empty', '100ml', 'full', or 'overload')
        state: MotorCalState,
        /// Optional physical maximum RPM at 100% duty cycle
        max_rpm: Option<u32>,
        /// Optional maximum RPM safety limit to configure
        rpm_limit: Option<u32>,
    },
    /// Query proximity (ToF) sensors
    Proximity,
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
    /// Format/erase the filesystem partition
    Format,
    /// Simulate activity event
    Activity,
    /// Trigger a panic to test the crash dump / panic flow
    Crash,
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
                    let _ = <ShellCliCommand as embedded_cli::service::Help>::list_commands(writer);
                }
                embedded_cli::help::HelpRequest::Command(subcommand) => {
                    let mut parent = |_writer: &mut embedded_cli::writer::Writer<'_, W, E>| Ok(());
                    if let Err(embedded_cli::service::HelpError::UnknownCommand) =
                        <ShellCliCommand as embedded_cli::service::Help>::command_help(
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

        let cmd = <ShellCliCommand as embedded_cli::service::FromRaw>::parse(raw)?;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "ShellController: received command {:?}",
            defmt::Debug2Format(&cmd)
        );

        let res: Result<(), &'static str> = match cmd {
            ShellCliCommand::Battery => {
                if let Some(ctrl) = self.battery_ctrl_ptr {
                    process_battery_command(unsafe { &*ctrl }, writer, BatteryCliCommand::Battery)
                } else {
                    Err("Battery controller not available")
                }
            }
            ShellCliCommand::Thermal => {
                if let Some(ctrl) = self.thermal_ctrl_ptr {
                    process_thermal_command(
                        unsafe { &*ctrl },
                        None::<&mut C::TempSensor>,
                        writer,
                        ThermalCliCommand::Thermal,
                    )
                } else {
                    Err("Thermal controller not available")
                }
            }
            ShellCliCommand::McuTemp => {
                if let Some(ctrl) = self.thermal_ctrl_ptr {
                    let temp_sensor = self.temp_sensor_ptr.map(|p| unsafe { &mut *p });
                    process_thermal_command(
                        unsafe { &*ctrl },
                        temp_sensor,
                        writer,
                        ThermalCliCommand::McuTemp,
                    )
                } else {
                    Err("Thermal controller not available")
                }
            }
            ShellCliCommand::Motor { speed } => {
                if let Some(ctrl) = self.motor_ctrl_ptr {
                    process_motor_command(
                        unsafe { &mut *ctrl },
                        &self.motor_tx,
                        self.motor_ptr,
                        self.i2c_ptr,
                        self.flash_ptr,
                        self.storage_start,
                        self.storage_end,
                        writer,
                        MotorCliCommand::Motor { speed },
                    )
                } else {
                    Err("Motor controller not available")
                }
            }
            ShellCliCommand::Stop => {
                if let Some(ctrl) = self.motor_ctrl_ptr {
                    process_motor_command(
                        unsafe { &mut *ctrl },
                        &self.motor_tx,
                        self.motor_ptr,
                        self.i2c_ptr,
                        self.flash_ptr,
                        self.storage_start,
                        self.storage_end,
                        writer,
                        MotorCliCommand::Stop,
                    )
                } else {
                    Err("Motor controller not available")
                }
            }
            ShellCliCommand::CalMotor {
                state,
                max_rpm,
                rpm_limit,
            } => {
                if let Some(ctrl) = self.motor_ctrl_ptr {
                    process_motor_command(
                        unsafe { &mut *ctrl },
                        &self.motor_tx,
                        self.motor_ptr,
                        self.i2c_ptr,
                        self.flash_ptr,
                        self.storage_start,
                        self.storage_end,
                        writer,
                        MotorCliCommand::CalMotor {
                            state,
                            max_rpm,
                            rpm_limit,
                        },
                    )
                } else {
                    Err("Motor controller not available")
                }
            }
            ShellCliCommand::Proximity => process_sensor_command(
                self.sensor_north_ctrl_ptr,
                self.sensor_east_ctrl_ptr,
                self.sensor_west_ctrl_ptr,
                self.i2c_ptr,
                self.flash_ptr,
                self.storage_start,
                self.storage_end,
                writer,
                SensorCliCommand::Proximity,
            ),
            ShellCliCommand::CalNear { direction } => process_sensor_command(
                self.sensor_north_ctrl_ptr,
                self.sensor_east_ctrl_ptr,
                self.sensor_west_ctrl_ptr,
                self.i2c_ptr,
                self.flash_ptr,
                self.storage_start,
                self.storage_end,
                writer,
                SensorCliCommand::CalNear { direction },
            ),
            ShellCliCommand::CalFar { direction } => process_sensor_command(
                self.sensor_north_ctrl_ptr,
                self.sensor_east_ctrl_ptr,
                self.sensor_west_ctrl_ptr,
                self.i2c_ptr,
                self.flash_ptr,
                self.storage_start,
                self.storage_end,
                writer,
                SensorCliCommand::CalFar { direction },
            ),
            ShellCliCommand::Format => process_filesystem_command(
                self.flash_ptr,
                self.storage_start,
                self.storage_end,
                writer,
                FilesystemCliCommand::Format,
            ),
            ShellCliCommand::Activity => {
                process_system_command(&self.system_tx, writer, SystemCliCommand::Activity)
            }
            ShellCliCommand::Crash => {
                process_system_command(&self.system_tx, writer, SystemCliCommand::Crash)
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
