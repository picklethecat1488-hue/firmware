//! Shell controller for routing interactive bringup CLI commands.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use crate::{
    BlockingBatteryReader, BlockingMotorReader, BlockingMotorWriter, BlockingProximityReader,
    BlockingThermalReader,
};
use embassy_sync::blocking_mutex::raw::RawMutex;
use model::interfaces::TemperatureSensor;

/// A guard that locks the shared filesystem scratch buffer for exclusive access.
/// Releases the lock when dropped.
pub struct FsBufferGuard<'a> {
    buffer: *mut [u8],
    lock: &'a core::cell::Cell<bool>,
}

unsafe impl<'a> Send for FsBufferGuard<'a> {}
unsafe impl<'a> Sync for FsBufferGuard<'a> {}

impl<'a> core::ops::Deref for FsBufferGuard<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.buffer }
    }
}

impl<'a> core::ops::DerefMut for FsBufferGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.buffer }
    }
}

impl<'a> Drop for FsBufferGuard<'a> {
    fn drop(&mut self) {
        self.lock.set(false);
    }
}

impl<'a> FsBufferGuard<'a> {
    /// Retrieve the underlying static buffer reference.
    ///
    /// # Safety
    /// The caller must ensure that the returned static reference is not stored
    /// or used after this guard is dropped.
    pub unsafe fn as_static_mut(&mut self) -> &'static mut [u8] {
        &mut *self.buffer
    }
}

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

/// Helper macro to extract a type by name from a list of key-value pairs, or fallback to a default type.
#[macro_export]
macro_rules! get_key_or_default {
    (I2c, [ I2c = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (Motor, [ Motor = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (Flash, [ Flash = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (TempSensor, [ TempSensor = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (BatteryCtrl, [ BatteryCtrl = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (ThermalCtrl, [ ThermalCtrl = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (SensorCtrl, [ SensorCtrl = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (MotorCtrl, [ MotorCtrl = $val:ty, $($rest:tt)* ], $default:ty) => { $val };
    (SystemCtrl, [ SystemCtrl = $val:ty, $($rest:tt)* ], $default:ty) => { $val };

    // Fallthrough: discard non-matching head and recurse
    ($key:ident, [ $other:ident = $val:ty, $($rest:tt)* ], $default:ty) => {
        $crate::get_key_or_default!($key, [ $($rest)* ], $default)
    };

    // Base case: key not found
    ($key:ident, [], $default:ty) => { $default };
}

/// Macro to implement the `ShellConfig` trait for a custom configuration struct.
///
/// Permits specifying the associated types in any order, defaulting unspecified ones to `()`.
#[macro_export]
macro_rules! impl_shell_config {
    (
        $name:ty {
            MutexRaw: $mutex:ty,
            $($key:ident = $val:ty),* $(,)?
        }
    ) => {
        impl $crate::shell_controller::ShellConfig for $name {
            type MutexRaw = $mutex;
            type I2c = $crate::get_key_or_default!(I2c, [ $($key = $val,)* ], ());
            type Motor = $crate::get_key_or_default!(Motor, [ $($key = $val,)* ], ());
            type Flash = $crate::get_key_or_default!(Flash, [ $($key = $val,)* ], ());
            type TempSensor = $crate::get_key_or_default!(TempSensor, [ $($key = $val,)* ], ());
            type BatteryCtrl = $crate::get_key_or_default!(BatteryCtrl, [ $($key = $val,)* ], ());
            type ThermalCtrl = $crate::get_key_or_default!(ThermalCtrl, [ $($key = $val,)* ], ());
            type SensorCtrl = $crate::get_key_or_default!(SensorCtrl, [ $($key = $val,)* ], ());
            type MotorCtrl = $crate::get_key_or_default!(MotorCtrl, [ $($key = $val,)* ], ());
            type SystemCtrl = $crate::get_key_or_default!(SystemCtrl, [ $($key = $val,)* ], ());
        }
    };
}

/// Macro to define `ShellControllerPointers`, `ShellController`, and the `ShellDeviceResolver` trait.
///
/// This serves as the single source of truth for all shell metadata required to support subcommands.
macro_rules! define_shell_resolver_and_controller {
    (
        $(
            #[doc = $doc:expr]
            $associated_type:ident, $field:ident, $resolve_fn:ident
        ),* $(,)?
    ) => {
        /// Controller responsible for processing shell commands.
        /// Context pointers to drivers and controllers for direct diagnostics.
        pub struct ShellControllerPointers<'a, C: ShellConfig> {
            $(
                #[doc = $doc]
                pub $field: &'a [crate::NamedDevice<C::$associated_type>],
            )*
            /// Named flash storage partitions.
            pub flash_partitions: &'a [crate::NamedPartition<C::Flash>],
            /// Shared filesystem scratch buffer.
            pub fs_buffer: &'a mut [u8],
        }

        impl<'a, C: ShellConfig> Default for ShellControllerPointers<'a, C> {
            fn default() -> Self {
                Self {
                    $( $field: &[], )*
                    flash_partitions: &[],
                    fs_buffer: &mut [],
                }
            }
        }

        /// Trait to resolve devices and partitions for CLI handlers.
        #[allow(clippy::mut_from_ref)]
        pub trait ShellDeviceResolver<C: ShellConfig> {
            $(
                #[doc = $doc]
                fn $resolve_fn(&self, name: Option<&str>) -> Result<&mut C::$associated_type, &'static str>;
            )*
            /// Resolves the flash partition.
            fn resolve_partition(
                &self,
                name: Option<&str>,
            ) -> Result<crate::FlashPartition<C::Flash>, &'static str>;
            /// Lock the shared filesystem scratch buffer for exclusive access.
            fn lock_fs_buffer(&self) -> Result<crate::shell_controller::FsBufferGuard<'_>, &'static str>;
        }

        /// Controller responsible for processing shell commands.
        pub struct ShellController<'a, C: ShellConfig> {
            $( $field: &'a [crate::NamedDevice<C::$associated_type>], )*
            flash_partitions: &'a [crate::NamedPartition<C::Flash>],
            fs_buffer: *mut [u8],
            fs_buffer_locked: core::cell::Cell<bool>,
        }

        // Implement Send and Sync manually since ShellController contains raw pointers
        unsafe impl<'a, C: ShellConfig> Send for ShellController<'a, C> {}
        unsafe impl<'a, C: ShellConfig> Sync for ShellController<'a, C> {}

        impl<'a, C: ShellConfig> ShellController<'a, C> {
            /// Creates a new ShellController.
            pub fn new(pointers: ShellControllerPointers<'a, C>) -> Self {
                Self {
                    $( $field: pointers.$field, )*
                    flash_partitions: pointers.flash_partitions,
                    fs_buffer: pointers.fs_buffer as *mut [u8],
                    fs_buffer_locked: core::cell::Cell::new(false),
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

        impl<'a, C: ShellConfig> ShellDeviceResolver<C> for ShellController<'a, C> {
            $(
                fn $resolve_fn(&self, name: Option<&str>) -> Result<&mut C::$associated_type, &'static str> {
                    self.resolve_device(self.$field, name)
                }
            )*
            fn resolve_partition(
                &self,
                name: Option<&str>,
            ) -> Result<crate::FlashPartition<C::Flash>, &'static str> {
                self.resolve_partition(name)
            }
            fn lock_fs_buffer(&self) -> Result<crate::shell_controller::FsBufferGuard<'_>, &'static str> {
                if self.fs_buffer_locked.get() {
                    return Err("Filesystem scratch buffer is already locked");
                }
                if unsafe { (&*self.fs_buffer).is_empty() } {
                    return Err("Filesystem scratch buffer is not configured");
                }
                self.fs_buffer_locked.set(true);
                Ok(crate::shell_controller::FsBufferGuard {
                    buffer: self.fs_buffer,
                    lock: &self.fs_buffer_locked,
                })
            }
        }
    };
}

define_shell_resolver_and_controller! {
    #[doc = "Named I2C buses."]
    I2c, i2c_buses, resolve_i2c,

    #[doc = "Named motor drivers."]
    Motor, motors, resolve_motor,

    #[doc = "Named battery gauges."]
    BatteryCtrl, batteries, resolve_battery,

    #[doc = "Named thermal sensors."]
    ThermalCtrl, thermals, resolve_thermal,

    #[doc = "Named sensor controllers."]
    SensorCtrl, sensors, resolve_sensor,

    #[doc = "Named motor current controllers."]
    MotorCtrl, motor_ctrls, resolve_motor_ctrl,

    #[doc = "Named microcontroller temperature sensors."]
    TempSensor, temp_sensors, resolve_temp_sensor,

    #[doc = "Named system controllers."]
    SystemCtrl, system_ctrls, resolve_system_ctrl,
}

/// Helper macro to append a specific command group's variant and match arm to the accumulator.
///
/// ### Wildcard Forwarding & Custom Command Processors
///
/// In modular firmware designs, different projects (app crates) want to extend the interactive CLI
/// console with their own custom, project-specific command sets (e.g. `cat_detector` might add a `dispense`
/// or `status` command) while still reusing the shared controller diagnostic commands (`motor`, `system`, `fs`, etc.).
///
/// To support this without modifying the generic `ShellController` codebase, `declare_shell_commands!`
/// supports generating a **wrapper processor** struct (e.g. `CatDetectorCliProcessor`).
///
/// 1. **Custom enum with a Wildcard**:
///    The application defines a custom command enum (e.g., `AppCli`) that includes a catch-all wildcard variant:
///    ```rust
///    #[derive(embedded_cli::Command)]
///    pub enum AppCli<'a> {
///        Dispense,
///        // Catch all other commands to forward them
///        #[command(wildcard)]
///        Other(embedded_cli::command::RawCommand<'a>),
///    }
///    ```
///
/// 2. **Custom Processor Delegating via Wildcard Forwarding**:
///    The application then implements `CommandProcessor` for its own processor, intercepting its custom variants,
///    and forwarding the raw command in the `Other` variant directly to the wrapper processor:
///    ```rust
///    impl<'a, 'b, W, E> CommandProcessor<W, E> for AppProcessor<'a, 'b> {
///        fn process(&mut self, cli: &mut CliHandle<W, E>, raw: RawCommand) -> Result<(), ProcessError<E>> {
///            match AppCli::parse(raw) {
///                Ok(AppCli::Dispense) => { self.handle_dispense(cli) }
///                Ok(AppCli::Other(raw_subcmd)) => {
///                    // Forward unhandled commands to the controller's wrapper processor
///                    self.wrapper_processor.process(cli, raw_subcmd)
///                }
///                Err(err) => Err(err)
///            }
///        }
///    }
///    ```
/// This design keeps the controllers completely decoupled from the specific applications while allowing
/// infinite CLI customizability and code reuse.
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
            $name::Battery { subcommand } => $crate::battery_controller::handle_battery_cli($ctrl, subcommand, $writer),
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
            $name::Thermal { subcommand } => $crate::thermal_controller::handle_thermal_cli($ctrl, subcommand, $writer),
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
            $name::Motor { subcommand, arg1, arg2, arg3 } => $crate::motor_controller::handle_motor_cli($ctrl, subcommand, arg1, arg2, arg3, $writer),
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
            $name::Sensor { subcommand, arg1 } => $crate::sensor_controller::handle_sensor_cli($ctrl, subcommand, arg1, $writer),
        ] -> $mode, $proc_name);
    };
    (Fs, $name:ident, $ctrl:ident, $writer:ident, [$($tail:ident),*], [$($variants:tt)*], [$($matches:tt)*] -> $mode:tt, $proc_name:ident) => {
        $crate::declare_shell_commands!(@accum $name, $ctrl, $writer, [$($tail),*] -> [
            $($variants)*
            /// Filesystem commands (fs format, fs ls)
            #[command(name = "fs")]
            Fs {
                /// Subcommand (format, ls)
                subcommand: Option<&'a str>,
            },
        ] [
            $($matches)*
            $name::Fs { subcommand } => $crate::filesystem_controller::handle_fs_cli($ctrl, subcommand, $writer),
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
            $name::System { subcommand } => $crate::system_controller::handle_system_cli($ctrl, subcommand, $writer),
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
