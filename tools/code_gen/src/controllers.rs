use rinja::Template;
use serde::Deserialize;

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

/// The outer configuration structure for controllers.toml.
#[derive(Deserialize)]
pub struct ControllerConfig {
    /// List of configured controllers.
    pub controllers: Vec<Controller>,
}

/// Template structure for runloop generator.
#[derive(Template)]
#[template(path = "run_loop.rs.jinja")]
pub struct RunLoopTemplate {
    /// Name of the controller.
    pub name: String,
    /// Type path of the message channel message type.
    pub msg_type: String,
    /// Flag indicating whether telemetry is enabled.
    pub has_telemetry: bool,
    /// Flag indicating whether this is the system run loop.
    pub is_system: bool,
    /// Impl block generics definition.
    pub impl_generics: String,
    /// Struct target type name.
    pub impl_type: String,
    /// Type to be held in PhantomData inside the generated struct boilerplate.
    pub impl_phantom: String,
}
