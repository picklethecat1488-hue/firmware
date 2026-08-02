use rinja::Template;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const LOG_LEVEL: &str = "trace";

#[derive(Deserialize, Clone)]
struct Param {
    #[serde(rename = "param")]
    name: String,
    r#type: String,
}

#[derive(Deserialize, Clone)]
struct Controller {
    name: String,
    msg_type: String,
    macro_params: Vec<String>,
    task_call_args: Vec<String>,
    controller_type: String,
    spawn_call_args: Vec<String>,
    spawn_extra_rxs: Option<Vec<String>>,
    macro_generics: Option<Vec<String>>,
    spawn_generics: Option<Vec<String>>,
    attributes: Option<Vec<String>>,

    has_telemetry: Option<bool>,
    receiver_capacity: Option<String>,
    is_system: Option<bool>,
    impl_generics: String,
    impl_type: String,
    impl_phantom: Option<String>,
    run_params: Vec<Param>,
}

impl Controller {
    fn macro_params_slice(&self) -> &[String] {
        &self.macro_params
    }

    fn task_call_args_slice(&self) -> &[String] {
        &self.task_call_args
    }

    fn spawn_call_args_slice(&self) -> &[String] {
        &self.spawn_call_args
    }

    fn spawn_extra_rxs_slice(&self) -> &[String] {
        self.spawn_extra_rxs.as_deref().unwrap_or(&[])
    }

    fn macro_generics_slice(&self) -> &[String] {
        self.macro_generics.as_deref().unwrap_or(&[])
    }

    fn spawn_generics_slice(&self) -> &[String] {
        self.spawn_generics.as_deref().unwrap_or(&[])
    }

    fn attributes_slice(&self) -> &[String] {
        self.attributes.as_deref().unwrap_or(&[])
    }

    fn extra_params_inferred(&self) -> &[Param] {
        &self.run_params
    }

    fn impl_phantom_str(&self) -> &str {
        self.impl_phantom.as_deref().unwrap_or("")
    }

    fn default_capacity_suffix(&self) -> String {
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
struct CliResolverField {
    associated_type: String,
    field: String,
    resolve_fn: String,
    doc: String,
    bounds: String,
    type_lifetime: Option<String>,
}

impl CliResolverField {
    fn bounds(&self) -> String {
        let lifetime = self.type_lifetime.as_deref().unwrap_or("'static");
        format!("{} + {}", self.bounds, lifetime)
    }
}

#[derive(Deserialize, Clone)]
struct CliArg {
    name: String,
    r#type: String,
    help: String,
    attributes: Option<Vec<String>>,
}

impl CliArg {
    fn attributes_slice(&self) -> Vec<String> {
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

    fn rust_type(&self) -> String {
        match self.r#type.as_str() {
            "string" => "Option<&'a str>".to_string(),
            "int" => "Option<i32>".to_string(),
            "float" => "Option<f32>".to_string(),
            "bool" => "Option<bool>".to_string(),
            custom => resolve_crate_path(custom),
        }
    }

    fn rust_type_sample(&self) -> String {
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
struct CliCommand {
    group: String,
    cmd_name: String,
    variant: String,
    subcommand_type: String,
    handler: String,
    help: String,
    args: Option<Vec<CliArg>>,
}

impl CliCommand {
    fn args_slice(&self) -> &[CliArg] {
        self.args.as_deref().unwrap_or(&[])
    }

    fn handler_short_name(&self) -> &str {
        self.handler.split("::").last().unwrap()
    }

    fn subcommand_type_path(&self) -> String {
        if self.subcommand_type.contains("::") && !self.subcommand_type.starts_with("platform::") {
            format!("controller::{}", self.subcommand_type)
        } else {
            self.subcommand_type.clone()
        }
    }
}

#[derive(Deserialize, Clone)]
struct ControllerConfig {
    controllers: Vec<Controller>,
}

#[derive(Deserialize, Clone)]
struct ShellConfigToml {
    #[serde(default)]
    cli_resolver_fields: Vec<CliResolverField>,
    #[serde(default)]
    cli_commands: Vec<CliCommand>,
}

#[derive(Template)]
#[template(path = "generated_controllers.rs.jinja", escape = "none")]
struct GeneratedControllersTemplate {
    controllers: Vec<Controller>,
    cli_resolver_fields: Vec<CliResolverField>,
    cli_commands: Vec<CliCommand>,
}

#[derive(Template)]
#[template(path = "run_loop.rs.jinja", escape = "none")]
struct RunLoopTemplate {
    name: String,
    msg_type: String,
    has_telemetry: bool,
    is_system: bool,
    impl_generics: String,
    impl_type: String,
    impl_phantom: String,
}

#[derive(Template)]
#[template(path = "test_mocks.rs.jinja", escape = "none")]
struct TestMocksTemplate {
    controllers: Vec<Controller>,
}

#[derive(Template)]
#[template(path = "sample_cli.rs.jinja", escape = "none")]
struct SampleCliTemplate {
    cli_commands: Vec<CliCommand>,
}

#[derive(Template)]
#[template(path = "cli_skeletons_test.rs.jinja", escape = "none")]
struct CliSkeletonsTestTemplate {
    cli_commands: Vec<CliCommand>,
}

fn main() {
    if std::env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG={}", LOG_LEVEL);
    }

    // Tell Cargo to rerun this build script if config or template changes
    println!("cargo:rerun-if-changed=controllers.toml");
    println!("cargo:rerun-if-changed=shell.toml");
    println!("cargo:rerun-if-changed=templates/generated_controllers.rs.jinja");
    println!("cargo:rerun-if-changed=templates/run_loop.rs.jinja");
    println!("cargo:rerun-if-changed=templates/test_mocks.rs.jinja");
    println!("cargo:rerun-if-changed=templates/cli_skeletons_test.rs.jinja");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_controllers.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Read and parse controllers.toml config file
    let config_content =
        std::fs::read_to_string("controllers.toml").expect("Failed to read controllers.toml");
    let config: ControllerConfig =
        toml::from_str(&config_content).expect("Failed to parse controllers.toml");

    // Read and parse shell.toml config file
    let shell_content = std::fs::read_to_string("shell.toml")
        .or_else(|_| std::fs::read_to_string(Path::new("controller").join("shell.toml")))
        .expect("Failed to read shell.toml");
    let shell_config: ShellConfigToml =
        toml::from_str(&shell_content).expect("Failed to parse shell.toml");

    // Render the controllers template using Rinja
    let template = GeneratedControllersTemplate {
        controllers: config.controllers.clone(),
        cli_resolver_fields: shell_config.cli_resolver_fields.clone(),
        cli_commands: shell_config.cli_commands.clone(),
    };
    let output = template.render().expect("Failed to render Rinja template");
    f.write_all(output.as_bytes()).unwrap();

    // Generate sample CLI implementation for validation
    let mut sample_cli_f =
        File::create(Path::new(&out_dir).join("generated_sample_cli.rs")).unwrap();
    let sample_cli_tmpl = SampleCliTemplate {
        cli_commands: shell_config.cli_commands.clone(),
    };
    let sample_cli_content = sample_cli_tmpl
        .render()
        .expect("Failed to render sample CLI template in build.rs");
    sample_cli_f
        .write_all(sample_cli_content.as_bytes())
        .unwrap();

    // Generate skeleton CLI handlers integration test
    let mut skeletons_f = File::create(Path::new(&out_dir).join("cli_skeletons_test.rs")).unwrap();
    let skeletons_tmpl = CliSkeletonsTestTemplate {
        cli_commands: shell_config.cli_commands.clone(),
    };
    let skeletons_content = skeletons_tmpl
        .render()
        .expect("Failed to render CLI skeletons test template");
    skeletons_f.write_all(skeletons_content.as_bytes()).unwrap();

    // Generate boilerplate runloops code for validation in integration tests
    let mut runloops_f = File::create(Path::new(&out_dir).join("generated_runloops.rs")).unwrap();
    let mut runloops_content = String::new();
    for ctrl in &config.controllers {
        let runloop_tmpl = RunLoopTemplate {
            name: ctrl.name.clone(),
            msg_type: ctrl.msg_type.clone(),
            has_telemetry: ctrl.has_telemetry.unwrap_or(true),
            is_system: ctrl.is_system.unwrap_or(false),
            impl_generics: ctrl.impl_generics.clone(),
            impl_type: ctrl.impl_type.clone(),
            impl_phantom: ctrl.impl_phantom_str().to_string(),
        };
        let rendered = runloop_tmpl
            .render()
            .expect("Failed to render runloop template in build.rs");
        runloops_content.push_str(&rendered);
        runloops_content.push_str("\n\n");
    }
    runloops_f.write_all(runloops_content.as_bytes()).unwrap();

    // Generate mock structs and receiver aliases for integration tests
    let mut mocks_f = File::create(Path::new(&out_dir).join("generated_test_mocks.rs")).unwrap();
    let mocks_template = TestMocksTemplate {
        controllers: config.controllers,
    };
    let mocks_output = mocks_template
        .render()
        .expect("Failed to render mocks template in build.rs");
    mocks_f.write_all(mocks_output.as_bytes()).unwrap();
}
