//! Host command-line utility for streaming defmt logs via RTT or Serial Port (UART) from an attached target device.

use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod rtt;
mod uart;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Streams defmt logs from a target device via RTT or Serial Port (UART)"
)]
pub struct Cli {
    /// Path to the ELF binary containing defmt table and symbols
    #[arg(short, long)]
    pub elf: PathBuf,

    /// Chip name (e.g., "rp2040"). If omitted, --autodetect must be specified (not required for serial port).
    #[arg(short, long)]
    pub chip: Option<String>,

    /// Automatically detect chip and layout parameters from the ELF's metadata section
    #[arg(short, long)]
    pub autodetect: bool,

    /// Serial port to stream logs from (e.g. "/dev/tty.usbserial-10" or "COM3"). If specified, RTT is bypassed.
    #[arg(long)]
    pub port: Option<String>,

    /// Baud rate for the serial port
    #[arg(long, default_value_t = 115200)]
    pub baud: u32,

    /// Dump currently buffered logs and exit immediately
    #[arg(short, long)]
    pub dump: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. Validate arguments
    if cli.port.is_none() && cli.chip.is_none() && !cli.autodetect {
        return Err("Either --chip, --autodetect, or --port must be specified".into());
    }

    // 2. Parse defmt table from ELF
    let elf_data = fs::read(&cli.elf)?;
    let table = defmt_decoder::Table::parse(&elf_data)?
        .ok_or("Could not parse defmt table from the specified ELF file")?;

    // 3. Dispatch to appropriate connection runner
    if let Some(port_path) = cli.port {
        uart::run_uart(&port_path, cli.baud, &table, cli.dump)?;
    } else {
        let chip = if cli.autodetect {
            let (autodetect_chip, _, _) = defmt_host::autodetect_project_info(&cli.elf)?;
            autodetect_chip
        } else {
            cli.chip.clone().ok_or("Missing --chip or --autodetect")?
        };
        rtt::run_rtt(&chip, &table, cli.dump)?;
    }

    Ok(())
}
