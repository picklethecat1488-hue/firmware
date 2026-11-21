//! Multi-listener TCP GDB RSP server and non-halting RTT memory bridge decoupled from
//! processor-specific register/breakpoint implementations via [`DebugBackend`].

use crate::arch::TargetRegisterLayout;
use crate::rsp::{
    decode_hex, encode_hex, extract_next_rsp_frame, format_rsp_packet, unescape_rsp_binary,
    RspAction, RspCoreHandler, RspFrame, RspReplyCode, MAX_RSP_MEMORY_READ_BYTES,
};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tool_common::{DEFAULT_GDB_CORE0_PORT, DEFAULT_GDB_CORE1_PORT, DEFAULT_RTT_BRIDGE_PORT};

/// Poll interval while waiting for a running core to hit a breakpoint or receive Ctrl-C (`0x03`).
pub const CORE_POLL_INTERVAL_MS: u64 = 5;

/// Listener accept loop poll interval allowing clean cooperative shutdown.
pub const ACCEPT_POLL_INTERVAL_MS: u64 = 25;

/// Architecture-independent debug probe interface backing [`RspCoreHandler`] and the RTT memory bridge.
pub trait DebugBackend: Send {
    /// Human-readable target chip identifier (e.g. `"rp2040"`, `"MCXN947"`).
    fn chip_name(&self) -> &str;

    /// Number of debuggable processor cores exposed by this target session.
    fn core_count(&self) -> usize;

    /// Single-sourced architectural register layout and rendered `target.xml` for `core_idx`.
    fn core_register_layout(&mut self, core_idx: usize) -> TargetRegisterLayout;

    /// Return `true` if `core_idx` is currently in Debug Halt state.
    fn core_halted(&mut self, core_idx: usize) -> Result<bool, String>;

    /// Request `core_idx` to enter Debug Halt state and return its halted `PC`.
    fn halt_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Resume execution on `core_idx` (stepping over any software breakpoint and ensuring comparators are armed).
    fn resume_core(&mut self, core_idx: usize) -> Result<(), String>;

    /// Single-step one instruction on `core_idx` and return the resulting `PC`.
    fn step_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Reset `core_idx` and halt immediately at its reset entry point.
    fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Reset `core_idx` and resume normal execution.
    fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String>;

    /// Read a 32-bit architectural register (`hw_reg_id`) on `core_idx`.
    fn read_core_reg(&mut self, core_idx: usize, hw_reg_id: u16) -> Result<u32, String>;

    /// Write a 32-bit architectural register (`hw_reg_id`) on `core_idx`.
    fn write_core_reg(&mut self, core_idx: usize, hw_reg_id: u16, value: u32)
        -> Result<(), String>;

    /// Read `buf.len()` raw bytes of target memory at `addr` without halting `core_idx`.
    fn read_memory(&mut self, core_idx: usize, addr: u64, buf: &mut [u8]) -> Result<(), String>;

    /// Read `buf.len()` bytes of target memory at `addr`, masking any active software breakpoint opcodes.
    fn read_memory_masked(
        &mut self,
        core_idx: usize,
        addr: u64,
        buf: &mut [u8],
    ) -> Result<(), String>;

    /// Write `data` bytes to target memory at `addr` without halting `core_idx`.
    fn write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String>;

    /// Insert a hardware or software breakpoint at `addr` on `core_idx`.
    fn insert_breakpoint(
        &mut self,
        core_idx: usize,
        addr: u64,
        allow_sw_ram: bool,
    ) -> Result<bool, String>;

    /// Remove an active hardware or software breakpoint at `addr` on `core_idx`.
    fn remove_breakpoint(&mut self, core_idx: usize, addr: u64) -> Result<(), String>;

    /// Clear all active hardware and software breakpoints on `core_idx`.
    fn clear_all_breakpoints(&mut self, core_idx: usize) -> Result<(), String>;

    /// Return `true` if `core_idx` currently has an active user breakpoint registered at `addr`.
    fn has_active_breakpoint_at(&self, _core_idx: usize, _addr: u64) -> bool {
        false
    }

    /// Run periodic background breakpoint/reset maintenance on `core_idx`.
    ///
    /// Returns `true` if `core_idx` hit an internal maintenance trap and was transparently resumed.
    fn poll_background_core_maintenance(&mut self, _core_idx: usize) -> bool {
        false
    }
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
    /// Optional absolute ELF executable path advertised to GDB via `qXfer:exec-file:read`.
    pub exec_file_path: Option<String>,
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
            exec_file_path: None,
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
    let core0_layout = backend.core_register_layout(0);
    let core1_layout = if cores >= 2 {
        Some(backend.core_register_layout(1))
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
    let exec_file_path = config.exec_file_path.clone();
    let mut threads = Vec::new();

    {
        let backend_c0 = Arc::clone(&shared_backend);
        let shutdown_c0 = Arc::clone(&shutdown);
        let exec_c0 = exec_file_path.clone();
        threads.push(thread::spawn(move || {
            run_core_gdb_listener(
                core0_listener,
                0,
                core0_layout,
                exec_c0,
                backend_c0,
                shutdown_c0,
                json_events,
            );
        }));
    }

    if let (Some(listener), Some(c1_layout)) = (core1_listener, core1_layout) {
        let backend_c1 = Arc::clone(&shared_backend);
        let shutdown_c1 = Arc::clone(&shutdown);
        let exec_c1 = exec_file_path;
        threads.push(thread::spawn(move || {
            run_core_gdb_listener(
                listener,
                1,
                c1_layout,
                exec_c1,
                backend_c1,
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

fn run_core_gdb_listener<B: DebugBackend + 'static>(
    listener: TcpListener,
    core_idx: usize,
    arch_layout: TargetRegisterLayout,
    exec_file_path: Option<String>,
    backend: Arc<Mutex<B>>,
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
                    arch_layout.clone(),
                    exec_file_path.clone(),
                    &backend,
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
                        let _ = back_guard.poll_background_core_maintenance(core_idx);
                    }
                }
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS));
            }
        }
    }

    if let Ok(mut back_guard) = backend.lock() {
        let _ = back_guard.clear_all_breakpoints(core_idx);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RunningWaitOutcome {
    HaltedOrInterrupted,
    DisconnectClient,
    StreamClosed,
}

fn serve_core_gdb_client<B: DebugBackend>(
    mut stream: TcpStream,
    core_idx: usize,
    arch_layout: TargetRegisterLayout,
    exec_file_path: Option<String>,
    backend: &Arc<Mutex<B>>,
    shutdown: &Arc<AtomicBool>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS)));
    let mut handler = RspCoreHandler::new(core_idx, arch_layout, exec_file_path);
    let mut rx_buf: Vec<u8> = Vec::with_capacity(4096);
    let mut scratch = [0u8; 2048];
    let mut last_unacked_rx_payload: Option<Vec<u8>> = None;
    let mut last_sent_reply: Option<String> = None;

    while !shutdown.load(Ordering::SeqCst) {
        while let Some(frame) = extract_next_rsp_frame(&mut rx_buf) {
            match frame {
                RspFrame::Ack => {
                    last_unacked_rx_payload = None;
                }
                RspFrame::Nack => {
                    if !handler.no_ack_mode {
                        if let Some(ref prev_reply) = last_sent_reply {
                            if send_rsp_reply(&mut stream, prev_reply).is_err() {
                                return;
                            }
                        }
                    }
                }
                RspFrame::CtrlC => {
                    if let Ok(mut guard) = backend.lock() {
                        let _ = guard.halt_core(core_idx);
                        handler.record_stop_pc(&mut *guard);
                    }
                    let sig = RspReplyCode::SigInt.as_str();
                    last_sent_reply = Some(sig.to_string());
                    if send_rsp_reply(&mut stream, sig).is_err() {
                        return;
                    }
                }
                RspFrame::Packet {
                    payload,
                    checksum_valid,
                } => {
                    let was_ack_mode = !handler.no_ack_mode;
                    if was_ack_mode {
                        let ack = if checksum_valid { b"+" } else { b"-" };
                        if stream.write_all(ack).is_err() || stream.flush().is_err() {
                            return;
                        }
                    }
                    if !checksum_valid {
                        continue;
                    }
                    if was_ack_mode
                        && last_unacked_rx_payload.as_deref() == Some(payload.as_slice())
                    {
                        continue;
                    }

                    let action =
                        if let Some(stateless_act) = handler.handle_stateless_packet(&payload) {
                            stateless_act
                        } else {
                            let mut back_guard = match backend.lock() {
                                Ok(g) => g,
                                Err(_) => return,
                            };
                            handler.handle_packet(&payload, &mut *back_guard)
                        };

                    if handler.no_ack_mode {
                        last_unacked_rx_payload = None;
                    } else {
                        last_unacked_rx_payload = Some(payload);
                    }

                    match action {
                        RspAction::Reply(reply) => {
                            last_sent_reply = Some(reply.clone());
                            if send_rsp_reply(&mut stream, &reply).is_err() {
                                return;
                            }
                        }
                        RspAction::ReplyPackets(packets) => {
                            for pkt_payload in packets {
                                last_sent_reply = Some(pkt_payload.clone());
                                if send_rsp_reply(&mut stream, &pkt_payload).is_err() {
                                    return;
                                }
                            }
                        }
                        RspAction::Disconnect(opt_reply) => {
                            if let Some(reply) = opt_reply {
                                let _ = send_rsp_reply(&mut stream, &reply);
                            }
                            return;
                        }
                        RspAction::WaitForHalt => {
                            match wait_until_halted_or_interrupted(
                                &mut stream,
                                &mut rx_buf,
                                &mut handler,
                                &mut last_unacked_rx_payload,
                                &mut last_sent_reply,
                                core_idx,
                                backend,
                                shutdown,
                            ) {
                                RunningWaitOutcome::HaltedOrInterrupted => {}
                                RunningWaitOutcome::DisconnectClient
                                | RunningWaitOutcome::StreamClosed => return,
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
                        let _ = back_guard.poll_background_core_maintenance(core_idx);
                    }
                }
            }
            Err(_) => return,
        }
    }
}

fn drain_running_rx_frames<B: DebugBackend>(
    stream: &mut TcpStream,
    rx_buf: &mut Vec<u8>,
    handler: &mut RspCoreHandler,
    last_unacked_rx_payload: &mut Option<Vec<u8>>,
    last_sent_reply: &mut Option<String>,
    core_idx: usize,
    backend: &Arc<Mutex<B>>,
) -> Option<RunningWaitOutcome> {
    while let Some(frame) = extract_next_rsp_frame(rx_buf) {
        match frame {
            RspFrame::Ack => {
                *last_unacked_rx_payload = None;
            }
            RspFrame::Nack => {
                if !handler.no_ack_mode {
                    if let Some(ref prev_reply) = last_sent_reply {
                        if send_rsp_reply(stream, prev_reply).is_err() {
                            return Some(RunningWaitOutcome::StreamClosed);
                        }
                    }
                }
            }
            RspFrame::CtrlC => {
                if let Ok(mut back_guard) = backend.lock() {
                    let _ = back_guard.halt_core(core_idx);
                    handler.record_stop_pc(&mut *back_guard);
                }
                let sig = RspReplyCode::SigInt.as_str();
                *last_sent_reply = Some(sig.to_string());
                return if send_rsp_reply(stream, sig).is_ok() {
                    Some(RunningWaitOutcome::HaltedOrInterrupted)
                } else {
                    Some(RunningWaitOutcome::StreamClosed)
                };
            }
            RspFrame::Packet {
                payload,
                checksum_valid,
            } => {
                if !handler.no_ack_mode {
                    let ack = if checksum_valid { b"+" } else { b"-" };
                    if stream.write_all(ack).is_err() || stream.flush().is_err() {
                        return Some(RunningWaitOutcome::StreamClosed);
                    }
                }
                if !checksum_valid {
                    continue;
                }
                if !handler.no_ack_mode
                    && last_unacked_rx_payload.as_deref() == Some(payload.as_slice())
                {
                    continue;
                }
                if !handler.no_ack_mode {
                    *last_unacked_rx_payload = Some(payload.clone());
                }

                let cmd = std::str::from_utf8(&payload).unwrap_or("");
                if cmd == "D"
                    || cmd.starts_with("D;")
                    || cmd == "vKill"
                    || cmd.starts_with("vKill;")
                {
                    if let Ok(mut back_guard) = backend.lock() {
                        let _ = back_guard.clear_all_breakpoints(core_idx);
                        let _ = back_guard.resume_core(core_idx);
                    }
                    let _ = send_rsp_reply(stream, RspReplyCode::Ok.as_str());
                    return Some(RunningWaitOutcome::DisconnectClient);
                }
                if cmd == "k" {
                    if let Ok(mut back_guard) = backend.lock() {
                        let _ = back_guard.clear_all_breakpoints(core_idx);
                        let _ = back_guard.resume_core(core_idx);
                    }
                    return Some(RunningWaitOutcome::DisconnectClient);
                }

                if let Ok(mut back_guard) = backend.lock() {
                    let _ = back_guard.halt_core(core_idx);
                    handler.record_stop_pc(&mut *back_guard);
                }
                let trap = RspReplyCode::SigTrap.as_str();
                *last_sent_reply = Some(trap.to_string());
                return if send_rsp_reply(stream, trap).is_ok() {
                    Some(RunningWaitOutcome::HaltedOrInterrupted)
                } else {
                    Some(RunningWaitOutcome::StreamClosed)
                };
            }
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn wait_until_halted_or_interrupted<B: DebugBackend>(
    stream: &mut TcpStream,
    rx_buf: &mut Vec<u8>,
    handler: &mut RspCoreHandler,
    last_unacked_rx_payload: &mut Option<Vec<u8>>,
    last_sent_reply: &mut Option<String>,
    core_idx: usize,
    backend: &Arc<Mutex<B>>,
    shutdown: &Arc<AtomicBool>,
) -> RunningWaitOutcome {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(CORE_POLL_INTERVAL_MS)));
    let mut poll_buf = [0u8; 256];
    let mut outcome = RunningWaitOutcome::HaltedOrInterrupted;

    while !shutdown.load(Ordering::SeqCst) {
        if !rx_buf.is_empty() {
            if let Some(frame_outcome) = drain_running_rx_frames(
                stream,
                rx_buf,
                handler,
                last_unacked_rx_payload,
                last_sent_reply,
                core_idx,
                backend,
            ) {
                outcome = frame_outcome;
                break;
            }
        }

        let halted = {
            let mut back_guard = match backend.lock() {
                Ok(g) => g,
                Err(_) => {
                    outcome = RunningWaitOutcome::StreamClosed;
                    break;
                }
            };
            if back_guard.poll_background_core_maintenance(core_idx) {
                false
            } else {
                let is_halted = back_guard.core_halted(core_idx).unwrap_or(false);
                if is_halted {
                    handler.record_stop_pc(&mut *back_guard);
                }
                is_halted
            }
        };

        if halted {
            let trap = RspReplyCode::SigTrap.as_str();
            *last_sent_reply = Some(trap.to_string());
            if send_rsp_reply(stream, trap).is_err() {
                outcome = RunningWaitOutcome::StreamClosed;
            }
            break;
        }

        match stream.read(&mut poll_buf) {
            Ok(0) => {
                outcome = RunningWaitOutcome::StreamClosed;
                break;
            }
            Ok(n) => {
                rx_buf.extend_from_slice(&poll_buf[..n]);
                if let Some(frame_outcome) = drain_running_rx_frames(
                    stream,
                    rx_buf,
                    handler,
                    last_unacked_rx_payload,
                    last_sent_reply,
                    core_idx,
                    backend,
                ) {
                    outcome = frame_outcome;
                    break;
                }
            }
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => {
                outcome = RunningWaitOutcome::StreamClosed;
                break;
            }
        }
    }

    let _ = stream.set_read_timeout(Some(Duration::from_millis(ACCEPT_POLL_INTERVAL_MS)));
    outcome
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

fn execute_bridge_binary_write<B: DebugBackend>(
    rest: &[u8],
    backend: &Arc<Mutex<B>>,
) -> RspReplyCode {
    let Some(colon) = rest.iter().position(|&b| b == b':') else {
        return RspReplyCode::ErrBadRequest;
    };
    let Ok(hdr) = std::str::from_utf8(&rest[..colon]) else {
        return RspReplyCode::ErrBadRequest;
    };
    let Some((addr_s, len_s)) = hdr.split_once(',') else {
        return RspReplyCode::ErrBadRequest;
    };
    let (Ok(addr), Ok(len)) = (
        u64::from_str_radix(addr_s.trim(), 16),
        usize::from_str_radix(len_s.trim(), 16),
    ) else {
        return RspReplyCode::ErrBadRequest;
    };
    let data = unescape_rsp_binary(&rest[colon + 1..]);
    if data.len() != len {
        return RspReplyCode::ErrBadRequest;
    }
    let write_ok = backend
        .lock()
        .ok()
        .and_then(|mut g| g.write_memory(0, addr, &data).ok())
        .is_some();
    if write_ok {
        RspReplyCode::Ok
    } else {
        RspReplyCode::ErrBadRequest
    }
}

fn handle_non_halting_bridge_packet<B: DebugBackend>(
    payload: &[u8],
    no_ack: &mut bool,
    backend: &Arc<Mutex<B>>,
) -> (Option<String>, bool) {
    if let Some(rest) = payload.strip_prefix(b"X") {
        let code = execute_bridge_binary_write(rest, backend);
        return (Some(code.as_reply_string()), false);
    }

    let cmd = match std::str::from_utf8(payload) {
        Ok(s) => s,
        Err(_) => return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false),
    };

    if cmd == "QStartNoAckMode" {
        *no_ack = true;
        return (Some(RspReplyCode::Ok.as_reply_string()), false);
    }
    if cmd.starts_with("qSupported") {
        return (Some("PacketSize=4096;QStartNoAckMode+".to_string()), false);
    }
    if let Some(mem_args) = cmd.strip_prefix('m') {
        let Some((addr_s, len_s)) = mem_args.split_once(',') else {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        };
        let (Ok(addr), Ok(len)) = (
            u64::from_str_radix(addr_s.trim(), 16),
            usize::from_str_radix(len_s.trim(), 16),
        ) else {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        };
        if len > MAX_RSP_MEMORY_READ_BYTES {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        }
        if len == 0 {
            return (Some(RspReplyCode::Empty.as_reply_string()), false);
        }
        let mut buf = vec![0u8; len];
        let res = backend
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|mut g| g.read_memory(0, addr, &mut buf));
        return match res {
            Ok(()) => (Some(encode_hex(&buf)), false),
            Err(_) => (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false),
        };
    }
    if let Some(mem_args) = cmd.strip_prefix('M') {
        let Some((hdr, hex_data)) = mem_args.split_once(':') else {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        };
        let Some((addr_s, len_s)) = hdr.split_once(',') else {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        };
        let (Ok(addr), Ok(len), Ok(data)) = (
            u64::from_str_radix(addr_s.trim(), 16),
            usize::from_str_radix(len_s.trim(), 16),
            decode_hex(hex_data.trim()),
        ) else {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        };
        if data.len() != len {
            return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
        }
        let res = backend
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|mut g| g.write_memory(0, addr, &data));
        return match res {
            Ok(()) => (Some(RspReplyCode::Ok.as_reply_string()), false),
            Err(_) => (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false),
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
            let code = if res.is_ok() {
                RspReplyCode::Ok
            } else {
                RspReplyCode::ErrTargetState
            };
            return (Some(code.as_reply_string()), false);
        }
        return (Some(RspReplyCode::ErrBadRequest.as_reply_string()), false);
    }
    if cmd == "D" || cmd.starts_with("D;") {
        return (Some(RspReplyCode::Ok.as_reply_string()), true);
    }
    if cmd == "k" {
        return (None, true);
    }

    (Some(RspReplyCode::Empty.as_reply_string()), false)
}

fn send_rsp_reply(stream: &mut TcpStream, payload: &str) -> std::io::Result<()> {
    let framed = format_rsp_packet(payload);
    stream.write_all(&framed)?;
    stream.flush()
}
