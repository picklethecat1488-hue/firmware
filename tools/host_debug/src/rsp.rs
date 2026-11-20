//! Architecture-independent GDB Remote Serial Protocol (RSP) packet framing,
//! typed reply status codes, XML/exec-file transfer chunking, and command dispatch.

use crate::arch::{TargetArchProfile, TargetRegisterLayout};
use crate::server::DebugBackend;

/// Maximum single RSP memory read chunk size in bytes.
pub const MAX_RSP_MEMORY_READ_BYTES: usize = 4096;

/// Strongly typed GDB Remote Serial Protocol status and error reply codes.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RspReplyCode {
    /// Command succeeded (`"OK"`).
    Ok,
    /// Empty reply (`""`) indicating unsupported optional query or `vMustReplyEmpty`.
    Empty,
    /// Malformed command syntax, hex string, or memory bounds (`"E01"`).
    ErrBadRequest,
    /// Target register read/write failure (`"E02"`).
    ErrRegAccess,
    /// Target execution/halt/reset failure (`"E05"`).
    ErrTargetState,
    /// Hardware/software breakpoint resource exhausted (`"E0E"`).
    ErrNoBreakpointSlot,
    /// Asynchronous host interrupt (`SIGINT`, `"S02"`).
    SigInt,
    /// Debug trap / breakpoint / single-step stop (`SIGTRAP`, `"S05"`).
    SigTrap,
}

impl RspReplyCode {
    /// Wire-protocol string representation of this reply code.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Empty => "",
            Self::ErrBadRequest => "E01",
            Self::ErrRegAccess => "E02",
            Self::ErrTargetState => "E05",
            Self::ErrNoBreakpointSlot => "E0E",
            Self::SigInt => "S02",
            Self::SigTrap => "S05",
        }
    }

    /// Allocate an owned wire-protocol string for this reply code.
    pub fn as_reply_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// Parsed frame extracted from an incoming GDB RSP byte stream.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RspFrame {
    /// Positive acknowledgement (`'+'`).
    Ack,
    /// Negative acknowledgement / retransmit request (`'-'`).
    Nack,
    /// Out-of-band asynchronous interrupt (`0x03`, Ctrl-C).
    CtrlC,
    /// Framed `$payload#XX` RSP packet with validated checksum flag.
    Packet {
        /// Raw packet payload bytes between `'$'` and `'#'`.
        payload: Vec<u8>,
        /// Whether the two-digit hex checksum matched `sum(payload) % 256`.
        checksum_valid: bool,
    },
}

/// Post-command action requested by [`RspCoreHandler::handle_packet`].
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RspAction {
    /// Send a single RSP reply payload immediately and wait for the next client command.
    Reply(String),
    /// Send an ordered sequence of RSP packets (e.g. `O<hex>` console output followed by `OK`).
    ReplyPackets(Vec<String>),
    /// Target core has been resumed (`c` / `vCont;c`); poll for halt or `0x03` interrupt before replying.
    WaitForHalt,
    /// Send the optional RSP reply (if `Some`) and close the client TCP connection (`D` / `k` / `vKill`).
    Disconnect(Option<String>),
}

impl RspAction {
    /// Construct an [`RspAction::Reply`] from a typed [`RspReplyCode`].
    pub fn status(code: RspReplyCode) -> Self {
        Self::Reply(code.as_reply_string())
    }
}

/// Encode raw bytes as lowercase hexadecimal ASCII.
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{:02x}", b);
    }
    out
}

/// Decode hexadecimal ASCII into raw bytes.
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, String> {
    if !hex.len().is_multiple_of(2) {
        return Err("Odd-length hex string".to_string());
    }
    let bytes = hex.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut idx = 0;
    while idx < bytes.len() {
        let chunk = std::str::from_utf8(&bytes[idx..idx + 2]).map_err(|e| e.to_string())?;
        let val = u8::from_str_radix(chunk, 16).map_err(|e| e.to_string())?;
        out.push(val);
        idx += 2;
    }
    Ok(out)
}

/// Unescape GDB binary packet data (`0x7d` escape prefix XOR `0x20`).
pub fn unescape_rsp_binary(escaped: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(escaped.len());
    let mut iter = escaped.iter().copied();
    while let Some(byte) = iter.next() {
        if byte == 0x7d {
            if let Some(next) = iter.next() {
                out.push(next ^ 0x20);
            }
        } else {
            out.push(byte);
        }
    }
    out
}

/// Format an outgoing GDB RSP `$payload#XX` packet with two-digit modulo-256 checksum.
pub fn format_rsp_packet(payload: &str) -> Vec<u8> {
    let checksum: u8 = payload
        .as_bytes()
        .iter()
        .fold(0u8, |acc, &b| acc.wrapping_add(b));
    format!("${}#{:02x}", payload, checksum).into_bytes()
}

/// Extract the next complete [`RspFrame`] from `buf`, draining consumed bytes in place.
pub fn extract_next_rsp_frame(buf: &mut Vec<u8>) -> Option<RspFrame> {
    let first = *buf.first()?;
    match first {
        b'+' => {
            buf.remove(0);
            Some(RspFrame::Ack)
        }
        b'-' => {
            buf.remove(0);
            Some(RspFrame::Nack)
        }
        0x03 => {
            buf.remove(0);
            Some(RspFrame::CtrlC)
        }
        b'$' => {
            let hash_pos = buf.iter().skip(1).position(|&b| b == b'#').map(|p| p + 1)?;
            if buf.len() < hash_pos + 3 {
                return None;
            }
            let payload = buf[1..hash_pos].to_vec();
            let csum_str = std::str::from_utf8(&buf[hash_pos + 1..hash_pos + 3]).ok();
            let parsed_csum = csum_str.and_then(|s| u8::from_str_radix(s, 16).ok());
            let expected: u8 = payload.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
            buf.drain(..hash_pos + 3);
            Some(RspFrame::Packet {
                payload,
                checksum_valid: parsed_csum == Some(expected),
            })
        }
        _ => {
            let next_marker = buf
                .iter()
                .position(|&b| matches!(b, b'+' | b'-' | 0x03 | b'$'))
                .unwrap_or(buf.len());
            buf.drain(..next_marker);
            if buf.is_empty() {
                None
            } else {
                extract_next_rsp_frame(buf)
            }
        }
    }
}

fn slice_qxfer_blob(blob: &str, xfer_args: &str) -> String {
    let mut parts = xfer_args.split(',');
    let offset = parts
        .next()
        .and_then(|s| usize::from_str_radix(s, 16).ok())
        .unwrap_or(0);
    let length = parts
        .next()
        .and_then(|s| usize::from_str_radix(s, 16).ok())
        .unwrap_or(0);
    let bytes = blob.as_bytes();
    if offset >= bytes.len() {
        return "l".to_string();
    }
    let end = std::cmp::min(bytes.len(), offset.saturating_add(length));
    let chunk = &blob[offset..end];
    if end >= bytes.len() {
        format!("l{}", chunk)
    } else {
        format!("m{}", chunk)
    }
}

/// Stateful architecture-independent GDB RSP command handler bound to a single MCU core index.
pub struct RspCoreHandler {
    /// Target core index (`0` for Core 0, `1` for Core 1).
    pub core_idx: usize,
    /// Whether `QStartNoAckMode` has been negotiated on this socket.
    pub no_ack_mode: bool,
    /// Architecture register profile supplying `target.xml` and register mappings.
    pub arch_profile: TargetRegisterLayout,
    /// Optional absolute target ELF executable path served over `qXfer:exec-file:read`.
    pub exec_file_path: Option<String>,
}

impl RspCoreHandler {
    /// Create a new per-core GDB RSP handler for `core_idx` and `arch_profile`.
    pub fn new(
        core_idx: usize,
        arch_profile: TargetRegisterLayout,
        exec_file_path: Option<String>,
    ) -> Self {
        Self {
            core_idx,
            no_ack_mode: false,
            arch_profile,
            exec_file_path,
        }
    }

    /// Handle pure protocol handshake/metadata packets that do not require locking [`DebugBackend`].
    pub fn handle_stateless_packet(&mut self, payload: &[u8]) -> Option<RspAction> {
        if payload.starts_with(b"X") {
            return None;
        }
        let cmd = match std::str::from_utf8(payload) {
            Ok(s) => s,
            Err(_) => return Some(RspAction::status(RspReplyCode::ErrBadRequest)),
        };

        if cmd == "QStartNoAckMode" {
            self.no_ack_mode = true;
            return Some(RspAction::status(RspReplyCode::Ok));
        }
        if cmd.starts_with("qSupported") {
            let mut caps = String::from(
                "PacketSize=4096;qXfer:features:read+;QStartNoAckMode+;vContSupported+;hwbreak+;swbreak+",
            );
            if self.exec_file_path.is_some() {
                caps.push_str(";qXfer:exec-file:read+");
            }
            return Some(RspAction::Reply(caps));
        }
        if cmd == "!" {
            return Some(RspAction::status(RspReplyCode::Ok));
        }
        if cmd == "vMustReplyEmpty" || cmd == "qOffsets" || cmd == "qTStatus" {
            return Some(RspAction::status(RspReplyCode::Empty));
        }
        if cmd == "vCont?" {
            return Some(RspAction::Reply("vCont;c;C;s;S;t".to_string()));
        }
        if cmd == "qAttached" {
            return Some(RspAction::Reply("1".to_string()));
        }
        if cmd == "qC" {
            return Some(RspAction::Reply("QC01".to_string()));
        }
        if cmd == "qfThreadInfo" {
            return Some(RspAction::Reply("m01".to_string()));
        }
        if cmd == "qsThreadInfo" {
            return Some(RspAction::Reply("l".to_string()));
        }
        if cmd.starts_with("qThreadExtraInfo") {
            let info = format!("Core {}", self.core_idx);
            return Some(RspAction::Reply(encode_hex(info.as_bytes())));
        }
        if cmd.starts_with("qSymbol")
            || cmd.starts_with("Hg")
            || cmd.starts_with("Hc")
            || cmd.starts_with('T')
        {
            return Some(RspAction::status(RspReplyCode::Ok));
        }
        if let Some(xfer_args) = cmd.strip_prefix("qXfer:features:read:target.xml:") {
            return Some(RspAction::Reply(slice_qxfer_blob(
                self.arch_profile.target_xml(),
                xfer_args,
            )));
        }
        if let Some(rest) = cmd.strip_prefix("qXfer:exec-file:read:") {
            if let Some((_, xfer_args)) = rest.split_once(':') {
                if let Some(ref exec_path) = self.exec_file_path {
                    return Some(RspAction::Reply(slice_qxfer_blob(exec_path, xfer_args)));
                }
            }
            return Some(RspAction::status(RspReplyCode::Empty));
        }
        None
    }

    /// Process a single unframed RSP packet payload against `backend`.
    pub fn handle_packet<B: DebugBackend + ?Sized>(
        &mut self,
        payload: &[u8],
        backend: &mut B,
    ) -> RspAction {
        if let Some(stateless) = self.handle_stateless_packet(payload) {
            return stateless;
        }
        if let Some(rest) = payload.strip_prefix(b"X") {
            return RspAction::status(self.handle_binary_write(rest, backend));
        }

        let cmd = match std::str::from_utf8(payload) {
            Ok(s) => s,
            Err(_) => return RspAction::status(RspReplyCode::ErrBadRequest),
        };

        if cmd == "?" {
            if !backend.core_halted(self.core_idx).unwrap_or(true) {
                let _ = backend.halt_core(self.core_idx);
            }
            return RspAction::status(RspReplyCode::SigTrap);
        }
        if cmd == "g" {
            return RspAction::Reply(self.handle_read_all_regs(backend));
        }
        if let Some(hex_regs) = cmd.strip_prefix('G') {
            return RspAction::status(self.handle_write_all_regs(hex_regs, backend));
        }
        if let Some(reg_hex) = cmd.strip_prefix('p') {
            return RspAction::Reply(self.handle_read_single_reg(reg_hex, backend));
        }
        if let Some(assign) = cmd.strip_prefix('P') {
            return RspAction::status(self.handle_write_single_reg(assign, backend));
        }
        if let Some(mem_args) = cmd.strip_prefix('m') {
            return RspAction::Reply(self.handle_read_mem(mem_args, backend));
        }
        if let Some(mem_args) = cmd.strip_prefix('M') {
            return RspAction::status(self.handle_write_mem_hex(mem_args, backend));
        }
        if cmd.starts_with("Z0,") || cmd.starts_with("Z1,") {
            let allow_sw_ram = true;
            return RspAction::status(self.handle_insert_bp(&cmd[3..], allow_sw_ram, backend));
        }
        if cmd.starts_with("z0,") || cmd.starts_with("z1,") {
            return RspAction::status(self.handle_remove_bp(&cmd[3..], backend));
        }
        if cmd == "s" || (cmd.starts_with('s') && cmd[1..].chars().all(|c| c.is_ascii_hexdigit())) {
            if cmd.len() > 1 {
                if let (Ok(addr), Some(pc_desc)) = (
                    u32::from_str_radix(&cmd[1..], 16),
                    self.arch_profile.pc_register().cloned(),
                ) {
                    let norm = self.arch_profile.normalize_pc(addr);
                    let _ = backend.write_core_reg(self.core_idx, pc_desc.hw_reg_id, norm);
                }
            }
            return match backend.step_core(self.core_idx) {
                Ok(_) => RspAction::status(RspReplyCode::SigTrap),
                Err(_) => RspAction::status(RspReplyCode::ErrTargetState),
            };
        }
        if let Some(vcont_body) = cmd.strip_prefix("vCont;") {
            let first_action = vcont_body.split(';').next().unwrap_or("");
            let action_char = first_action.chars().next().unwrap_or('c');
            match action_char {
                's' | 'S' => {
                    return match backend.step_core(self.core_idx) {
                        Ok(_) => RspAction::status(RspReplyCode::SigTrap),
                        Err(_) => RspAction::status(RspReplyCode::ErrTargetState),
                    };
                }
                't' => {
                    let _ = backend.halt_core(self.core_idx);
                    return RspAction::status(RspReplyCode::SigTrap);
                }
                _ => return self.resume_core_execution(None, backend),
            }
        }
        if cmd == "c" || (cmd.starts_with('c') && cmd[1..].chars().all(|c| c.is_ascii_hexdigit())) {
            let opt_addr = if cmd.len() > 1 {
                u32::from_str_radix(&cmd[1..], 16).ok()
            } else {
                None
            };
            return self.resume_core_execution(opt_addr, backend);
        }
        if let Some(rcmd_hex) = cmd.strip_prefix("qRcmd,") {
            return self.handle_monitor_command(rcmd_hex, backend);
        }
        if cmd == "D" || cmd.starts_with("D;") || cmd == "vKill" || cmd.starts_with("vKill;") {
            let _ = backend.clear_all_breakpoints(self.core_idx);
            let _ = backend.resume_core(self.core_idx);
            return RspAction::Disconnect(Some(RspReplyCode::Ok.as_reply_string()));
        }
        if cmd == "k" {
            let _ = backend.clear_all_breakpoints(self.core_idx);
            let _ = backend.resume_core(self.core_idx);
            return RspAction::Disconnect(None);
        }

        RspAction::status(RspReplyCode::Empty)
    }

    fn handle_read_all_regs<B: DebugBackend + ?Sized>(&self, backend: &mut B) -> String {
        if !backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = backend.halt_core(self.core_idx);
        }
        let regs = self.arch_profile.general_registers();
        let total_hex_chars: usize = regs.iter().map(|r| r.byte_width() * 2).sum();
        let mut hex = String::with_capacity(total_hex_chars);
        for reg in regs {
            let val = match backend.read_core_reg(self.core_idx, reg.hw_reg_id) {
                Ok(v) if reg.is_pc => self.arch_profile.normalize_pc(v),
                Ok(v) => v,
                Err(_) => 0,
            };
            let le_bytes = val.to_le_bytes();
            let width = reg.byte_width().min(4);
            hex.push_str(&encode_hex(&le_bytes[..width]));
        }
        hex
    }

    fn handle_write_all_regs<B: DebugBackend + ?Sized>(
        &self,
        hex_regs: &str,
        backend: &mut B,
    ) -> RspReplyCode {
        let raw = match decode_hex(hex_regs) {
            Ok(b) => b,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let mut offset = 0usize;
        for reg in self.arch_profile.general_registers() {
            let width = reg.byte_width().min(4);
            if offset + width > raw.len() {
                break;
            }
            let mut word = [0u8; 4];
            word[..width].copy_from_slice(&raw[offset..offset + width]);
            offset += width;
            let val = u32::from_le_bytes(word);
            let normalized = if reg.is_pc {
                self.arch_profile.normalize_pc(val)
            } else {
                val
            };
            if backend
                .write_core_reg(self.core_idx, reg.hw_reg_id, normalized)
                .is_err()
            {
                return RspReplyCode::ErrRegAccess;
            }
        }
        RspReplyCode::Ok
    }

    fn handle_read_single_reg<B: DebugBackend + ?Sized>(
        &self,
        reg_hex: &str,
        backend: &mut B,
    ) -> String {
        let gdb_reg = match u32::from_str_radix(reg_hex.trim(), 16) {
            Ok(r) => r,
            Err(_) => return RspReplyCode::ErrBadRequest.as_reply_string(),
        };
        let Some(desc) = self.arch_profile.lookup_gdb_register(gdb_reg) else {
            return RspReplyCode::ErrBadRequest.as_reply_string();
        };
        if !backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = backend.halt_core(self.core_idx);
        }
        match backend.read_core_reg(self.core_idx, desc.hw_reg_id) {
            Ok(val) => {
                let normalized = if desc.is_pc {
                    self.arch_profile.normalize_pc(val)
                } else {
                    val
                };
                let le_bytes = normalized.to_le_bytes();
                let width = desc.byte_width().min(4);
                encode_hex(&le_bytes[..width])
            }
            Err(_) => RspReplyCode::ErrRegAccess.as_reply_string(),
        }
    }

    fn handle_write_single_reg<B: DebugBackend + ?Sized>(
        &self,
        assign: &str,
        backend: &mut B,
    ) -> RspReplyCode {
        let (reg_hex, val_hex) = match assign.split_once('=') {
            Some(pair) => pair,
            None => return RspReplyCode::ErrBadRequest,
        };
        let gdb_reg = match u32::from_str_radix(reg_hex.trim(), 16) {
            Ok(r) => r,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let Some(desc) = self.arch_profile.lookup_gdb_register(gdb_reg) else {
            return RspReplyCode::ErrBadRequest;
        };
        let width = desc.byte_width().min(4);
        let bytes = match decode_hex(val_hex.trim()) {
            Ok(b) if b.len() == width => b,
            _ => return RspReplyCode::ErrBadRequest,
        };
        let mut word = [0u8; 4];
        word[..width].copy_from_slice(&bytes);
        let val = u32::from_le_bytes(word);
        let normalized = if desc.is_pc {
            self.arch_profile.normalize_pc(val)
        } else {
            val
        };
        match backend.write_core_reg(self.core_idx, desc.hw_reg_id, normalized) {
            Ok(()) => RspReplyCode::Ok,
            Err(_) => RspReplyCode::ErrRegAccess,
        }
    }

    fn handle_read_mem<B: DebugBackend + ?Sized>(&self, mem_args: &str, backend: &mut B) -> String {
        let (addr_hex, len_hex) = match mem_args.split_once(',') {
            Some(pair) => pair,
            None => return RspReplyCode::ErrBadRequest.as_reply_string(),
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return RspReplyCode::ErrBadRequest.as_reply_string(),
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) if l <= MAX_RSP_MEMORY_READ_BYTES => l,
            _ => return RspReplyCode::ErrBadRequest.as_reply_string(),
        };
        if len == 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len];
        if backend
            .read_memory_masked(self.core_idx, addr, &mut buf)
            .is_err()
        {
            return RspReplyCode::ErrBadRequest.as_reply_string();
        }
        encode_hex(&buf)
    }

    fn handle_write_mem_hex<B: DebugBackend + ?Sized>(
        &self,
        mem_args: &str,
        backend: &mut B,
    ) -> RspReplyCode {
        let (header, hex_data) = match mem_args.split_once(':') {
            Some(pair) => pair,
            None => return RspReplyCode::ErrBadRequest,
        };
        let (addr_hex, len_hex) = match header.split_once(',') {
            Some(pair) => pair,
            None => return RspReplyCode::ErrBadRequest,
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) => l,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let data = match decode_hex(hex_data.trim()) {
            Ok(b) if b.len() == len => b,
            _ => return RspReplyCode::ErrBadRequest,
        };
        if data.is_empty() {
            return RspReplyCode::Ok;
        }
        match backend.write_memory(self.core_idx, addr, &data) {
            Ok(()) => RspReplyCode::Ok,
            Err(_) => RspReplyCode::ErrBadRequest,
        }
    }

    fn handle_binary_write<B: DebugBackend + ?Sized>(
        &self,
        rest: &[u8],
        backend: &mut B,
    ) -> RspReplyCode {
        let colon_pos = match rest.iter().position(|&b| b == b':') {
            Some(p) => p,
            None => return RspReplyCode::ErrBadRequest,
        };
        let header = match std::str::from_utf8(&rest[..colon_pos]) {
            Ok(s) => s,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let (addr_hex, len_hex) = match header.split_once(',') {
            Some(pair) => pair,
            None => return RspReplyCode::ErrBadRequest,
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) => l,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        let data = unescape_rsp_binary(&rest[colon_pos + 1..]);
        if data.len() != len {
            return RspReplyCode::ErrBadRequest;
        }
        if data.is_empty() {
            return RspReplyCode::Ok;
        }
        match backend.write_memory(self.core_idx, addr, &data) {
            Ok(()) => RspReplyCode::Ok,
            Err(_) => RspReplyCode::ErrBadRequest,
        }
    }

    fn handle_insert_bp<B: DebugBackend + ?Sized>(
        &self,
        args: &str,
        allow_sw_ram: bool,
        backend: &mut B,
    ) -> RspReplyCode {
        let addr_hex = args.split(',').next().unwrap_or("").trim();
        let addr = match u64::from_str_radix(addr_hex, 16) {
            Ok(a) => a,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        match backend.insert_breakpoint(self.core_idx, addr, allow_sw_ram) {
            Ok(true) => RspReplyCode::Ok,
            Ok(false) | Err(_) => RspReplyCode::ErrNoBreakpointSlot,
        }
    }

    fn handle_remove_bp<B: DebugBackend + ?Sized>(
        &self,
        args: &str,
        backend: &mut B,
    ) -> RspReplyCode {
        let addr_hex = args.split(',').next().unwrap_or("").trim();
        let addr = match u64::from_str_radix(addr_hex, 16) {
            Ok(a) => a,
            Err(_) => return RspReplyCode::ErrBadRequest,
        };
        match backend.remove_breakpoint(self.core_idx, addr) {
            Ok(()) => RspReplyCode::Ok,
            Err(_) => RspReplyCode::ErrBadRequest,
        }
    }

    fn resume_core_execution<B: DebugBackend + ?Sized>(
        &self,
        opt_addr: Option<u32>,
        backend: &mut B,
    ) -> RspAction {
        if let (Some(addr), Some(pc_desc)) = (opt_addr, self.arch_profile.pc_register().cloned()) {
            let norm = self.arch_profile.normalize_pc(addr);
            let _ = backend.write_core_reg(self.core_idx, pc_desc.hw_reg_id, norm);
        }
        match backend.resume_core(self.core_idx) {
            Ok(()) => RspAction::WaitForHalt,
            Err(_) => RspAction::status(RspReplyCode::ErrTargetState),
        }
    }

    fn handle_monitor_command<B: DebugBackend + ?Sized>(
        &self,
        rcmd_hex: &str,
        backend: &mut B,
    ) -> RspAction {
        let cmd_bytes = match decode_hex(rcmd_hex.trim()) {
            Ok(b) => b,
            Err(_) => return RspAction::status(RspReplyCode::ErrBadRequest),
        };
        let monitor_cmd = String::from_utf8_lossy(&cmd_bytes)
            .trim()
            .to_ascii_lowercase();
        match monitor_cmd.as_str() {
            "reset halt" => {
                if backend.reset_and_halt_core(0).is_err() {
                    return RspAction::status(RspReplyCode::ErrTargetState);
                }
                let out_pkt = format!(
                    "O{}",
                    encode_hex(b"Target reset and halted at Reset_Handler\n")
                );
                RspAction::ReplyPackets(vec![out_pkt, RspReplyCode::Ok.as_reply_string()])
            }
            "reset" | "reset run" => {
                if backend.reset_and_run_core(0).is_err() {
                    return RspAction::status(RspReplyCode::ErrTargetState);
                }
                let out_pkt = format!("O{}", encode_hex(b"Target reset and running\n"));
                RspAction::ReplyPackets(vec![out_pkt, RspReplyCode::Ok.as_reply_string()])
            }
            "halt" => match backend.halt_core(self.core_idx) {
                Ok(_) => RspAction::status(RspReplyCode::Ok),
                Err(_) => RspAction::status(RspReplyCode::ErrTargetState),
            },
            _ => RspAction::status(RspReplyCode::Ok),
        }
    }
}
