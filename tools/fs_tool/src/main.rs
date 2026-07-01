//! Host command-line utility for querying and exploring flash storage dumps
//! extracted from the RP2040 microcontroller's sequential-storage partition.

use clap::{Parser, Subcommand};
use embedded_storage::nor_flash::NorFlashErrorKind;
use embedded_storage_async::nor_flash::ReadNorFlash;
use std::cmp;
use std::fs::File;
use std::io::{self, Read, Write};

/// A mock flash driver that implements the embedded-storage-async traits
/// over an in-memory buffer containing the pulled raw flash binary image.
struct HostFlash {
    /// In-memory buffer representing the flash contents
    data: Vec<u8>,
}

impl HostFlash {
    /// Creates a new HostFlash instance with the provided byte buffer.
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl embedded_storage_async::nor_flash::ErrorType for HostFlash {
    type Error = NorFlashErrorKind;
}

impl embedded_storage_async::nor_flash::ReadNorFlash for HostFlash {
    const READ_SIZE: usize = 1;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            bytes.copy_from_slice(&self.data[start..end]);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl embedded_storage_async::nor_flash::NorFlash for HostFlash {
    const WRITE_SIZE: usize = 4;
    const ERASE_SIZE: usize = 1024;

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        let start = offset as usize;
        let end = start + bytes.len();
        if end <= self.data.len() {
            self.data[start..end].copy_from_slice(bytes);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let start = from as usize;
        let end = to as usize;
        if end <= self.data.len() {
            self.data[start..end].fill(0xFF);
            Ok(())
        } else {
            Err(NorFlashErrorKind::OutOfBounds)
        }
    }
}

impl embedded_storage_async::nor_flash::MultiwriteNorFlash for HostFlash {}

/// Helper utility to hash or pad a string filename into a 32-byte key
/// used by the sequential-storage map.
fn string_to_key(name: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let bytes = name.as_bytes();
    let len = cmp::min(bytes.len(), 32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

/// Known semantic data types stored in our flash filesystem.
enum DataType {
    /// Protobuf calibration parameters
    Calibration,
    /// CBOR periodic telemetry recordings
    Telemetry,
    /// Saved stack traces and panic info from MCU crash logs
    CrashLog,
    /// Untyped or arbitrary raw binary data
    Unknown,
}

impl DataType {
    /// Maps a filename to its known semantic data type.
    fn from_filename(name: &str) -> Self {
        match name {
            "calibration.bin" | "calibration.protobuf" => DataType::Calibration,
            "telemetry.bin" | "telemetry.cbor" | "telemetry.rrd" => DataType::Telemetry,
            "crash.log" | "crash.cbor" => DataType::CrashLog,
            _ => DataType::Unknown,
        }
    }

    /// Returns a human-readable description of the data type.
    fn to_str(&self) -> &'static str {
        match self {
            DataType::Calibration => "Protobuf Calibration Data",
            DataType::Telemetry => "CBOR Telemetry Data",
            DataType::CrashLog => "CBOR Crash Log / Stack Trace",
            DataType::Unknown => "Raw Binary / Unknown Type",
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Host utility for querying sequential-storage filesystem dumps from RP2040", long_about = None)]
struct Cli {
    /// Path to the binary flash dump file (e.g. flash_dump.bin)
    #[arg(short, long)]
    dump: String,

    /// Subcommand to run against the dump
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all files in the partition along with their data types
    Ls,
    /// Print file contents
    Cat {
        /// Name of the file to print
        filename: String,
    },
    /// Export telemetry.rrd to a CSV file for plotting or visualization (e.g. in Rerun)
    ExportTelemetry {
        /// Output path for the CSV file (e.g. telemetry.csv)
        out_csv: String,
    },
    /// Read and decode all stored crash dumps into human-readable backtraces
    CrashLog {
        /// Optional path to the ELF binary containing debug symbols (e.g. target/thumbv6m-none-eabi/debug/cat_detector)
        #[arg(short, long)]
        elf: Option<String>,
    },
}

/// Main entry point for the host-side fs_tool utility.
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

    spinner.set_message("Loading flash dump file...");

    // Read flash dump file
    let mut file = File::open(&cli.dump)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut flash = HostFlash::new(data);
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
        object_file = Some(
            object::File::parse(&*file_data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
        context = Some(
            addr2line::Context::new(object_file.as_ref().unwrap())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
    }

    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();

        match &cli.command {
            Commands::Ls => {
                spinner.set_message("Reading directory (.dir)...");
                let mut dir_buf = [0u8; 512];
                let key = string_to_key(".dir");
                let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut flash,
                    flash_range.clone(),
                    &mut cache,
                    &mut dir_buf,
                    &key,
                )
                .await;

                spinner.finish_and_clear();

                match res {
                    Ok(Some(list)) => {
                        if let Ok(s) = std::str::from_utf8(list) {
                            println!("{:<24} | Data Type / Format", "Filename");
                            println!("{}", "-".repeat(50));
                            for line in s.split('\n') {
                                if !line.is_empty() {
                                    let dt = DataType::from_filename(line);
                                    println!("{:<24} | {}", line, dt.to_str());
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        println!("No files found (directory empty).");
                    }
                    Err(e) => {
                        eprintln!("Error reading directory: {:?}", e);
                    }
                }
            }
            Commands::Cat { filename } => {
                spinner.set_message(format!("Reading {}...", filename));
                let key = string_to_key(filename);
                let mut out_buf = vec![0u8; 1024 * 16]; // support up to 16KB files

                let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut flash,
                    flash_range.clone(),
                    &mut cache,
                    &mut out_buf,
                    &key,
                )
                .await;

                spinner.finish_and_clear();

                match res {
                    Ok(Some(content)) => {
                        // Check if content is UTF-8 text or binary
                        if let Ok(text) = std::str::from_utf8(content) {
                            print!("{}", text);
                        } else {
                            // Print hex dump for binary contents
                            println!("(Binary content, {} bytes)", content.len());
                            for chunk in content.chunks(16) {
                                for byte in chunk {
                                    print!("{:02X} ", byte);
                                }
                                println!();
                            }
                        }
                    }
                    Ok(None) => {
                        eprintln!("File not found: {}", filename);
                        std::process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {:?}", e);
                        std::process::exit(1);
                    }
                }
            }
            Commands::ExportTelemetry { out_csv } => {
                spinner.set_message("Fetching telemetry.rrd from filesystem...");
                let key = string_to_key("telemetry.rrd");
                let mut out_buf = vec![0u8; 1024 * 16]; // support up to 16KB files

                let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut flash,
                    flash_range.clone(),
                    &mut cache,
                    &mut out_buf,
                    &key,
                )
                .await;

                match res {
                    Ok(Some(content)) => {
                        if content.len() < 12 {
                            spinner.finish_and_clear();
                            eprintln!(
                                "Error: Telemetry file is too short ({} bytes)",
                                content.len()
                            );
                            std::process::exit(1);
                        }

                        spinner.set_message("Parsing CBOR telemetry records...");
                        let len = content[0] as usize;
                        if len == 0 || len > 11 {
                            spinner.finish_and_clear();
                            eprintln!("Error: Invalid telemetry header length byte ({})", len);
                            std::process::exit(1);
                        }

                        let payload = &content[1..1 + len];
                        let mut decoder = minicbor::Decoder::new(payload);
                        let mut count = 0;
                        let mut next_idx = 0;
                        if let Ok(_array_len) = decoder.array() {
                            if let Ok(c) = decoder.u32() {
                                if let Ok(n) = decoder.u32() {
                                    count = c as usize;
                                    next_idx = n as usize;
                                }
                            }
                        }

                        let max_records = 45;
                        if count > max_records || next_idx > max_records {
                            spinner.finish_and_clear();
                            eprintln!("Error: Invalid header count/next_idx in telemetry file");
                            std::process::exit(1);
                        }

                        let mut records = Vec::new();

                        let mut process_record = |offset: usize| {
                            if offset + 20 <= content.len() {
                                let slot: &[u8; 20] =
                                    content[offset..offset + 20].try_into().unwrap();
                                if let Some((ts, rec)) =
                                    model::telemetry::TelemetryRecord::deserialize(slot)
                                {
                                    records.push((ts, rec));
                                }
                            }
                        };

                        if count < max_records {
                            for i in 0..count {
                                process_record(12 + i * 20);
                            }
                        } else {
                            for i in 0..max_records {
                                let idx = (next_idx + i) % max_records;
                                process_record(12 + idx * 20);
                            }
                        }

                        spinner.set_message(format!("Writing records to {}...", out_csv));
                        let mut csv_file = File::create(out_csv)?;
                        writeln!(csv_file, "timestamp_us,record_type,val1,val2,val3,val4")?;

                        for (ts, rec) in records {
                            match rec {
                                model::telemetry::TelemetryRecord::Battery(b) => match b {
                                    model::types::BatteryStatus::VolTempState(vol, temp, state) => {
                                        writeln!(
                                            csv_file,
                                            "{},Battery,{},{},{:?},",
                                            ts, vol, temp, state
                                        )?;
                                    }
                                },
                                model::telemetry::TelemetryRecord::Motor(m) => match m {
                                    model::types::MotorStatus::SpeedRunTemp(
                                        speed,
                                        running,
                                        temp,
                                    ) => {
                                        writeln!(
                                            csv_file,
                                            "{},Motor,{},{},{},",
                                            ts, speed, running, temp
                                        )?;
                                    }
                                },
                                model::telemetry::TelemetryRecord::Thermal(t) => match t {
                                    model::types::ThermalStatus::TempOverheating(
                                        temp,
                                        overheating,
                                    ) => {
                                        writeln!(
                                            csv_file,
                                            "{},Thermal,{},{},,",
                                            ts, temp, overheating
                                        )?;
                                    }
                                },
                                model::telemetry::TelemetryRecord::System(s) => {
                                    writeln!(csv_file, "{},System,{:?},,,", ts, s)?;
                                }
                                model::telemetry::TelemetryRecord::FuelGauge(fg) => match fg {
                                    model::types::FuelGaugeTelemetry::VolSoc(vol, soc) => {
                                        writeln!(csv_file, "{},FuelGauge,{},{},,", ts, vol, soc)?;
                                    }
                                },
                                model::telemetry::TelemetryRecord::Proximity(p) => match p {
                                    model::types::ProximityTelemetry::InRange(d) => {
                                        writeln!(csv_file, "{},Proximity,InRange,{},,", ts, d)?;
                                    }
                                    model::types::ProximityTelemetry::OutRange(d) => {
                                        writeln!(csv_file, "{},Proximity,OutRange,{},,", ts, d)?;
                                    }
                                },
                                model::telemetry::TelemetryRecord::Led(led) => {
                                    writeln!(csv_file, "{},Led,{:?},,,", ts, led)?;
                                }
                                model::telemetry::TelemetryRecord::Gesture(g) => {
                                    writeln!(csv_file, "{},Gesture,{:?},,,", ts, g)?;
                                }
                                model::telemetry::TelemetryRecord::FlashTelemetry(ft) => {
                                    writeln!(
                                        csv_file,
                                        "{},FlashTelemetry,{},{},{},",
                                        ts, ft.sector, ft.duration_ms, ft.erase_count
                                    )?;
                                }
                            }
                        }

                        spinner.finish_with_message(format!(
                            "Successfully exported {} telemetry records to {}",
                            count, out_csv
                        ));
                    }
                    Ok(None) => {
                        spinner.finish_and_clear();
                        eprintln!("File not found: telemetry.rrd");
                        std::process::exit(1);
                    }
                    Err(e) => {
                        spinner.finish_and_clear();
                        eprintln!("Error reading file: {:?}", e);
                        std::process::exit(1);
                    }
                }
            }
            Commands::CrashLog { elf: _ } => {
                spinner.set_message("Fetching directory list (.dir)...");
                let mut dir_buf = [0u8; 512];
                let key = string_to_key(".dir");
                let res = sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                    &mut flash,
                    flash_range.clone(),
                    &mut cache,
                    &mut dir_buf,
                    &key,
                )
                .await;

                spinner.finish_and_clear();

                match res {
                    Ok(Some(list)) => {
                        if let Ok(s) = std::str::from_utf8(list) {
                            let mut found_crash = false;
                            for filename in s.split('\n') {
                                if filename.starts_with("crash_") && filename.ends_with(".log") {
                                    found_crash = true;
                                    let log_key = string_to_key(filename);
                                    let mut out_buf = vec![0u8; 1024 * 16];
                                    let content_res =
                                        sequential_storage::map::fetch_item::<[u8; 32], &[u8], _>(
                                            &mut flash,
                                            flash_range.clone(),
                                            &mut cache,
                                            &mut out_buf,
                                            &log_key,
                                        )
                                        .await;

                                    match content_res {
                                        Ok(Some(content)) => {
                                            if let Ok(text) = std::str::from_utf8(content) {
                                                let mut in_backtrace = false;
                                                for line in text.lines() {
                                                    if line.starts_with("Backtrace:") {
                                                        in_backtrace = true;
                                                        println!("{}", line);
                                                        continue;
                                                    }
                                                    if in_backtrace {
                                                        if line.trim().is_empty()
                                                            || line.starts_with("System Logs:")
                                                        {
                                                            in_backtrace = false;
                                                        } else if line.trim().starts_with("0x") {
                                                            let addr_str = line
                                                                .trim()
                                                                .trim_start_matches("0x");
                                                            if let Ok(addr) =
                                                                u64::from_str_radix(addr_str, 16)
                                                            {
                                                                if let Some(ctx) = &context {
                                                                    match ctx
                                                                        .find_frames(addr)
                                                                        .skip_all_loads()
                                                                    {
                                                                        Ok(mut frames) => {
                                                                            let mut found = false;
                                                                            while let Ok(Some(
                                                                                frame,
                                                                            )) = frames.next()
                                                                            {
                                                                                found = true;
                                                                                let func_name = if let Some(f) = &frame.function {
                                                                                    let raw = f.raw_name().unwrap_or(std::borrow::Cow::Borrowed("??"));
                                                                                    format!("{:#}", rustc_demangle::demangle(&raw))
                                                                                } else {
                                                                                    "??".to_string()
                                                                                };
                                                                                if let Some(loc) =
                                                                                    frame.location
                                                                                {
                                                                                    println!(
                                                                                        "  0x{:08X} - {} ({}:{})",
                                                                                        addr,
                                                                                        func_name,
                                                                                        loc.file.unwrap_or("??"),
                                                                                        loc.line.unwrap_or(0)
                                                                                    );
                                                                                } else {
                                                                                    println!("  0x{:08X} - {} (??:0)", addr, func_name);
                                                                                }
                                                                            }
                                                                            if !found {
                                                                                println!("  0x{:08X} - (no symbol found)", addr);
                                                                            }
                                                                        }
                                                                        Err(_) => {
                                                                            println!("  0x{:08X} - (symbolication error)", addr);
                                                                        }
                                                                    }
                                                                } else {
                                                                    println!("{}", line);
                                                                }
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    println!("{}", line);
                                                }
                                            } else {
                                                println!(
                                                    "(Binary crash log, {} bytes)",
                                                    content.len()
                                                );
                                            }
                                        }
                                        _ => {
                                            eprintln!(
                                                "Failed to read crash log content for {}",
                                                filename
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        println!("No files found (directory empty).");
                    }
                    Err(e) => {
                        eprintln!("Error reading directory: {:?}", e);
                    }
                }
            }
        }
        Ok::<(), io::Error>(())
    })?;

    Ok(())
}
