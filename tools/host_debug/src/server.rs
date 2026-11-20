//! Multi-listener TCP GDB RSP server, non-halting RTT memory bridge, and `probe-rs` session backend.

use crate::rsp::{
    decode_hex, encode_hex, extract_next_rsp_frame, format_rsp_packet, unescape_rsp_binary,
    CoreBreakpointTable, RspAction, RspCoreHandler, RspFrame, DCRSR_PC, MAX_RSP_MEMORY_READ_BYTES,
};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Default TCP port for Core 0 GDB RSP attachments.
pub const DEFAULT_GDB_CORE0_PORT: u16 = 3333;

/// Default TCP port for Core 1 GDB RSP attachments on dual-core targets.
pub const DEFAULT_GDB_CORE1_PORT: u16 = 3334;

/// Default TCP port for the non-halting RTT memory bridge (`host_cli --gdb-bridge`).
pub const DEFAULT_RTT_BRIDGE_PORT: u16 = 50000;

/// Default SWD probe clock frequency in kHz.
pub const DEFAULT_SWD_SPEED_KHZ: u32 = 1000;

/// Cortex-M Debug Halting Control and Status Register (`DHCSR`) address.
pub const CORTEX_M_DHCSR_ADDR: u64 = 0xE000_EDF0;

/// `DHCSR` write value (`DBGKEY = 0xA05F << 16 | C_DEBUGEN = 1`) enabling halting debug.
pub const CORTEX_M_DHCSR_DBGKEY_ENABLE: u32 = 0xA05F_0001;

/// Cortex-M Flash Patch / Breakpoint Control Register (`FP_CTRL` / `BP_CTRL`) address.
pub const CORTEX_M_BP_CTRL_ADDR: u64 = 0xE000_2000;

/// `FP_CTRL` / `BP_CTRL` write value (`KEY = 1 << 1 | ENABLE = 1`) enabling hardware comparators.
pub const CORTEX_M_BP_CTRL_KEY_ENABLE: u32 = 0x0000_0003;

/// Base address of Cortex-M Breakpoint Comparator Register 0 (`FP_COMP0` / `BP_COMP0`).
pub const CORTEX_M_BP_COMP0_ADDR: u64 = 0xE000_2008;

/// Exclusive upper address bound for ARMv6-M BPU v1 hardware comparators (`0x2000_0000`).
pub const ARMV6M_HW_BP_LIMIT: u64 = 0x2000_0000;

/// Poll interval while waiting for a running core to hit a breakpoint or receive Ctrl-C (`0x03`).
pub const CORE_POLL_INTERVAL_MS: u64 = 5;

/// Listener accept loop poll interval allowing clean cooperative shutdown.
pub const ACCEPT_POLL_INTERVAL_MS: u64 = 25;

/// Hardware abstraction trait decoupling the GDB RSP server from `probe_rs::Session`.
pub trait DebugBackend: Send {
    /// Human-readable target chip identifier (e.g. `"rp2040"`, `"MCXN947"`).
    fn chip_name(&self) -> &str;

    /// Number of debuggable processor cores exposed by this target session.
    fn core_count(&self) -> usize;

    /// Return `true` if `core_idx` exposes ARM VFP single-precision floating-point registers.
    fn core_has_fpu(&mut self, core_idx: usize) -> bool;

    /// Return the number of hardware FPB breakpoint comparators on `core_idx`.
    fn available_hw_breakpoints(&mut self, core_idx: usize) -> usize;

    /// Return `true` if `core_idx` hardware FPB comparators can match instruction fetches at `addr`.
    fn supports_hw_breakpoint_at(&self, core_idx: usize, addr: u64) -> bool;

    /// Return `true` if `core_idx` is currently in Debug Halt state.
    fn core_halted(&mut self, core_idx: usize) -> Result<bool, String>;

    /// Request `core_idx` to enter Debug Halt state and return its halted `PC`.
    fn halt_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Resume execution on `core_idx`.
    fn resume_core(&mut self, core_idx: usize) -> Result<(), String>;

    /// Single-step one instruction on `core_idx` and return the new `PC`.
    fn step_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Reset `core_idx` and halt immediately at `Reset_Handler` via vector catch.
    fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Reset `core_idx` and resume normal execution.
    fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String>;

    /// Read a 32-bit architectural register (`dcrsr`) on `core_idx`.
    fn read_core_reg(&mut self, core_idx: usize, dcrsr: u16) -> Result<u32, String>;

    /// Write a 32-bit architectural register (`dcrsr`) on `core_idx`.
    fn write_core_reg(&mut self, core_idx: usize, dcrsr: u16, value: u32) -> Result<(), String>;

    /// Read `buf.len()` bytes of target memory at `addr` without halting `core_idx`.
    fn read_memory(&mut self, core_idx: usize, addr: u64, buf: &mut [u8]) -> Result<(), String>;

    /// Write `data` bytes to target memory at `addr` without halting `core_idx`.
    fn write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String>;

    /// Program hardware FPB comparator `slot_idx` on `core_idx` to match `addr`.
    fn set_hw_breakpoint_slot(
        &mut self,
        core_idx: usize,
        slot_idx: usize,
        addr: u64,
    ) -> Result<(), String>;

    /// Disable hardware FPB comparator `slot_idx` (`addr`) on `core_idx`.
    fn clear_hw_breakpoint_slot(
        &mut self,
        core_idx: usize,
        slot_idx: usize,
        addr: u64,
    ) -> Result<(), String>;

    /// Inspect live `BP_CTRL` / `BP_COMP` registers on `core_idx` and re-arm comparators if cleared by reset.
    fn ensure_hw_breakpoints_active(
        &mut self,
        core_idx: usize,
        slots: &[Option<u64>],
    ) -> Result<bool, String>;
}

/// Configuration parameters for launching `host_debug`.
#[derive(Clone, Debug)]
pub struct DebugServerConfig {
    /// Host interface address to bind (`"127.0.0.1"`).
    pub bind_host: String,
    /// Desired Core 0 GDB RSP TCP port (`0` selects an OS-assigned ephemeral port).
    pub gdb_core0_port: u16,
    /// Optional Core 1 GDB RSP TCP port on dual-core targets (`None` disables Core 1 listener; `Some(0)` selects ephemeral port).
    pub gdb_core1_port: Option<u16>,
    /// Optional non-halting RTT memory bridge TCP port (`None` disables bridge; `Some(0)` selects ephemeral port).
    pub rtt_bridge_port: Option<u16>,
    /// Whether `core0.reset_and_halt()` was executed prior to serving connections.
    pub halted_on_reset: bool,
    /// Optional Core 0 post-`PSM.frce_off.proc1` instruction address used to re-arm Core 1 FPB before FIFO jump.
    pub core1_post_reset_trap_addr: Option<u64>,
    /// Emit structured line-delimited JSON lifecycle events to stdout.
    pub json_events: bool,
}

impl Default for DebugServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            gdb_core0_port: DEFAULT_GDB_CORE0_PORT,
            gdb_core1_port: Some(DEFAULT_GDB_CORE1_PORT),
            rtt_bridge_port: Some(DEFAULT_RTT_BRIDGE_PORT),
            halted_on_reset: false,
            core1_post_reset_trap_addr: None,
            json_events: false,
        }
    }
}

/// Structured lifecycle event emitted by `host_debug`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DebugServerEvent {
    /// All configured TCP listeners are bound and ready for GDB / `host_cli` connections.
    Ready {
        /// Target chip name.
        chip: String,
        /// Number of target cores.
        cores: usize,
        /// Bound Core 0 GDB RSP port.
        gdb_core0_port: u16,
        /// Bound Core 1 GDB RSP port (when `cores >= 2` and enabled).
        gdb_core1_port: Option<u16>,
        /// Bound RTT memory bridge port (when enabled).
        rtt_bridge_port: Option<u16>,
        /// True when Core 0 is halted at `Reset_Handler` awaiting GDB attachment.
        halted_on_reset: bool,
    },
    /// A GDB client attached or detached on `core_idx`.
    ClientState {
        /// Target core index (`0` or `1`).
        core: usize,
        /// Whether a GDB client is currently connected.
        connected: bool,
    },
    /// Server stopped cleanly and released the debug probe session.
    Stopped,
}

impl DebugServerEvent {
    /// Serialize this event as a single-line JSON string.
    pub fn to_json_line(&self) -> String {
        match self {
            Self::Ready {
                chip,
                cores,
                gdb_core0_port,
                gdb_core1_port,
                rtt_bridge_port,
                halted_on_reset,
            } => json!({
                "event": "ready",
                "chip": chip,
                "cores": cores,
                "gdb_core0_port": gdb_core0_port,
                "gdb_core1_port": gdb_core1_port,
                "rtt_bridge_port": rtt_bridge_port,
                "halted_on_reset": halted_on_reset,
            })
            .to_string(),
            Self::ClientState { core, connected } => json!({
                "event": if *connected { "gdb_connected" } else { "gdb_disconnected" },
                "core": core,
                "connected": connected,
            })
            .to_string(),
            Self::Stopped => json!({ "event": "stopped" }).to_string(),
        }
    }
}

/// Handle to a running background debug server instance (used by CLI main and TCP integration tests).
pub struct DebugServerHandle {
    /// Bound Core 0 GDB RSP TCP port.
    pub gdb_core0_port: u16,
    /// Bound Core 1 GDB RSP TCP port, if active.
    pub gdb_core1_port: Option<u16>,
    /// Bound non-halting RTT memory bridge TCP port, if active.
    pub rtt_bridge_port: Option<u16>,
    shutdown: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl DebugServerHandle {
    /// Signal all listener and client threads to terminate and join them cleanly.
    pub fn stop(mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}

fn emit_event(event: &DebugServerEvent, json_events: bool) {
    if json_events {
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{}", event.to_json_line());
        let _ = out.flush();
    } else if let DebugServerEvent::Ready {
        chip,
        cores,
        gdb_core0_port,
        gdb_core1_port,
        rtt_bridge_port,
        halted_on_reset,
    } = event
    {
        let core1_desc = gdb_core1_port
            .map(|p| format!(", Core 1 GDB :{}", p))
            .unwrap_or_default();
        let rtt_desc = rtt_bridge_port
            .map(|p| format!(", RTT bridge :{}", p))
            .unwrap_or_default();
        println!(
            "host_debug ready on '{}' ({} cores): Core 0 GDB :{}{}{} (halted_on_reset={})",
            chip, cores, gdb_core0_port, core1_desc, rtt_desc, halted_on_reset
        );
    }
}

/// Spawn non-blocking GDB RSP and RTT bridge TCP listeners backed by `backend`.
pub fn start_debug_server_with_backend<B: DebugBackend + 'static>(
    mut backend: B,
    config: &DebugServerConfig,
    shutdown: Arc<AtomicBool>,
) -> Result<DebugServerHandle, String> {
    let chip = backend.chip_name().to_string();
    let cores = backend.core_count();
    let core0_fpu = backend.core_has_fpu(0);
    let core0_bp_units = backend.available_hw_breakpoints(0);
    let core1_meta = if cores >= 2 {
        Some((backend.core_has_fpu(1), backend.available_hw_breakpoints(1)))
    } else {
        None
    };

    let core0_listener = TcpListener::bind((config.bind_host.as_str(), config.gdb_core0_port))
        .map_err(|e| {
            format!(
                "Failed to bind Core 0 GDB port {}: {}",
                config.gdb_core0_port, e
            )
        })?;
    core0_listener
        .set_nonblocking(true)
        .map_err(|e| e.to_string())?;
    let actual_core0_port = core0_listener
        .local_addr()
        .map_err(|e| e.to_string())?
        .port();

    let (core1_listener, actual_core1_port) = match (cores >= 2, config.gdb_core1_port) {
        (true, Some(req_port)) => {
            let listener = TcpListener::bind((config.bind_host.as_str(), req_port))
                .map_err(|e| format!("Failed to bind Core 1 GDB port {}: {}", req_port, e))?;
            listener.set_nonblocking(true).map_err(|e| e.to_string())?;
            let bound = listener.local_addr().map_err(|e| e.to_string())?.port();
            (Some(listener), Some(bound))
        }
        _ => (None, None),
    };

    let (rtt_listener, actual_rtt_port) = match config.rtt_bridge_port {
        Some(req_port) => {
            let listener = TcpListener::bind((config.bind_host.as_str(), req_port))
                .map_err(|e| format!("Failed to bind RTT bridge port {}: {}", req_port, e))?;
            listener.set_nonblocking(true).map_err(|e| e.to_string())?;
            let bound = listener.local_addr().map_err(|e| e.to_string())?.port();
            (Some(listener), Some(bound))
        }
        None => (None, None),
    };

    let shared_backend = Arc::new(Mutex::new(backend));
    let json_events = config.json_events;
    let mut threads = Vec::new();

    let bp_c0 = Arc::new(Mutex::new(CoreBreakpointTable::new(core0_bp_units)));
    let bp_c1 = core1_meta
        .as_ref()
        .map(|(_, units)| Arc::new(Mutex::new(CoreBreakpointTable::new(*units))));
    let post_reset_coord = Arc::new(Core1PostResetCoordinator {
        trap_addr: config.core1_post_reset_trap_addr.map(|a| a & !1),
        armed_core0_slot: Mutex::new(None),
        bp_c0: Arc::clone(&bp_c0),
        bp_c1: bp_c1.clone(),
    });

    {
        let backend_c0 = Arc::clone(&shared_backend);
        let shutdown_c0 = Arc::clone(&shutdown);
        let bp_c0_clone = Arc::clone(&bp_c0);
        let coord_c0 = Arc::clone(&post_reset_coord);
        threads.push(thread::spawn(move || {
            run_core_gdb_listener(
                core0_listener,
                0,
                core0_fpu,
                backend_c0,
                bp_c0_clone,
                coord_c0,
                shutdown_c0,
                json_events,
            );
        }));
    }

    if let (Some(listener), Some((core1_fpu, _)), Some(bp_c1_arc)) =
        (core1_listener, core1_meta, bp_c1)
    {
        let backend_c1 = Arc::clone(&shared_backend);
        let shutdown_c1 = Arc::clone(&shutdown);
        let coord_c1 = Arc::clone(&post_reset_coord);
        threads.push(thread::spawn(move || {
            run_core_gdb_listener(
                listener,
                1,
                core1_fpu,
                backend_c1,
                bp_c1_arc,
                coord_c1,
                shutdown_c1,
                json_events,
            );
        }));
    }

    if let Some(listener) = rtt_listener {
        let backend_rtt = Arc::clone(&shared_backend);
        let shutdown_rtt = Arc::clone(&shutdown);
        threads.push(thread::spawn(move || {
            run_rtt_bridge_listener(listener, backend_rtt, shutdown_rtt);
        }));
    }

    emit_event(
        &DebugServerEvent::Ready {
            chip,
            cores,
            gdb_core0_port: actual_core0_port,
            gdb_core1_port: actual_core1_port,
            rtt_bridge_port: actual_rtt_port,
            halted_on_reset: config.halted_on_reset,
        },
        json_events,
    );

    Ok(DebugServerHandle {
        gdb_core0_port: actual_core0_port,
        gdb_core1_port: actual_core1_port,
        rtt_bridge_port: actual_rtt_port,
        shutdown,
        threads,
    })
}

/// Run the debug server synchronously until `shutdown` becomes `true`.
pub fn run_debug_server_with_backend<B: DebugBackend + 'static>(
    backend: B,
    config: &DebugServerConfig,
    shutdown: Arc<AtomicBool>,
) -> Result<(), String> {
    let json_events = config.json_events;
    let handle = start_debug_server_with_backend(backend, config, Arc::clone(&shutdown))?;
    while !shutdown.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
    }
    handle.stop();
    emit_event(&DebugServerEvent::Stopped, json_events);
    Ok(())
}

struct Core1PostResetCoordinator {
    trap_addr: Option<u64>,
    armed_core0_slot: Mutex<Option<usize>>,
    bp_c0: Arc<Mutex<CoreBreakpointTable>>,
    bp_c1: Option<Arc<Mutex<CoreBreakpointTable>>>,
}

impl Core1PostResetCoordinator {
    fn sync_trap<B: DebugBackend + ?Sized>(&self, backend: &mut B) {
        let (Some(trap_addr), Some(ref bp_c1)) = (self.trap_addr, &self.bp_c1) else {
            return;
        };
        let c1_needs = bp_c1
            .lock()
            .map(|g| g.has_active_hw_breakpoints())
            .unwrap_or(false);
        let Ok(mut armed_guard) = self.armed_core0_slot.lock() else {
            return;
        };
        if !c1_needs {
            if let Some(slot_idx) = armed_guard.take() {
                let _ = backend.clear_hw_breakpoint_slot(0, slot_idx, trap_addr);
            }
            return;
        }
        if let Ok(c0_guard) = self.bp_c0.lock() {
            if let Some(existing_slot) = *armed_guard {
                if c0_guard
                    .hw_slots
                    .get(existing_slot)
                    .copied()
                    .flatten()
                    .is_some()
                {
                    *armed_guard = None;
                } else {
                    return;
                }
            }
            if c0_guard.hw_slots.contains(&Some(trap_addr)) {
                return;
            }
            if let Some(free_slot) = c0_guard.hw_slots.iter().rposition(Option::is_none) {
                if backend
                    .set_hw_breakpoint_slot(0, free_slot, trap_addr)
                    .is_ok()
                {
                    *armed_guard = Some(free_slot);
                }
            }
        }
    }

    fn service_if_core0_hit_trap<B: DebugBackend + ?Sized>(&self, backend: &mut B) -> bool {
        let (Some(trap_addr), Some(ref bp_c1)) = (self.trap_addr, &self.bp_c1) else {
            return false;
        };
        let slot_opt = self.armed_core0_slot.lock().ok().and_then(|g| *g);
        let Some(slot_idx) = slot_opt else {
            return false;
        };
        if !backend.core_halted(0).unwrap_or(false) {
            return false;
        }
        let pc = (backend.read_core_reg(0, DCRSR_PC).unwrap_or(0) & !1) as u64;
        if pc != trap_addr {
            return false;
        }
        let user_has_bp_at_trap = self
            .bp_c0
            .lock()
            .map(|g| g.hw_slots.contains(&Some(trap_addr)))
            .unwrap_or(false);
        if user_has_bp_at_trap {
            return false;
        }
        if let Ok(c1_guard) = bp_c1.lock() {
            let _ = c1_guard.ensure_hw_breakpoints_armed(backend, 1);
        }
        let _ = backend.clear_hw_breakpoint_slot(0, slot_idx, trap_addr);
        if let Ok(mut armed_guard) = self.armed_core0_slot.lock() {
            *armed_guard = None;
        }
        let _ = backend.resume_core(0);
        true
    }
}

fn run_core_gdb_listener<B: DebugBackend + 'static>(
    listener: TcpListener,
    core_idx: usize,
    has_fpu: bool,
    backend: Arc<Mutex<B>>,
    breakpoints: Arc<Mutex<CoreBreakpointTable>>,
    coord: Arc<Core1PostResetCoordinator>,
    shutdown: Arc<AtomicBool>,
    json_events: bool,
) {
    while !shutdown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _peer)) => {
                let _ = stream.set_nonblocking(false);
                let _ = stream.set_nodelay(true);
                emit_event(
                    &DebugServerEvent::ClientState {
                        core: core_idx,
                        connected: true,
                    },
                    json_events,
                );
                serve_core_gdb_client(
                    stream,
                    core_idx,
                    has_fpu,
                    &backend,
                    &breakpoints,
                    &coord,
                    &shutdown,
                );
                emit_event(
                    &DebugServerEvent::ClientState {
                        core: core_idx,
                        connected: false,
                    },
                    json_events,
                );
            }
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if core_idx >= 1 {
                    if let Ok(mut back_guard) = backend.lock() {
                        let _ = coord.service_if_core0_hit_trap(&mut *back_guard);
                        if let Ok(bp_guard) = breakpoints.lock() {
                            let _ =
                                bp_guard.ensure_hw_breakpoints_armed(&mut *back_guard, core_idx);
                        }
                    }
                }
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
        }
    }

    if let (Ok(mut bp_guard), Ok(mut back_guard)) = (breakpoints.lock(), backend.lock()) {
        let _ = bp_guard.clear_all(&mut *back_guard, core_idx);
        coord.sync_trap(&mut *back_guard);
    }
}

fn serve_core_gdb_client<B: DebugBackend>(
    mut stream: TcpStream,
    core_idx: usize,
    has_fpu: bool,
    backend: &Arc<Mutex<B>>,
    breakpoints: &Arc<Mutex<CoreBreakpointTable>>,
    coord: &Arc<Core1PostResetCoordinator>,
    shutdown: &Arc<AtomicBool>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS)));
    let mut handler = RspCoreHandler::new(core_idx, has_fpu);
    let mut rx_buf: Vec<u8> = Vec::with_capacity(4096);
    let mut scratch = [0u8; 2048];

    while !shutdown.load(Ordering::SeqCst) {
        while let Some(frame) = extract_next_rsp_frame(&mut rx_buf) {
            match frame {
                RspFrame::Ack | RspFrame::Nack => {}
                RspFrame::CtrlC => {
                    if let Ok(mut guard) = backend.lock() {
                        let _ = guard.halt_core(core_idx);
                    }
                    if send_rsp_reply(&mut stream, "S02").is_err() {
                        return;
                    }
                }
                RspFrame::Packet {
                    payload,
                    checksum_valid,
                } => {
                    if !handler.no_ack_mode {
                        let ack = if checksum_valid { b"+" } else { b"-" };
                        if stream.write_all(ack).is_err() || stream.flush().is_err() {
                            return;
                        }
                    }
                    if !checksum_valid {
                        continue;
                    }

                    let action = {
                        let mut back_guard = match backend.lock() {
                            Ok(g) => g,
                            Err(_) => return,
                        };
                        let act = {
                            let mut bp_guard = match breakpoints.lock() {
                                Ok(g) => g,
                                Err(_) => return,
                            };
                            handler.handle_packet(&payload, &mut *back_guard, &mut bp_guard)
                        };
                        if payload.starts_with(b"qRcmd,") {
                            if let Ok(mut armed_guard) = coord.armed_core0_slot.lock() {
                                *armed_guard = None;
                            }
                        }
                        coord.sync_trap(&mut *back_guard);
                        act
                    };

                    match action {
                        RspAction::Reply(reply) => {
                            if send_rsp_reply(&mut stream, &reply).is_err() {
                                return;
                            }
                        }
                        RspAction::Disconnect(opt_reply) => {
                            if let Some(reply) = opt_reply {
                                let _ = send_rsp_reply(&mut stream, &reply);
                            }
                            return;
                        }
                        RspAction::WaitForHalt => {
                            if !wait_until_halted_or_interrupted(
                                &mut stream,
                                &mut rx_buf,
                                &handler,
                                core_idx,
                                backend,
                                breakpoints,
                                coord,
                                shutdown,
                            ) {
                                return;
                            }
                        }
                    }
                }
            }
        }

        match stream.read(&mut scratch) {
            Ok(0) => return,
            Ok(n) => rx_buf.extend_from_slice(&scratch[..n]),
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                if core_idx >= 1 {
                    if let Ok(mut back_guard) = backend.lock() {
                        let _ = coord.service_if_core0_hit_trap(&mut *back_guard);
                        if let Ok(bp_guard) = breakpoints.lock() {
                            let _ =
                                bp_guard.ensure_hw_breakpoints_armed(&mut *back_guard, core_idx);
                        }
                    }
                }
            }
            Err(_) => return,
        }
    }
}

fn wait_until_halted_or_interrupted<B: DebugBackend>(
    stream: &mut TcpStream,
    rx_buf: &mut Vec<u8>,
    handler: &RspCoreHandler,
    core_idx: usize,
    backend: &Arc<Mutex<B>>,
    breakpoints: &Arc<Mutex<CoreBreakpointTable>>,
    coord: &Arc<Core1PostResetCoordinator>,
    shutdown: &Arc<AtomicBool>,
) -> bool {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(CORE_POLL_INTERVAL_MS)));
    let mut poll_buf = [0u8; 256];
    let mut result_ok = true;

    while !shutdown.load(Ordering::SeqCst) {
        let halted = {
            let mut back_guard = match backend.lock() {
                Ok(g) => g,
                Err(_) => {
                    result_ok = false;
                    break;
                }
            };
            if coord.service_if_core0_hit_trap(&mut *back_guard) && core_idx == 0 {
                false
            } else {
                if core_idx >= 1 {
                    if let Ok(bp_guard) = breakpoints.lock() {
                        let _ = bp_guard.ensure_hw_breakpoints_armed(&mut *back_guard, core_idx);
                    }
                }
                back_guard.core_halted(core_idx).unwrap_or(false)
            }
        };

        if halted {
            if send_rsp_reply(stream, "S05").is_err() {
                result_ok = false;
            }
            break;
        }

        match stream.read(&mut poll_buf) {
            Ok(0) => {
                result_ok = false;
                break;
            }
            Ok(n) => {
                rx_buf.extend_from_slice(&poll_buf[..n]);
                if let Some(ctrl_pos) = rx_buf.iter().position(|&b| b == 0x03) {
                    rx_buf.remove(ctrl_pos);
                    if let Ok(mut back_guard) = backend.lock() {
                        let _ = back_guard.halt_core(core_idx);
                    }
                    if send_rsp_reply(stream, "S02").is_err() {
                        result_ok = false;
                    }
                    break;
                }
                // Also support `vCont;t` halt requests while running.
                if let Some(RspFrame::Packet {
                    payload: _,
                    checksum_valid,
                }) = extract_next_rsp_frame(rx_buf)
                {
                    if !handler.no_ack_mode {
                        let ack = if checksum_valid { b"+" } else { b"-" };
                        if stream.write_all(ack).is_err() || stream.flush().is_err() {
                            result_ok = false;
                            break;
                        }
                    }
                    if checksum_valid {
                        if let Ok(mut back_guard) = backend.lock() {
                            let _ = back_guard.halt_core(core_idx);
                        }
                        if send_rsp_reply(stream, "S05").is_err() {
                            result_ok = false;
                        }
                        break;
                    }
                }
            }
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => {
                result_ok = false;
                break;
            }
        }
    }

    let _ = stream.set_read_timeout(Some(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS)));
    result_ok
}

fn run_rtt_bridge_listener<B: DebugBackend + 'static>(
    listener: TcpListener,
    backend: Arc<Mutex<B>>,
    shutdown: Arc<AtomicBool>,
) {
    let mut client_threads = Vec::new();
    while !shutdown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _peer)) => {
                let _ = stream.set_nonblocking(false);
                let _ = stream.set_nodelay(true);
                let client_backend = Arc::clone(&backend);
                let client_shutdown = Arc::clone(&shutdown);
                client_threads.push(thread::spawn(move || {
                    serve_rtt_bridge_client(stream, client_backend, client_shutdown);
                }));
            }
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
        }
    }
    for handle in client_threads {
        let _ = handle.join();
    }
}

fn serve_rtt_bridge_client<B: DebugBackend>(
    mut stream: TcpStream,
    backend: Arc<Mutex<B>>,
    shutdown: Arc<AtomicBool>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS)));
    let mut no_ack = false;
    let mut rx_buf: Vec<u8> = Vec::with_capacity(2048);
    let mut scratch = [0u8; 2048];

    while !shutdown.load(Ordering::SeqCst) {
        while let Some(frame) = extract_next_rsp_frame(&mut rx_buf) {
            if let RspFrame::Packet {
                payload,
                checksum_valid,
            } = frame
            {
                if !no_ack {
                    let ack = if checksum_valid { b"+" } else { b"-" };
                    if stream.write_all(ack).is_err() || stream.flush().is_err() {
                        return;
                    }
                }
                if !checksum_valid {
                    continue;
                }
                let (reply, close_after) =
                    handle_non_halting_bridge_packet(&payload, &mut no_ack, &backend);
                if let Some(reply_text) = reply {
                    if send_rsp_reply(&mut stream, &reply_text).is_err() {
                        return;
                    }
                }
                if close_after {
                    return;
                }
            }
        }

        match stream.read(&mut scratch) {
            Ok(0) => return,
            Ok(n) => rx_buf.extend_from_slice(&scratch[..n]),
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => return,
        }
    }
}

fn handle_non_halting_bridge_packet<B: DebugBackend>(
    payload: &[u8],
    no_ack: &mut bool,
    backend: &Arc<Mutex<B>>,
) -> (Option<String>, bool) {
    if let Some(rest) = payload.strip_prefix(b"X") {
        let reply = if let Some(colon) = rest.iter().position(|&b| b == b':') {
            if let Ok(hdr) = std::str::from_utf8(&rest[..colon]) {
                if let Some((addr_s, len_s)) = hdr.split_once(',') {
                    if let (Ok(addr), Ok(len)) = (
                        u64::from_str_radix(addr_s.trim(), 16),
                        usize::from_str_radix(len_s.trim(), 16),
                    ) {
                        let data = unescape_rsp_binary(&rest[colon + 1..]);
                        if data.len() == len {
                            let res = backend
                                .lock()
                                .map_err(|e| e.to_string())
                                .and_then(|mut g| g.write_memory(0, addr, &data));
                            if res.is_ok() {
                                "OK".to_string()
                            } else {
                                "E01".to_string()
                            }
                        } else {
                            "E01".to_string()
                        }
                    } else {
                        "E01".to_string()
                    }
                } else {
                    "E01".to_string()
                }
            } else {
                "E01".to_string()
            }
        } else {
            "E01".to_string()
        };
        return (Some(reply), false);
    }

    let cmd = match std::str::from_utf8(payload) {
        Ok(s) => s,
        Err(_) => return (Some("E01".to_string()), false),
    };

    if cmd == "QStartNoAckMode" {
        *no_ack = true;
        return (Some("OK".to_string()), false);
    }
    if cmd.starts_with("qSupported") {
        return (Some("PacketSize=4096;QStartNoAckMode+".to_string()), false);
    }
    if let Some(mem_args) = cmd.strip_prefix('m') {
        let Some((addr_s, len_s)) = mem_args.split_once(',') else {
            return (Some("E01".to_string()), false);
        };
        let (Ok(addr), Ok(len)) = (
            u64::from_str_radix(addr_s.trim(), 16),
            usize::from_str_radix(len_s.trim(), 16),
        ) else {
            return (Some("E01".to_string()), false);
        };
        if len > MAX_RSP_MEMORY_READ_BYTES {
            return (Some("E01".to_string()), false);
        }
        if len == 0 {
            return (Some(String::new()), false);
        }
        let mut buf = vec![0u8; len];
        let res = backend
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|mut g| g.read_memory(0, addr, &mut buf));
        return match res {
            Ok(()) => (Some(encode_hex(&buf)), false),
            Err(_) => (Some("E01".to_string()), false),
        };
    }
    if let Some(mem_args) = cmd.strip_prefix('M') {
        let Some((hdr, hex_data)) = mem_args.split_once(':') else {
            return (Some("E01".to_string()), false);
        };
        let Some((addr_s, len_s)) = hdr.split_once(',') else {
            return (Some("E01".to_string()), false);
        };
        let (Ok(addr), Ok(len), Ok(data)) = (
            u64::from_str_radix(addr_s.trim(), 16),
            usize::from_str_radix(len_s.trim(), 16),
            decode_hex(hex_data.trim()),
        ) else {
            return (Some("E01".to_string()), false);
        };
        if data.len() != len {
            return (Some("E01".to_string()), false);
        }
        let res = backend
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|mut g| g.write_memory(0, addr, &data));
        return match res {
            Ok(()) => (Some("OK".to_string()), false),
            Err(_) => (Some("E01".to_string()), false),
        };
    }
    if let Some(rcmd_hex) = cmd.strip_prefix("qRcmd,") {
        if let Ok(raw) = decode_hex(rcmd_hex.trim()) {
            let text = String::from_utf8_lossy(&raw).trim().to_ascii_lowercase();
            let res = backend.lock().map_err(|e| e.to_string()).and_then(|mut g| {
                if text == "reset halt" {
                    g.reset_and_halt_core(0).map(|_| ())
                } else if text == "reset" || text == "reset run" {
                    g.reset_and_run_core(0)
                } else {
                    Ok(())
                }
            });
            return (
                Some(if res.is_ok() { "OK" } else { "E05" }.to_string()),
                false,
            );
        }
        return (Some("E01".to_string()), false);
    }
    if cmd == "D" || cmd.starts_with("D;") {
        return (Some("OK".to_string()), true);
    }
    if cmd == "k" {
        return (None, true);
    }

    (Some(String::new()), false)
}

fn send_rsp_reply(stream: &mut TcpStream, payload: &str) -> std::io::Result<()> {
    let framed = format_rsp_packet(payload);
    stream.write_all(&framed)?;
    stream.flush()
}

/// Hardware `probe_rs::Session` implementation of [`DebugBackend`].
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub mod probe_backend {
    use super::*;
    use probe_rs::probe::list::Lister;
    use probe_rs::probe::DebugProbeSelector;
    use probe_rs::{CoreType, MemoryInterface as _, Permissions, RegisterId, Session};

    /// Concrete `probe-rs` target session implementing [`DebugBackend`].
    pub struct ProbeRsDebugBackend {
        session: Session,
        chip: String,
        cores: usize,
        is_armv6m: bool,
        reset_handler_addr: Option<u64>,
    }

    impl ProbeRsDebugBackend {
        fn halt_core0_at_reset_entry(
            session: &mut Session,
            reset_handler_addr: Option<u64>,
        ) -> Result<u32, String> {
            let mut core0 = session
                .core(0)
                .map_err(|e| format!("Failed to access Core 0: {:?}", e))?;
            let info = core0
                .reset_and_halt(Duration::from_millis(500))
                .map_err(|e| format!("Failed to reset_and_halt Core 0: {:?}", e))?;
            let halted_pc = (info.pc as u32) & !1;

            let Some(raw_rh) = reset_handler_addr else {
                return Ok(halted_pc);
            };
            let target_pc = (raw_rh as u32) & !1;
            if halted_pc == target_pc {
                return Ok(halted_pc);
            }

            // On chips with a Mask Boot ROM (e.g. RP2040 halting at 0x0000_00EA before boot2 enables XIP),
            // set a temporary hardware comparator on `Reset_Handler`, run through boot2, and clear it.
            if let Ok(dhcsr) = core0.read_word_32(CORTEX_M_DHCSR_ADDR) {
                if (dhcsr & 1) == 0 {
                    let _ = core0.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                }
            }
            let _ = core0.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE);
            let _ = core0.flush();
            core0
                .set_hw_breakpoint_unit(0, target_pc as u64)
                .map_err(|e| format!("Failed to arm Reset_Handler breakpoint: {:?}", e))?;
            core0
                .run()
                .map_err(|e| format!("Failed to run Core 0 to Reset_Handler: {:?}", e))?;

            let deadline = std::time::Instant::now() + Duration::from_millis(500);
            while std::time::Instant::now() < deadline {
                if core0.core_halted().unwrap_or(false) {
                    break;
                }
                thread::sleep(Duration::from_millis(2));
            }
            if !core0.core_halted().unwrap_or(false) {
                let _ = core0.halt(Duration::from_millis(100));
            }
            if core0.clear_hw_breakpoint(target_pc as u64).is_err() {
                let _ = core0.write_word_32(CORTEX_M_BP_COMP0_ADDR, 0);
                let _ = core0.flush();
            }
            let final_pc = core0
                .read_core_reg::<u32>(RegisterId(15))
                .unwrap_or(halted_pc)
                & !1;
            Ok(final_pc)
        }

        /// Open a SWD debug probe session for `chip` and optionally reset or reset-and-halt Core 0.
        pub fn attach(
            chip: &str,
            probe_selector: Option<&str>,
            speed_khz: u32,
            reset: bool,
            halt_on_reset: bool,
            reset_handler_addr: Option<u64>,
        ) -> Result<Self, String> {
            let lister = Lister::new();
            let probes = if let Some(sel_str) = probe_selector {
                let selector = DebugProbeSelector::try_from(sel_str)
                    .map_err(|e| format!("Invalid --probe selector '{}': {}", sel_str, e))?;
                lister.list(Some(&selector))
            } else {
                lister.list_all()
            };

            let probe_info = probes
                .first()
                .ok_or_else(|| "No debug probes connected".to_string())?;
            let mut probe = probe_info
                .open()
                .map_err(|e| format!("Failed to open debug probe: {:?}", e))?;
            let _ = probe.set_speed(speed_khz);

            let mut session = probe
                .attach(chip, Permissions::default())
                .map_err(|e| format!("Failed to attach probe to '{}': {:?}", chip, e))?;

            let cores = session.list_cores().len();
            if halt_on_reset {
                Self::halt_core0_at_reset_entry(&mut session, reset_handler_addr)?;
            } else if reset {
                let mut core0 = session
                    .core(0)
                    .map_err(|e| format!("Failed to access Core 0: {:?}", e))?;
                core0
                    .reset()
                    .map_err(|e| format!("Failed to reset Core 0: {:?}", e))?;
                let _ = core0.run();
            }
            let is_armv6m = {
                let core0 = session
                    .core(0)
                    .map_err(|e| format!("Failed to access Core 0: {:?}", e))?;
                matches!(core0.core_type(), CoreType::Armv6m)
            };

            Ok(Self {
                session,
                chip: chip.to_string(),
                cores,
                is_armv6m,
                reset_handler_addr,
            })
        }
    }

    impl DebugBackend for ProbeRsDebugBackend {
        fn chip_name(&self) -> &str {
            &self.chip
        }

        fn core_count(&self) -> usize {
            self.cores
        }

        fn core_has_fpu(&mut self, core_idx: usize) -> bool {
            self.session
                .core(core_idx)
                .ok()
                .and_then(|mut c| c.fpu_support().ok())
                .unwrap_or(false)
        }

        fn available_hw_breakpoints(&mut self, core_idx: usize) -> usize {
            self.session
                .core(core_idx)
                .ok()
                .and_then(|mut c| c.available_breakpoint_units().ok())
                .unwrap_or(4) as usize
        }

        fn supports_hw_breakpoint_at(&self, _core_idx: usize, addr: u64) -> bool {
            if self.is_armv6m {
                addr < ARMV6M_HW_BP_LIMIT
            } else {
                addr <= u32::MAX as u64
            }
        }

        fn core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.core_halted().map_err(|e| format!("{:?}", e))
        }

        fn halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            let info = core
                .halt(Duration::from_millis(250))
                .map_err(|e| format!("{:?}", e))?;
            Ok((info.pc as u32) & !1)
        }

        fn resume_core(&mut self, core_idx: usize) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            if !core.core_halted().unwrap_or(true) {
                return Ok(());
            }
            core.run().map_err(|e| format!("{:?}", e))
        }

        fn step_core(&mut self, core_idx: usize) -> Result<u32, String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            let info = core.step().map_err(|e| format!("{:?}", e))?;
            Ok((info.pc as u32) & !1)
        }

        fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            if core_idx == 0 {
                return Self::halt_core0_at_reset_entry(&mut self.session, self.reset_handler_addr);
            }
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            let info = core
                .reset_and_halt(Duration::from_millis(500))
                .map_err(|e| format!("{:?}", e))?;
            Ok((info.pc as u32) & !1)
        }

        fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.reset().map_err(|e| format!("{:?}", e))?;
            core.run().map_err(|e| format!("{:?}", e))
        }

        fn read_core_reg(&mut self, core_idx: usize, dcrsr: u16) -> Result<u32, String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.read_core_reg::<u32>(RegisterId(dcrsr))
                .map_err(|e| format!("{:?}", e))
        }

        fn write_core_reg(
            &mut self,
            core_idx: usize,
            dcrsr: u16,
            value: u32,
        ) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.write_core_reg(RegisterId(dcrsr), value)
                .map_err(|e| format!("{:?}", e))
        }

        fn read_memory(
            &mut self,
            core_idx: usize,
            addr: u64,
            buf: &mut [u8],
        ) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.read_8(addr, buf).map_err(|e| format!("{:?}", e))
        }

        fn write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            core.write_8(addr, data).map_err(|e| format!("{:?}", e))?;
            core.flush().map_err(|e| format!("{:?}", e))
        }

        fn set_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                if (dhcsr & 1) == 0 {
                    let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                }
            }
            let _ = core.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE);
            let _ = core.flush();
            core.set_hw_breakpoint_unit(slot_idx, addr)
                .map_err(|e| format!("{:?}", e))
        }

        fn clear_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            if core.clear_hw_breakpoint(addr).is_err() {
                let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                let _ = core.write_word_32(comp_addr, 0);
                let _ = core.flush();
            }
            Ok(())
        }

        fn ensure_hw_breakpoints_active(
            &mut self,
            core_idx: usize,
            slots: &[Option<u64>],
        ) -> Result<bool, String> {
            let Some((first_idx, _)) = slots.iter().enumerate().find(|(_, entry)| entry.is_some())
            else {
                return Ok(false);
            };
            let mut core = self
                .session
                .core(core_idx)
                .map_err(|e| format!("{:?}", e))?;
            let bp_ctrl = core
                .read_word_32(CORTEX_M_BP_CTRL_ADDR)
                .map_err(|e| format!("{:?}", e))?;
            let first_comp_addr = CORTEX_M_BP_COMP0_ADDR + (first_idx as u64) * 4;
            let first_comp = core
                .read_word_32(first_comp_addr)
                .map_err(|e| format!("{:?}", e))?;

            if (bp_ctrl & 1) != 0 && (first_comp & 1) != 0 {
                return Ok(false);
            }

            if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                if (dhcsr & 1) == 0 {
                    let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                }
            }
            core.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE)
                .map_err(|e| format!("{:?}", e))?;
            let _ = core.flush();

            for (slot_idx, entry) in slots.iter().enumerate() {
                if let Some(addr) = *entry {
                    core.set_hw_breakpoint_unit(slot_idx, addr)
                        .map_err(|e| format!("{:?}", e))?;
                }
            }
            Ok(true)
        }
    }
}
