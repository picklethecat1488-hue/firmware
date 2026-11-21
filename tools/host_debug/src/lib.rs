//! Dual-core `probe-rs` GDB Remote Serial Protocol (RSP) server and non-intrusive RTT memory bridge.
//!
//! Exposes independent per-core GDB RSP TCP endpoints (`--gdb-core0-port`, `--gdb-core1-port`)
//! alongside a non-halting `:50000` RTT memory bridge (`--rtt-bridge-port`) over a single shared
//! [`DebugBackend`]. Architecture-specific register layouts (`target.xml` rendered via Rinja) and
//! ARM Cortex-M hardware FPB / SRAM `BKPT #0` handling live in [`arch`] and [`arm`].

#![deny(missing_docs)]

pub mod arch;
pub mod arm;
pub mod probe_status;
pub mod rsp;
pub mod server;
pub mod session;

pub use arch::{
    ProcessorArch, RegisterDescriptor, RegisterFeatureGroup, TargetArchProfile,
    TargetRegisterLayout,
};
pub use arm::{
    build_arm_cortex_m_register_layout, supports_arm_hw_breakpoint_at, ArmBreakpointManager,
    ArmCoreBreakpoints, ArmHardwareOps,
};
pub use rsp::{
    decode_hex, encode_hex, extract_next_rsp_frame, format_rsp_packet, RspAction, RspCoreHandler,
    RspFrame, RspReplyCode,
};
pub use server::{
    run_debug_server_with_backend, start_debug_server_with_backend, DebugBackend,
    DebugServerConfig, DebugServerEvent, DebugServerHandle,
};
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub use session::open_probe_session_with_retry;
pub use session::{retry_probe_attach, PROBE_ATTACH_MAX_ATTEMPTS, PROBE_ATTACH_RETRY_DELAY_MS};
