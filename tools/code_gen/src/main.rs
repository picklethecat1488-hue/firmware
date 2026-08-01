//! Host-side code generator utility for inspection of controllers.
//!
//! This tool reads the controller configurations from `controllers.toml`, processes their
//! fields and type parameters, and outputs their generated macros, channels, or boilerplate
//! `run` loop implementations to standard output.

use code_gen::{
    find_controllers_toml, find_shell_toml, print_help, CliHandlerSkeletonTemplate,
    ControllerConfig, GeneratedControllersTemplate, RunLoopTemplate, ShellConfigToml,
};
use rinja::Template;
use std::fs;

/// Entry point of the `code_gen` host tool.
fn main() {
    let args: Vec<String> = std::env::args().collect();

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
                for ctrl in &config.controllers {
                    println!("{}", ctrl.name);
                }
            }
            "list-clis" | "--list-clis" => {
                for cmd in &shell_config.cli_commands {
                    println!("{}", cmd.group);
                }
            }
            "cli-sample" | "--cli-sample" => {
                if args.len() > 2 {
                    let mut commands = Vec::new();
                    // Validate all targets first
                    for target in &args[2..] {
                        let found = shell_config.cli_commands.iter().find(|cmd| {
                            cmd.group.eq_ignore_ascii_case(target)
                                || cmd.cmd_name.eq_ignore_ascii_case(target)
                        });
                        match found {
                            Some(cmd) => commands.push(cmd.clone()),
                            None => {
                                eprintln!("Error: Unknown CLI command/group '{}'", target);
                                std::process::exit(1);
                            }
                        }
                    }
                    // Render and print all validated targets
                    for cmd in commands {
                        let template = CliHandlerSkeletonTemplate { cmd };
                        let output = template
                            .render()
                            .expect("Failed to render skeleton template");
                        print!("{}", output);
                    }
                } else {
                    let template = code_gen::SampleCliTemplate {
                        cli_commands: shell_config.cli_commands,
                    };
                    let output = template
                        .render()
                        .expect("Failed to render sample CLI template");
                    print!("{}", output);
                }
            }
            "runloop-sample" | "--runloop-sample" => {
                if args.len() > 2 {
                    let mut controllers = Vec::new();
                    // Validate all targets first
                    for target in &args[2..] {
                        let chosen = config
                            .controllers
                            .iter()
                            .find(|c| c.name.eq_ignore_ascii_case(target));
                        match chosen {
                            Some(ctrl) => controllers.push(ctrl.clone()),
                            None => {
                                eprintln!("Error: Unknown controller '{}'", target);
                                std::process::exit(1);
                            }
                        }
                    }
                    // Render and print all validated targets
                    for ctrl in controllers {
                        let run_loop_template = RunLoopTemplate {
                            name: ctrl.name.clone(),
                            msg_type: ctrl.msg_type.clone(),
                            has_telemetry: ctrl.has_telemetry.unwrap_or(true),
                            is_system: ctrl.is_system.unwrap_or(false),
                            impl_generics: ctrl.impl_generics.clone(),
                            impl_type: ctrl.impl_type.clone(),
                            impl_phantom: ctrl.impl_phantom_str().to_string(),
                        };
                        let run_loop_output = run_loop_template
                            .render()
                            .expect("Failed to render runloop template");
                        println!("{}", run_loop_output);
                    }
                } else {
                    for ctrl in &config.controllers {
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
                            impl_phantom: ctrl.impl_phantom_str().to_string(),
                        };
                        let run_loop_output = run_loop_template
                            .render()
                            .expect("Failed to render runloop template");
                        println!("{}", run_loop_output);
                        println!();
                    }
                }
            }
            target => {
                // Find the requested controller (backwards-compatible behavior)
                let chosen = config
                    .controllers
                    .iter()
                    .find(|c| c.name.eq_ignore_ascii_case(target));
                match chosen {
                    Some(ctrl) => {
                        println!("// --- Generated Macros and Channels for {} ---", ctrl.name);
                        let template = GeneratedControllersTemplate {
                            controllers: vec![ctrl.clone()],
                            cli_resolver_fields: shell_config.cli_resolver_fields.clone(),
                            cli_commands: shell_config.cli_commands.clone(),
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
                            impl_phantom: ctrl.impl_phantom_str().to_string(),
                        };
                        let run_loop_output = run_loop_template
                            .render()
                            .expect("Failed to render runloop template");
                        println!("{}", run_loop_output);
                    }
                    None => {
                        eprintln!("Error: Unknown subcommand or controller '{}'", target);
                        eprintln!();
                        print_help(&config.controllers);
                        std::process::exit(1);
                    }
                }
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
