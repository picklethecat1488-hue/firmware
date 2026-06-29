//! Host command-line utility for querying and exploring flash storage dumps
//! extracted from the RP2040 microcontroller's sequential-storage partition.

use clap::{Parser, Subcommand};
use embedded_storage::nor_flash::NorFlashErrorKind;
use embedded_storage_async::nor_flash::ReadNorFlash;
use std::cmp;
use std::fs::File;
use std::io::{self, Read};

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
            "telemetry.bin" | "telemetry.cbor" => DataType::Telemetry,
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
}

/// Main entry point for the host-side fs_tool utility.
fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read flash dump file
    let mut file = File::open(&cli.dump)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut flash = HostFlash::new(data);
    let flash_range = 0..flash.capacity() as u32;

    futures::executor::block_on(async {
        let mut cache = sequential_storage::cache::NoCache::new();

        match &cli.command {
            Commands::Ls => {
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
        }
    });

    Ok(())
}
