//! Host command-line utility for streaming defmt logs and running interactive console via RTT from an attached target device.

use clap::{Args, Parser, Subcommand};
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

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Query current target firmware boot status from `_SEGGER_RTT` without resetting the MCU
    #[command(name = "boot-status")]
    BootStatus(BootStatusArgs),
}

#[derive(Args, Debug, Clone)]
pub struct BootStatusArgs {
    /// Path to the ELF binary containing `_SEGGER_RTT` and `.program_metadata`
    #[arg(short, long)]
    pub elf: PathBuf,

    /// Chip name (e.g., "rp2040"). Autodetected from ELF metadata if omitted.
    #[arg(short, long)]
    pub chip: Option<String>,

    /// Connect via a `host_debug` GDB RSP memory bridge (e.g. "127.0.0.1:50000")
    #[arg(short = 'b', long)]
    pub gdb_bridge: Option<String>,

    /// Probe protocol speed in kHz
    #[arg(long)]
    pub speed: Option<u32>,

    /// Output single-line JSON instead of key-value text
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Streams defmt logs, interactive CLI console, and target boot status via RTT",
    args_conflicts_with_subcommands = true
)]
pub struct Cli {
    /// Optional explicit subcommand (e.g. `boot-status`)
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the ELF binary containing defmt table and symbols
    #[arg(short, long)]
    pub elf: Option<PathBuf>,

    /// Chip name (e.g., "rp2040"). If omitted, it will be automatically detected from the ELF's metadata.
    #[arg(short, long)]
    pub chip: Option<String>,

    /// Dump currently buffered logs and exit immediately
    #[arg(short, long)]
    pub dump: bool,

    /// Query current target firmware boot status from `_SEGGER_RTT` and exit
    #[arg(long)]
    pub boot_status: bool,

    /// Force raw bidirectional console mode, skipping all defmt decoding
    #[arg(short, long)]
    pub raw: bool,

    /// Read and dump raw _SEGGER_RTT memory block and RTT buffers directly from target RAM
    #[arg(long)]
    pub dump_mem: bool,

    /// Reset the target CPU on start
    #[arg(long)]
    pub reset: bool,

    /// Connect via a `host_debug` GDB RSP memory bridge (e.g. "127.0.0.1:50000")
    #[arg(short = 'b', long)]
    pub gdb_bridge: Option<String>,

    /// Emit structured single-line NDJSON events on stdout for `tools/host.py`
    #[arg(long, alias = "json")]
    pub json_events: bool,

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

    /// Optional log file path receiving decoded RTT defmt and CLI output instead of stdout/stderr
    #[arg(long, value_name = "LOG_FILE")]
    pub log_file: Option<PathBuf>,

    /// Trace collection duration in seconds. Required when --trace is specified.
    #[arg(long, value_name = "SECONDS")]
    pub duration: Option<u64>,

    /// Probe protocol speed in kHz
    #[arg(long)]
    pub speed: Option<u32>,
}

fn resolve_chip_name(
    explicit_chip: Option<&str>,
    elf_path: &std::path::Path,
    has_gdb_bridge: bool,
) -> Result<String, String> {
    if let Some(c) = explicit_chip {
        return Ok(c.to_string());
    }
    if let Ok(info) = host_cli::autodetect_project_info(elf_path) {
        return Ok(info.chip);
    }
    if has_gdb_bridge {
        return Ok("gdb_bridge".to_string());
    }
    Err(host_cli::autodetect_project_info(elf_path)
        .err()
        .unwrap_or_else(|| "Failed to infer chip from ELF".to_string()))
}

fn resolve_probe_speed(explicit_speed: Option<u32>) -> u32 {
    explicit_speed.unwrap_or_else(|| {
        std::env::var("PROBE_RS_SPEED")
            .ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(1000)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(Commands::BootStatus(args)) = cli.command {
        let chip = resolve_chip_name(args.chip.as_deref(), &args.elf, args.gdb_bridge.is_some())?;
        let speed = resolve_probe_speed(args.speed);
        return rtt::run_boot_status(
            &chip,
            &args.elf,
            args.gdb_bridge.as_deref(),
            speed,
            args.json,
        );
    }

    let elf_path = cli
        .elf
        .as_ref()
        .ok_or("Argument error: --elf <ELF> is required")?;

    if cli.boot_status {
        let chip = resolve_chip_name(cli.chip.as_deref(), elf_path, cli.gdb_bridge.is_some())?;
        let speed = resolve_probe_speed(cli.speed);
        return rtt::run_boot_status(
            &chip,
            elf_path,
            cli.gdb_bridge.as_deref(),
            speed,
            cli.json_events,
        );
    }

    if cli.trace.is_some() && cli.duration.is_none() {
        return Err("Argument error: --duration is required when --trace is specified".into());
    }

    let _chrome_guard = tracing::init_tracing(cli.trace.as_deref());

    if cli.print_chip {
        let info = host_cli::autodetect_project_info(elf_path)?;
        println!("{}", info.chip);
        return Ok(());
    }

    // Initialize the progress spinner (hidden in structured JSON event or file-redirection mode)
    let spinner = if cli.json_events || cli.log_file.is_some() {
        indicatif::ProgressBar::hidden()
    } else {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb
    };

    spinner.set_message("Reading project settings...");

    // 1. Resolve target chip (default to autodetection if not specified)
    let chip = resolve_chip_name(cli.chip.as_deref(), elf_path, cli.gdb_bridge.is_some())?;

    // 2. Parse defmt table from ELF (if not in raw/dump-mem mode)
    spinner.set_message("Loading ELF and RTT symbols...");
    let table = if cli.raw || cli.dump_mem {
        None
    } else {
        let elf_data = fs::read(elf_path)?;
        // It's possible the ELF doesn't have a defmt section if it's a pure raw console shell
        match defmt_decoder::Table::parse(&elf_data) {
            Ok(Some(parsed_table)) => Some(parsed_table),
            _ => None,
        }
    };

    let speed = resolve_probe_speed(cli.speed);

    // 3. Run RTT connection runner
    rtt::run_rtt(rtt::RttOptions {
        chip: &chip,
        table: table.as_ref(),
        elf_path,
        dump: cli.dump,
        raw: cli.raw,
        dump_mem: cli.dump_mem,
        reset: cli.reset,
        gdb_bridge: cli.gdb_bridge.as_deref(),
        json_events: cli.json_events,
        show_raw_cli: cli.show_raw_cli,
        spinner: &spinner,
        channel_mode: cli.channel,
        trace: cli.trace.is_some(),
        duration: cli.duration,
        speed,
        log_file: cli.log_file.as_deref(),
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
        if let Err(e) = tracing::post_process_trace(trace_file, elf_path.to_str()) {
            eprintln!(
                "Warning: failed to post-process Perfetto trace file: {:?}",
                e
            );
        }
    }
    spinner.finish_and_clear();

    Ok(())
}
