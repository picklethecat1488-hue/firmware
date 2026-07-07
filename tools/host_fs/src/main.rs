//! Host command-line utility for querying and exploring flash storage dumps
//! extracted from the RP2040 microcontroller's sequential-storage partition.

use clap::Parser;
use host_fs::flash::{EitherFlash, ProbeFlash};
use host_fs::{Cli, Commands, HostFlash};
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Initialize the progress spinner
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    // Connect to probe directly or load from file dump
    let mut flash = match &cli.dump {
        Some(dump_path) => {
            spinner.set_message("Loading flash dump file...");
            let mut file = File::open(dump_path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            EitherFlash::Host(HostFlash::new(buf))
        }
        None => {
            spinner.set_message("Reading project settings...");
            let elf_path = cli.elf.as_ref().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "ELF path is required via --elf to detect layout parameters when not loading a dump file",
                )
            })?;
            let (chip, base_addr, size) =
                tool_common::autodetect_project_info(std::path::Path::new(elf_path))
                    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

            if let Some(host) = &cli.openocd_host {
                let addr = if host.contains(':') {
                    host.clone()
                } else {
                    format!("{}:3333", host)
                };
                spinner.set_message(format!(
                    "Connecting to device at {} (address: 0x{:08X}) via OpenOCD GDB...",
                    addr, base_addr
                ));
                let gdb_flash = host_fs::flash::GdbFlash::new(&addr, base_addr, size)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                EitherFlash::Gdb(Box::new(gdb_flash))
            } else {
                spinner.set_message(format!(
                    "Connecting to device (chip: {}, address: 0x{:08X}) via probe-rs...",
                    chip, base_addr
                ));
                let probe_flash =
                    ProbeFlash::new(&chip, base_addr, size).map_err(io::Error::other)?;
                EitherFlash::Probe(Box::new(probe_flash))
            }
        }
    };

    let flash_range = 0..flash.capacity() as u32;

    // Outer-scope variables to manage ELF file lifespans for Context references
    #[allow(unused_assignments)]
    let mut file_data = Vec::new();
    #[allow(unused_assignments)]
    let mut object_file = None;
    let mut context = None;
    let mut defmt_table = None;

    let elf_path = if let Commands::CrashLog {
        elf: Some(elf_path),
    } = &cli.command
    {
        Some(elf_path.clone())
    } else {
        cli.elf.clone()
    };

    if let Some(ref path) = elf_path {
        spinner.set_message("Loading ELF and DWARF debug symbols...");
        file_data = std::fs::read(path)?;
        let obj = object::File::parse(&*file_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        object_file = Some(obj);
        let ctx = addr2line::Context::new(object_file.as_ref().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        context = Some(ctx);

        if let Some(table) = defmt_decoder::Table::parse(&file_data).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse defmt table: {}", e),
            )
        })? {
            defmt_table = Some(table);
        }
    }

    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();

        match &cli.command {
            Commands::Ls => {
                host_fs::commands::ls::run(&mut flash, flash_range, &mut cache, &spinner).await?;
            }
            Commands::Cat { filename } => {
                host_fs::commands::cat::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    filename,
                )
                .await?;
            }
            Commands::ExportTelemetry { out_csv } => {
                host_fs::commands::export_telemetry::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    out_csv,
                )
                .await?;
            }
            Commands::CrashLog { .. } => {
                host_fs::commands::crash_log::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    &context,
                    &defmt_table,
                )
                .await?;
            }
            Commands::Cp { src, dest } => {
                host_fs::commands::cp::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    src,
                    dest,
                    &cli.dump,
                )
                .await?;
            }
        }
        Ok::<(), io::Error>(())
    })?;

    Ok(())
}
