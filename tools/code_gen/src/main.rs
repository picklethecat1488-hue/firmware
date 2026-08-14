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
    }
}
