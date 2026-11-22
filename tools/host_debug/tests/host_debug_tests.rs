//! Unit, RSP command-coverage, and live `arm-none-eabi-gdb` TCP integration tests for `host_debug`.

use host_debug::{
    build_arm_cortex_m_register_layout, decode_hex, encode_hex, extract_next_rsp_frame,
    format_rsp_packet, start_debug_server_with_backend, supports_arm_hw_breakpoint_at,
    ArmBreakpointManager, ArmHardwareOps, DebugBackend, DebugServerConfig, ProcessorArch,
    RspAction, RspCoreHandler, RspFrame, RspReplyCode, TargetArchProfile, TargetRegisterLayout,
};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tool_common::GdbClient;

const RESET_HANDLER_ADDR: u32 = 0x1000_01C0;
const CORE0_MAIN_LOC_ADDR: u32 = 0x1000_0420;
const CORE1_RESET_HANDLER_ADDR: u32 = 0x1001_EF60;
const CORE1_TASK_LOC_ADDR: u32 = 0x1001_EF98;
const SRAM_FUNC_ADDR: u64 = 0x2000_0230;
const RTT_TEST_BLOCK_ADDR: u64 = 0x2000_0800;

#[derive(Clone, Debug)]
struct MockCoreState {
    arch: ProcessorArch,
    has_fpu: bool,
    halted: bool,
    interrupts_masked: bool,
    pending_irq_pc: Option<u32>,
    regs: BTreeMap<u16, u32>,
    bp_ctrl_enabled: bool,
    hw_comparators: Vec<Option<u64>>,
    pending_target_pcs: Vec<u32>,
    fail_reg_writes: bool,
}

impl MockCoreState {
    fn new(
        arch: ProcessorArch,
        has_fpu: bool,
        initial_pc: u32,
        halted: bool,
        bp_units: usize,
    ) -> Self {
        let mut regs = BTreeMap::new();
        for r in 0..=14u16 {
            regs.insert(r, 0);
        }
        regs.insert(13, 0x2004_0000);
        regs.insert(15, initial_pc);
        regs.insert(16, 0x0100_0000);
        regs.insert(17, 0x2004_0000);
        regs.insert(18, 0);
        if has_fpu {
            for s in 0..32u16 {
                regs.insert(64 + s, 0);
            }
            regs.insert(33, 0);
        }
        Self {
            arch,
            has_fpu,
            halted,
            interrupts_masked: false,
            pending_irq_pc: None,
            regs,
            bp_ctrl_enabled: false,
            hw_comparators: vec![None; bp_units],
            pending_target_pcs: Vec::new(),
            fail_reg_writes: false,
        }
    }
}

#[derive(Clone, Debug)]
struct SharedMockHardware {
    cores: Vec<MockCoreState>,
    memory: BTreeMap<u64, u8>,
    core1_rearm_count: usize,
    non_halting_mem_ops: usize,
}

struct SharedHwAdapter<'a> {
    state: &'a Arc<Mutex<SharedMockHardware>>,
}

impl ArmHardwareOps for SharedHwAdapter<'_> {
    fn core_arch(&self, core_idx: usize) -> ProcessorArch {
        self.state.lock().unwrap().cores[core_idx].arch
    }

    fn raw_read_memory(
        &mut self,
        _core_idx: usize,
        addr: u64,
        buf: &mut [u8],
    ) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.non_halting_mem_ops += 1;
        for (idx, slot) in buf.iter_mut().enumerate() {
            *slot = guard.memory.get(&(addr + idx as u64)).copied().unwrap_or(0);
        }
        Ok(())
    }

    fn raw_write_memory(&mut self, _core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.non_halting_mem_ops += 1;
        for (idx, &b) in data.iter().enumerate() {
            guard.memory.insert(addr + idx as u64, b);
        }
        Ok(())
    }

    fn raw_read_pc(&mut self, core_idx: usize) -> Result<u32, String> {
        let guard = self.state.lock().unwrap();
        Ok(guard.cores[core_idx].regs.get(&15).copied().unwrap_or(0))
    }

    fn raw_write_pc(&mut self, core_idx: usize, pc: u32) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.regs.insert(15, pc & !1);
        core.halted = true;
        Ok(())
    }

    fn raw_core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        if !core.halted && core.bp_ctrl_enabled {
            if let Some(&next_pc) = core.pending_target_pcs.first() {
                if core.hw_comparators.contains(&Some(next_pc as u64)) {
                    core.pending_target_pcs.remove(0);
                    core.regs.insert(15, next_pc);
                    core.halted = true;
                }
            }
        }
        Ok(core.halted)
    }

    fn raw_step_core(&mut self, core_idx: usize) -> Result<u32, String> {
        let mut guard = self.state.lock().unwrap();
        let cur_pc = guard.cores[core_idx].regs.get(&15).copied().unwrap_or(0) & !1;
        let b0 = guard.memory.get(&(cur_pc as u64)).copied().unwrap_or(0);
        let b1 = guard.memory.get(&(cur_pc as u64 + 1)).copied().unwrap_or(0);
        let b2 = guard.memory.get(&(cur_pc as u64 + 2)).copied().unwrap_or(0);
        let b3 = guard.memory.get(&(cur_pc as u64 + 3)).copied().unwrap_or(0);
        let core = &mut guard.cores[core_idx];
        // Hardware BPU v1/FPB re-traps at cur_pc if comparator at cur_pc was not disarmed!
        if core.bp_ctrl_enabled && core.hw_comparators.contains(&Some(cur_pc as u64)) {
            core.halted = true;
            return Ok(cur_pc);
        }
        // Real ARM Cortex-M (`probe-rs` `armv6m::step` / `armv8m::step`) enters `CoreStatus::Sleeping`
        // (`DHCSR.S_SLEEP = 1, S_HALT = 0`) and times out when stepping `WFE` (`0xBF20` / `0xF3AF_8002`)
        // or `WFI` (`0xBF30` / `0xF3AF_8003`) with `DHCSR.C_MASKINTS = 1` (`WORM-LITHE-ENCHYTRAEID-678`).
        let is_sleep_insn = ([b0, b1] == [0x20, 0xBF])
            || ([b0, b1] == [0x30, 0xBF])
            || ([b0, b1, b2, b3] == [0xAF, 0xF3, 0x02, 0x80])
            || ([b0, b1, b2, b3] == [0xAF, 0xF3, 0x03, 0x80]);
        if is_sleep_insn && core.interrupts_masked {
            core.halted = false;
            return Err("Timeout waiting for core halt on WFE/WFI under C_MASKINTS".to_string());
        }
        // Unmasked pending interrupt vectors into ISR instead of stepping the current instruction!
        let next_pc = if !core.interrupts_masked {
            if let Some(isr_pc) = core.pending_irq_pc.take() {
                core.regs.insert(14, 0xFFFF_FFF9);
                isr_pc
            } else {
                cur_pc.wrapping_add(2)
            }
        } else {
            cur_pc.wrapping_add(2)
        };
        core.regs.insert(15, next_pc);
        core.halted = true;
        Ok(next_pc)
    }

    fn set_interrupt_mask(&mut self, core_idx: usize, masked: bool) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.cores[core_idx].interrupts_masked = masked;
        Ok(())
    }

    fn raw_resume_core(&mut self, core_idx: usize) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.cores[core_idx].halted = false;
        Ok(())
    }

    fn set_hw_breakpoint_slot(
        &mut self,
        core_idx: usize,
        slot_idx: usize,
        addr: u64,
    ) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.bp_ctrl_enabled = true;
        core.hw_comparators[slot_idx] = Some(addr);
        Ok(())
    }

    fn clear_hw_breakpoint_slot(
        &mut self,
        core_idx: usize,
        slot_idx: usize,
        _addr: u64,
    ) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.cores[core_idx].hw_comparators[slot_idx] = None;
        Ok(())
    }

    fn ensure_hw_breakpoints_active(
        &mut self,
        core_idx: usize,
        slots: &[Option<u64>],
    ) -> Result<bool, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        if core.bp_ctrl_enabled && core.hw_comparators == slots {
            return Ok(false);
        }
        core.bp_ctrl_enabled = true;
        for (idx, entry) in slots.iter().enumerate() {
            core.hw_comparators[idx] = *entry;
        }
        if core_idx == 1 {
            guard.core1_rearm_count += 1;
        }
        Ok(true)
    }
}

struct MockDualCoreBackend {
    state: Arc<Mutex<SharedMockHardware>>,
    bp_mgr: ArmBreakpointManager,
}

impl MockDualCoreBackend {
    fn new() -> (Self, Arc<Mutex<SharedMockHardware>>) {
        Self::with_arch(ProcessorArch::Armv6m, false)
    }

    fn with_arch(arch: ProcessorArch, has_fpu: bool) -> (Self, Arc<Mutex<SharedMockHardware>>) {
        let mut memory = BTreeMap::new();
        // Seed Thumb `nop` (`0x46C0`, little-endian `[0xC0, 0x46]`) at Reset_Handler, main, and SRAM slot.
        for base in [
            RESET_HANDLER_ADDR as u64,
            (RESET_HANDLER_ADDR + 2) as u64,
            CORE0_MAIN_LOC_ADDR as u64,
            SRAM_FUNC_ADDR,
        ] {
            memory.insert(base, 0xC0);
            memory.insert(base + 1, 0x46);
        }
        for (idx, &b) in b"SEGGER RTT\0\0\0\0\0\0".iter().enumerate() {
            memory.insert(RTT_TEST_BLOCK_ADDR + idx as u64, b);
        }

        let shared = Arc::new(Mutex::new(SharedMockHardware {
            cores: vec![
                MockCoreState::new(arch, has_fpu, RESET_HANDLER_ADDR, true, 4),
                MockCoreState::new(arch, has_fpu, 0x0000_0100, false, 4),
            ],
            memory,
            core1_rearm_count: 0,
            non_halting_mem_ops: 0,
        }));
        (
            Self {
                state: Arc::clone(&shared),
                bp_mgr: ArmBreakpointManager::new(&[4, 4], None),
            },
            shared,
        )
    }
}

impl DebugBackend for MockDualCoreBackend {
    fn chip_name(&self) -> &str {
        "rp2040"
    }

    fn core_count(&self) -> usize {
        self.state.lock().unwrap().cores.len()
    }

    fn core_register_layout(&mut self, core_idx: usize) -> TargetRegisterLayout {
        let guard = self.state.lock().unwrap();
        let c = &guard.cores[core_idx];
        build_arm_cortex_m_register_layout(c.arch, c.has_fpu)
    }

    fn core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        hw.raw_core_halted(core_idx)
    }

    fn halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.halted = true;
        Ok(*core.regs.get(&15).unwrap_or(&0))
    }

    fn resume_core(&mut self, core_idx: usize) -> Result<(), String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        let _ = self.bp_mgr.prepare_resume(&mut hw, core_idx);
        hw.raw_resume_core(core_idx)
    }

    fn step_core(&mut self, core_idx: usize) -> Result<u32, String> {
        if !self.core_halted(core_idx)? {
            let _ = self.halt_core(core_idx);
        }
        let mut hw = SharedHwAdapter { state: &self.state };
        self.bp_mgr.step_core(&mut hw, core_idx)
    }

    fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
        {
            let mut guard = self.state.lock().unwrap();
            let core = &mut guard.cores[core_idx];
            let entry = if core_idx == 0 {
                RESET_HANDLER_ADDR
            } else {
                CORE1_RESET_HANDLER_ADDR
            };
            core.regs.insert(15, entry);
            core.halted = true;
            core.bp_ctrl_enabled = false;
            for slot in core.hw_comparators.iter_mut() {
                *slot = None;
            }
        }
        let mut hw = SharedHwAdapter { state: &self.state };
        let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
        Ok(if core_idx == 0 {
            RESET_HANDLER_ADDR
        } else {
            CORE1_RESET_HANDLER_ADDR
        })
    }

    fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String> {
        {
            let mut guard = self.state.lock().unwrap();
            let core = &mut guard.cores[core_idx];
            core.regs.insert(15, RESET_HANDLER_ADDR);
            core.halted = false;
            core.bp_ctrl_enabled = false;
            for slot in core.hw_comparators.iter_mut() {
                *slot = None;
            }
        }
        let mut hw = SharedHwAdapter { state: &self.state };
        let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
        Ok(())
    }

    fn read_core_reg(&mut self, core_idx: usize, hw_reg_id: u16) -> Result<u32, String> {
        let guard = self.state.lock().unwrap();
        Ok(guard.cores[core_idx]
            .regs
            .get(&hw_reg_id)
            .copied()
            .unwrap_or(0))
    }

    fn write_core_reg(
        &mut self,
        core_idx: usize,
        hw_reg_id: u16,
        value: u32,
    ) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        if guard.cores[core_idx].fail_reg_writes {
            return Err("Simulated register write failure".to_string());
        }
        guard.cores[core_idx].regs.insert(hw_reg_id, value);
        Ok(())
    }

    fn read_memory(&mut self, core_idx: usize, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        hw.raw_read_memory(core_idx, addr, buf)
    }

    fn read_memory_masked(
        &mut self,
        core_idx: usize,
        addr: u64,
        buf: &mut [u8],
    ) -> Result<(), String> {
        self.read_memory(core_idx, addr, buf)?;
        if let Some(table) = self.bp_mgr.cores.get(core_idx) {
            table.mask_memory_read(addr, buf);
        }
        Ok(())
    }

    fn write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        hw.raw_write_memory(core_idx, addr, data)
    }

    fn insert_breakpoint(
        &mut self,
        core_idx: usize,
        addr: u64,
        allow_sw_ram: bool,
    ) -> Result<bool, String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        self.bp_mgr
            .insert_breakpoint(&mut hw, core_idx, addr, allow_sw_ram)
    }

    fn remove_breakpoint(&mut self, core_idx: usize, addr: u64) -> Result<(), String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        self.bp_mgr.remove_breakpoint(&mut hw, core_idx, addr)
    }

    fn clear_all_breakpoints(&mut self, core_idx: usize) -> Result<(), String> {
        let mut hw = SharedHwAdapter { state: &self.state };
        self.bp_mgr.clear_all(&mut hw, core_idx)
    }

    fn has_active_breakpoint_at(&self, core_idx: usize, addr: u64) -> bool {
        self.bp_mgr.has_breakpoint_at(core_idx, addr)
    }

    fn poll_background_core_maintenance(&mut self, core_idx: usize) -> bool {
        let mut hw = SharedHwAdapter { state: &self.state };
        self.bp_mgr.poll_maintenance(&mut hw, core_idx)
    }
}

struct TestRspClient {
    stream: TcpStream,
    rx_buf: Vec<u8>,
}

impl TestRspClient {
    fn connect(port: u16) -> Self {
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect RSP port");
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("set_read_timeout");
        Self {
            stream,
            rx_buf: Vec::new(),
        }
    }

    fn send_cmd(&mut self, cmd: &str) -> String {
        let pkt = format_rsp_packet(cmd);
        self.stream.write_all(&pkt).expect("write RSP packet");
        self.stream.flush().expect("flush RSP packet");
        self.read_packet_reply()
    }

    fn send_continue(&mut self) {
        let pkt = format_rsp_packet("c");
        self.stream.write_all(&pkt).expect("write continue");
        self.stream.flush().expect("flush continue");
    }

    fn send_ctrl_c(&mut self) -> String {
        self.stream.write_all(&[0x03]).expect("write Ctrl-C");
        self.stream.flush().expect("flush Ctrl-C");
        self.read_packet_reply()
    }

    fn read_packet_reply(&mut self) -> String {
        let mut scratch = [0u8; 2048];
        loop {
            while let Some(frame) = extract_next_rsp_frame(&mut self.rx_buf) {
                if let RspFrame::Packet {
                    payload,
                    checksum_valid,
                } = frame
                {
                    assert!(checksum_valid, "Expected valid RSP checksum from server");
                    return String::from_utf8(payload).expect("UTF-8 RSP reply");
                }
            }
            let n = self.stream.read(&mut scratch).expect("read RSP stream");
            assert!(n > 0, "Unexpected EOF waiting for RSP reply");
            self.rx_buf.extend_from_slice(&scratch[..n]);
        }
    }

    fn read_pc(&mut self) -> u32 {
        let hex = self.send_cmd("p0f");
        let bytes = decode_hex(&hex).expect("decode PC hex");
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

#[test]
fn test_rsp_framing_rinja_target_xml_sram_sw_breakpoints_and_arch_limits() {
    let mut raw_stream = Vec::new();
    raw_stream.extend_from_slice(b"+$qSupported#37$bad#00\x03");
    assert_eq!(extract_next_rsp_frame(&mut raw_stream), Some(RspFrame::Ack));
    assert_eq!(
        extract_next_rsp_frame(&mut raw_stream),
        Some(RspFrame::Packet {
            payload: b"qSupported".to_vec(),
            checksum_valid: true,
        })
    );
    assert_eq!(
        extract_next_rsp_frame(&mut raw_stream),
        Some(RspFrame::Packet {
            payload: b"bad".to_vec(),
            checksum_valid: false,
        })
    );
    assert_eq!(
        extract_next_rsp_frame(&mut raw_stream),
        Some(RspFrame::CtrlC)
    );

    // Verify ProcessorArch breakpoint address limits across ARMv6-M vs ARMv8-M.
    assert!(supports_arm_hw_breakpoint_at(
        ProcessorArch::Armv6m,
        0x1000_1000
    ));
    assert!(!supports_arm_hw_breakpoint_at(
        ProcessorArch::Armv6m,
        0x2000_0200
    ));
    assert!(supports_arm_hw_breakpoint_at(
        ProcessorArch::Armv8m,
        0x2000_0200
    ));

    // Verify Rinja-rendered XML for ARMv8-M with FPU single-sources every register number.
    let v8m_layout = build_arm_cortex_m_register_layout(ProcessorArch::Armv8m, true);
    assert!(v8m_layout.target_xml().contains("org.gnu.gdb.arm.vfp"));
    assert!(v8m_layout
        .target_xml()
        .contains("<reg name=\"fpscr\" bitsize=\"32\" regnum=\"64\" type=\"int\"/>"));
    assert_eq!(
        v8m_layout.lookup_gdb_register(64).map(|r| r.hw_reg_id),
        Some(33)
    );

    let (mut backend, shared) = MockDualCoreBackend::new();
    let layout = backend.core_register_layout(0);
    let mut handler = RspCoreHandler::new(0, layout, Some("/tmp/test_fw.elf".to_string()));

    // 1. Verify chunked target.xml and exec-file reads.
    let xml_reply =
        match handler.handle_packet(b"qXfer:features:read:target.xml:0,fff", &mut backend) {
            RspAction::Reply(s) => s,
            other => panic!("Unexpected action: {:?}", other),
        };
    assert!(xml_reply.starts_with('l'));
    assert!(xml_reply.contains("org.gnu.gdb.arm.m-profile"));
    assert!(xml_reply.contains("regnum=\"25\""));

    assert_eq!(
        handler.handle_packet(b"qXfer:exec-file:read::0,100", &mut backend),
        RspAction::Reply("l/tmp/test_fw.elf".to_string())
    );

    // 2. Allocate all 4 hardware Flash breakpoints; 5th Flash breakpoint must return E0E.
    for idx in 0..4u64 {
        let cmd = format!("Z1,{:x},2", 0x1000_1000 + idx * 4);
        assert_eq!(
            handler.handle_packet(cmd.as_bytes(), &mut backend),
            RspAction::status(RspReplyCode::Ok)
        );
    }
    assert_eq!(
        handler.handle_packet(b"Z1,10001020,2", &mut backend),
        RspAction::status(RspReplyCode::ErrNoBreakpointSlot)
    );

    // 3. Even with all 4 Flash FPB slots full, an SRAM breakpoint (`0x2000_0230`) installs `0xBE00`
    //    while masking `m` reads and stepping cleanly over the restored Thumb instruction!
    let sram_insert = format!("Z0,{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(sram_insert.as_bytes(), &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    {
        let guard = shared.lock().unwrap();
        assert_eq!(guard.memory.get(&SRAM_FUNC_ADDR), Some(&0x00));
        assert_eq!(guard.memory.get(&(SRAM_FUNC_ADDR + 1)), Some(&0xBE));
    }

    let read_sram_cmd = format!("m{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(read_sram_cmd.as_bytes(), &mut backend),
        RspAction::Reply("c046".to_string())
    );

    let set_pc_cmd = format!(
        "P0f={}",
        encode_hex(&((SRAM_FUNC_ADDR as u32 | 1).to_le_bytes()))
    );
    assert_eq!(
        handler.handle_packet(set_pc_cmd.as_bytes(), &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    let step_reply = match handler.handle_packet(b"s", &mut backend) {
        RspAction::Reply(s) => s,
        other => panic!(
            "Expected T05 expedited stop reply after target.xml fetch, got {:?}",
            other
        ),
    };
    assert!(
        step_reply.starts_with("T05")
            && step_reply.contains("0f:32020020;")
            && step_reply.ends_with("thread:01;"),
        "Unexpected expedited T05 reply: {}",
        step_reply
    );
    assert_eq!(
        shared.lock().unwrap().cores[0].regs.get(&15).copied(),
        Some(SRAM_FUNC_ADDR as u32 + 2)
    );

    let sram_remove = format!("z0,{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(sram_remove.as_bytes(), &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    {
        let guard = shared.lock().unwrap();
        assert_eq!(guard.memory.get(&SRAM_FUNC_ADDR), Some(&0xC0));
        assert_eq!(guard.memory.get(&(SRAM_FUNC_ADDR + 1)), Some(&0x46));
    }
}

#[test]
fn test_exhaustive_rsp_handler_packets_fpu_registers_binary_writes_and_live_gdb_cli() {
    let (mut backend, shared) = MockDualCoreBackend::with_arch(ProcessorArch::Armv8m, true);
    let layout = backend.core_register_layout(0);
    let mut handler = RspCoreHandler::new(0, layout.clone(), Some("/workspace/fw.elf".to_string()));

    // 1. Handshake & thread queries
    assert_eq!(
        handler.handle_packet(b"qSupported:multiprocess+", &mut backend),
        RspAction::Reply(
            "PacketSize=4096;qXfer:features:read+;qXfer:memory-map:read+;QStartNoAckMode+;vContSupported+;hwbreak+;swbreak+;qXfer:exec-file:read+"
                .to_string()
        )
    );
    assert_eq!(
        handler.handle_packet(b"QStartNoAckMode", &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert!(handler.no_ack_mode);
    assert_eq!(
        handler.handle_packet(b"!", &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert_eq!(
        handler.handle_packet(b"vMustReplyEmpty", &mut backend),
        RspAction::status(RspReplyCode::Empty)
    );
    assert_eq!(
        handler.handle_packet(b"qOffsets", &mut backend),
        RspAction::status(RspReplyCode::Empty)
    );
    assert_eq!(
        handler.handle_packet(b"qTStatus", &mut backend),
        RspAction::status(RspReplyCode::Empty)
    );
    assert_eq!(
        handler.handle_packet(b"qAttached", &mut backend),
        RspAction::Reply("1".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"qC", &mut backend),
        RspAction::Reply("QC01".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"qfThreadInfo", &mut backend),
        RspAction::Reply("m01".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"qsThreadInfo", &mut backend),
        RspAction::Reply("l".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"qThreadExtraInfo,1", &mut backend),
        RspAction::Reply(encode_hex(b"Core 0"))
    );
    for ok_cmd in [
        b"qSymbol::".as_slice(),
        b"Hg0",
        b"Hc-1",
        b"T01",
        b"qRcmd,68616c74",
    ] {
        assert_eq!(
            handler.handle_packet(ok_cmd, &mut backend),
            RspAction::status(RspReplyCode::Ok)
        );
    }

    // 2. Paginated `target.xml` reading (`m` chunk followed by `l` chunk)
    let first_chunk =
        match handler.handle_packet(b"qXfer:features:read:target.xml:0,20", &mut backend) {
            RspAction::Reply(s) => s,
            other => panic!("unexpected: {:?}", other),
        };
    assert!(first_chunk.starts_with('m'));
    assert_eq!(first_chunk.len(), 1 + 0x20);

    // 3. `g`/`G` bulk register read/write & `p`/`P` single register (including FPU `s0` & `fpscr`)
    let g_initial = match handler.handle_packet(b"g", &mut backend) {
        RspAction::Reply(s) => s,
        other => panic!("unexpected: {:?}", other),
    };
    assert_eq!(g_initial.len(), layout.general_registers().len() * 8);
    assert_eq!(
        handler.handle_packet(format!("G{}", g_initial).as_bytes(), &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );

    // Write FPU `s0` (GDB reg 0x20 = 32) and `fpscr` (GDB reg 0x40 = 64)
    assert_eq!(
        handler.handle_packet(b"P20=0000803f", &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert_eq!(
        handler.handle_packet(b"p20", &mut backend),
        RspAction::Reply("0000803f".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"P40=10000000", &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert_eq!(
        handler.handle_packet(b"p40", &mut backend),
        RspAction::Reply("10000000".to_string())
    );

    // Simulated register write error returns `E02`, invalid reg returns `E01`
    assert_eq!(
        handler.handle_packet(b"pff", &mut backend),
        RspAction::status(RspReplyCode::ErrBadRequest)
    );
    shared.lock().unwrap().cores[0].fail_reg_writes = true;
    assert_eq!(
        handler.handle_packet(b"P00=11223344", &mut backend),
        RspAction::status(RspReplyCode::ErrRegAccess)
    );
    shared.lock().unwrap().cores[0].fail_reg_writes = false;

    // 4. `M` hex memory write & `X` binary write with escaped `$` (`0x24` -> `0x7d 0x04`), `#` (`0x23` -> `0x7d 0x03`), `}` (`0x7d` -> `0x7d 0x5d`)
    assert_eq!(
        handler.handle_packet(b"M20000900,4:deadbeef", &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert_eq!(
        handler.handle_packet(b"m20000900,4", &mut backend),
        RspAction::Reply("deadbeef".to_string())
    );

    let mut x_pkt = b"X20000910,4:".to_vec();
    x_pkt.extend_from_slice(&[0x7d, 0x04, 0x7d, 0x03, 0x7d, 0x5d, 0x42]);
    assert_eq!(
        handler.handle_packet(&x_pkt, &mut backend),
        RspAction::status(RspReplyCode::Ok)
    );
    assert_eq!(
        handler.handle_packet(b"m20000910,4", &mut backend),
        RspAction::Reply("24237d42".to_string())
    );

    // 5. `s<addr>`, `vCont;s`, `vCont;t`, `c<addr>`, and `qRcmd` `reset run`
    assert_eq!(
        handler.handle_packet(b"s10000500", &mut backend),
        RspAction::status(RspReplyCode::SigTrap)
    );
    assert_eq!(
        shared.lock().unwrap().cores[0].regs.get(&15).copied(),
        Some(0x1000_0502)
    );
    assert_eq!(
        handler.handle_packet(b"vCont;s:1", &mut backend),
        RspAction::status(RspReplyCode::SigTrap)
    );
    assert_eq!(
        handler.handle_packet(b"c10000600", &mut backend),
        RspAction::WaitForHalt
    );
    assert_eq!(
        handler.handle_packet(b"vCont;t", &mut backend),
        RspAction::status(RspReplyCode::SigTrap)
    );
    let reset_run_act = handler.handle_packet(
        format!("qRcmd,{}", encode_hex(b"reset run")).as_bytes(),
        &mut backend,
    );
    assert!(matches!(reset_run_act, RspAction::ReplyPackets(_)));

    // Invalid UTF-8 returns `E01`
    assert_eq!(
        handler.handle_packet(&[0xff, 0xfe], &mut backend),
        RspAction::status(RspReplyCode::ErrBadRequest)
    );

    // 6. Live `arm-none-eabi-gdb` subprocess integration test against TCP `host_debug` server
    if Command::new("arm-none-eabi-gdb")
        .arg("--version")
        .output()
        .is_ok()
    {
        let (gdb_backend, gdb_shared) = MockDualCoreBackend::new();
        let shutdown = Arc::new(AtomicBool::new(false));
        let cfg = DebugServerConfig {
            bind_host: "127.0.0.1".to_string(),
            gdb_core0_port: 0,
            gdb_core1_port: Some(0),
            rtt_bridge_port: Some(0),
            halted_on_reset: true,
            exec_file_path: None,
            json_events: false,
            log_file: None,
        };
        let srv = start_debug_server_with_backend(gdb_backend, &cfg, shutdown)
            .expect("start gdb integration server");
        gdb_shared.lock().unwrap().cores[0]
            .pending_target_pcs
            .push(CORE0_MAIN_LOC_ADDR);

        let gdb_out = Command::new("arm-none-eabi-gdb")
            .args([
                "-q",
                "-batch",
                "-ex",
                &format!("target extended-remote 127.0.0.1:{}", srv.gdb_core0_port),
                "-ex",
                "set confirm off",
                "-ex",
                "set {int}0x20000840 = 0x13572468",
                "-ex",
                "x/1wx 0x20000840",
                "-ex",
                &format!("hbreak *0x{:x}", CORE0_MAIN_LOC_ADDR),
                "-ex",
                "continue",
                "-ex",
                "stepi",
                "-ex",
                "monitor reset halt",
                "-ex",
                "maintenance flush register-cache",
                "-ex",
                "info reg pc sp",
                "-ex",
                "detach",
                "-ex",
                "disconnect",
            ])
            .output()
            .expect("invoke arm-none-eabi-gdb");
        srv.stop();

        let stdout = String::from_utf8_lossy(&gdb_out.stdout);
        let stderr = String::from_utf8_lossy(&gdb_out.stderr);
        let combined = format!("{}\n{}", stdout, stderr);
        assert!(
            gdb_out.status.success(),
            "arm-none-eabi-gdb exited with {:?}\nstdout:\n{}\nstderr:\n{}",
            gdb_out.status,
            stdout,
            stderr
        );
        assert!(stdout.contains("0x13572468"), "stdout:\n{}", stdout);
        assert!(
            combined.contains("Target reset and halted at Reset_Handler"),
            "combined:\n{}",
            combined
        );
        assert!(stdout.contains("0x100001c0"), "stdout:\n{}", stdout);
        assert!(
            !combined.contains("deprecated")
                && !stderr.contains("Invalid remote reply")
                && !stderr.contains("Remote replied unexpectedly"),
            "combined:\n{}",
            combined
        );
    }
}

#[test]
fn test_dual_core_tcp_reset_handler_to_loc_core1_psm_rearm_and_rtt_bridge() {
    let (backend, shared) = MockDualCoreBackend::new();
    let shutdown = Arc::new(AtomicBool::new(false));
    let config = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: Some(0),
        rtt_bridge_port: Some(0),
        halted_on_reset: true,
        exec_file_path: None,
        json_events: false,
        log_file: None,
    };

    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let c0_port = handle.gdb_core0_port;
    let c1_port = handle.gdb_core1_port.expect("Core 1 GDB port");
    let rtt_port = handle.rtt_bridge_port.expect("RTT bridge port");

    let mut gdb_c0 = TestRspClient::connect(c0_port);
    let mut gdb_c1 = TestRspClient::connect(c1_port);
    let mut rtt_bridge =
        GdbClient::connect(&format!("127.0.0.1:{}", rtt_port)).expect("connect RTT bridge");

    assert_eq!(gdb_c0.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(gdb_c1.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(gdb_c0.send_cmd("?"), "S05");
    assert_eq!(gdb_c0.read_pc(), RESET_HANDLER_ADDR);

    let mut rtt_hdr = [0u8; 10];
    rtt_bridge
        .read_mem(RTT_TEST_BLOCK_ADDR, &mut rtt_hdr)
        .expect("RTT bridge read");
    assert_eq!(&rtt_hdr, b"SEGGER RTT");
    rtt_bridge
        .write_mem(RTT_TEST_BLOCK_ADDR + 32, &[0xDE, 0xAD, 0xBE, 0xEF])
        .expect("RTT bridge write");

    assert_eq!(
        gdb_c0.send_cmd(&format!("Z1,{:x},2", CORE0_MAIN_LOC_ADDR)),
        "OK"
    );
    assert_eq!(
        gdb_c1.send_cmd(&format!("Z1,{:x},2", CORE1_RESET_HANDLER_ADDR | 1)),
        "OK"
    );

    shared.lock().unwrap().cores[0]
        .pending_target_pcs
        .push(CORE0_MAIN_LOC_ADDR);
    gdb_c0.send_continue();
    assert_eq!(gdb_c0.read_packet_reply(), "S05");
    assert_eq!(gdb_c0.read_pc(), CORE0_MAIN_LOC_ADDR);

    gdb_c1.send_continue();
    {
        let mut guard = shared.lock().unwrap();
        let c1 = &mut guard.cores[1];
        c1.bp_ctrl_enabled = false;
        c1.hw_comparators = vec![None; 4];
        c1.pending_target_pcs.push(CORE1_RESET_HANDLER_ADDR);
    }

    assert_eq!(gdb_c1.read_packet_reply(), "S05");
    assert_eq!(gdb_c1.read_pc(), CORE1_RESET_HANDLER_ADDR);
    assert!(shared.lock().unwrap().core1_rearm_count >= 1);

    assert_eq!(
        gdb_c1.send_cmd(&format!("Z1,{:x},2", CORE1_TASK_LOC_ADDR)),
        "OK"
    );
    shared.lock().unwrap().cores[1]
        .pending_target_pcs
        .push(CORE1_TASK_LOC_ADDR);
    gdb_c1.send_continue();
    assert_eq!(gdb_c1.read_packet_reply(), "S05");
    assert_eq!(gdb_c1.read_pc(), CORE1_TASK_LOC_ADDR);

    gdb_c0.send_continue();
    std::thread::sleep(Duration::from_millis(15));
    assert_eq!(gdb_c0.send_ctrl_c(), "S02");

    assert_eq!(gdb_c0.send_cmd("D"), "OK");
    assert_eq!(gdb_c1.send_cmd("D"), "OK");
    drop(gdb_c0);
    drop(gdb_c1);
    drop(rtt_bridge);

    handle.stop();
    let final_guard = shared.lock().unwrap();
    assert!(!final_guard.cores[0].halted);
    assert!(!final_guard.cores[1].halted);
    assert!(final_guard.non_halting_mem_ops >= 2);
}

#[test]
fn test_worm_glowing_burrower_8_ack_retransmit_running_detach_and_reset_rearm() {
    let (backend, shared) = MockDualCoreBackend::new();
    let shutdown = Arc::new(AtomicBool::new(false));
    let config = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: Some(0),
        rtt_bridge_port: None,
        halted_on_reset: true,
        exec_file_path: None,
        json_events: false,
        log_file: None,
    };

    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let c0_port = handle.gdb_core0_port;

    let mut client1 = TestRspClient::connect(c0_port);
    let qsup = format_rsp_packet("qSupported:multiprocess+;swbreak+;hwbreak+;vContSupported+");
    let mut burst = Vec::new();
    burst.push(b'+');
    burst.extend_from_slice(&qsup);
    burst.extend_from_slice(&qsup);
    client1
        .stream
        .write_all(&burst)
        .expect("write duplicate qSupported");
    client1.stream.flush().expect("flush duplicate qSupported");

    let qsup_reply = client1.read_packet_reply();
    assert!(qsup_reply.starts_with("PacketSize=4096;"), "{}", qsup_reply);

    client1.stream.write_all(b"+").expect("ack qSupported");
    assert_eq!(client1.send_cmd("vCont?"), "vCont;c;C;s;S;t;r");
    client1.stream.write_all(b"+").expect("ack vCont?");
    assert_eq!(
        client1.send_cmd("vMustReplyEmpty"),
        "",
        "vMustReplyEmpty must not receive a shifted vCont reply after qSupported retransmit"
    );
    client1.stream.write_all(b"+").expect("ack vMustReplyEmpty");
    assert_eq!(client1.send_cmd("QStartNoAckMode"), "OK");
    client1.stream.write_all(b"+").expect("final ack");
    assert_eq!(
        client1.send_cmd("!"),
        "OK",
        "Extended-remote mode ('!') must reply OK"
    );

    assert_eq!(client1.send_cmd("Z1,10000300,2"), "OK");
    assert_eq!(
        client1.send_cmd(&format!("Z1,{:x},2", CORE0_MAIN_LOC_ADDR)),
        "OK"
    );
    assert_eq!(client1.send_cmd("z1,10000300,2"), "OK");

    let rcmd_pkt = format_rsp_packet(&format!("qRcmd,{}", encode_hex(b"reset halt")));
    client1
        .stream
        .write_all(&rcmd_pkt)
        .expect("write monitor reset halt");
    client1.stream.flush().expect("flush monitor reset halt");
    let first_rcmd_reply = client1.read_packet_reply();
    assert!(
        first_rcmd_reply.starts_with('O'),
        "Expected O<hex> console packet from monitor reset halt, got: {}",
        first_rcmd_reply
    );
    let console_msg =
        String::from_utf8(decode_hex(&first_rcmd_reply[1..]).expect("decode O<hex>")).unwrap();
    assert!(console_msg.contains("Target reset and halted at Reset_Handler"));
    assert_eq!(client1.read_packet_reply(), "OK");

    shared.lock().unwrap().cores[0]
        .pending_target_pcs
        .push(CORE0_MAIN_LOC_ADDR);
    client1.send_continue();
    assert_eq!(client1.read_packet_reply(), "S05");
    assert_eq!(client1.read_pc(), CORE0_MAIN_LOC_ADDR);

    client1
        .stream
        .write_all(&format_rsp_packet("vCont;c"))
        .expect("write vCont;c");
    client1.stream.flush().expect("flush vCont;c");
    std::thread::sleep(Duration::from_millis(20));

    let mut detach_while_running = vec![b'+'];
    detach_while_running.extend_from_slice(&format_rsp_packet("D"));
    client1
        .stream
        .write_all(&detach_while_running)
        .expect("write +$D#44 while running");
    client1.stream.flush().expect("flush +$D#44 while running");
    assert_eq!(
        client1.read_packet_reply(),
        "OK",
        "Detaching while target is running must reply OK, not S05"
    );
    drop(client1);
    {
        let guard = shared.lock().unwrap();
        assert!(!guard.cores[0].halted);
        assert!(guard.cores[0].hw_comparators.iter().all(Option::is_none));
    }

    let mut client2 = TestRspClient::connect(c0_port);
    assert!(client2
        .send_cmd("qSupported:swbreak+;hwbreak+;vContSupported+")
        .starts_with("PacketSize=4096;"));
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vCont?"), "vCont;c;C;s;S;t;r");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vMustReplyEmpty"), "");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("QStartNoAckMode"), "OK");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vKill;1"), "OK");
    drop(client2);

    handle.stop();
}

#[test]
fn test_worm_bristly_roundworm_431_gdb_zero_warnings_on_exec_file_and_reg_cache_flush() {
    if Command::new("arm-none-eabi-gdb")
        .arg("--version")
        .output()
        .is_err()
    {
        return;
    }

    let tmp_elf = std::env::temp_dir().join(format!("worm_431_test_{}.elf", std::process::id()));
    // Minimal valid 32-bit little-endian ARM ELF header (`ET_EXEC`, `EM_ARM`)
    let mut elf_bytes = vec![0u8; 52];
    elf_bytes[0..4].copy_from_slice(b"\x7fELF");
    elf_bytes[4] = 1; // ELFCLASS32
    elf_bytes[5] = 1; // ELFDATA2LSB
    elf_bytes[6] = 1; // EV_CURRENT
    elf_bytes[16..18].copy_from_slice(&2u16.to_le_bytes()); // ET_EXEC
    elf_bytes[18..20].copy_from_slice(&40u16.to_le_bytes()); // EM_ARM
    elf_bytes[20..24].copy_from_slice(&1u32.to_le_bytes()); // EV_CURRENT
    elf_bytes[24..28].copy_from_slice(&RESET_HANDLER_ADDR.to_le_bytes());
    elf_bytes[40..42].copy_from_slice(&52u16.to_le_bytes()); // e_ehsize
    std::fs::write(&tmp_elf, &elf_bytes).expect("write minimal ARM ELF");

    let (gdb_backend, _gdb_shared) = MockDualCoreBackend::new();
    let shutdown = Arc::new(AtomicBool::new(false));
    let cfg = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: Some(0),
        rtt_bridge_port: None,
        halted_on_reset: true,
        exec_file_path: Some(tmp_elf.to_string_lossy().to_string()),
        json_events: false,
        log_file: None,
    };
    let srv = start_debug_server_with_backend(gdb_backend, &cfg, shutdown)
        .expect("start WORM-431 gdb server");

    let gdb_out = Command::new("arm-none-eabi-gdb")
        .args([
            "-q",
            "-batch",
            "-ex",
            "set sysroot /",
            "-ex",
            &format!("target extended-remote 127.0.0.1:{}", srv.gdb_core0_port),
            "-ex",
            "monitor reset halt",
            "-ex",
            "maintenance flush register-cache",
            "-ex",
            "detach",
            "-ex",
            "disconnect",
        ])
        .output()
        .expect("run arm-none-eabi-gdb for WORM-431");
    srv.stop();
    let _ = std::fs::remove_file(&tmp_elf);

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&gdb_out.stdout),
        String::from_utf8_lossy(&gdb_out.stderr)
    );
    assert!(gdb_out.status.success(), "{}", combined);
    assert!(
        !combined.to_lowercase().contains("warning:"),
        "Expected zero GDB warnings on attach + monitor reset halt + maintenance flush register-cache, got:\n{}",
        combined
    );
}

#[test]
fn test_worm_lubricated_flatworm_512_masked_single_step_past_hw_bp_and_vcont_clause_selection() {
    let (mut backend, shared) = MockDualCoreBackend::new();
    let layout = backend.core_register_layout(0);
    let mut rsp = RspCoreHandler::new(0, layout, None);

    // Arm hardware breakpoint at CORE0_MAIN_LOC_ADDR and stop Core 0 there with a pending timer IRQ
    assert_eq!(
        rsp.handle_packet(
            format!("Z1,{:x},2", CORE0_MAIN_LOC_ADDR).as_bytes(),
            &mut backend
        ),
        RspAction::Reply("OK".to_string())
    );
    {
        let mut guard = shared.lock().unwrap();
        guard.cores[0].regs.insert(15, CORE0_MAIN_LOC_ADDR);
        guard.cores[0].halted = true;
        guard.cores[0].pending_irq_pc = Some(0x1000_0900);
    }

    // Send multi-clause `vCont;c:2;s:1` — thread 1 (this core) MUST single-step with interrupts masked
    // AND temporarily disarm the active FPB slot at CORE0_MAIN_LOC_ADDR so PC advances to +2!
    let step_act = rsp.handle_packet(b"vCont;c:2;s:1", &mut backend);
    assert_eq!(step_act, RspAction::Reply("S05".to_string()));
    {
        let guard = shared.lock().unwrap();
        assert_eq!(
            guard.cores[0].regs.get(&15).copied().unwrap_or(0) & !1,
            CORE0_MAIN_LOC_ADDR + 2,
            "Single-step must advance past armed hardware breakpoint without vectoring into pending ISR"
        );
        assert!(
            !guard.cores[0].interrupts_masked,
            "Interrupt masking (DHCSR.C_MASKINTS) must be cleared after single-step completes"
        );
        assert!(
            guard.cores[0]
                .hw_comparators
                .contains(&Some(CORE0_MAIN_LOC_ADDR as u64)),
            "Hardware breakpoint comparator must be re-armed immediately after stepping past it"
        );
    }

    // `Z0` at Cortex-M EXC_RETURN (`0xFFFF_FFF8`) must return Empty ("") rather than E0E error
    assert_eq!(
        rsp.handle_packet(b"Z0,fffffff8,2", &mut backend),
        RspAction::Reply(String::new())
    );

    // Verify `.vscode/cortex_debug_init.gdb` silences `from ...` REPL injections in `arm-none-eabi-gdb`
    if Command::new("arm-none-eabi-gdb")
        .arg("--version")
        .output()
        .is_ok()
    {
        let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root")
            .to_path_buf();
        let init_gdb = repo_root.join(".vscode").join("cortex_debug_init.gdb");
        assert!(init_gdb.is_file(), "Missing {}", init_gdb.display());
        let gdb_from = Command::new("arm-none-eabi-gdb")
            .current_dir(&repo_root)
            .args([
                "-q",
                "-batch",
                "-ex",
                "set sysroot /",
                "-ex",
                "source .vscode/cortex_debug_init.gdb",
                "-ex",
                "from ocp_vscode import show",
            ])
            .output()
            .expect("run arm-none-eabi-gdb with cortex_debug_init.gdb");
        let combined = format!(
            "{}\n{}",
            String::from_utf8_lossy(&gdb_from.stdout),
            String::from_utf8_lossy(&gdb_from.stderr)
        );
        assert!(gdb_from.status.success(), "{}", combined);
        assert!(
            !combined.contains("Undefined command: \"from\""),
            "Expected no Undefined command 'from' error, got:\n{}",
            combined
        );
    }
}

#[test]
fn test_dual_core_boot_race_core1_already_halted_at_breakpoint_before_continue() {
    let (mut backend, shared) = MockDualCoreBackend::new();
    let layout1 = backend.core_register_layout(1);
    let mut rsp1 = RspCoreHandler::new(1, layout1, None);

    // Core 1 starts halted in Mask Boot ROM (`0x0000_0178`) when Dual-Core GDB attaches (`?`)
    {
        let mut guard = shared.lock().unwrap();
        guard.cores[1].regs.insert(15, 0x0000_0178);
        guard.cores[1].halted = true;
    }
    assert_eq!(
        rsp1.handle_packet(b"?", &mut backend),
        RspAction::Reply("S05".to_string())
    );
    assert_eq!(
        rsp1.handle_packet(
            format!("Z1,{:x},2", CORE1_RESET_HANDLER_ADDR | 1).as_bytes(),
            &mut backend
        ),
        RspAction::Reply("OK".to_string())
    );

    // Simulate Core 0 resuming first, releasing Boot-ROM Core 1, and Core 1 hitting `Core1_Reset_Handler`
    // BEFORE Core 1's own `$c` packet acquires the backend lock (exact race caught under `trace_mode=PERF`)!
    {
        let mut guard = shared.lock().unwrap();
        guard.cores[1]
            .regs
            .insert(15, CORE1_RESET_HANDLER_ADDR & !1);
        guard.cores[1].halted = true;
    }

    // Core 1's `$c` must immediately report `S05` at `Core1_Reset_Handler` rather than calling `resume_core(1)`
    // and skipping past the newly hit breakpoint!
    assert_eq!(
        rsp1.handle_packet(b"c", &mut backend),
        RspAction::Reply("S05".to_string())
    );
    assert!(
        shared.lock().unwrap().cores[1].halted,
        "Core 1 must remain halted at Core1_Reset_Handler"
    );

    // A subsequent `$c` after removing the breakpoint (`z1`) when PC has not changed must resume normally!
    assert_eq!(
        rsp1.handle_packet(
            format!("z1,{:x},2", CORE1_RESET_HANDLER_ADDR | 1).as_bytes(),
            &mut backend
        ),
        RspAction::Reply("OK".to_string())
    );
    assert_eq!(
        rsp1.handle_packet(b"c", &mut backend),
        RspAction::WaitForHalt
    );
}

#[test]
fn test_host_debug_subcommand_probe_status_json() {
    let bin_path = env!("CARGO_BIN_EXE_host_debug");
    let out = Command::new(bin_path)
        .args(["probe-status", "--probe", "ffff:ffff:nonexistent", "--json"])
        .output()
        .expect("invoke host_debug probe-status --json");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let probe_json = String::from_utf8(out.stdout).expect("utf8 stdout");
    assert!(probe_json.contains("\"interface\":\"swd_usb\""));
    assert!(probe_json.contains("\"availability\":\"disconnected\""));
    assert!(probe_json.contains("\"in_use\":false"));
}

#[test]
fn test_worm_lithe_enchytraeid_678_single_step_across_wfe_wfi_and_abrupt_client_disconnect() {
    let (backend, shared) = MockDualCoreBackend::new();
    {
        let mut guard = shared.lock().unwrap();
        // Seed a realistic scheduler loop starting at `0x1002_13F6`:
        // 0x1002_13F6: 0x46C0 (`nop`)
        // 0x1002_13F8: 0xBF20 (`wfe` 16-bit Thumb)
        // 0x1002_13FA: 0xBF30 (`wfi` 16-bit Thumb)
        // 0x1002_13FC: 0xF3AF 0x8002 (`wfe.w` 32-bit Thumb-2)
        // 0x1002_1400: 0xF3AF 0x8003 (`wfi.w` 32-bit Thumb-2)
        // 0x1002_1404..0x1002_1414: `nop`s
        let loop_bytes: &[(u64, &[u8])] = &[
            (0x1002_13F6, &[0xC0, 0x46]),
            (0x1002_13F8, &[0x20, 0xBF]),
            (0x1002_13FA, &[0x30, 0xBF]),
            (0x1002_13FC, &[0xAF, 0xF3, 0x02, 0x80]),
            (0x1002_1400, &[0xAF, 0xF3, 0x03, 0x80]),
            (0x1002_1404, &[0xC0, 0x46]),
            (0x1002_1406, &[0xC0, 0x46]),
            (0x1002_1408, &[0xC0, 0x46]),
            (0x1002_140A, &[0xC0, 0x46]),
            (0x1002_140C, &[0xC0, 0x46]),
            (0x1002_140E, &[0xC0, 0x46]),
            (0x1002_1410, &[0xC0, 0x46]),
        ];
        for &(addr, bytes) in loop_bytes {
            for (off, &b) in bytes.iter().enumerate() {
                guard.memory.insert(addr + off as u64, b);
            }
        }
        guard.cores[0].regs.insert(15, 0x1002_13F6);
        guard.cores[0].halted = true;
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    let config = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: None,
        rtt_bridge_port: None,
        halted_on_reset: true,
        exec_file_path: None,
        json_events: false,
        log_file: None,
    };
    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let mut gdb = TestRspClient::connect(handle.gdb_core0_port);
    assert_eq!(gdb.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(gdb.send_cmd("?"), "S05");

    // Arm a hardware breakpoint right on the `WFE` instruction (`0x1002_13F8`) as well!
    assert_eq!(gdb.send_cmd("Z1,100213f8,2"), "OK");

    let expected_pcs: &[u32] = &[
        0x1002_13F8, // after nop
        0x1002_13FA, // after 16-bit wfe (with active hw bp disarmed & re-armed!)
        0x1002_13FC, // after 16-bit wfi
        0x1002_1400, // after 32-bit wfe.w (+4)
        0x1002_1404, // after 32-bit wfi.w (+4)
        0x1002_1406,
        0x1002_1408,
        0x1002_140A,
        0x1002_140C,
        0x1002_140E,
        0x1002_1410,
        0x1002_1412,
    ];

    for (idx, &exp_pc) in expected_pcs.iter().enumerate() {
        let reply = gdb.send_cmd("vCont;s:1");
        assert_eq!(
            reply,
            "S05",
            "Step {} failed with reply '{}' (expected S05 without WFE/WFI timeout)",
            idx + 1,
            reply
        );
        assert_eq!(
            gdb.read_pc(),
            exp_pc,
            "Unexpected PC after step {}",
            idx + 1
        );
    }

    // Verify `vCont;r<start>,<end>` range-stepping steps cleanly across multiple instructions
    // (including `WFE` / `WFI`) in a single RSP packet and stops immediately upon leaving `[start, end)`.
    assert_eq!(gdb.send_cmd("z1,100213f8,2"), "OK");
    assert_eq!(gdb.send_cmd("P0f=f6130210"), "OK");
    assert_eq!(gdb.send_cmd("vCont;r100213f6,10021406:1;c"), "S05");
    assert_eq!(gdb.read_pc(), 0x1002_1406);

    // Once GDB fetches `target.xml`, `T05` stop replies MUST expedite general registers + `xpsr` (`19:`)
    // so GDB's ARM Thumb unwinder (`arm_pc_is_thumb` + DWARF CFI) never falls back to 40ms `$g` reads
    // on every inlined `vCont;s` instruction (`WORM-LITHE-ENCHYTRAEID-678`).
    let xml = gdb.send_cmd("qXfer:features:read:target.xml:0,1000");
    assert!(xml.starts_with('l'));
    let t05 = gdb.send_cmd("vCont;s:1");
    for required_tag in ["00:", "04:", "07:", "0d:", "0e:", "0f:", "19:"] {
        assert!(
            t05.contains(required_tag),
            "Expedited T05 must include general + xpsr register '{}' to eliminate per-step $g reads; got: {}",
            required_tag,
            t05
        );
    }

    // Verify Thumb-16 dirty-register classification across store, load, ALU, stack, branch, and Thumb-2 instructions.
    use host_debug::thumb16_dirty_core_regs;
    let thread_xpsr = Some(0x0100_0000);
    // `str r1, [r0, #20]` (`0x6141`) -> 0 dirty GPRs, xpsr preserved
    assert_eq!(
        thumb16_dirty_core_regs(0x6141, 0x1000, 0x1002, thread_xpsr),
        Some((0, false))
    );
    // `ldr r1, [sp, #300]` (`0x994B`) -> only r1 (`1 << 1`) dirty, xpsr preserved
    assert_eq!(
        thumb16_dirty_core_regs(0x994B, 0x1000, 0x1002, thread_xpsr),
        Some((1 << 1, false))
    );
    // `adds r2, #4` (`0x3204`) -> r2 (`1 << 2`) + xpsr dirty
    assert_eq!(
        thumb16_dirty_core_regs(0x3204, 0x1000, 0x1002, thread_xpsr),
        Some((1 << 2, true))
    );
    // `push {r4, lr}` (`0xB510`) -> only sp (`1 << 13`) dirty, xpsr preserved
    assert_eq!(
        thumb16_dirty_core_regs(0xB510, 0x1000, 0x1002, thread_xpsr),
        Some((1 << 13, false))
    );
    // `b.n +8` (`0xE004`) -> 0 dirty GPRs, xpsr preserved
    assert_eq!(
        thumb16_dirty_core_regs(0xE004, 0x1000, 0x100C, thread_xpsr),
        Some((0, false))
    );
    // 32-bit Thumb-2 `bl` prefix (`0xF000`) -> None (full register refresh)
    assert_eq!(
        thumb16_dirty_core_regs(0xF000, 0x1000, 0x2000, thread_xpsr),
        None
    );

    // Simulate VS Code killing GDB abruptly (closing TCP socket without sending `$D#44`):
    // `serve_core_gdb_client` must clear Core 0 breakpoints and resume Core 0 automatically!
    drop(gdb);
    let deadline = std::time::Instant::now() + Duration::from_millis(500);
    while std::time::Instant::now() < deadline {
        let guard = shared.lock().unwrap();
        if !guard.cores[0].halted && guard.cores[0].hw_comparators.iter().all(Option::is_none) {
            break;
        }
        drop(guard);
        std::thread::sleep(Duration::from_millis(10));
    }
    {
        let guard = shared.lock().unwrap();
        assert!(
            !guard.cores[0].halted,
            "Core 0 must be resumed automatically on abrupt GDB TCP disconnect"
        );
        assert!(
            guard.cores[0].hw_comparators.iter().all(Option::is_none),
            "Core 0 breakpoints must be cleared automatically on abrupt GDB TCP disconnect"
        );
    }

    handle.stop();
}

#[test]
fn test_frontend_probe_attach_retry_recovers_transient_errors_and_surfaces_exhaustion() {
    use host_debug::{retry_probe_attach, PROBE_ATTACH_MAX_ATTEMPTS};

    let mut calls = 0usize;
    let recovered =
        retry_probe_attach(PROBE_ATTACH_MAX_ATTEMPTS, Duration::from_millis(1), |idx| {
            calls += 1;
            if idx < 2 {
                Err(format!("probe busy on attempt {}", idx + 1))
            } else {
                Ok("attached-session")
            }
        });
    assert_eq!(recovered.as_deref(), Ok("attached-session"));
    assert_eq!(calls, 3);

    let mut fail_calls = 0usize;
    let exhausted: Result<(), String> = retry_probe_attach(3, Duration::from_millis(1), |idx| {
        fail_calls += 1;
        Err(format!("probe claim failed #{}", idx + 1))
    });
    assert_eq!(fail_calls, 3);
    assert_eq!(
        exhausted.as_ref().err().map(String::as_str),
        Some("probe claim failed #3")
    );
}

#[test]
fn test_debug_server_log_file_forwards_ready_client_and_stopped_events() {
    let (backend, _shared) = MockDualCoreBackend::new();
    let shutdown = Arc::new(AtomicBool::new(false));
    let log_path = std::env::temp_dir().join(format!(
        "host_debug_log_test_{}_{}.log",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_file(&log_path);

    let config = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: None,
        rtt_bridge_port: None,
        halted_on_reset: true,
        exec_file_path: None,
        json_events: false,
        log_file: Some(log_path.clone()),
    };

    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let mut gdb = TestRspClient::connect(handle.gdb_core0_port);
    assert_eq!(gdb.send_cmd("?"), "S05");
    assert_eq!(gdb.send_cmd("D"), "OK");
    drop(gdb);
    std::thread::sleep(Duration::from_millis(25));
    handle.stop();

    let logged = std::fs::read_to_string(&log_path).expect("read host_debug log file");
    let _ = std::fs::remove_file(&log_path);
    assert!(logged.contains("\"event\":\"ready\""), "{}", logged);
    assert!(logged.contains("\"event\":\"gdb_connected\""), "{}", logged);
    assert!(
        logged.contains("\"event\":\"gdb_disconnected\""),
        "{}",
        logged
    );
    assert!(logged.contains("\"event\":\"stopped\""), "{}", logged);
}

#[test]
fn test_worm_slithering_snail_823_idle_ctrl_c_memory_map_and_restart_never_desync_rsp() {
    let (backend, shared) = MockDualCoreBackend::new();
    {
        let mut guard = shared.lock().unwrap();
        guard.cores[0].pending_target_pcs.push(0x1000_8ACA);
    }
    let shutdown = Arc::new(AtomicBool::new(false));
    let log_path = std::env::temp_dir().join(format!(
        "host_debug_worm_823_{}_{}.log",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_file(&log_path);

    let config = DebugServerConfig {
        bind_host: "127.0.0.1".to_string(),
        gdb_core0_port: 0,
        gdb_core1_port: Some(0),
        rtt_bridge_port: None,
        halted_on_reset: true,
        exec_file_path: None,
        json_events: false,
        log_file: Some(log_path.clone()),
    };

    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let mut c0 = TestRspClient::connect(handle.gdb_core0_port);
    let mut c1 = TestRspClient::connect(handle.gdb_core1_port.expect("core1 port"));

    // 1. qSupported must advertise qXfer:memory-map:read+ so GDB uses Z1 hwbreak on XIP Flash.
    let caps = c0.send_cmd("qSupported:multiprocess+;swbreak+;hwbreak+");
    assert!(
        caps.contains("qXfer:memory-map:read+"),
        "qSupported must advertise qXfer:memory-map:read+, got: {}",
        caps
    );
    let mem_map = c0.send_cmd("qXfer:memory-map:read::0,400");
    assert!(
        mem_map.starts_with('l') && mem_map.contains("<memory type=\"rom\""),
        "Expected complete <memory-map> XML with ROM region, got: {}",
        mem_map
    );

    assert_eq!(c0.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(c1.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(c0.send_cmd("?"), "S05");

    // 2. Sending redundant Ctrl-C (0x03) while already halted (or pipelined immediately ahead of
    // vCont;c / breakpoint insertion / monitor reset halt) MUST NOT inject an unsolicited S02 packet!
    c0.stream
        .write_all(&[0x03])
        .expect("write redundant Ctrl-C");
    c0.stream.flush().expect("flush redundant Ctrl-C");
    assert_eq!(c0.send_cmd("Z1,10008aca,2"), "OK");
    assert_eq!(c0.send_cmd("Z1,10008e42,2"), "OK");

    let mut vcont_pkt = format_rsp_packet("vCont;c");
    let _ = &mut vcont_pkt;
    c0.stream.write_all(&vcont_pkt).expect("write vCont;c");
    c0.stream.flush().expect("flush vCont;c");
    let stop_reply = c0.read_packet_reply();
    assert!(
        stop_reply.starts_with("S05") || stop_reply.starts_with("T05"),
        "vCont;c must receive breakpoint trap S05/T05 (not stale S02!), got: {}",
        stop_reply
    );

    // Subsequent $g read after redundant Ctrl-C + vCont;c must receive hex registers, NOT a shifted S05!
    let regs_hex = c0.send_cmd("g");
    assert!(
        regs_hex.len() >= 16 * 8 && regs_hex.chars().all(|ch| ch.is_ascii_hexdigit()),
        "Expected valid hex register bank for $g (zero packet shift), got: {}",
        regs_hex
    );

    // 3. Dual-core restart sequence: Core 0 runs `monitor reset halt`, Core 1 runs `monitor reset secondary`
    // without re-clobbering Core 0's PC if Core 0 has already stepped!
    let reset_cmd = format!("qRcmd,{}", encode_hex(b"reset halt"));
    let first_pkt = c0.send_cmd(&reset_cmd);
    let ok_pkt = if first_pkt.starts_with('O') {
        c0.read_packet_reply()
    } else {
        first_pkt
    };
    assert_eq!(ok_pkt, "OK");
    {
        let mut guard = shared.lock().unwrap();
        guard.cores[0].regs.insert(15, CORE0_MAIN_LOC_ADDR);
    }

    let sec_reset_cmd = format!("qRcmd,{}", encode_hex(b"reset halt"));
    let sec_first = c1.send_cmd(&sec_reset_cmd);
    let sec_ok = if sec_first.starts_with('O') {
        c1.read_packet_reply()
    } else {
        sec_first
    };
    assert_eq!(sec_ok, "OK");
    assert_eq!(
        shared.lock().unwrap().cores[0].regs.get(&15).copied(),
        Some(CORE0_MAIN_LOC_ADDR),
        "monitor reset halt on Core 1 while Core 0 GDB is connected must not double-reset Core 0"
    );

    // Re-inserting breakpoints after restart must succeed cleanly.
    assert_eq!(c0.send_cmd("Z1,10008aca,2"), "OK");
    assert_eq!(c0.send_cmd("Z1,10008e42,2"), "OK");

    // Trigger breakpoint slot exhaustion (5th breakpoint on 4-unit ARMv6-M) to verify log_file records it.
    assert_eq!(c0.send_cmd("Z1,10009000,2"), "OK");
    assert_eq!(c0.send_cmd("Z1,10009100,2"), "OK");
    assert_eq!(c0.send_cmd("Z1,10009200,2"), "E0E");

    assert_eq!(c0.send_cmd("D"), "OK");
    assert_eq!(c1.send_cmd("D"), "OK");
    drop(c0);
    drop(c1);
    std::thread::sleep(Duration::from_millis(25));
    handle.stop();

    let logged = std::fs::read_to_string(&log_path).expect("read host_debug log");
    let _ = std::fs::remove_file(&log_path);
    assert!(
        logged.contains("reset halt"),
        "host_debug log_file should record monitor reset halt command, got: {}",
        logged
    );
    assert!(
        logged.contains("0x10009200") || logged.contains("10009200"),
        "host_debug log_file should record failed breakpoint insertion address, got: {}",
        logged
    );
}
