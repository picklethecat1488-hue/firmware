//! Unit and TCP integration tests for `host_debug` GDB RSP and RTT memory bridge.

use host_debug::{
    decode_hex, encode_hex, extract_next_rsp_frame, format_rsp_packet,
    start_debug_server_with_backend, CoreBreakpointTable, DebugBackend, DebugServerConfig,
    RspAction, RspCoreHandler, RspFrame,
};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
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
    halted: bool,
    regs: BTreeMap<u16, u32>,
    bp_ctrl_enabled: bool,
    hw_comparators: Vec<Option<u64>>,
    pending_target_pcs: Vec<u32>,
}

impl MockCoreState {
    fn new(initial_pc: u32, halted: bool, bp_units: usize) -> Self {
        let mut regs = BTreeMap::new();
        for r in 0..=14u16 {
            regs.insert(r, 0);
        }
        regs.insert(13, 0x2004_0000);
        regs.insert(15, initial_pc);
        regs.insert(16, 0x0100_0000);
        regs.insert(17, 0x2004_0000);
        regs.insert(18, 0);
        Self {
            halted,
            regs,
            bp_ctrl_enabled: false,
            hw_comparators: vec![None; bp_units],
            pending_target_pcs: Vec::new(),
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

struct MockDualCoreBackend {
    state: Arc<Mutex<SharedMockHardware>>,
}

impl MockDualCoreBackend {
    fn new() -> (Self, Arc<Mutex<SharedMockHardware>>) {
        let mut memory = BTreeMap::new();
        // Seed Thumb `nop` (`0x46C0`, little-endian `[0xC0, 0x46]`) in SRAM function slot.
        memory.insert(SRAM_FUNC_ADDR, 0xC0);
        memory.insert(SRAM_FUNC_ADDR + 1, 0x46);
        for (idx, &b) in b"SEGGER RTT\0\0\0\0\0\0".iter().enumerate() {
            memory.insert(RTT_TEST_BLOCK_ADDR + idx as u64, b);
        }

        let shared = Arc::new(Mutex::new(SharedMockHardware {
            cores: vec![
                MockCoreState::new(RESET_HANDLER_ADDR, true, 4),
                MockCoreState::new(0x0000_0100, false, 4),
            ],
            memory,
            core1_rearm_count: 0,
            non_halting_mem_ops: 0,
        }));
        (
            Self {
                state: Arc::clone(&shared),
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
        2
    }

    fn core_has_fpu(&mut self, _core_idx: usize) -> bool {
        false
    }

    fn available_hw_breakpoints(&mut self, core_idx: usize) -> usize {
        self.state.lock().unwrap().cores[core_idx]
            .hw_comparators
            .len()
    }

    fn supports_hw_breakpoint_at(&self, _core_idx: usize, addr: u64) -> bool {
        addr < 0x2000_0000
    }

    fn core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
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

    fn halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.halted = true;
        Ok(*core.regs.get(&15).unwrap_or(&0))
    }

    fn resume_core(&mut self, core_idx: usize) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.cores[core_idx].halted = false;
        Ok(())
    }

    fn step_core(&mut self, core_idx: usize) -> Result<u32, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        let next_pc = core.regs.get(&15).copied().unwrap_or(0).wrapping_add(2);
        core.regs.insert(15, next_pc);
        core.halted = true;
        Ok(next_pc)
    }

    fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.regs.insert(15, RESET_HANDLER_ADDR);
        core.halted = true;
        core.bp_ctrl_enabled = false;
        for slot in core.hw_comparators.iter_mut() {
            *slot = None;
        }
        Ok(RESET_HANDLER_ADDR)
    }

    fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        let core = &mut guard.cores[core_idx];
        core.regs.insert(15, RESET_HANDLER_ADDR);
        core.halted = false;
        core.bp_ctrl_enabled = false;
        for slot in core.hw_comparators.iter_mut() {
            *slot = None;
        }
        Ok(())
    }

    fn read_core_reg(&mut self, core_idx: usize, dcrsr: u16) -> Result<u32, String> {
        let guard = self.state.lock().unwrap();
        Ok(guard.cores[core_idx].regs.get(&dcrsr).copied().unwrap_or(0))
    }

    fn write_core_reg(&mut self, core_idx: usize, dcrsr: u16, value: u32) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.cores[core_idx].regs.insert(dcrsr, value);
        Ok(())
    }

    fn read_memory(&mut self, _core_idx: usize, addr: u64, buf: &mut [u8]) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.non_halting_mem_ops += 1;
        for (idx, slot) in buf.iter_mut().enumerate() {
            *slot = guard.memory.get(&(addr + idx as u64)).copied().unwrap_or(0);
        }
        Ok(())
    }

    fn write_memory(&mut self, _core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String> {
        let mut guard = self.state.lock().unwrap();
        guard.non_halting_mem_ops += 1;
        for (idx, &b) in data.iter().enumerate() {
            guard.memory.insert(addr + idx as u64, b);
        }
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
fn test_rsp_framing_target_xml_sram_sw_breakpoints_and_hw_limit() {
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

    let (mut backend, shared) = MockDualCoreBackend::new();
    let mut bp_table = CoreBreakpointTable::new(4);
    let mut handler = RspCoreHandler::new(0, false);

    // 1. Verify chunked target.xml read contains Cortex-M m-profile and m-system features.
    let xml_reply = match handler.handle_packet(
        b"qXfer:features:read:target.xml:0,fff",
        &mut backend,
        &mut bp_table,
    ) {
        RspAction::Reply(s) => s,
        other => panic!("Unexpected action: {:?}", other),
    };
    assert!(xml_reply.starts_with('l'));
    assert!(xml_reply.contains("org.gnu.gdb.arm.m-profile"));
    assert!(xml_reply.contains("regnum=\"25\""));

    // 2. Allocate all 4 hardware Flash breakpoints; 5th Flash breakpoint must return E0E.
    for idx in 0..4u64 {
        let cmd = format!("Z1,{:x},2", 0x1000_1000 + idx * 4);
        assert_eq!(
            handler.handle_packet(cmd.as_bytes(), &mut backend, &mut bp_table),
            RspAction::Reply("OK".to_string())
        );
    }
    assert_eq!(
        handler.handle_packet(b"Z1,10001020,2", &mut backend, &mut bp_table),
        RspAction::Reply("E0E".to_string())
    );

    // 3. Even with all 4 Flash FPB slots full, an SRAM breakpoint (`0x2000_0230`) installs `0xBE00`
    //    while masking `m` reads and stepping cleanly over the restored Thumb instruction!
    let sram_insert = format!("Z0,{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(sram_insert.as_bytes(), &mut backend, &mut bp_table),
        RspAction::Reply("OK".to_string())
    );
    {
        let guard = shared.lock().unwrap();
        assert_eq!(guard.memory.get(&SRAM_FUNC_ADDR), Some(&0x00));
        assert_eq!(guard.memory.get(&(SRAM_FUNC_ADDR + 1)), Some(&0xBE));
    }

    // GDB memory read (`m`) sees original Thumb `nop` (`c046`), not `00be`!
    let read_sram_cmd = format!("m{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(read_sram_cmd.as_bytes(), &mut backend, &mut bp_table),
        RspAction::Reply("c046".to_string())
    );

    // Set PC to SRAM_FUNC_ADDR | 1 and step once: PC advances by 2 and `0xBE00` remains armed.
    let set_pc_cmd = format!(
        "P0f={}",
        encode_hex(&((SRAM_FUNC_ADDR as u32 | 1).to_le_bytes()))
    );
    assert_eq!(
        handler.handle_packet(set_pc_cmd.as_bytes(), &mut backend, &mut bp_table),
        RspAction::Reply("OK".to_string())
    );
    assert_eq!(
        handler.handle_packet(b"s", &mut backend, &mut bp_table),
        RspAction::Reply("S05".to_string())
    );
    assert_eq!(
        shared.lock().unwrap().cores[0].regs.get(&15).copied(),
        Some(SRAM_FUNC_ADDR as u32 + 2)
    );

    // Removing the SRAM breakpoint restores original `[0xC0, 0x46]` in RAM.
    let sram_remove = format!("z0,{:x},2", SRAM_FUNC_ADDR);
    assert_eq!(
        handler.handle_packet(sram_remove.as_bytes(), &mut backend, &mut bp_table),
        RspAction::Reply("OK".to_string())
    );
    {
        let guard = shared.lock().unwrap();
        assert_eq!(guard.memory.get(&SRAM_FUNC_ADDR), Some(&0xC0));
        assert_eq!(guard.memory.get(&(SRAM_FUNC_ADDR + 1)), Some(&0x46));
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
        core1_post_reset_trap_addr: None,
        json_events: false,
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

    // 1. Core 0 starts halted at Reset_Handler (`0x1000_01C0`).
    assert_eq!(gdb_c0.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(gdb_c1.send_cmd("QStartNoAckMode"), "OK");
    assert_eq!(gdb_c0.send_cmd("?"), "S05");
    assert_eq!(gdb_c0.read_pc(), RESET_HANDLER_ADDR);

    // 2. Non-halting RTT memory bridge reads and writes SRAM while Core 0 is halted at Reset_Handler.
    let mut rtt_hdr = [0u8; 10];
    rtt_bridge
        .read_mem(RTT_TEST_BLOCK_ADDR, &mut rtt_hdr)
        .expect("RTT bridge read");
    assert_eq!(&rtt_hdr, b"SEGGER RTT");
    rtt_bridge
        .write_mem(RTT_TEST_BLOCK_ADDR + 32, &[0xDE, 0xAD, 0xBE, 0xEF])
        .expect("RTT bridge write");

    // 3. Set Core 0 breakpoint at a Rust LOC (`CORE0_MAIN_LOC_ADDR`) and Core 1 breakpoint at
    //    `Core1_Reset_Handler` (`CORE1_RESET_HANDLER_ADDR`, passing Thumb bit `| 1`).
    assert_eq!(
        gdb_c0.send_cmd(&format!("Z1,{:x},2", CORE0_MAIN_LOC_ADDR)),
        "OK"
    );
    assert_eq!(
        gdb_c1.send_cmd(&format!("Z1,{:x},2", CORE1_RESET_HANDLER_ADDR | 1)),
        "OK"
    );

    // Queue Core 0 hitting `CORE0_MAIN_LOC_ADDR`.
    shared.lock().unwrap().cores[0]
        .pending_target_pcs
        .push(CORE0_MAIN_LOC_ADDR);
    gdb_c0.send_continue();
    assert_eq!(gdb_c0.read_packet_reply(), "S05");
    assert_eq!(gdb_c0.read_pc(), CORE0_MAIN_LOC_ADDR);

    // 4. Simulate Core 0 calling `spawn_core1()`, which pulses RP2040 `PSM.frce_off.proc1` and
    //    wipes Core 1's hardware `bp_ctrl_enabled` + `hw_comparators` while Core 1 is waiting!
    gdb_c1.send_continue();
    {
        let mut guard = shared.lock().unwrap();
        let c1 = &mut guard.cores[1];
        c1.bp_ctrl_enabled = false;
        c1.hw_comparators = vec![None; 4];
        c1.pending_target_pcs.push(CORE1_RESET_HANDLER_ADDR);
    }

    // `host_debug` detects the wiped BPU on Core 1, re-arms `Core1_Reset_Handler`, and halts Core 1!
    assert_eq!(gdb_c1.read_packet_reply(), "S05");
    assert_eq!(gdb_c1.read_pc(), CORE1_RESET_HANDLER_ADDR);
    assert!(shared.lock().unwrap().core1_rearm_count >= 1);

    // 5. Advance Core 1 from `Core1_Reset_Handler` to an arbitrary Core 1 task LOC (`CORE1_TASK_LOC_ADDR`).
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

    // 6. Verify async Ctrl-C (`0x03`) interrupt halts running Core 0 cleanly with `S02`.
    gdb_c0.send_continue();
    std::thread::sleep(Duration::from_millis(15));
    assert_eq!(gdb_c0.send_ctrl_c(), "S02");

    // 7. Detach both GDB sessions (`D`) and verify breakpoints are cleared and cores resumed.
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
        core1_post_reset_trap_addr: None,
        json_events: false,
    };

    let handle =
        start_debug_server_with_backend(backend, &config, shutdown).expect("start debug server");
    let c0_port = handle.gdb_core0_port;

    // 1. Reproduce pre-`QStartNoAckMode` `$qSupported` timeout retransmission (no `+` ACK between
    //    duplicate packets). Without deduplication, `vMustReplyEmpty` receives `vCont;c;C;s;S;t`!
    let mut client1 = TestRspClient::connect(c0_port);
    let qsup = format_rsp_packet("qSupported:multiprocess+;swbreak+;hwbreak+;vContSupported+");
    let mut burst = Vec::new();
    burst.push(b'+');
    burst.extend_from_slice(&qsup);
    burst.extend_from_slice(&qsup); // Retransmitted before server ACK arrived
    client1
        .stream
        .write_all(&burst)
        .expect("write duplicate qSupported");
    client1.stream.flush().expect("flush duplicate qSupported");

    let qsup_reply = client1.read_packet_reply();
    assert!(qsup_reply.starts_with("PacketSize=4096;"), "{}", qsup_reply);

    client1.stream.write_all(b"+").expect("ack qSupported");
    assert_eq!(client1.send_cmd("vCont?"), "vCont;c;C;s;S;t");
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

    // 2. Verify `monitor reset halt` (`qRcmd`) streams `O<hex>` console output followed by `OK`
    //    and re-arms non-zero hardware breakpoint slots wiped by MCU reset so `main` still hits!
    assert_eq!(client1.send_cmd("Z1,10000300,2"), "OK"); // occupies slot 0
    assert_eq!(
        client1.send_cmd(&format!("Z1,{:x},2", CORE0_MAIN_LOC_ADDR)),
        "OK"
    ); // occupies slot 1
    assert_eq!(client1.send_cmd("z1,10000300,2"), "OK"); // frees slot 0 while slot 1 stays active

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

    // 3. Resume Core 0 (`vCont;c`) so server enters `wait_until_halted_or_interrupted`, then send
    //    a leading `+` ACK followed by `$D#44` (`target-detach` while target is running).
    //    Server must clear breakpoints, leave Core 0 running, reply `"OK"` (never `"S05"`), and close!
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

    // 4. Verify a subsequent GDB session immediately connects cleanly on `:3333` with `vKill;1`.
    let mut client2 = TestRspClient::connect(c0_port);
    assert!(client2
        .send_cmd("qSupported:swbreak+;hwbreak+;vContSupported+")
        .starts_with("PacketSize=4096;"));
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vCont?"), "vCont;c;C;s;S;t");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vMustReplyEmpty"), "");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("QStartNoAckMode"), "OK");
    client2.stream.write_all(b"+").expect("ack");
    assert_eq!(client2.send_cmd("vKill;1"), "OK");
    drop(client2);

    handle.stop();
}
