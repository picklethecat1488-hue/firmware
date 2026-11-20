//! Dual-core `probe-rs` GDB Remote Serial Protocol (RSP) server and non-intrusive RTT memory bridge.
//!
//! Exposes independent per-core GDB RSP TCP endpoints (`--gdb-core0-port`, `--gdb-core1-port`)
//! alongside a non-halting `:50000` RTT memory bridge (`--rtt-bridge-port`) over a single shared
//! `probe_rs::Session`. Automatically re-arms secondary-core hardware FPB comparators across
//! RP2040 `PSM.frce_off` power-on resets and supports software `BKPT #0` breakpoints in SRAM.

#![deny(missing_docs)]

pub mod rsp;
pub mod server;

pub use rsp::{
    build_target_xml, decode_hex, encode_hex, extract_next_rsp_frame, format_rsp_packet,
    CoreBreakpointTable, RspAction, RspCoreHandler, RspFrame,
};
pub use server::{
    run_debug_server_with_backend, start_debug_server_with_backend, DebugBackend,
    DebugServerConfig, DebugServerEvent, DebugServerHandle,
};
