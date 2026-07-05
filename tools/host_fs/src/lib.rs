//! Library module for host-side flash storage tools.

use clap::{Parser, Subcommand};
use std::cmp;

pub mod commands;
pub mod flash;

pub use flash::{HostFlash, ProbeFlash};

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
        if name.starts_with("crash_") && (name.ends_with(".log") || name.ends_with(".cbor")) {
            return DataType::CrashLog;
        }
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
    /// Path to the binary flash dump file (e.g. flash_dump.bin).
    /// If not specified, direct attachment to the device via probe-rs is used.
    #[arg(short, long)]
    pub dump: Option<String>,

    /// Path to the ELF binary containing project metadata and symbols
    #[arg(short, long)]
    pub elf: Option<String>,

    /// Automatically detect chip and layout parameters from the ELF's metadata section
    #[arg(short, long)]
    pub autodetect: bool,

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
        /// Optional path to the ELF binary containing debug symbols (e.g. target/thumbv6m-none-eabi/debug/cat_detector/app)
        #[arg(short, long)]
        elf: Option<String>,
    },
    /// Copy a file to or from the device flash partition
    Cp {
        /// Source path (prefix with 'dev:' for device files, e.g. dev:telemetry.rrd)
        src: String,
        /// Destination path (prefix with 'dev:' for device files, e.g. dev:telemetry.rrd)
        dest: String,
    },
}
