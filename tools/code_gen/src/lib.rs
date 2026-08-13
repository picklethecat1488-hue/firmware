//! Library module for the controller code generator.
//!
//! Exposes parsing, metadata representation, and templating engines for both the CLI
//! binary and integration test suites.

use rinja::Template;
use serde::Deserialize;
use std::path::PathBuf;

/// A single parameter for the controller's task/run functions.
#[derive(Deserialize, Clone)]
pub struct Param {
    /// Name of the parameter.
    #[serde(rename = "param")]
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
    /// Explicit parameter types passed to the controller's run method.
    pub run_params: Vec<Param>,
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
    pub fn extra_params_inferred(&self) -> &[Param] {
        &self.run_params
    }

    /// Helper to get a clean string representation of the phantom generics.
    pub fn impl_phantom_str(&self) -> &str {
        self.impl_phantom.as_deref().unwrap_or("")
    }

    /// Gets default capacity suffix for generated controllers.
    pub fn default_capacity_suffix(&self) -> String {
        if let Some(ref cap) = self.receiver_capacity {
            if cap.starts_with('$') {
                " = { crate::telemetry_controller::CHANNEL_CAPACITY }".to_string()
            } else {
                format!(" = {}", cap)
            }
        } else {
            " = 4".to_string()
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct CliResolverField {
    pub associated_type: String,
    pub field: String,
    pub resolve_fn: String,
    pub doc: String,
    pub bounds: String,
    pub type_lifetime: Option<String>,
}

impl CliResolverField {
    pub fn bounds(&self) -> String {
        let lifetime = self.type_lifetime.as_deref().unwrap_or("'static");
        format!("{} + {}", self.bounds, lifetime)
    }
}

#[derive(Deserialize, Clone)]
pub struct CliArg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
    pub help: String,
    pub attributes: Option<Vec<String>>,
}

impl CliArg {
    pub fn attributes_slice(&self) -> Vec<String> {
        if let Some(ref attrs) = self.attributes {
            attrs.clone()
        } else if self.name != "arg1"
            && self.name != "arg2"
            && self.name != "arg3"
            && self.name != "target"
        {
            vec![format!("#[arg(long = \"{}\")]", self.name)]
        } else {
            vec![]
        }
    }

    pub fn rust_type(&self) -> String {
        match self.arg_type.as_str() {
            "string" => "Option<&'a str>".to_string(),
            "int" => "Option<i32>".to_string(),
            "float" => "Option<f32>".to_string(),
            "bool" => "Option<bool>".to_string(),
            custom => resolve_crate_path(custom),
        }
    }

    pub fn rust_type_sample(&self) -> String {
        self.rust_type()
            .replace("$crate::", "controller::")
            .replace("&'a str", "&str")
    }
}

fn resolve_crate_path(custom: &str) -> String {
    let mut result = custom.to_string();
    if let Some(idx) = result.find("_controller::") {
        let bytes = result.as_bytes();
        let mut start = idx;
        while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
            start -= 1;
        }
        if !(start >= 8 && &result[start - 8..start] == "$crate::") {
            result.insert_str(start, "$crate::");
        }
    }
    result
}

#[derive(Deserialize, Clone)]
pub struct CliCommand {
    pub group: String,
    pub cmd_name: String,
    pub variant: String,
    pub subcommand_type: String,
    #[serde(default)]
    pub async_cli: bool,
    pub handler: String,
    pub help: String,
    pub args: Option<Vec<CliArg>>,
}

impl CliCommand {
    pub fn args_slice(&self) -> &[CliArg] {
        self.args.as_deref().unwrap_or(&[])
    }

    pub fn handler_short_name(&self) -> &str {
        self.handler.split("::").last().unwrap()
    }

    pub fn subcommand_type_path(&self) -> String {
        if self.subcommand_type.contains("::") && !self.subcommand_type.starts_with("platform::") {
            format!("controller::{}", self.subcommand_type)
        } else {
            self.subcommand_type.clone()
        }
    }
}

/// Root TOML configuration mapping holding the list of all defined controllers.
#[derive(Deserialize)]
pub struct ControllerConfig {
    /// List of all configured controllers.
    pub controllers: Vec<Controller>,
}

#[derive(Deserialize, Clone)]
pub struct ShellConfigToml {
    #[serde(default)]
    pub cli_resolver_fields: Vec<CliResolverField>,
    #[serde(default)]
    pub cli_commands: Vec<CliCommand>,
}

/// Rinja template for generated macro and channel definitions.
#[derive(Template)]
#[template(path = "generated_controllers.rs.jinja", escape = "none")]
pub struct GeneratedControllersTemplate {
    pub has_async_cli: bool,
    /// The list of controllers to render.
    pub controllers: Vec<Controller>,
    pub cli_resolver_fields: Vec<CliResolverField>,
    pub cli_commands: Vec<CliCommand>,
}

/// Rinja template for rendering a sample CLI implementation.
#[derive(Template)]
#[template(path = "sample_cli.rs.jinja", escape = "none")]
pub struct SampleCliTemplate {
    pub cli_commands: Vec<CliCommand>,
}

/// Rinja template for rendering a single CLI handler function skeleton.
#[derive(Template)]
#[template(path = "cli_handler_skeleton.rs.jinja", escape = "none")]
pub struct CliHandlerSkeletonTemplate {
    pub cmd: CliCommand,
}

/// Rinja template for the boilerplate implementation of the asynchronous `run(...)` loop.
#[derive(Template)]
#[template(path = "run_loop.rs.jinja", escape = "none")]
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

/// Searches upward from the current directory to locate the path of `shell.toml`.
pub fn find_shell_toml() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        let toml_path = path.join("controller/shell.toml");
        if toml_path.exists() {
            return toml_path;
        }
        let direct_toml_path = path.join("shell.toml");
        if direct_toml_path.exists() {
            return direct_toml_path;
        }
        if !path.pop() {
            panic!("Could not locate shell.toml in current directory or any parent directories!");
        }
    }
}

/// Prints usage help instructions.
pub fn print_help() {
    println!("Code Generator host tool");
    println!();
    println!("Usage:");
    println!("  cargo run -p code_gen -- list-controllers");
    println!("  cargo run -p code_gen -- list-clis");
    println!("  cargo run -p code_gen -- cli-sample [<group_or_command>]");
    println!("  cargo run -p code_gen -- runloop-sample [<ControllerName>]");
    println!();
    println!("Options:");
    println!("  -h, --help            Show this help message");
    println!("  list-controllers      List all defined controllers");
    println!("  list-clis             List all defined CLI commands/groups");
    println!("  cli-sample            Output compiling sample CLI implementation (or specific command handler if target is given)");
    println!("  runloop-sample        Output boilerplate runloop implementations");
}
