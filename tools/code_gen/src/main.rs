//! Host-side code generator utility for inspection of controllers.
//!
//! This tool reads the controller configurations from `controllers.toml`, processes their
//! fields and type parameters, and outputs their generated macros, channels, or boilerplate
//! `run` loop implementations to standard output or to the specified output directory.

mod cli_sample;
mod list_clis;
mod list_controllers;
mod runloop_sample;

use clap::{Parser, Subcommand};
use code_gen::{find_controllers_toml, find_shell_toml, ControllerConfig, ShellConfigToml};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "code_gen")]
#[command(about = "Code Generator host tool", long_about = None)]
struct Cli {
    /// Output directory for files.
    #[arg(long, default_value = "target/out")]
    out_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// List all defined controllers.
    ListControllers,
    /// List all defined CLI commands/groups.
    ListClis,
    /// List all defined boards.
    ListBoards,
    /// List all defined apps.
    ListApps,
    /// List all defined peripherals.
    ListPeripherals,
    /// Output compiling sample CLI implementation.
    CliSample {
        /// Optional specific command/group targets.
        targets: Vec<String>,
    },
    /// Output boilerplate runloop implementations.
    RunloopSample {
        /// Optional specific controller targets.
        targets: Vec<String>,
    },
    /// Generate peripheral initializers from peripherals.toml and write to file.
    PeripheralInit,
    /// Output a sample peripheral definition.
    PeripheralSample {
        /// Name of the peripheral to show a sample for.
        name: Option<String>,
    },
    /// Generate and output board configuration definitions.
    BoardSample {
        /// Name of the board to generate (default is first board found).
        name: Option<String>,
    },
    /// Generate and output application topology definitions.
    AppSample {
        /// Name of the app to generate (default is first app found).
        name: Option<String>,
    },
}

/// Entry point of the `code_gen` host tool.
fn main() {
    let cli = Cli::parse();

    let toml_path = find_controllers_toml();
    let config_content = fs::read_to_string(&toml_path).expect("Failed to read controllers.toml");
    let config: ControllerConfig =
        toml::from_str(&config_content).expect("Failed to parse controllers.toml");

    let shell_path = find_shell_toml();
    let shell_content = fs::read_to_string(&shell_path).expect("Failed to read shell.toml");
    let shell_config: ShellConfigToml =
        toml::from_str(&shell_content).expect("Failed to parse shell.toml");

    match cli.command {
        Commands::ListControllers => {
            list_controllers::handle(&config);
        }
        Commands::ListClis => {
            list_clis::handle(&shell_config);
        }
        Commands::ListBoards => {
            let board_toml_path = code_gen::find_board_toml();
            let content = fs::read_to_string(&board_toml_path).expect("Failed to read board.toml");
            let boards_config: code_gen::BoardsConfig =
                toml::from_str(&content).expect("Failed to parse board.toml");
            for board_name in boards_config.boards.keys() {
                println!("{}", board_name);
            }
        }
        Commands::ListApps => {
            let app_toml_path = code_gen::find_app_toml();
            let content = fs::read_to_string(&app_toml_path).expect("Failed to read app.toml");
            let app_config: code_gen::MultiAppConfig =
                toml::from_str(&content).expect("Failed to parse app.toml");
            for app_name in app_config.apps.keys() {
                println!("{}", app_name);
            }
        }
        Commands::ListPeripherals => {
            let peripherals_toml_path = code_gen::find_peripherals_toml();
            let content = fs::read_to_string(&peripherals_toml_path)
                .expect("Failed to read peripherals.toml");
            let peripheral_config: code_gen::PeripheralConfig =
                toml::from_str(&content).expect("Failed to parse peripherals.toml");
            for p in &peripheral_config.peripherals {
                println!("{}", p.name);
            }
        }
        Commands::CliSample { targets } => {
            cli_sample::handle(&targets, &cli.out_dir, &shell_config);
        }
        Commands::RunloopSample { targets } => {
            runloop_sample::handle(&targets, &cli.out_dir, &config);
        }
        Commands::PeripheralInit => {
            let peripherals_toml_path = code_gen::find_peripherals_toml();
            let peripherals_content = fs::read_to_string(&peripherals_toml_path)
                .expect("Failed to read peripherals.toml");
            let generated_code = code_gen::generate_peripheral_initializers(&peripherals_content);
            fs::create_dir_all(&cli.out_dir).expect("Failed to create output directory");
            let dest_path = cli.out_dir.join("generated_initializers.rs");
            fs::write(&dest_path, &generated_code).expect("Failed to write file");
            println!(
                "Generated peripheral initializers to {}",
                dest_path.display()
            );
        }
        Commands::PeripheralSample { name } => {
            use code_gen::PeripheralSampleTemplate;
            use rinja::Template;
            let target_name = name.unwrap_or_else(|| "SampleDevice".to_string());

            // Default values for custom/unspecified peripherals
            let mut has_probeable = true;
            let mut has_led_driver = false;
            let mut has_fuel_gauge = false;
            let mut has_tickable = false;
            let mut has_charge_status = false;
            let mut has_proximity_sensor = false;

            // Read peripherals.toml to check if the target name exists and what its bus is
            let peripherals_toml_path = code_gen::find_peripherals_toml();
            let peripherals_content = fs::read_to_string(&peripherals_toml_path)
                .expect("Failed to read peripherals.toml");
            let config: code_gen::PeripheralConfig =
                toml::from_str(&peripherals_content).expect("Failed to parse peripherals.toml");

            // Look up by name case-insensitively
            let matched_p = config
                .peripherals
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(&target_name));
            if let Some(p) = matched_p {
                // If it has multiple buses, or a single bus, check if "i2c" is supported
                let buses = p.buses();
                if !buses.contains(&"i2c".to_string()) {
                    eprintln!(
                        "Error: Bus type {:?} for peripheral '{}' is not supported. Only 'i2c' bus type is supported for sample code generation.",
                        buses, p.name
                    );
                    std::process::exit(1);
                }
                has_probeable = target_name.eq_ignore_ascii_case("Max17048")
                    || target_name.eq_ignore_ascii_case("Ina219")
                    || target_name.eq_ignore_ascii_case("Vl53l0x");
                has_led_driver = target_name.eq_ignore_ascii_case("Ws2812");
                has_fuel_gauge = target_name.eq_ignore_ascii_case("Max17048");
                has_tickable = target_name.eq_ignore_ascii_case("Max17048");
                has_charge_status = target_name.eq_ignore_ascii_case("Max17048");
                has_proximity_sensor = target_name.eq_ignore_ascii_case("Vl53l0x");
            }

            let template = PeripheralSampleTemplate {
                name: target_name,
                has_probeable,
                has_led_driver,
                has_fuel_gauge,
                has_tickable,
                has_charge_status,
                has_proximity_sensor,
            };
            let output = template
                .render()
                .expect("Failed to render peripheral sample template");
            println!("{}", output);
        }
        Commands::BoardSample { name } => {
            let board_toml_path = code_gen::find_board_toml();
            let content = fs::read_to_string(&board_toml_path).expect("Failed to read board.toml");
            let boards_config: code_gen::BoardsConfig =
                toml::from_str(&content).expect("Failed to parse board.toml");
            let target_name = name.unwrap_or_else(|| {
                boards_config
                    .boards
                    .keys()
                    .next()
                    .expect("No boards found in board.toml")
                    .clone()
            });
            if !boards_config.boards.contains_key(&target_name) {
                eprintln!("Error: Board '{}' not found in board.toml", target_name);
                std::process::exit(1);
            }

            // Write the generated definitions file
            let generated_defs = code_gen::generate_board_definitions(&content, &target_name);
            let target_dir = cli.out_dir.join(format!("board_{}", target_name));
            fs::create_dir_all(&target_dir).expect("Failed to create board directory");

            let defs_path = target_dir.join("generated_board.rs");
            fs::write(&defs_path, &generated_defs).expect("Failed to write board definitions");

            // Derive names
            let name_pascal = target_name
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>();

            // Write bsp.rs skeleton
            let bsp_content = code_gen::render_board_skeleton(&name_pascal);
            let bsp_path = target_dir.join("bsp.rs");
            fs::write(&bsp_path, &bsp_content).expect("Failed to write bsp.rs");

            println!(
                "Generated board skeleton project under {}",
                target_dir.display()
            );
            println!("  - generated_board.rs (declarative constant definitions)");
            println!("  - bsp.rs (minimal Board Support Package implementation skeleton)");
        }
        Commands::AppSample { name } => {
            let app_toml_path = code_gen::find_app_toml();
            let content = fs::read_to_string(&app_toml_path).expect("Failed to read app.toml");
            let app_config: code_gen::MultiAppConfig =
                toml::from_str(&content).expect("Failed to parse app.toml");
            let target_name = name.unwrap_or_else(|| {
                app_config
                    .apps
                    .keys()
                    .next()
                    .expect("No apps found in app.toml")
                    .clone()
            });
            if !app_config.apps.contains_key(&target_name) {
                eprintln!("Error: App '{}' not found in app.toml", target_name);
                std::process::exit(1);
            }

            let app_topology = app_config.apps.get(&target_name).unwrap();

            // Write the generated app topology definitions file
            let generated_defs = code_gen::generate_app_topology(&content, &target_name);
            let target_dir = cli.out_dir.join(format!("app_{}", target_name));
            fs::create_dir_all(&target_dir).expect("Failed to create app directory");

            let defs_path = target_dir.join("generated_app.rs");
            fs::write(&defs_path, &generated_defs).expect("Failed to write app definitions");

            // Derive names
            let name_pascal = target_name
                .split('_')
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>();

            // Write lib.rs skeleton
            let lib_content = code_gen::render_app_skeleton(&name_pascal);
            let lib_path = target_dir.join("lib.rs");
            fs::write(&lib_path, &lib_content).expect("Failed to write lib.rs");

            // Write app.rs skeleton
            let app_runner_content =
                code_gen::render_app_runner_skeleton(&target_name, &name_pascal);
            let app_runner_path = target_dir.join("app.rs");
            fs::write(&app_runner_path, &app_runner_content).expect("Failed to write app.rs");

            // Write shell.rs skeleton
            let shell_content = code_gen::render_app_shell_skeleton(
                &target_name,
                &name_pascal,
                &app_topology.shell_config,
            );
            let shell_path = target_dir.join("shell.rs");
            fs::write(&shell_path, &shell_content).expect("Failed to write shell.rs");

            println!(
                "Generated app skeleton project under {}",
                target_dir.display()
            );
            println!("  - generated_app.rs (declarative feature sets & spawning macros)");
            println!("  - lib.rs (minimal application logic skeleton)");
            println!("  - app.rs (firmware binary main entry skeleton)");
            println!("  - shell.rs (interactive CLI shell entry skeleton)");
        }
    }
}
