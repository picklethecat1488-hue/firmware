//! Common utilities shared across target-attached host CLI tools.

pub mod gdb;
pub mod metadata;
pub mod symbolicate;
pub mod telemetry;

pub use gdb::GdbClient;
pub use metadata::{autodetect_project_info, find_symbol_address, ProjectInfo};
pub use symbolicate::{print_crash_dump, symbolicate_addr, SymbolicatedFrame};
pub use telemetry::{FlashTelemetryParser, TelemetryParser, TraceTelemetryParser};
