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

    // Determine the explicit ELF path from either the subcommand or global flag
    let elf_path = match &cli.command {
        Commands::CrashLog { elf: Some(path) } => Some(std::path::PathBuf::from(path)),
        _ => cli.elf.as_ref().map(std::path::PathBuf::from),
    };

    // If we have an ELF path, parse target partition settings from it
    let info = if let Some(ref path) = elf_path {
        let project_info = tool_common::autodetect_project_info(path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to parse project info from ELF at '{}': {}",
                    path.display(),
                    e
                ),
            )
        })?;
        Some(project_info)
    } else {
        None
    };

    let offset_override = if let Some(ref off_str) = cli.offset {
        let parsed = if off_str.starts_with("0x") || off_str.starts_with("0X") {
            u32::from_str_radix(&off_str[2..], 16)
        } else {
            off_str.parse::<u32>()
        };
        Some(parsed.map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid --offset value '{}': {}", off_str, e),
            )
        })?)
    } else {
        None
    };
    let size_override = cli.size.map(|s| s as u32);

    // If connecting directly to the target device, we require valid project settings (thus an ELF file)
    if cli.dump.is_none() && info.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Direct target connection requires an ELF file to parse partition settings. Please specify it using --elf.",
        ));
    }

    let (partition_offset, partition_size_val) = if let Some(ref info) = info {
        let default_offset = info.partition_address.saturating_sub(0x1000_0000);
        let default_size = info.partition_size as u32;
        (
            offset_override.unwrap_or(default_offset),
            size_override.unwrap_or(default_size),
        )
    } else {
        // No ELF file specified. Since we are operating on a dump file, we default
        // the offset to 0 and the size to the dump file's size.
        let dump_size = if let Some(ref dump_path) = cli.dump {
            std::fs::metadata(dump_path)?.len() as u32
        } else {
            0
        };
        (
            offset_override.unwrap_or(0),
            size_override.unwrap_or(dump_size),
        )
    };

    // Connect to probe directly or load from file dump
    let mut flash = match &cli.dump {
        Some(dump_path) => {
            spinner.set_message("Loading flash dump file...");
            let mut file = File::open(dump_path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            EitherFlash::Host(HostFlash::new_with_shift(buf, partition_offset))
        }
        None => {
            let info = info.as_ref().unwrap();
            // Ensure flash parameters in ELF match the compiled host_fs tool constants
            use embedded_storage_async::nor_flash::NorFlash;
            if info.flash_write_size != <EitherFlash as NorFlash>::WRITE_SIZE as u32
                || info.flash_erase_size != <EitherFlash as NorFlash>::ERASE_SIZE as u32
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Flash parameters mismatch! ELF expects write_size={}, erase_size={}, but host_fs is compiled with write_size={}, erase_size={}.",
                        info.flash_write_size,
                        info.flash_erase_size,
                        <EitherFlash as NorFlash>::WRITE_SIZE,
                        <EitherFlash as NorFlash>::ERASE_SIZE
                    ),
                ));
            }

            if let Some(host) = &cli.openocd_host {
                let addr = if host.contains(':') {
                    host.clone()
                } else {
                    format!("{}:3333", host)
                };
                spinner.set_message(format!(
                    "Connecting to device at {} (address: 0x{:08X}) via OpenOCD GDB...",
                    addr, info.partition_address
                ));
                let gdb_flash = host_fs::flash::GdbFlash::new(
                    &addr,
                    info.partition_address,
                    info.partition_size,
                )
                .map_err(io::Error::other)?;
                EitherFlash::Gdb(Box::new(gdb_flash))
            } else {
                spinner.set_message(format!(
                    "Connecting to device (chip: {}, address: 0x{:08X}) via probe-rs...",
                    info.chip, info.partition_address
                ));
                let probe_flash =
                    ProbeFlash::new(&info.chip, info.partition_address, info.partition_size)
                        .map_err(io::Error::other)?;
                EitherFlash::Probe(Box::new(probe_flash))
            }
        }
    };

    let flash_range = partition_offset..partition_offset + partition_size_val;

    // Outer-scope variables to manage ELF file lifespans for Context references
    #[allow(unused_assignments)]
    let mut file_data = Vec::new();
    #[allow(unused_assignments)]
    let mut object_file = None;
    let mut context = None;
    let mut defmt_table = None;

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

    // Determine buffer size from project metadata partition size, falling back to flash capacity
    let buffer_size = if let Some(ref info) = info {
        info.partition_size
    } else {
        flash.capacity()
    };

    let mut unified_buf = vec![0u8; buffer_size];

    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();

        match &cli.command {
            Commands::Ls => {
                host_fs::commands::ls::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    &mut unified_buf,
                )
                .await?;
            }
            Commands::Cat { filename } => {
                host_fs::commands::cat::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    filename,
                    &mut unified_buf,
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
                    &mut unified_buf,
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
                    &mut unified_buf,
                )
                .await?;
            }
            Commands::Cp { src, dest } => {
                host_fs::commands::cp::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    host_fs::commands::cp::CpArgs {
                        src,
                        dest,
                        dump_option: &cli.dump,
                    },
                    &mut unified_buf,
                )
                .await?;
            }
            Commands::Rm { filename } => {
                host_fs::commands::rm::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    filename,
                    &cli.dump,
                    &mut unified_buf,
                )
                .await?;
            }
            Commands::Format => {
                if cli.dump.is_some() {
                    spinner.finish_and_clear();
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Formatting is only supported directly on target (remove --dump)",
                    ));
                }
                host_fs::commands::format::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    &mut unified_buf,
                )
                .await?;
            }
        }
        Ok::<(), io::Error>(())
    })?;

    Ok(())
}
