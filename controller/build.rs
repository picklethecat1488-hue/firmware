use rinja::Template;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const LOG_LEVEL: &str = "trace";

struct Param {
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

    fn extra_params_inferred(&self) -> Vec<Param> {
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
struct ControllerConfig {
    controllers: Vec<Controller>,
}

#[derive(Template)]
#[template(path = "generated_controllers.rs.jinja")]
struct GeneratedControllersTemplate {
    controllers: Vec<Controller>,
}

#[derive(Template)]
#[template(path = "run_loop.rs.jinja")]
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
#[template(path = "test_mocks.rs.jinja")]
struct TestMocksTemplate {
    controllers: Vec<Controller>,
}

fn main() {
    if std::env::var("CARGO_FEATURE_TRACING").is_ok() {
        println!("cargo:rustc-env=DEFMT_LOG={}", LOG_LEVEL);
    }

    // Tell Cargo to rerun this build script if config or template changes
    println!("cargo:rerun-if-changed=controllers.toml");
    println!("cargo:rerun-if-changed=templates/generated_controllers.rs.jinja");
    println!("cargo:rerun-if-changed=templates/run_loop.rs.jinja");
    println!("cargo:rerun-if-changed=templates/test_mocks.rs.jinja");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_controllers.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Read and parse controllers.toml config file
    let config_content =
        std::fs::read_to_string("controllers.toml").expect("Failed to read controllers.toml");
    let config: ControllerConfig =
        toml::from_str(&config_content).expect("Failed to parse controllers.toml");

    // Render the controllers template using Rinja
    let template = GeneratedControllersTemplate {
        controllers: config.controllers.clone(),
    };
    let output = template.render().expect("Failed to render Rinja template");
    f.write_all(output.as_bytes()).unwrap();

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
