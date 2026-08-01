//! Host-side code generator utility for inspection of controllers.
//!
//! This tool reads the controller configurations from `controllers.toml`, processes their
//! fields and type parameters, and outputs their generated macros, channels, or boilerplate
//! `run` loop implementations to standard output.

use controller_gen::{
    find_controllers_toml, print_help, ControllerConfig, GeneratedControllersTemplate,
    RunLoopTemplate,
};
use rinja::Template;
use std::fs;

/// Entry point of the `controller_gen` host tool.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let toml_path = find_controllers_toml();
    let config_content = fs::read_to_string(&toml_path).expect("Failed to read controllers.toml");
    let config: ControllerConfig =
        toml::from_str(&config_content).expect("Failed to parse controllers.toml");

    if args.len() > 1 {
        let arg = &args[1];
        if arg == "-h" || arg == "--help" {
            print_help(&config.controllers);
            return;
        }

        if arg == "list" || arg == "--list" || arg == "-l" {
            for ctrl in &config.controllers {
                println!("{}", ctrl.name);
            }
            return;
        }

        // Find the requested controller (case-insensitive match)
        let chosen = config
            .controllers
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(arg));
        match chosen {
            Some(ctrl) => {
                println!("// --- Generated Macros and Channels for {} ---", ctrl.name);
                let template = GeneratedControllersTemplate {
                    controllers: vec![ctrl.clone()],
                };
                let output = template.render().expect("Failed to render Rinja template");
                println!("{}", output);

                println!();
                println!(
                    "// --- Boilerplate runloop implementation for {} ---",
                    ctrl.name
                );
                let run_loop_template = RunLoopTemplate {
                    name: ctrl.name.clone(),
                    msg_type: ctrl.msg_type.clone(),
                    has_telemetry: ctrl.has_telemetry.unwrap_or(true),
                    is_system: ctrl.is_system.unwrap_or(false),
                    impl_generics: ctrl.impl_generics.clone(),
                    impl_type: ctrl.impl_type.clone(),
                };
                let run_loop_output = run_loop_template
                    .render()
                    .expect("Failed to render runloop template");
                println!("{}", run_loop_output);
            }
            None => {
                eprintln!("Error: Unknown controller '{}'", arg);
                eprintln!();
                print_help(&config.controllers);
                std::process::exit(1);
            }
        }
    } else {
        // Render all controllers if no argument is passed
        let template = GeneratedControllersTemplate {
            controllers: config.controllers,
        };
        let output = template.render().expect("Failed to render Rinja template");
        print!("{}", output);
    }
}
