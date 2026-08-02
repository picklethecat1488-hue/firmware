//! Host-side code generator utility for inspection of controllers.
//!
//! This tool reads the controller configurations from `controllers.toml`, processes their
//! fields and type parameters, and outputs their generated macros, channels, or boilerplate
//! `run` loop implementations to standard output or to the specified output directory.

mod cli_sample;
mod list_clis;
mod list_controllers;
mod runloop_sample;

use code_gen::{
    find_controllers_toml, find_shell_toml, print_help, ControllerConfig,
    GeneratedControllersTemplate, ShellConfigToml,
};
use rinja::Template;
use std::fs;
use std::path::PathBuf;

/// Entry point of the `code_gen` host tool.
fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    // Default output directory is target/out
    let mut out_dir = PathBuf::from("target/out");

    // Parse and remove --out-dir argument if present
    if let Some(pos) = args.iter().position(|x| x == "--out-dir") {
        if pos + 1 < args.len() {
            out_dir = PathBuf::from(&args[pos + 1]);
            args.remove(pos + 1);
            args.remove(pos);
        } else {
            eprintln!("Error: Missing value for --out-dir");
            std::process::exit(1);
        }
    }

    let toml_path = find_controllers_toml();
    let config_content = fs::read_to_string(&toml_path).expect("Failed to read controllers.toml");
    let config: ControllerConfig =
        toml::from_str(&config_content).expect("Failed to parse controllers.toml");

    let shell_path = find_shell_toml();
    let shell_content = fs::read_to_string(&shell_path).expect("Failed to read shell.toml");
    let shell_config: ShellConfigToml =
        toml::from_str(&shell_content).expect("Failed to parse shell.toml");

    if args.len() > 1 {
        let arg = &args[1];
        if arg == "-h" || arg == "--help" {
            print_help(&config.controllers);
            return;
        }

        match arg.as_str() {
            "list-controllers" | "--list-controllers" => {
                list_controllers::handle(&config);
            }
            "list-clis" | "--list-clis" => {
                list_clis::handle(&shell_config);
            }
            "cli-sample" | "--cli-sample" => {
                let targets = if args.len() > 2 {
                    args[2..].to_vec()
                } else {
                    Vec::new()
                };
                cli_sample::handle(&targets, &out_dir, &shell_config);
            }
            "runloop-sample" | "--runloop-sample" => {
                let targets = if args.len() > 2 {
                    args[2..].to_vec()
                } else {
                    Vec::new()
                };
                runloop_sample::handle(&targets, &out_dir, &config);
            }
            unknown => {
                eprintln!("Error: Unknown subcommand '{}'", unknown);
                eprintln!();
                print_help(&config.controllers);
                std::process::exit(1);
            }
        }
    } else {
        // Render all controllers if no argument is passed
        let template = GeneratedControllersTemplate {
            controllers: config.controllers,
            cli_resolver_fields: shell_config.cli_resolver_fields,
            cli_commands: shell_config.cli_commands,
        };
        let output = template.render().expect("Failed to render Rinja template");
        print!("{}", output);
    }
}
