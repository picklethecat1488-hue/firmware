//! Library module for host-side flash storage tools.

use clap::{Parser, Subcommand};
use embedded_storage::nor_flash::NorFlashErrorKind;

use std::cmp;

/// A mock flash driver that implements the embedded-storage-async traits
/// over an in-memory buffer containing the pulled raw flash binary image.
pub struct HostFlash {
    /// In-memory buffer representing the flash contents
    pub data: Vec<u8>,
}

impl HostFlash {
    /// Creates a new HostFlash instance with the provided byte buffer.
    pub fn new(data: Vec<u8>) -> Self {
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
pub fn string_to_key(name: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let bytes = name.as_bytes();
    let len = cmp::min(bytes.len(), 32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

/// Known semantic data types stored in our flash filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
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
    pub fn from_filename(name: &str) -> Self {
        match name {
            "calibration.bin" | "calibration.protobuf" => DataType::Calibration,
            "telemetry.bin" | "telemetry.cbor" | "telemetry.rrd" => DataType::Telemetry,
            "crash.log" | "crash.cbor" => DataType::CrashLog,
            _ => DataType::Unknown,
        }
    }

    /// Returns a human-readable description of the data type.
    pub fn to_str(&self) -> &'static str {
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
pub struct Cli {
    /// Path to the binary flash dump file (e.g. flash_dump.bin)
    #[arg(short, long)]
    pub dump: String,

    /// Subcommand to run against the dump
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
