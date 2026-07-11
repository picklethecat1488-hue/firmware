//! Shell controller for routing interactive bringup CLI commands.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use crate::battery_controller::process_battery_command;
use crate::filesystem_controller::process_filesystem_command;
use crate::motor_controller::process_motor_command;
use crate::sensor_controller::process_sensor_command;
use crate::system_controller::process_system_command;
use crate::thermal_controller::process_thermal_command;
use crate::{
    BlockingBatteryReader, BlockingMotorReader, BlockingMotorWriter, BlockingProximityReader,
    BlockingThermalReader,
};
use embassy_sync::blocking_mutex::raw::RawMutex;
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
    /// Type of the system orchestrator/writer.
    type SystemCtrl: crate::BlockingSystemWriter + 'static;
}

/// A generic implementation of ShellConfig that enables type inference.
#[allow(clippy::type_complexity)]
pub struct ShellConfigImpl<
    MutexRaw,
    I2c = crate::DummyI2c,
    Motor = crate::DummyMotor,
    Flash = crate::DummyFlash,
    TempSensor = crate::DummyTempSensor,
    BatteryCtrl = (),
    ThermalCtrl = (),
    SensorCtrl = (),
    MotorCtrl = (),
    SystemCtrl = (),
>(
    core::marker::PhantomData<(
        MutexRaw,
        I2c,
        Motor,
        Flash,
        TempSensor,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        SystemCtrl,
    )>,
);

impl<
        MutexRaw: RawMutex + 'static,
        I2c: embedded_hal::i2c::I2c + 'static,
        Motor: model::interfaces::Motor + 'static,
        Flash: embedded_storage::nor_flash::NorFlash + 'static,
        TempSensor: TemperatureSensor + 'static,
        BatteryCtrl: BlockingBatteryReader + 'static,
        ThermalCtrl: BlockingThermalReader + 'static,
        SensorCtrl: BlockingProximityReader + 'static,
        MotorCtrl: BlockingMotorReader + BlockingMotorWriter + 'static,
        SystemCtrl: crate::BlockingSystemWriter + 'static,
    > ShellConfig
    for ShellConfigImpl<
        MutexRaw,
        I2c,
        Motor,
        Flash,
        TempSensor,
        BatteryCtrl,
        ThermalCtrl,
        SensorCtrl,
        MotorCtrl,
        SystemCtrl,
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
    type SystemCtrl = SystemCtrl;
}

/// Controller responsible for processing shell commands.
/// Context pointers to drivers and controllers for direct diagnostics.
pub struct ShellControllerPointers<'a, C: ShellConfig> {
    /// Named I2C buses.
    pub i2c_buses: &'a [crate::NamedDevice<C::I2c>],
    /// Named motor drivers.
    pub motors: &'a [crate::NamedDevice<C::Motor>],
    /// Named flash storage partitions.
    pub flash_partitions: &'a [crate::NamedPartition<C::Flash>],
    /// Named battery gauges.
    pub batteries: &'a [crate::NamedDevice<C::BatteryCtrl>],
    /// Named thermal sensors.
    pub thermals: &'a [crate::NamedDevice<C::ThermalCtrl>],
    /// Named sensor controllers.
    pub sensors: &'a [crate::NamedDevice<C::SensorCtrl>],
    /// Named motor current controllers.
    pub motor_ctrls: &'a [crate::NamedDevice<C::MotorCtrl>],
    /// Named microcontroller temperature sensors.
    pub temp_sensors: &'a [crate::NamedDevice<C::TempSensor>],
    /// Named system controllers.
    pub system_ctrls: &'a [crate::NamedDevice<C::SystemCtrl>],
}

impl<'a, C: ShellConfig> Default for ShellControllerPointers<'a, C> {
    fn default() -> Self {
        Self {
            i2c_buses: &[],
            motors: &[],
            flash_partitions: &[],
            batteries: &[],
            thermals: &[],
            sensors: &[],
            motor_ctrls: &[],
            temp_sensors: &[],
            system_ctrls: &[],
        }
    }
}

/// Controller responsible for processing shell commands.
pub struct ShellController<'a, C: ShellConfig> {
    i2c_buses: &'a [crate::NamedDevice<C::I2c>],
    motors: &'a [crate::NamedDevice<C::Motor>],
    flash_partitions: &'a [crate::NamedPartition<C::Flash>],
    batteries: &'a [crate::NamedDevice<C::BatteryCtrl>],
    thermals: &'a [crate::NamedDevice<C::ThermalCtrl>],
    sensors: &'a [crate::NamedDevice<C::SensorCtrl>],
    motor_ctrls: &'a [crate::NamedDevice<C::MotorCtrl>],
    temp_sensors: &'a [crate::NamedDevice<C::TempSensor>],
    system_ctrls: &'a [crate::NamedDevice<C::SystemCtrl>],
}

// Implement Send and Sync manually since ShellController contains raw pointers
unsafe impl<'a, C: ShellConfig> Send for ShellController<'a, C> {}
unsafe impl<'a, C: ShellConfig> Sync for ShellController<'a, C> {}

impl<'a, C: ShellConfig> ShellController<'a, C> {
    /// Creates a new ShellController.
    pub fn new(pointers: ShellControllerPointers<'a, C>) -> Self {
        Self {
            i2c_buses: pointers.i2c_buses,
            motors: pointers.motors,
            flash_partitions: pointers.flash_partitions,
            batteries: pointers.batteries,
            thermals: pointers.thermals,
            sensors: pointers.sensors,
            motor_ctrls: pointers.motor_ctrls,
            temp_sensors: pointers.temp_sensors,
            system_ctrls: pointers.system_ctrls,
        }
    }

    /// Resolves a named device from a slice of NamedDevice entries.
    /// If no name is provided, it defaults to the first available device.
    #[allow(clippy::mut_from_ref)]
    pub fn resolve_device<'b, D>(
        &self,
        devices: &'b [crate::NamedDevice<D>],
        name: Option<&str>,
    ) -> Result<&'b mut D, &'static str> {
        let matched = match name {
            Some(n) => devices.iter().find(|d| d.name == n),
            None => devices.first(),
        };
        matched
            .map(|d| unsafe { &mut *d.device })
            .ok_or("Requested device not found or none registered")
    }

    /// Resolves a named partition from a slice of NamedPartition entries.
    /// If no name is provided, it defaults to the first available partition.
    pub fn resolve_partition(
        &self,
        name: Option<&str>,
    ) -> Result<crate::FlashPartition<C::Flash>, &'static str> {
        let matched = match name {
            Some(n) => self.flash_partitions.iter().find(|p| p.name == n),
            None => self.flash_partitions.first(),
        };
        matched
            .map(|p| p.partition)
            .ok_or("Requested flash partition not found or none registered")
    }
}

/// Helper macro to append a specific command group's variant and match arm to the accumulator.
#[macro_export]
macro_rules! append_group_arm {
    (Battery, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Battery commands (battery status)
            #[command(name = "battery")]
            Battery {
                /// Subcommand (status)
                subcommand: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Battery { subcommand } => $ctrl.handle_battery(subcommand, $writer),
        ] -> $mode, $proc_name);
    };
    (Thermal, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Thermal commands (thermal status, thermal mcu)
            #[command(name = "thermal")]
            Thermal {
                /// Subcommand (status, mcu)
                subcommand: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Thermal { subcommand } => $ctrl.handle_thermal(subcommand, $writer),
        ] -> $mode, $proc_name);
    };
    (Motor, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Motor commands (motor speed <speed>, motor stop, motor calibrate <state> [max_rpm] [rpm_limit])
            #[command(name = "motor")]
            Motor {
                /// Subcommand (speed, stop, calibrate)
                subcommand: Option<&'a str>,
                /// First argument (speed or calibration state)
                arg1: Option<&'a str>,
                /// Second argument (max_rpm)
                arg2: Option<&'a str>,
                /// Third argument (rpm_limit)
                arg3: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Motor { subcommand, arg1, arg2, arg3 } => $ctrl.handle_motor(subcommand, arg1, arg2, arg3, $writer),
        ] -> $mode, $proc_name);
    };
    (Sensor, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Sensor commands (sensor status, sensor cal_near <dir>, sensor cal_far <dir>)
            #[command(name = "sensor")]
            Sensor {
                /// Subcommand (status, cal_near, cal_far)
                subcommand: Option<&'a str>,
                /// First argument (direction: north, east, west)
                arg1: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Sensor { subcommand, arg1 } => $ctrl.handle_sensor(subcommand, arg1, $writer),
        ] -> $mode, $proc_name);
    };
    (Fs, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Filesystem commands (fs format)
            #[command(name = "fs")]
            Fs {
                /// Subcommand (format)
                subcommand: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Fs { subcommand } => $ctrl.handle_fs(subcommand, $writer),
        ] -> $mode, $proc_name);
    };
    (System, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// System commands (system activity, system crash)
            #[command(name = "system")]
            System {
                /// Subcommand (activity, crash)
                subcommand: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::System { subcommand } => $ctrl.handle_system(subcommand, $writer),
        ] -> $mode, $proc_name);
    };
}

/// Macro to emit shell commands processor directly on ShellController.
#[macro_export]
macro_rules! emit_direct_commands {
    ($name:ident, $proc_name:ident, $ctrl:ident, $writer:ident, [$($variants:tt)*], [$($matches:tt)*]) => {
        /// Generated combined CLI command set.
        #[derive(Debug, $crate::embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
        pub enum $name<'a> {
            $($variants)*
        }

        impl<'a, 'b, C: $crate::shell_controller::ShellConfig, W: $crate::embedded_io::Write<Error = E>, E: $crate::embedded_io::Error>
            $crate::embedded_cli::service::CommandProcessor<W, E> for $crate::shell_controller::ShellController<'a, C>
        {
            fn process<'c>(
                &mut self,
                cli: &mut $crate::embedded_cli::cli::CliHandle<'_, W, E>,
                raw: $crate::embedded_cli::command::RawCommand<'c>,
            ) -> Result<(), $crate::embedded_cli::service::ProcessError<'c, E>> {
                use core::fmt::Write as _;
                let $ctrl = self;
                let $writer = cli.writer();

                // Intercept help commands
                if let Some(help_req) = $crate::embedded_cli::help::HelpRequest::from_command(&raw) {
                    match help_req {
                        $crate::embedded_cli::help::HelpRequest::All => {
                            let _ = <$name<'_> as $crate::embedded_cli::service::Help>::list_commands($writer);
                        }
                        $crate::embedded_cli::help::HelpRequest::Command(subcommand) => {
                            let mut parent = |_writer: &mut $crate::embedded_cli::writer::Writer<'_, W, E>| Ok(());
                            if let Err($crate::embedded_cli::service::HelpError::UnknownCommand) =
                                <$name<'_> as $crate::embedded_cli::service::Help>::command_help(
                                    &mut parent,
                                    subcommand,
                                    $writer,
                                )
                            {
                                  let _ = core::writeln!($writer, "\r\nUnknown command");
                            }
                        }
                    }
                    return Ok(());
                }

                let cmd = <$name<'c> as $crate::embedded_cli::service::FromRaw<'c>>::parse(raw)?;

                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!(
                    "received command {:?}",
                    defmt::Debug2Format(&cmd)
                );

                let res = match cmd {
                    $($matches)*
                };

                match res {
                    Ok(()) => {
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        defmt::info!("command execution succeeded");
                    }
                    Err(err) => {
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        defmt::error!("command execution failed: {}", err);
                        let _ = core::writeln!($writer, "Command failed: {}", err);
                    }
                }
                Ok(())
            }
        }
    };
}

/// Macro to emit shell commands processor via a wrapper struct.
#[macro_export]
macro_rules! emit_wrapper_commands {
    ($name:ident, $proc_name:ident, $ctrl:ident, $writer:ident, [$($variants:tt)*], [$($matches:tt)*]) => {
        /// Generated combined CLI command set.
        #[derive(Debug, $crate::embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
        pub enum $name<'a> {
            $($variants)*
        }

        /// Generated wrapper processor.
        pub struct $proc_name<'a, 'b, C: $crate::shell_controller::ShellConfig> {
            controller: &'b mut $crate::shell_controller::ShellController<'a, C>,
        }

        impl<'a, 'b, C: $crate::shell_controller::ShellConfig> $proc_name<'a, 'b, C> {
            /// Create a new processor wrapper.
            pub fn new(controller: &'b mut $crate::shell_controller::ShellController<'a, C>) -> Self {
                Self { controller }
            }
        }

        impl<'a, 'b, 'c, C: $crate::shell_controller::ShellConfig, W: $crate::embedded_io::Write<Error = E>, E: $crate::embedded_io::Error>
            $crate::embedded_cli::service::CommandProcessor<W, E> for $proc_name<'a, 'b, C>
        {
            fn process<'d>(
                &mut self,
                cli: &mut $crate::embedded_cli::cli::CliHandle<'_, W, E>,
                raw: $crate::embedded_cli::command::RawCommand<'d>,
            ) -> Result<(), $crate::embedded_cli::service::ProcessError<'d, E>> {
                use core::fmt::Write as _;
                let $ctrl = &mut *self.controller;
                let $writer = cli.writer();

                // Intercept help commands
                if let Some(help_req) = $crate::embedded_cli::help::HelpRequest::from_command(&raw) {
                    match help_req {
                        $crate::embedded_cli::help::HelpRequest::All => {
                            let _ = <$name<'_> as $crate::embedded_cli::service::Help>::list_commands($writer);
                        }
                        $crate::embedded_cli::help::HelpRequest::Command(subcommand) => {
                            let mut parent = |_writer: &mut $crate::embedded_cli::writer::Writer<'_, W, E>| Ok(());
                            if let Err($crate::embedded_cli::service::HelpError::UnknownCommand) =
                                <$name<'_> as $crate::embedded_cli::service::Help>::command_help(
                                    &mut parent,
                                    subcommand,
                                    $writer,
                                )
                            {
                                  let _ = core::writeln!($writer, "\r\nUnknown command");
                            }
                        }
                    }
                    return Ok(());
                }

                let cmd = <$name<'d> as $crate::embedded_cli::service::FromRaw<'d>>::parse(raw)?;

                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!(
                    "received command {:?}",
                    defmt::Debug2Format(&cmd)
                );

                let res = match cmd {
                    $($matches)*
                };

                match res {
                    Ok(()) => {
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        defmt::info!("command execution succeeded");
                    }
                    Err(err) => {
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        defmt::error!("command execution failed: {}", err);
                        let _ = core::writeln!($writer, "Command failed: {}", err);
                    }
                }
                Ok(())
            }
        }
    };
}

/// Macro to declare a shell command set and automatically implement CommandProcessor for it.
#[macro_export]
macro_rules! declare_shell_commands {
    // Direct entrypoint (for DefaultShellCli)
    (
        @direct
        $name:ident {
            $($group:ident),* $(,)?
        }
    ) => {
        $crate::declare_shell_commands!(@accum $name, ctrl, writer, [$($group),*] -> [] [] -> direct, DummyProc);
    };

    // Wrapper entrypoint (for custom commands in app crates)
    (
        $name:ident ($proc_name:ident) {
            $($group:ident),* $(,)?
        }
    ) => {
        $crate::declare_shell_commands!(@accum $name, ctrl, writer, [$($group),*] -> [] [] -> wrapper, $proc_name);
    };

    // Accumulate variants and matches
    (@accum $name:ident, $ctrl:ident, $writer:ident, [$head:ident $(, $tail:ident)* $(,)?] -> [$($variants:tt)*] [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::append_group_arm!($head, $name, $ctrl, $writer, [$($tail),*], [$($variants)*], [$($matches)*] -> $mode, $proc_name);
    };

    // Base case: dispatch to the callback macro to emit the structures and processor
    (@accum $name:ident, $ctrl:ident, $writer:ident, [] -> [$($variants:tt)*] [$($matches:tt)*] -> direct, $proc_name:ident) => {
        $crate::emit_direct_commands!($name, $proc_name, $ctrl, $writer, [$($variants)*], [$($matches)*]);
    };
    (@accum $name:ident, $ctrl:ident, $writer:ident, [] -> [$($variants:tt)*] [$($matches:tt)*] -> wrapper, $proc_name:ident) => {
        $crate::emit_wrapper_commands!($name, $proc_name, $ctrl, $writer, [$($variants)*], [$($matches)*]);
    };
}

crate::declare_shell_commands! {
    @direct
    DefaultShellCli {
        Battery,
        Thermal,
        Motor,
        Sensor,
        Fs,
        System,
    }
}

impl<'a, C: ShellConfig> ShellController<'a, C> {
    /// Processes battery-specific CLI subcommands.
    pub fn handle_battery<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("status") => {
                let ctrl = self.resolve_device(self.batteries, None)?;
                process_battery_command(
                    ctrl,
                    writer,
                    crate::battery_controller::BatteryCliCommand::Status,
                )
            }
            _ => Err("Invalid battery subcommand. Expected: status"),
        }
    }

    /// Processes thermal-specific CLI subcommands.
    pub fn handle_thermal<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("status") => {
                let ctrl = self.resolve_device(self.thermals, None)?;
                let temp_sensor = self.resolve_device(self.temp_sensors, None).ok();
                process_thermal_command(
                    ctrl,
                    temp_sensor,
                    writer,
                    crate::thermal_controller::ThermalCliCommand::Status,
                )
            }
            Some("mcu") => {
                let ctrl = self.resolve_device(self.thermals, None)?;
                let temp_sensor = self.resolve_device(self.temp_sensors, None).ok();
                process_thermal_command(
                    ctrl,
                    temp_sensor,
                    writer,
                    crate::thermal_controller::ThermalCliCommand::Mcu,
                )
            }
            _ => Err("Invalid thermal subcommand. Expected: status, mcu"),
        }
    }

    /// Processes motor-specific CLI subcommands.
    pub fn handle_motor<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        arg1: Option<&str>,
        arg2: Option<&str>,
        arg3: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("speed") => {
                let speed_str = arg1.ok_or("Missing speed parameter")?;
                let speed = speed_str
                    .parse::<i8>()
                    .map_err(|_| "Invalid speed parameter")?;
                let ctrl = self.resolve_device(self.motor_ctrls, None)?;
                let motor = self
                    .resolve_device(self.motors, None)
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_motor_command(
                    ctrl,
                    motor,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::motor_controller::MotorCliCommand::Speed { speed },
                )
            }
            Some("stop") => {
                let ctrl = self.resolve_device(self.motor_ctrls, None)?;
                let motor = self
                    .resolve_device(self.motors, None)
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_motor_command(
                    ctrl,
                    motor,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::motor_controller::MotorCliCommand::Stop,
                )
            }
            Some("calibrate") => {
                let state_str = arg1.ok_or("Missing calibration state")?;
                let state = match state_str {
                    "empty" => crate::motor_controller::MotorCalState::Empty,
                    "water_100ml" => crate::motor_controller::MotorCalState::Water100ml,
                    "full" => crate::motor_controller::MotorCalState::Full,
                    "overload" => crate::motor_controller::MotorCalState::Overload,
                    _ => return Err(
                        "Invalid calibration state. Expected: empty, water_100ml, full, overload",
                    ),
                };
                let max_rpm = arg2.and_then(|s| s.parse::<u32>().ok());
                let rpm_limit = arg3.and_then(|s| s.parse::<u32>().ok());
                let ctrl = self.resolve_device(self.motor_ctrls, None)?;
                let motor = self
                    .resolve_device(self.motors, None)
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_motor_command(
                    ctrl,
                    motor,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::motor_controller::MotorCliCommand::Calibrate {
                        state,
                        max_rpm,
                        rpm_limit,
                    },
                )
            }
            _ => Err("Invalid motor subcommand. Expected: speed, stop, calibrate"),
        }
    }

    /// Processes sensor-specific CLI subcommands.
    pub fn handle_sensor<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        arg1: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("status") => {
                let sensor_north = self
                    .resolve_device(self.sensors, Some("north"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_east = self
                    .resolve_device(self.sensors, Some("east"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_west = self
                    .resolve_device(self.sensors, Some("west"))
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_sensor_command(
                    sensor_north,
                    sensor_east,
                    sensor_west,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::sensor_controller::SensorCliCommand::Status,
                )
            }
            Some("cal_near") => {
                let dir_str = arg1.ok_or("Missing direction parameter")?;
                let direction = match dir_str {
                    "north" => crate::sensor_controller::SensorDirection::North,
                    "east" => crate::sensor_controller::SensorDirection::East,
                    "west" => crate::sensor_controller::SensorDirection::West,
                    _ => return Err("Invalid direction. Expected: north, east, west"),
                };
                let sensor_north = self
                    .resolve_device(self.sensors, Some("north"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_east = self
                    .resolve_device(self.sensors, Some("east"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_west = self
                    .resolve_device(self.sensors, Some("west"))
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_sensor_command(
                    sensor_north,
                    sensor_east,
                    sensor_west,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::sensor_controller::SensorCliCommand::CalNear { direction },
                )
            }
            Some("cal_far") => {
                let dir_str = arg1.ok_or("Missing direction parameter")?;
                let direction = match dir_str {
                    "north" => crate::sensor_controller::SensorDirection::North,
                    "east" => crate::sensor_controller::SensorDirection::East,
                    "west" => crate::sensor_controller::SensorDirection::West,
                    _ => return Err("Invalid direction. Expected: north, east, west"),
                };
                let sensor_north = self
                    .resolve_device(self.sensors, Some("north"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_east = self
                    .resolve_device(self.sensors, Some("east"))
                    .ok()
                    .map(|d| d as *mut _);
                let sensor_west = self
                    .resolve_device(self.sensors, Some("west"))
                    .ok()
                    .map(|d| d as *mut _);
                let i2c = self
                    .resolve_device(self.i2c_buses, None)
                    .ok()
                    .map(|d| d as *mut _);
                let partition = self.resolve_partition(None).ok();
                let (flash, start, end) = match partition {
                    Some(p) => (Some(p.flash_ptr), p.start_address, p.end_address),
                    None => (None, 0, 0),
                };
                process_sensor_command(
                    sensor_north,
                    sensor_east,
                    sensor_west,
                    i2c,
                    flash,
                    start,
                    end,
                    writer,
                    crate::sensor_controller::SensorCliCommand::CalFar { direction },
                )
            }
            _ => Err("Invalid sensor subcommand. Expected: status, cal_near, cal_far"),
        }
    }

    /// Processes filesystem-specific CLI subcommands.
    pub fn handle_fs<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("format") => {
                let partition = self.resolve_partition(None)?;
                process_filesystem_command(
                    Some(partition.flash_ptr),
                    partition.start_address,
                    partition.end_address,
                    writer,
                    crate::filesystem_controller::FilesystemCliCommand::Format,
                )
            }
            _ => Err("Invalid fs subcommand. Expected: format"),
        }
    }

    /// Processes system-specific CLI subcommands.
    pub fn handle_system<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
        &mut self,
        subcommand: Option<&str>,
        writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    ) -> Result<(), &'static str> {
        match subcommand {
            Some("activity") => {
                let system_ctrl = self.resolve_device(self.system_ctrls, None)?;
                process_system_command(
                    system_ctrl,
                    writer,
                    crate::system_controller::SystemCliCommand::Activity,
                )
            }
            Some("crash") => {
                let system_ctrl = self.resolve_device(self.system_ctrls, None)?;
                process_system_command(
                    system_ctrl,
                    writer,
                    crate::system_controller::SystemCliCommand::Crash,
                )
            }
            _ => Err("Invalid system subcommand. Expected: activity, crash"),
        }
    }
}
