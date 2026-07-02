//! Host command-line utility for querying and exploring flash storage dumps
//! extracted from the RP2040 microcontroller's sequential-storage partition.

use clap::Parser;
use fs_tool::flash::{decode_project_info, EitherFlash, ProbeFlash};
use fs_tool::{Cli, Commands, HostFlash};
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
            let project_name = cli.project.as_ref().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Project name is required when --dump is not specified",
                )
            })?;
            let (chip, base_addr, size) = decode_project_info(project_name)
                .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

            spinner.set_message(format!(
                "Connecting to device (chip: {}, address: 0x{:08X}) via probe-rs...",
                chip, base_addr
            ));
            let probe_flash = ProbeFlash::new(&chip, base_addr, size)
                .map_err(io::Error::other)?;
            EitherFlash::Probe(Box::new(probe_flash))
        }
    };

    use embedded_storage_async::nor_flash::ReadNorFlash;
    let flash_range = 0..flash.capacity() as u32;

    // Outer-scope variables to manage ELF file lifespans for Context references
    #[allow(unused_assignments)]
    let mut file_data = Vec::new();
    #[allow(unused_assignments)]
    let mut object_file = None;
    let mut context = None;

    if let Commands::CrashLog {
        elf: Some(elf_path),
    } = &cli.command
    {
        spinner.set_message("Loading ELF and DWARF debug symbols...");
        file_data = std::fs::read(elf_path)?;
        let obj = object::File::parse(&*file_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        object_file = Some(obj);
        let ctx = addr2line::Context::new(object_file.as_ref().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        context = Some(ctx);
    }

    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();

        match &cli.command {
            Commands::Ls => {
                fs_tool::commands::ls::run(&mut flash, flash_range, &mut cache, &spinner).await?;
            }
            Commands::Cat { filename } => {
                fs_tool::commands::cat::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    filename,
                )
                .await?;
            }
            Commands::ExportTelemetry { out_csv } => {
                fs_tool::commands::export_telemetry::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    out_csv,
                )
                .await?;
            }
            Commands::CrashLog { .. } => {
                fs_tool::commands::crash_log::run(
                    &mut flash,
                    flash_range,
                    &mut cache,
                    &spinner,
                    &context,
                )
                .await?;
            }
            Commands::Cp { src, dest } => {
                fs_tool::commands::cp::run(
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
