//! Shell controller for routing interactive bringup CLI commands.

#![deny(missing_docs)]
#![allow(static_mut_refs)]

use platform::FsBufferGuard;

pub use crate::ShellConfig;

/// Macro to define `ShellControllerPointers`, `ShellController`, and the `ShellDeviceResolver` trait.
///
/// This serves as the single source of truth for all shell metadata required to support subcommands.
#[macro_export]
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
                pub $field: &'a [$crate::NamedDevice<C::$associated_type>],
            )*
            /// Named flash storage partitions.
            pub flash_partitions: &'a [$crate::NamedPartition<C::Flash>],
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

                /// Get the list of all registered named devices for this type.
                fn $field(&self) -> &[$crate::NamedDevice<C::$associated_type>];
            )*
            /// Resolves the flash partition.
            fn resolve_partition(
                &self,
                name: Option<&str>,
            ) -> Result<$crate::ResolvedPartition<C::Flash>, &'static str>;
            /// Lock the shared filesystem scratch buffer for exclusive access.
            fn lock_fs_buffer(&self) -> Result<FsBufferGuard<'_>, &'static str>;
            /// Trigger a panic/crash on a specific core.
            fn trigger_core_panic(&self, core_id: u32) -> Result<(), &'static str>;
        }

        /// Controller responsible for processing shell commands.
        pub struct ShellController<'a, C: ShellConfig> {
            $( $field: &'a [$crate::NamedDevice<C::$associated_type>], )*
            flash_partitions: &'a [$crate::NamedPartition<C::Flash>],
            fs_buffer: *mut [u8],
            fs_buffer_locked: core::cell::Cell<bool>,
            /// Deferred pending async command
            pub pending_command: Option<$crate::sensor_controller::PendingCommand>,
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
                    pending_command: None,
                }
            }

            /// Resolves a named device from a slice of NamedDevice entries.
            /// If no name is provided, it defaults to the first available device.
            #[allow(clippy::mut_from_ref)]
            pub fn resolve_device<'b, D>(
                &self,
                devices: &'b [$crate::NamedDevice<D>],
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
            ) -> Result<$crate::ResolvedPartition<C::Flash>, &'static str> {
                let matched = match name {
                    Some(n) => self.flash_partitions.iter().find(|p| p.name == n),
                    None => self.flash_partitions.first(),
                };
                let p = matched.ok_or("Requested flash partition not found or none registered")?;
                match p.kind {
                    $crate::PartitionKind::Map => Ok($crate::ResolvedPartition::Map(
                        $crate::MapFilesystem(p.partition.start_address..p.partition.end_address),
                        p.partition.flash_ptr,
                    )),
                    $crate::PartitionKind::Queue => Ok($crate::ResolvedPartition::Queue(
                        $crate::QueueFilesystem(p.partition.start_address..p.partition.end_address),
                        p.partition.flash_ptr,
                    )),
                }
            }

            /// Parses and registers a pending async sensor command.
            pub fn set_pending_sensor(
                &mut self,
                subcommand: $crate::sensor_controller::SensorSubcommand,
                arg1: Option<&str>,
                partition: Option<&str>,
            ) -> Result<(), &'static str> {
                let cmd = $crate::sensor_controller::PendingCommand::parse(subcommand, arg1, partition)?;
                self.pending_command = Some(cmd);
                Ok(())
            }

            /// Executes any pending async commands.
            pub async fn execute_pending<W, E>(
                &mut self,
                writer: &mut $crate::embedded_cli::writer::Writer<'_, W, E>,
            ) -> Result<(), &'static str>
            where
                W: $crate::embedded_io::Write<Error = E>,
                E: $crate::embedded_io::Error,
            {
                if let Some(pending) = self.pending_command.take() {
                    $crate::sensor_controller::handle_sensor_cli(self, pending, writer).await?;
                }
                Ok(())
            }
        }

        impl<'a, C: ShellConfig> platform::i2c::I2cResolver for ShellController<'a, C> {
            type I2c = C::I2c;
            #[allow(clippy::mut_from_ref)]
            fn resolve_i2c(&self, name: Option<&str>) -> Result<&mut Self::I2c, &'static str> {
                self.resolve_device(self.i2c_buses, name)
            }
        }

        impl<'a, C: ShellConfig> ShellDeviceResolver<C> for ShellController<'a, C> {
            $(
                fn $resolve_fn(&self, name: Option<&str>) -> Result<&mut C::$associated_type, &'static str> {
                    self.resolve_device(self.$field, name)
                }

                fn $field(&self) -> &[$crate::NamedDevice<C::$associated_type>] {
                    self.$field
                }
            )*
            fn resolve_partition(
                &self,
                name: Option<&str>,
            ) -> Result<$crate::ResolvedPartition<C::Flash>, &'static str> {
                self.resolve_partition(name)
            }
            fn lock_fs_buffer(&self) -> Result<FsBufferGuard<'_>, &'static str> {
                if self.fs_buffer_locked.get() {
                    return Err("Filesystem scratch buffer is already locked");
                }
                if unsafe { (&*self.fs_buffer).is_empty() } {
                    return Err("Filesystem scratch buffer is not configured");
                }
                self.fs_buffer_locked.set(true);
                Ok(unsafe {
                    FsBufferGuard::new(self.fs_buffer, &self.fs_buffer_locked)
                })
            }
            fn trigger_core_panic(&self, core_id: u32) -> Result<(), &'static str> {
                C::trigger_core_panic(self, core_id)
            }
        }
    };
}

invoke_define_shell_resolver_and_controller!();

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
///    ```rust,ignore
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
///    ```rust,ignore
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
            /// Reference to the underlying shell controller.
            pub controller: &'b mut $crate::shell_controller::ShellController<'a, C>,
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
