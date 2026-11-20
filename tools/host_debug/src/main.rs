//! CLI entry point for `host_debug` (`probe-rs` dual-core GDB RSP + non-halting RTT bridge).

#[cfg(all(target_arch = "arm", target_os = "none"))]
fn main() {}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod host_main {
    use clap::Parser;
    use host_debug::arm::probe_backend::ArmProbeDebugBackend;
    use host_debug::server::{run_debug_server_with_backend, DebugServerConfig};
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tool_common::{
        DEFAULT_GDB_CORE0_PORT, DEFAULT_GDB_CORE1_PORT, DEFAULT_RTT_BRIDGE_PORT,
        DEFAULT_SWD_SPEED_KHZ,
    };

    /// `probe-rs` dual-core GDB RSP debug server and non-intrusive RTT memory bridge.
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct CliArgs {
        /// Path to the compiled target ELF binary (used to infer chip identity and symbol addresses).
        #[arg(short, long)]
        pub elf: Option<PathBuf>,

        /// Explicit target chip name (e.g. rp2040, MCXN947). Inferred from --elf .program_metadata if omitted.
        #[arg(short, long)]
        pub chip: Option<String>,

        /// Optional debug probe selector (`VID:PID` or `VID:PID:SERIAL`) for multi-target setups.
        #[arg(short, long)]
        pub probe: Option<String>,

        /// SWD probe clock frequency in kHz.
        #[arg(short, long, default_value_t = DEFAULT_SWD_SPEED_KHZ)]
        pub speed: u32,

        /// Network address on which to bind GDB RSP and RTT bridge TCP sockets.
        #[arg(long, default_value = "127.0.0.1")]
        pub bind_host: String,

        /// TCP port for Core 0 GDB RSP connections (`0` selects an OS-assigned ephemeral port).
        #[arg(long, visible_alias = "gdb-port", default_value_t = DEFAULT_GDB_CORE0_PORT)]
        pub gdb_core0_port: u16,

        /// TCP port for Core 1 GDB RSP connections on dual-core targets (`0` selects an ephemeral port).
        #[arg(long, default_value_t = DEFAULT_GDB_CORE1_PORT)]
        pub gdb_core1_port: u16,

        /// Disable binding a dedicated Core 1 GDB RSP port even on dual-core targets.
        #[arg(long)]
        pub disable_core1_gdb: bool,

        /// TCP port for the non-halting RTT memory bridge consumed by `host_cli --gdb-bridge`.
        #[arg(long, default_value_t = DEFAULT_RTT_BRIDGE_PORT)]
        pub rtt_bridge_port: u16,

        /// Disable binding the non-halting RTT memory bridge listener.
        #[arg(long)]
        pub disable_rtt_bridge: bool,

        /// Reset Core 0 and resume normal execution upon initial probe attachment.
        #[arg(short, long)]
        pub reset: bool,

        /// Reset Core 0 and halt immediately at `Reset_Handler` awaiting GDB attachment.
        #[arg(long)]
        pub halt_on_reset: bool,

        /// Emit structured line-delimited JSON lifecycle events (`ready`, `gdb_connected`, `stopped`) to stdout.
        #[arg(long, visible_alias = "json")]
        pub json_events: bool,

        /// Terminate cleanly and release the USB probe session when stdin reaches EOF.
        #[arg(long)]
        pub exit_on_stdin_eof: bool,

        /// Resolve one or more symbol or `file:line` breakpoint specifications in `--elf` to JSON addresses and exit.
        #[arg(long = "resolve")]
        pub resolve_specs: Vec<String>,
    }

    fn resolve_target_chip(args: &CliArgs) -> Result<String, String> {
        if let Some(ref explicit) = args.chip {
            let trimmed = explicit.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }
        let Some(ref elf_path) = args.elf else {
            return Err(
                "Target chip cannot be inferred: pass --chip <CHIP> or --elf <ELF_PATH> containing .program_metadata"
                    .to_string(),
            );
        };
        let info = tool_common::autodetect_project_info(elf_path)
            .map_err(|e| format!("Failed to inspect ELF '{}': {}", elf_path.display(), e))?;
        let chip = info.chip.trim().to_string();
        if chip.is_empty() {
            return Err(format!(
                "ELF '{}' did not declare a target chip in .program_metadata",
                elf_path.display()
            ));
        }
        Ok(chip)
    }

    pub fn run() -> Result<(), String> {
        let args = CliArgs::parse();
        if !args.resolve_specs.is_empty() {
            let Some(ref elf_path) = args.elf else {
                return Err("--resolve requires --elf <ELF_PATH>".to_string());
            };
            let mut map = serde_json::Map::new();
            for spec in &args.resolve_specs {
                let addr = tool_common::resolve_breakpoint_target(elf_path, spec)?;
                map.insert(spec.clone(), serde_json::Value::Number(addr.into()));
            }
            println!(
                "{}",
                serde_json::to_string(&serde_json::Value::Object(map)).map_err(|e| e.to_string())?
            );
            return Ok(());
        }

        let chip = resolve_target_chip(&args)?;

        let shutdown = Arc::new(AtomicBool::new(false));
        {
            let sig_flag = Arc::clone(&shutdown);
            let _ = ctrlc::set_handler(move || {
                sig_flag.store(true, Ordering::SeqCst);
            });
        }

        if args.exit_on_stdin_eof {
            let eof_flag = Arc::clone(&shutdown);
            std::thread::spawn(move || {
                let mut stdin = std::io::stdin();
                let mut buf = [0u8; 64];
                loop {
                    match stdin.read(&mut buf) {
                        Ok(0) | Err(_) => {
                            eof_flag.store(true, Ordering::SeqCst);
                            break;
                        }
                        Ok(_) => {}
                    }
                }
            });
        }

        let core0_reset_addr = args.elf.as_ref().and_then(|elf_path| {
            tool_common::resolve_breakpoint_target(elf_path, "Reset_Handler")
                .or_else(|_| tool_common::resolve_breakpoint_target(elf_path, "Reset"))
                .ok()
        });
        let core1_reset_addr = args.elf.as_ref().and_then(|elf_path| {
            tool_common::resolve_breakpoint_target(elf_path, "Core1_Reset_Handler").ok()
        });
        let resolved_exec_file = args
            .elf
            .as_ref()
            .and_then(|p| std::fs::canonicalize(p).ok().or_else(|| Some(p.clone())))
            .map(|p| p.display().to_string());

        let backend = ArmProbeDebugBackend::attach(
            &chip,
            args.probe.as_deref(),
            args.speed,
            args.reset,
            args.halt_on_reset,
            core0_reset_addr,
            core1_reset_addr,
            args.elf.as_deref(),
        )?;

        let config = DebugServerConfig {
            bind_host: args.bind_host,
            gdb_core0_port: args.gdb_core0_port,
            gdb_core1_port: if args.disable_core1_gdb {
                None
            } else {
                Some(args.gdb_core1_port)
            },
            rtt_bridge_port: if args.disable_rtt_bridge {
                None
            } else {
                Some(args.rtt_bridge_port)
            },
            halted_on_reset: args.halt_on_reset,
            exec_file_path: resolved_exec_file,
            json_events: args.json_events,
        };

        run_debug_server_with_backend(backend, &config, shutdown)
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {
    if let Err(err) = host_main::run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
