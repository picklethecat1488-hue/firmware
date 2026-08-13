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
    }
}
