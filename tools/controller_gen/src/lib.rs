//! Library module for the controller code generator.
//!
//! Exposes parsing, metadata representation, and templating engines for both the CLI
//! binary and integration test suites.

use rinja::Template;
use serde::Deserialize;
use std::path::PathBuf;

/// A single parameter for the controller's task/run functions.
pub struct Param {
    /// Name of the parameter.
    pub name: String,
    /// Type signature of the parameter.
    pub r#type: String,
}

/// Metadata configuration for a controller parsed from `controllers.toml`.
#[derive(Deserialize, Clone)]
pub struct Controller {
    /// Name of the controller (e.g. "Led").
    pub name: String,
    /// Message type associated with the controller's channel.
    pub msg_type: String,
    /// Parameters used by the controller's macro.
    pub macro_params: Vec<String>,
    /// Arguments passed to the task call inside the macro.
    pub task_call_args: Vec<String>,
    /// Full type path of the controller struct.
    pub controller_type: String,
    /// Arguments passed to the spawn single helper macro.
    pub spawn_call_args: Vec<String>,
    /// Extra receivers passed during spawn.
    pub spawn_extra_rxs: Option<Vec<String>>,
    /// Generics used in the task macro declaration.
    pub macro_generics: Option<Vec<String>>,
    /// Generics used during single controller spawn.
    pub spawn_generics: Option<Vec<String>>,
    /// Any linker attributes applied to the task (e.g., link_section).
    pub attributes: Option<Vec<String>>,

    /// Whether this controller has a telemetry sender parameter. Defaults to true.
    pub has_telemetry: Option<bool>,
    /// Overrides the queue channel buffer capacity for the receiver.
    pub receiver_capacity: Option<String>,
    /// Flag indicating if this is the System controller loop.
    pub is_system: Option<bool>,
    /// Generic implementation parameters (e.g. `<D>`).
    pub impl_generics: String,
    /// Full generic type representation inside the impl block header.
    pub impl_type: String,
    /// Types that need to be held in PhantomData for dummy struct mock definitions.
    pub impl_phantom: Option<String>,
}

impl Controller {
    /// Gets a slice of the macro parameters.
    pub fn macro_params_slice(&self) -> &[String] {
        &self.macro_params
    }

    /// Gets a slice of the task call arguments.
    pub fn task_call_args_slice(&self) -> &[String] {
        &self.task_call_args
    }

    /// Gets a slice of the single spawn call arguments.
    pub fn spawn_call_args_slice(&self) -> &[String] {
        &self.spawn_call_args
    }

    /// Gets a slice of any extra receiver arguments needed for spawning.
    pub fn spawn_extra_rxs_slice(&self) -> &[String] {
        self.spawn_extra_rxs.as_deref().unwrap_or(&[])
    }

    /// Gets a slice of macro generic types.
    pub fn macro_generics_slice(&self) -> &[String] {
        self.macro_generics.as_deref().unwrap_or(&[])
    }

    /// Gets a slice of spawn generic types.
    pub fn spawn_generics_slice(&self) -> &[String] {
        self.spawn_generics.as_deref().unwrap_or(&[])
    }

    /// Gets a slice of attributes to apply to the generated controller tasks.
    pub fn attributes_slice(&self) -> &[String] {
        self.attributes.as_deref().unwrap_or(&[])
    }

    /// Programmatically infers the extra receiver and telemetry parameters for this controller.
    pub fn extra_params_inferred(&self) -> Vec<Param> {
        let mut params = Vec::new();

        let cap = self.receiver_capacity.as_deref().unwrap_or("4");
        params.push(Param {
            name: "r".to_string(),
            r#type: format!(
                "$crate::{}Receiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, {}>",
                self.name, cap
            ),
        });

        if self.is_system.unwrap_or(false) {
            params.push(Param {
                name: "r2".to_string(),
                r#type: "platform::gesture_detector::GestureReceiver<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>".to_string(),
            });
            params.push(Param {
                name: "r3".to_string(),
                r#type: "embassy_sync::channel::Receiver<'static, embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $crate::ThermalUpdateAction, 4>".to_string(),
            });
        }

        if self.has_telemetry.unwrap_or(true) {
            params.push(Param {
                name: "t".to_string(),
                r#type: "$crate::TelemetrySender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, { $crate::telemetry_controller::CHANNEL_CAPACITY }>".to_string(),
            });
        }

        params
    }

    /// Helper to get a clean string representation of the phantom generics.
    pub fn impl_phantom_str(&self) -> &str {
        self.impl_phantom.as_deref().unwrap_or("")
    }
}

/// Root TOML configuration mapping holding the list of all defined controllers.
#[derive(Deserialize)]
pub struct ControllerConfig {
    /// List of all configured controllers.
    pub controllers: Vec<Controller>,
}

/// Rinja template for generated macro and channel definitions.
#[derive(Template)]
#[template(path = "generated_controllers.rs.jinja")]
pub struct GeneratedControllersTemplate {
    /// The list of controllers to render.
    pub controllers: Vec<Controller>,
}

/// Rinja template for the boilerplate implementation of the asynchronous `run(...)` loop.
#[derive(Template)]
#[template(path = "run_loop.rs.jinja")]
pub struct RunLoopTemplate {
    /// Name of the controller.
    pub name: String,
    /// Message type associated with the command receiver.
    pub msg_type: String,
    /// Flag indicating if telemetry sender is present.
    pub has_telemetry: bool,
    /// Flag indicating if system-specific gesture/thermal channels are present.
    pub is_system: bool,
    /// Struct-level implementation generic parameters.
    pub impl_generics: String,
    /// Full type declaration of the controller in the impl block.
    pub impl_type: String,
    /// Type to be held in PhantomData inside the generated struct boilerplate.
    pub impl_phantom: String,
}

/// Searches upward from the current directory to locate the path of `controllers.toml`.
pub fn find_controllers_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/controllers.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("controllers.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!(
                "Could not locate controllers.toml in current directory or any parent directories!"
            );
        }
    }
}

/// Prints usage help instructions and lists the available controller options.
pub fn print_help(available: &[Controller]) {
    println!("Controller Code Generator host tool");
    println!();
    println!("Usage:");
    println!("  cargo run -p controller_gen -- [ControllerName]");
    println!("  cargo run -p controller_gen -- list");
    println!();
    println!("Options:");
    println!("  -h, --help      Show this help message");
    println!("  -l, --list      List all defined controllers");
    println!();
    println!("Available controllers in controllers.toml:");
    for ctrl in available {
        println!("  - {}", ctrl.name);
    }
}
