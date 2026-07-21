//! Host command-line utility for streaming defmt logs and running interactive console via RTT from an attached target device.

use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod rtt;
use host_cli::tracing;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum ChannelMode {
    /// Stream only defmt system logs
    Defmt,
    /// Stream only interactive CLI shell
    Cli,
    /// Stream both logs and interactive shell
    Both,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Streams defmt logs and interactive CLI console from a target device via RTT"
)]
pub struct Cli {
    /// Path to the ELF binary containing defmt table and symbols
    #[arg(short, long)]
    pub elf: PathBuf,

    /// Chip name (e.g., "rp2040"). If omitted, it will be automatically detected from the ELF's metadata.
    #[arg(short, long)]
    pub chip: Option<String>,

    /// Dump currently buffered logs and exit immediately
    #[arg(short, long)]
    pub dump: bool,

    /// Force raw bidirectional console mode, skipping all defmt decoding
    #[arg(short, long)]
    pub raw: bool,

    /// Read and dump raw _SEGGER_RTT memory block and RTT buffers directly from target RAM
    #[arg(long)]
    pub dump_mem: bool,

    /// Reset the target CPU on start
    #[arg(long)]
    pub reset: bool,

    /// Connect to an existing OpenOCD GDB server session (e.g. "localhost:3333" or "127.0.0.1")
    #[arg(short = 'o', long)]
    pub openocd_host: Option<String>,

    /// Show raw hex/ascii dump of initial cached RTT CLI buffer contents on start
    #[arg(long)]
    pub show_raw_cli: bool,

    /// RTT channel mode to stream
    #[arg(long, value_enum, default_value_t = ChannelMode::Both)]
    pub channel: ChannelMode,

    /// Print the autodetected chip name from ELF metadata and exit
    #[arg(long)]
    pub print_chip: bool,

    /// Optional filename to output tracing events to in Perfetto/Chrome trace format
    #[arg(long, value_name = "FILE")]
    pub trace: Option<String>,

    /// Trace collection duration in seconds. Required when --trace is specified.
    #[arg(long, value_name = "SECONDS")]
    pub duration: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.trace.is_some() && cli.duration.is_none() {
        return Err("Argument error: --duration is required when --trace is specified".into());
    }

    let _chrome_guard = tracing::init_tracing(cli.trace.as_deref());

    if cli.print_chip {
        let info = host_cli::autodetect_project_info(&cli.elf)?;
        println!("{}", info.chip);
        return Ok(());
    }

    // Initialize the progress spinner
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    spinner.set_message("Reading project settings...");

    // 1. Resolve target chip (default to autodetection if not specified)
    let chip = if cli.openocd_host.is_some() {
        "unknown".to_string()
    } else if let Some(ref c) = cli.chip {
        c.clone()
    } else {
        let info = host_cli::autodetect_project_info(&cli.elf)?;
        info.chip
    };

    // 2. Parse defmt table from ELF (if not in raw/dump-mem mode)
    spinner.set_message("Loading ELF and RTT symbols...");
    let table = if cli.raw || cli.dump_mem {
        None
    } else {
        let elf_data = fs::read(&cli.elf)?;
        // It's possible the ELF doesn't have a defmt section if it's a pure raw console shell
        match defmt_decoder::Table::parse(&elf_data) {
            Ok(Some(parsed_table)) => Some(parsed_table),
            _ => None,
        }
    };

    // 3. Run RTT connection runner
    rtt::run_rtt(rtt::RttOptions {
        chip: &chip,
        table: table.as_ref(),
        elf_path: &cli.elf,
        dump: cli.dump,
        raw: cli.raw,
        dump_mem: cli.dump_mem,
        reset: cli.reset,
        openocd_host: cli.openocd_host.as_deref(),
        show_raw_cli: cli.show_raw_cli,
        spinner: &spinner,
        channel_mode: cli.channel,
        trace: cli.trace.is_some(),
        duration: cli.duration,
    })?;

    // Drop the tracing guard to ensure the trace file is written and flushed
    if cli.trace.is_some() {
        spinner.set_message("Flushing trace buffer to disk...");
    }
    drop(_chrome_guard);

    // If tracing was enabled, post-process the trace file to group events by microcontroller task
    if let Some(ref trace_file) = cli.trace {
        spinner.set_message("Processing Perfetto trace timeline...");
        let raw_path = format!("{}.raw", trace_file);
        if let Err(e) = fs::copy(trace_file, &raw_path) {
            eprintln!(
                "Warning: failed to save raw trace copy to {}: {:?}",
                raw_path, e
            );
        }
        if let Err(e) = tracing::post_process_trace(trace_file, cli.elf.to_str()) {
            eprintln!(
                "Warning: failed to post-process Perfetto trace file: {:?}",
                e
            );
        }
    }
    spinner.finish_and_clear();

    Ok(())
}
