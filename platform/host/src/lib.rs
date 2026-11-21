//! Common utilities and network/probe constants shared across target-attached host CLI tools.

pub mod gdb;
pub mod metadata;
pub mod symbolicate;
pub mod telemetry;
pub mod transport;

/// Default SWD debug probe clock frequency in kHz (`1000` kHz = 1 MHz).
pub const DEFAULT_SWD_SPEED_KHZ: u32 = 1000;

/// Default TCP port for Core 0 GDB RSP connections (`3333`).
pub const DEFAULT_GDB_CORE0_PORT: u16 = 3333;

/// Default TCP port for Core 1 GDB RSP connections on dual-core targets (`3334`).
pub const DEFAULT_GDB_CORE1_PORT: u16 = 3334;

/// Default TCP port for the non-halting RTT memory bridge (`50000`).
pub const DEFAULT_RTT_BRIDGE_PORT: u16 = 50000;

/// Normalize a `host` or `host:port` GDB/RTT bridge endpoint, defaulting to [`DEFAULT_RTT_BRIDGE_PORT`].
pub fn normalize_rtt_bridge_addr(host_or_addr: &str) -> String {
    if host_or_addr.contains(':') {
        host_or_addr.to_string()
    } else {
        format!("{}:{}", host_or_addr, DEFAULT_RTT_BRIDGE_PORT)
    }
}

pub use gdb::GdbClient;
pub use metadata::{
    autodetect_project_info, discover_sibling_elf_tables, find_symbol_address,
    read_deferred_elf_table, CompactTargetMetadata, DeferredElfTable, ProjectInfo,
};
pub use platform::rtt::{
    decode_rtt_shell_flags, encode_rtt_shell_flags, RTT_CHANNEL_MODE_MASK, RTT_SHELL_EPOCH_SHIFT,
    RTT_SHELL_IDLE_FLAG,
};
pub use symbolicate::{
    print_crash_dump, resolve_breakpoint_target, symbolicate_addr, SymbolicatedFrame,
};
pub use telemetry::{FlashTelemetryParser, TelemetryParser, TraceTelemetryParser};
pub use transport::{
    EnumeratedInterfaceDevice, InterfaceAvailability, InterfaceStatusReport, TargetInterfaceKind,
};
