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
            let info = tool_common::autodetect_project_info(std::path::Path::new(elf_path))
                .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

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
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
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

    // Determine buffer size from project metadata partition size, falling back to flash capacity
    let buffer_size = if let Some(ref path) = elf_path {
        if let Ok(info) = tool_common::autodetect_project_info(std::path::Path::new(path)) {
            info.partition_size
        } else {
            flash.capacity()
        }
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
                    src,
                    dest,
                    &cli.dump,
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
        }
        Ok::<(), io::Error>(())
    })?;

    Ok(())
}
