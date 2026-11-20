//! GDB Remote Serial Protocol (RSP) packet framing, Cortex-M XML target descriptions,
//! per-core breakpoint management, and command dispatch.

use crate::server::DebugBackend;
use std::collections::BTreeMap;

/// Start address of Cortex-M SRAM region (`0x2000_0000`).
pub const CORTEX_M_SRAM_START: u64 = 0x2000_0000;

/// Exclusive end address of Cortex-M SRAM region (`0x4000_0000`).
pub const CORTEX_M_SRAM_END: u64 = 0x4000_0000;

/// Little-endian encoding of 16-bit Thumb `BKPT #0` (`0xBE00`).
pub const THUMB_BKPT_0_LE: [u8; 2] = [0x00, 0xBE];

/// Maximum single RSP memory read chunk size in bytes.
pub const MAX_RSP_MEMORY_READ_BYTES: usize = 4096;

/// ARM DCRSR register number for Program Counter (`r15`).
pub const DCRSR_PC: u16 = 15;

/// ARM DCRSR register number for Program Status Register (`xPSR`).
pub const DCRSR_XPSR: u16 = 16;

/// ARM DCRSR register number for Main Stack Pointer (`MSP`).
pub const DCRSR_MSP: u16 = 17;

/// ARM DCRSR register number for Process Stack Pointer (`PSP`).
pub const DCRSR_PSP: u16 = 18;

/// ARM DCRSR register number for Floating-Point Status and Control Register (`FPSCR`).
pub const DCRSR_FPSCR: u16 = 33;

/// ARM DCRSR base register number for single-precision FPU registers (`s0..s31`).
pub const DCRSR_S0_BASE: u16 = 64;

/// GDB XML `regnum` for `xPSR` in `org.gnu.gdb.arm.m-profile`.
pub const GDB_REGNUM_XPSR: u32 = 25;

/// GDB XML `regnum` for `MSP` in `org.gnu.gdb.arm.m-system`.
pub const GDB_REGNUM_MSP: u32 = 26;

/// GDB XML `regnum` for `PSP` in `org.gnu.gdb.arm.m-system`.
pub const GDB_REGNUM_PSP: u32 = 27;

/// GDB XML base `regnum` for `s0..s31` in `org.gnu.gdb.arm.vfp`.
pub const GDB_REGNUM_S0_BASE: u32 = 32;

/// GDB XML `regnum` for `fpscr` in `org.gnu.gdb.arm.vfp`.
pub const GDB_REGNUM_FPSCR: u32 = 64;

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
        /// Unescaped packet payload bytes between `'$'` and `'#'`.
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

/// Per-core shadow hardware FPB comparator and SRAM software breakpoint state.
#[derive(Clone, Debug)]
pub struct CoreBreakpointTable {
    /// Shadow table of hardware FPB comparator addresses (`None` when comparator unit is free).
    pub hw_slots: Vec<Option<u64>>,
    /// Original 2-byte Thumb instruction words replaced by `BKPT #0` (`0xBE00`) in SRAM.
    pub sw_ram_patches: BTreeMap<u64, [u8; 2]>,
}

impl CoreBreakpointTable {
    /// Create a new breakpoint table with `hw_unit_count` hardware comparator slots.
    pub fn new(hw_unit_count: usize) -> Self {
        Self {
            hw_slots: vec![None; hw_unit_count],
            sw_ram_patches: BTreeMap::new(),
        }
    }

    /// Return `true` if at least one hardware comparator slot is currently allocated.
    pub fn has_active_hw_breakpoints(&self) -> bool {
        self.hw_slots.iter().any(Option::is_some)
    }

    /// Insert a breakpoint at `raw_addr` on `core_idx`, using hardware FPB or SRAM `BKPT #0`.
    pub fn insert_breakpoint<B: DebugBackend + ?Sized>(
        &mut self,
        backend: &mut B,
        core_idx: usize,
        raw_addr: u64,
        allow_sw_ram: bool,
    ) -> Result<bool, String> {
        let addr = raw_addr & !1;
        if self.hw_slots.contains(&Some(addr)) || self.sw_ram_patches.contains_key(&addr) {
            return Ok(true);
        }

        if backend.supports_hw_breakpoint_at(core_idx, addr) {
            if let Some(free_slot) = self.hw_slots.iter().position(Option::is_none) {
                backend.set_hw_breakpoint_slot(core_idx, free_slot, addr)?;
                self.hw_slots[free_slot] = Some(addr);
                return Ok(true);
            }
        }

        if allow_sw_ram && (CORTEX_M_SRAM_START..CORTEX_M_SRAM_END).contains(&addr) {
            let mut orig = [0u8; 2];
            backend.read_memory(core_idx, addr, &mut orig)?;
            backend.write_memory(core_idx, addr, &THUMB_BKPT_0_LE)?;
            self.sw_ram_patches.insert(addr, orig);
            return Ok(true);
        }

        Ok(false)
    }

    /// Remove an active hardware or SRAM software breakpoint at `raw_addr` on `core_idx`.
    pub fn remove_breakpoint<B: DebugBackend + ?Sized>(
        &mut self,
        backend: &mut B,
        core_idx: usize,
        raw_addr: u64,
    ) -> Result<(), String> {
        let addr = raw_addr & !1;
        for (slot_idx, entry) in self.hw_slots.iter_mut().enumerate() {
            if *entry == Some(addr) {
                let _ = backend.clear_hw_breakpoint_slot(core_idx, slot_idx, addr);
                *entry = None;
            }
        }
        if let Some(orig) = self.sw_ram_patches.remove(&addr) {
            backend.write_memory(core_idx, addr, &orig)?;
        }
        Ok(())
    }

    /// Clear all hardware FPB comparators and restore all SRAM software `BKPT #0` patches on `core_idx`.
    pub fn clear_all<B: DebugBackend + ?Sized>(
        &mut self,
        backend: &mut B,
        core_idx: usize,
    ) -> Result<(), String> {
        for (slot_idx, entry) in self.hw_slots.iter_mut().enumerate() {
            if let Some(addr) = entry.take() {
                let _ = backend.clear_hw_breakpoint_slot(core_idx, slot_idx, addr);
            }
        }
        let patches: Vec<(u64, [u8; 2])> = std::mem::take(&mut self.sw_ram_patches)
            .into_iter()
            .collect();
        for (addr, orig) in patches {
            let _ = backend.write_memory(core_idx, addr, &orig);
        }
        Ok(())
    }

    /// Re-arm hardware FPB comparators if `core_idx` underwent a subsystem reset (e.g. RP2040 `PSM.frce_off`).
    pub fn ensure_hw_breakpoints_armed<B: DebugBackend + ?Sized>(
        &self,
        backend: &mut B,
        core_idx: usize,
    ) -> Result<bool, String> {
        if !self.has_active_hw_breakpoints() {
            return Ok(false);
        }
        backend.ensure_hw_breakpoints_active(core_idx, &self.hw_slots)
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
            // Discard stray leading transport bytes up to the next frame marker.
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

/// Generate a GDB `target.xml` description for an ARM Cortex-M core (`M0+` / `M33`).
pub fn build_target_xml(has_fpu: bool) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0"?>
<!DOCTYPE target SYSTEM "gdb-target.dtd">
<target version="1.0">
  <architecture>arm</architecture>
  <feature name="org.gnu.gdb.arm.m-profile">
    <reg name="r0" bitsize="32" regnum="0" type="uint32"/>
    <reg name="r1" bitsize="32" regnum="1" type="uint32"/>
    <reg name="r2" bitsize="32" regnum="2" type="uint32"/>
    <reg name="r3" bitsize="32" regnum="3" type="uint32"/>
    <reg name="r4" bitsize="32" regnum="4" type="uint32"/>
    <reg name="r5" bitsize="32" regnum="5" type="uint32"/>
    <reg name="r6" bitsize="32" regnum="6" type="uint32"/>
    <reg name="r7" bitsize="32" regnum="7" type="uint32"/>
    <reg name="r8" bitsize="32" regnum="8" type="uint32"/>
    <reg name="r9" bitsize="32" regnum="9" type="uint32"/>
    <reg name="r10" bitsize="32" regnum="10" type="uint32"/>
    <reg name="r11" bitsize="32" regnum="11" type="uint32"/>
    <reg name="r12" bitsize="32" regnum="12" type="uint32"/>
    <reg name="sp" bitsize="32" regnum="13" type="data_ptr"/>
    <reg name="lr" bitsize="32" regnum="14" type="int"/>
    <reg name="pc" bitsize="32" regnum="15" type="code_ptr"/>
    <reg name="xpsr" bitsize="32" regnum="25" type="uint32"/>
  </feature>
  <feature name="org.gnu.gdb.arm.m-system">
    <reg name="msp" bitsize="32" regnum="26" type="data_ptr"/>
    <reg name="psp" bitsize="32" regnum="27" type="data_ptr"/>
  </feature>
"#,
    );
    if has_fpu {
        xml.push_str("  <feature name=\"org.gnu.gdb.arm.vfp\">\n");
        for idx in 0..32u32 {
            use std::fmt::Write as _;
            let _ = writeln!(
                &mut xml,
                "    <reg name=\"s{}\" bitsize=\"32\" regnum=\"{}\" type=\"ieee_single\"/>",
                idx,
                GDB_REGNUM_S0_BASE + idx
            );
        }
        xml.push_str(&format!(
            "    <reg name=\"fpscr\" bitsize=\"32\" regnum=\"{}\" type=\"int\"/>\n  </feature>\n",
            GDB_REGNUM_FPSCR
        ));
    }
    xml.push_str("</target>\n");
    xml
}

/// Ordered list of DCRSR register IDs serialized in `g` / `G` packets matching [`build_target_xml`].
fn general_register_dcrsr_order(has_fpu: bool) -> Vec<u16> {
    let mut regs: Vec<u16> = (0..=15).collect();
    regs.push(DCRSR_XPSR);
    regs.push(DCRSR_MSP);
    regs.push(DCRSR_PSP);
    if has_fpu {
        for idx in 0..32u16 {
            regs.push(DCRSR_S0_BASE + idx);
        }
        regs.push(DCRSR_FPSCR);
    }
    regs
}

/// Map a GDB `p` / `P` register index to its ARM Cortex-M DCRSR register number.
fn map_gdb_regnum_to_dcrsr(gdb_regnum: u32, has_fpu: bool) -> Option<u16> {
    match gdb_regnum {
        0..=15 => Some(gdb_regnum as u16),
        16 | GDB_REGNUM_XPSR => Some(DCRSR_XPSR),
        17 | GDB_REGNUM_MSP => Some(DCRSR_MSP),
        18 | GDB_REGNUM_PSP => Some(DCRSR_PSP),
        r if has_fpu && (GDB_REGNUM_S0_BASE..GDB_REGNUM_S0_BASE + 32).contains(&r) => {
            Some(DCRSR_S0_BASE + (r - GDB_REGNUM_S0_BASE) as u16)
        }
        GDB_REGNUM_FPSCR if has_fpu => Some(DCRSR_FPSCR),
        _ => None,
    }
}

/// Stateful GDB RSP command handler bound to a single MCU core index.
pub struct RspCoreHandler {
    /// Target core index (`0` for Core 0, `1` for Core 1).
    pub core_idx: usize,
    /// Whether `QStartNoAckMode` has been negotiated on this socket.
    pub no_ack_mode: bool,
    /// Whether this core exposes Cortex-M single-precision FPU registers.
    pub has_fpu: bool,
    /// Cached `target.xml` payload for `qXfer:features:read:target.xml`.
    pub target_xml: String,
}

impl RspCoreHandler {
    /// Create a new per-core GDB RSP handler for `core_idx`.
    pub fn new(core_idx: usize, has_fpu: bool) -> Self {
        Self {
            core_idx,
            no_ack_mode: false,
            has_fpu,
            target_xml: build_target_xml(has_fpu),
        }
    }

    /// Handle pure protocol handshake/metadata packets that do not require locking [`DebugBackend`].
    pub fn handle_stateless_packet(&mut self, payload: &[u8]) -> Option<RspAction> {
        if payload.starts_with(b"X") {
            return None;
        }
        let cmd = match std::str::from_utf8(payload) {
            Ok(s) => s,
            Err(_) => return Some(RspAction::Reply("E01".to_string())),
        };

        if cmd == "QStartNoAckMode" {
            self.no_ack_mode = true;
            return Some(RspAction::Reply("OK".to_string()));
        }
        if cmd.starts_with("qSupported") {
            return Some(RspAction::Reply(
                "PacketSize=4096;qXfer:features:read+;QStartNoAckMode+;vContSupported+;hwbreak+;swbreak+"
                    .to_string(),
            ));
        }
        if cmd == "!" {
            return Some(RspAction::Reply("OK".to_string()));
        }
        if cmd == "vMustReplyEmpty" || cmd == "qOffsets" || cmd == "qTStatus" {
            return Some(RspAction::Reply(String::new()));
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
            return Some(RspAction::Reply("OK".to_string()));
        }
        if let Some(xfer_args) = cmd.strip_prefix("qXfer:features:read:target.xml:") {
            return Some(RspAction::Reply(self.handle_target_xml_read(xfer_args)));
        }
        None
    }

    /// Process a single unframed RSP packet payload against `backend` and `breakpoints`.
    pub fn handle_packet<B: DebugBackend + ?Sized>(
        &mut self,
        payload: &[u8],
        backend: &mut B,
        breakpoints: &mut CoreBreakpointTable,
    ) -> RspAction {
        if let Some(stateless) = self.handle_stateless_packet(payload) {
            return stateless;
        }
        if let Some(rest) = payload.strip_prefix(b"X") {
            return RspAction::Reply(self.handle_binary_write(rest, backend));
        }

        let cmd = match std::str::from_utf8(payload) {
            Ok(s) => s,
            Err(_) => return RspAction::Reply("E01".to_string()),
        };

        if cmd == "?" {
            if !backend.core_halted(self.core_idx).unwrap_or(true) {
                let _ = backend.halt_core(self.core_idx);
            }
            return RspAction::Reply("S05".to_string());
        }
        if cmd == "g" {
            return RspAction::Reply(self.handle_read_all_regs(backend));
        }
        if let Some(hex_regs) = cmd.strip_prefix('G') {
            return RspAction::Reply(self.handle_write_all_regs(hex_regs, backend));
        }
        if let Some(reg_hex) = cmd.strip_prefix('p') {
            return RspAction::Reply(self.handle_read_single_reg(reg_hex, backend));
        }
        if let Some(assign) = cmd.strip_prefix('P') {
            return RspAction::Reply(self.handle_write_single_reg(assign, backend));
        }
        if let Some(mem_args) = cmd.strip_prefix('m') {
            return RspAction::Reply(self.handle_read_mem(mem_args, backend, breakpoints));
        }
        if let Some(mem_args) = cmd.strip_prefix('M') {
            return RspAction::Reply(self.handle_write_mem_hex(mem_args, backend));
        }
        if cmd.starts_with("Z0,") || cmd.starts_with("Z1,") {
            let allow_sw_ram = true;
            return RspAction::Reply(self.handle_insert_bp(
                &cmd[3..],
                allow_sw_ram,
                backend,
                breakpoints,
            ));
        }
        if cmd.starts_with("z0,") || cmd.starts_with("z1,") {
            return RspAction::Reply(self.handle_remove_bp(&cmd[3..], backend, breakpoints));
        }
        if cmd == "s" || cmd.starts_with("s") && cmd[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            if cmd.len() > 1 {
                if let Ok(addr) = u32::from_str_radix(&cmd[1..], 16) {
                    let _ = backend.write_core_reg(self.core_idx, DCRSR_PC, addr & !1);
                }
            }
            return RspAction::Reply(self.step_core_once(backend, breakpoints));
        }
        if let Some(vcont_body) = cmd.strip_prefix("vCont;") {
            let first_action = vcont_body.split(';').next().unwrap_or("");
            let action_char = first_action.chars().next().unwrap_or('c');
            match action_char {
                's' | 'S' => return RspAction::Reply(self.step_core_once(backend, breakpoints)),
                't' => {
                    let _ = backend.halt_core(self.core_idx);
                    return RspAction::Reply("S05".to_string());
                }
                _ => return self.resume_core_execution(None, backend, breakpoints),
            }
        }
        if cmd == "c" || (cmd.starts_with('c') && cmd[1..].chars().all(|c| c.is_ascii_hexdigit())) {
            let opt_addr = if cmd.len() > 1 {
                u32::from_str_radix(&cmd[1..], 16).ok()
            } else {
                None
            };
            return self.resume_core_execution(opt_addr, backend, breakpoints);
        }
        if let Some(rcmd_hex) = cmd.strip_prefix("qRcmd,") {
            return self.handle_monitor_command(rcmd_hex, backend, breakpoints);
        }
        if cmd == "D" || cmd.starts_with("D;") {
            let _ = breakpoints.clear_all(backend, self.core_idx);
            let _ = backend.resume_core(self.core_idx);
            return RspAction::Disconnect(Some("OK".to_string()));
        }
        if cmd == "vKill" || cmd.starts_with("vKill;") {
            let _ = breakpoints.clear_all(backend, self.core_idx);
            let _ = backend.resume_core(self.core_idx);
            return RspAction::Disconnect(Some("OK".to_string()));
        }
        if cmd == "k" {
            let _ = breakpoints.clear_all(backend, self.core_idx);
            let _ = backend.resume_core(self.core_idx);
            return RspAction::Disconnect(None);
        }

        // Unsupported optional query: empty reply per GDB RSP specification.
        RspAction::Reply(String::new())
    }

    fn handle_target_xml_read(&self, xfer_args: &str) -> String {
        let mut parts = xfer_args.split(',');
        let offset = parts
            .next()
            .and_then(|s| usize::from_str_radix(s, 16).ok())
            .unwrap_or(0);
        let length = parts
            .next()
            .and_then(|s| usize::from_str_radix(s, 16).ok())
            .unwrap_or(0);
        let bytes = self.target_xml.as_bytes();
        if offset >= bytes.len() {
            return "l".to_string();
        }
        let end = std::cmp::min(bytes.len(), offset.saturating_add(length));
        let chunk = &self.target_xml[offset..end];
        if end >= bytes.len() {
            format!("l{}", chunk)
        } else {
            format!("m{}", chunk)
        }
    }

    fn handle_read_all_regs<B: DebugBackend + ?Sized>(&self, backend: &mut B) -> String {
        if !backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = backend.halt_core(self.core_idx);
        }
        let order = general_register_dcrsr_order(self.has_fpu);
        let mut hex = String::with_capacity(order.len() * 8);
        for reg_id in order {
            let val = match backend.read_core_reg(self.core_idx, reg_id) {
                Ok(v) if reg_id == DCRSR_PC => v & !1,
                Ok(v) => v,
                Err(_) => 0,
            };
            hex.push_str(&encode_hex(&val.to_le_bytes()));
        }
        hex
    }

    fn handle_write_all_regs<B: DebugBackend + ?Sized>(
        &self,
        hex_regs: &str,
        backend: &mut B,
    ) -> String {
        let raw = match decode_hex(hex_regs) {
            Ok(b) => b,
            Err(_) => return "E01".to_string(),
        };
        let order = general_register_dcrsr_order(self.has_fpu);
        for (idx, reg_id) in order.into_iter().enumerate() {
            let offset = idx * 4;
            if offset + 4 > raw.len() {
                break;
            }
            let mut word = [0u8; 4];
            word.copy_from_slice(&raw[offset..offset + 4]);
            let val = u32::from_le_bytes(word);
            let normalized = if reg_id == DCRSR_PC { val & !1 } else { val };
            if backend
                .write_core_reg(self.core_idx, reg_id, normalized)
                .is_err()
            {
                return "E02".to_string();
            }
        }
        "OK".to_string()
    }

    fn handle_read_single_reg<B: DebugBackend + ?Sized>(
        &self,
        reg_hex: &str,
        backend: &mut B,
    ) -> String {
        let gdb_reg = match u32::from_str_radix(reg_hex.trim(), 16) {
            Ok(r) => r,
            Err(_) => return "E01".to_string(),
        };
        let dcrsr = match map_gdb_regnum_to_dcrsr(gdb_reg, self.has_fpu) {
            Some(id) => id,
            None => return "E01".to_string(),
        };
        if !backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = backend.halt_core(self.core_idx);
        }
        match backend.read_core_reg(self.core_idx, dcrsr) {
            Ok(val) => {
                let normalized = if dcrsr == DCRSR_PC { val & !1 } else { val };
                encode_hex(&normalized.to_le_bytes())
            }
            Err(_) => "E02".to_string(),
        }
    }

    fn handle_write_single_reg<B: DebugBackend + ?Sized>(
        &self,
        assign: &str,
        backend: &mut B,
    ) -> String {
        let (reg_hex, val_hex) = match assign.split_once('=') {
            Some(pair) => pair,
            None => return "E01".to_string(),
        };
        let gdb_reg = match u32::from_str_radix(reg_hex.trim(), 16) {
            Ok(r) => r,
            Err(_) => return "E01".to_string(),
        };
        let dcrsr = match map_gdb_regnum_to_dcrsr(gdb_reg, self.has_fpu) {
            Some(id) => id,
            None => return "E01".to_string(),
        };
        let bytes = match decode_hex(val_hex.trim()) {
            Ok(b) if b.len() == 4 => b,
            _ => return "E01".to_string(),
        };
        let val = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let normalized = if dcrsr == DCRSR_PC { val & !1 } else { val };
        match backend.write_core_reg(self.core_idx, dcrsr, normalized) {
            Ok(()) => "OK".to_string(),
            Err(_) => "E02".to_string(),
        }
    }

    fn handle_read_mem<B: DebugBackend + ?Sized>(
        &self,
        mem_args: &str,
        backend: &mut B,
        breakpoints: &CoreBreakpointTable,
    ) -> String {
        let (addr_hex, len_hex) = match mem_args.split_once(',') {
            Some(pair) => pair,
            None => return "E01".to_string(),
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return "E01".to_string(),
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) if l <= MAX_RSP_MEMORY_READ_BYTES => l,
            _ => return "E01".to_string(),
        };
        if len == 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len];
        if backend.read_memory(self.core_idx, addr, &mut buf).is_err() {
            return "E01".to_string();
        }
        // Mask active SRAM software BKPT patches so GDB disassembly views original opcodes.
        let end_addr = addr + len as u64;
        for (&bp_addr, orig) in &breakpoints.sw_ram_patches {
            for (byte_idx, &orig_byte) in orig.iter().enumerate() {
                let target_byte_addr = bp_addr + byte_idx as u64;
                if target_byte_addr >= addr && target_byte_addr < end_addr {
                    buf[(target_byte_addr - addr) as usize] = orig_byte;
                }
            }
        }
        encode_hex(&buf)
    }

    fn handle_write_mem_hex<B: DebugBackend + ?Sized>(
        &self,
        mem_args: &str,
        backend: &mut B,
    ) -> String {
        let (header, hex_data) = match mem_args.split_once(':') {
            Some(pair) => pair,
            None => return "E01".to_string(),
        };
        let (addr_hex, len_hex) = match header.split_once(',') {
            Some(pair) => pair,
            None => return "E01".to_string(),
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return "E01".to_string(),
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) => l,
            Err(_) => return "E01".to_string(),
        };
        let data = match decode_hex(hex_data.trim()) {
            Ok(b) if b.len() == len => b,
            _ => return "E01".to_string(),
        };
        if data.is_empty() {
            return "OK".to_string();
        }
        match backend.write_memory(self.core_idx, addr, &data) {
            Ok(()) => "OK".to_string(),
            Err(_) => "E01".to_string(),
        }
    }

    fn handle_binary_write<B: DebugBackend + ?Sized>(
        &self,
        rest: &[u8],
        backend: &mut B,
    ) -> String {
        let colon_pos = match rest.iter().position(|&b| b == b':') {
            Some(p) => p,
            None => return "E01".to_string(),
        };
        let header = match std::str::from_utf8(&rest[..colon_pos]) {
            Ok(s) => s,
            Err(_) => return "E01".to_string(),
        };
        let (addr_hex, len_hex) = match header.split_once(',') {
            Some(pair) => pair,
            None => return "E01".to_string(),
        };
        let addr = match u64::from_str_radix(addr_hex.trim(), 16) {
            Ok(a) => a,
            Err(_) => return "E01".to_string(),
        };
        let len = match usize::from_str_radix(len_hex.trim(), 16) {
            Ok(l) => l,
            Err(_) => return "E01".to_string(),
        };
        let data = unescape_rsp_binary(&rest[colon_pos + 1..]);
        if data.len() != len {
            return "E01".to_string();
        }
        if data.is_empty() {
            return "OK".to_string();
        }
        match backend.write_memory(self.core_idx, addr, &data) {
            Ok(()) => "OK".to_string(),
            Err(_) => "E01".to_string(),
        }
    }

    fn handle_insert_bp<B: DebugBackend + ?Sized>(
        &self,
        args: &str,
        allow_sw_ram: bool,
        backend: &mut B,
        breakpoints: &mut CoreBreakpointTable,
    ) -> String {
        let addr_hex = args.split(',').next().unwrap_or("").trim();
        let addr = match u64::from_str_radix(addr_hex, 16) {
            Ok(a) => a,
            Err(_) => return "E01".to_string(),
        };
        match breakpoints.insert_breakpoint(backend, self.core_idx, addr, allow_sw_ram) {
            Ok(true) => "OK".to_string(),
            Ok(false) => "E0E".to_string(),
            Err(_) => "E0E".to_string(),
        }
    }

    fn handle_remove_bp<B: DebugBackend + ?Sized>(
        &self,
        args: &str,
        backend: &mut B,
        breakpoints: &mut CoreBreakpointTable,
    ) -> String {
        let addr_hex = args.split(',').next().unwrap_or("").trim();
        let addr = match u64::from_str_radix(addr_hex, 16) {
            Ok(a) => a,
            Err(_) => return "E01".to_string(),
        };
        match breakpoints.remove_breakpoint(backend, self.core_idx, addr) {
            Ok(()) => "OK".to_string(),
            Err(_) => "E01".to_string(),
        }
    }

    fn step_over_sram_bkpt_if_needed<B: DebugBackend + ?Sized>(
        &self,
        backend: &mut B,
        breakpoints: &CoreBreakpointTable,
    ) -> Result<bool, String> {
        let pc = (backend.read_core_reg(self.core_idx, DCRSR_PC)? & !1) as u64;
        if let Some(orig) = breakpoints.sw_ram_patches.get(&pc) {
            backend.write_memory(self.core_idx, pc, orig)?;
            let step_res = backend.step_core(self.core_idx);
            let _ = backend.write_memory(self.core_idx, pc, &THUMB_BKPT_0_LE);
            step_res?;
            return Ok(true);
        }
        Ok(false)
    }

    fn step_core_once<B: DebugBackend + ?Sized>(
        &self,
        backend: &mut B,
        breakpoints: &CoreBreakpointTable,
    ) -> String {
        if !backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = backend.halt_core(self.core_idx);
        }
        match self.step_over_sram_bkpt_if_needed(backend, breakpoints) {
            Ok(true) => "S05".to_string(),
            Ok(false) => match backend.step_core(self.core_idx) {
                Ok(_) => "S05".to_string(),
                Err(_) => "E05".to_string(),
            },
            Err(_) => "E05".to_string(),
        }
    }

    fn resume_core_execution<B: DebugBackend + ?Sized>(
        &self,
        opt_addr: Option<u32>,
        backend: &mut B,
        breakpoints: &CoreBreakpointTable,
    ) -> RspAction {
        if let Some(addr) = opt_addr {
            let _ = backend.write_core_reg(self.core_idx, DCRSR_PC, addr & !1);
        }
        if backend.core_halted(self.core_idx).unwrap_or(false) {
            let _ = self.step_over_sram_bkpt_if_needed(backend, breakpoints);
        }
        let _ = breakpoints.ensure_hw_breakpoints_armed(backend, self.core_idx);
        match backend.resume_core(self.core_idx) {
            Ok(()) => RspAction::WaitForHalt,
            Err(_) => RspAction::Reply("E05".to_string()),
        }
    }

    fn handle_monitor_command<B: DebugBackend + ?Sized>(
        &self,
        rcmd_hex: &str,
        backend: &mut B,
        breakpoints: &CoreBreakpointTable,
    ) -> RspAction {
        let cmd_bytes = match decode_hex(rcmd_hex.trim()) {
            Ok(b) => b,
            Err(_) => return RspAction::Reply("E01".to_string()),
        };
        let monitor_cmd = String::from_utf8_lossy(&cmd_bytes)
            .trim()
            .to_ascii_lowercase();
        match monitor_cmd.as_str() {
            "reset halt" => {
                if backend.reset_and_halt_core(0).is_err() {
                    return RspAction::Reply("E05".to_string());
                }
                let _ = breakpoints.ensure_hw_breakpoints_armed(backend, self.core_idx);
                let out_pkt = format!(
                    "O{}",
                    encode_hex(b"Target reset and halted at Reset_Handler\n")
                );
                RspAction::ReplyPackets(vec![out_pkt, "OK".to_string()])
            }
            "reset" | "reset run" => {
                if backend.reset_and_run_core(0).is_err() {
                    return RspAction::Reply("E05".to_string());
                }
                let _ = breakpoints.ensure_hw_breakpoints_armed(backend, self.core_idx);
                let out_pkt = format!("O{}", encode_hex(b"Target reset and running\n"));
                RspAction::ReplyPackets(vec![out_pkt, "OK".to_string()])
            }
            "halt" => match backend.halt_core(self.core_idx) {
                Ok(_) => RspAction::Reply("OK".to_string()),
                Err(_) => RspAction::Reply("E05".to_string()),
            },
            _ => RspAction::Reply("OK".to_string()),
        }
    }
}
