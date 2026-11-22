//! ARM Cortex-M (`ARMv6-M`, `ARMv7-M`, `ARMv8-M`) register descriptors, hardware FPB + SRAM
//! software `BKPT #0` management, and `probe-rs` SWD debug backend.

use crate::arch::{
    MemoryRegionDescriptor, MemoryRegionKind, ProcessorArch, RegisterDescriptor,
    RegisterFeatureGroup, TargetRegisterLayout,
};
use crate::server::DebugBackend;
use std::collections::BTreeMap;

/// Start address of Cortex-M Code / XIP Flash region (`0x0000_0000`).
pub const CORTEX_M_ROM_START: u64 = 0x0000_0000;

/// Byte length of Cortex-M Code / XIP Flash region (`0x2000_0000`, 512 MiB).
pub const CORTEX_M_ROM_LENGTH: u64 = 0x2000_0000;

/// Start address of Cortex-M SRAM region (`0x2000_0000`).
pub const CORTEX_M_SRAM_START: u64 = 0x2000_0000;

/// Exclusive end address of Cortex-M SRAM region (`0x4000_0000`).
pub const CORTEX_M_SRAM_END: u64 = 0x4000_0000;

/// Byte length of Cortex-M SRAM region (`0x2000_0000`, 512 MiB).
pub const CORTEX_M_SRAM_LENGTH: u64 = CORTEX_M_SRAM_END - CORTEX_M_SRAM_START;

/// Start address of Cortex-M Peripheral / System Control Space region (`0x4000_0000`).
pub const CORTEX_M_PERIPH_START: u64 = 0x4000_0000;

/// Byte length of Cortex-M Peripheral / System Control Space region (`0xC000_0000`).
pub const CORTEX_M_PERIPH_LENGTH: u64 = 0xC000_0000;

/// Little-endian encoding of 16-bit Thumb `BKPT #0` (`0xBE00`).
pub const THUMB_BKPT_0_LE: [u8; 2] = [0x00, 0xBE];

/// Little-endian encoding of 16-bit Thumb `WFE` (`0xBF20`).
pub const THUMB_WFE_16_LE: [u8; 2] = [0x20, 0xBF];

/// Little-endian encoding of 16-bit Thumb `WFI` (`0xBF30`).
pub const THUMB_WFI_16_LE: [u8; 2] = [0x30, 0xBF];

/// Little-endian encoding of 32-bit Thumb-2 `WFE.W` (`0xF3AF_8002`).
pub const THUMB2_WFE_32_LE: [u8; 4] = [0xAF, 0xF3, 0x02, 0x80];

/// Little-endian encoding of 32-bit Thumb-2 `WFI.W` (`0xF3AF_8003`).
pub const THUMB2_WFI_32_LE: [u8; 4] = [0xAF, 0xF3, 0x03, 0x80];

/// Return the architectural byte width (`2` or `4`) if `insn_bytes` starts with an ARM Thumb / Thumb-2
/// `WFE` or `WFI` low-power sleep-hint instruction (`WORM-LITHE-ENCHYTRAEID-678`).
pub fn decode_arm_thumb_sleep_hint_len(insn_bytes: &[u8]) -> Option<u32> {
    if insn_bytes.len() >= 2
        && (insn_bytes[0..2] == THUMB_WFE_16_LE || insn_bytes[0..2] == THUMB_WFI_16_LE)
    {
        return Some(2);
    }
    if insn_bytes.len() >= 4
        && (insn_bytes[0..4] == THUMB2_WFE_32_LE || insn_bytes[0..4] == THUMB2_WFI_32_LE)
    {
        return Some(4);
    }
    None
}

/// Cortex-M Debug Halting Control and Status Register (`DHCSR`) address.
pub const CORTEX_M_DHCSR_ADDR: u64 = 0xE000_EDF0;

/// `DHCSR` write value (`DBGKEY = 0xA05F << 16 | C_DEBUGEN = 1`) enabling halting debug.
pub const CORTEX_M_DHCSR_DBGKEY_ENABLE: u32 = 0xA05F_0001;

/// `DHCSR` write value (`DBGKEY | C_DEBUGEN | C_HALT`) keeping core halted with interrupts unmasked.
pub const CORTEX_M_DHCSR_DBGKEY_HALT: u32 = 0xA05F_0003;

/// `DHCSR` write value (`DBGKEY | C_DEBUGEN | C_HALT | C_MASKINTS`) masking PendSV/SysTick/IRQs during single-step.
pub const CORTEX_M_DHCSR_DBGKEY_HALT_MASKINTS: u32 = 0xA05F_000B;

/// `DHCSR` write value (`DBGKEY | C_DEBUGEN | C_STEP | C_MASKINTS`) executing one masked instruction step.
pub const CORTEX_M_DHCSR_DBGKEY_STEP_MASKINTS: u32 = 0xA05F_000D;

/// `DHCSR.S_HALT` status bit (`1 << 17`) asserted when the core is in Debug Halt state.
pub const CORTEX_M_DHCSR_S_HALT_MASK: u32 = 0x0002_0000;

/// Cortex-M Debug Core Register Selector Register (`DCRSR`) address.
pub const CORTEX_M_DCRSR_ADDR: u64 = 0xE000_EDF4;

/// Cortex-M Debug Core Register Data Register (`DCRDR`) address.
pub const CORTEX_M_DCRDR_ADDR: u64 = 0xE000_EDF8;

/// Cortex-M Debug Exception and Monitor Control Register (`DEMCR`) address.
pub const CORTEX_M_DEMCR_ADDR: u64 = 0xE000_EDFC;

/// `DEMCR.VC_CORERESET` bitmask (`1 << 0`) catching reset vector fetches.
pub const CORTEX_M_DEMCR_VC_CORERESET_MASK: u32 = 0x0000_0001;

/// RP2040 1 MHz hardware `TIMER.DBGPAUSE` register address (`0x4005_4000 + 0x2C`).
pub const RP2040_TIMER_DBGPAUSE_ADDR: u64 = 0x4005_402C;

/// `TIMER.DBGPAUSE` value (`DBG0 = 1 << 1, DBG1 = 0`) pausing `embassy_time` when Core 0 halts without deadlocking Core 0 `.await` timers when Core 1 is halted.
pub const RP2040_TIMER_DBGPAUSE_CORE0_ONLY: u32 = 0x0000_0002;

/// Bounded duration (`8 ms`) to advance the hardware timer when single-stepping across a `WFE`/`WFI`
/// sleep hint on RP2040 so pending `embassy_time` alarms mature and wake tasks on the next poll.
pub const WFE_STEP_TIMER_ADVANCE_MS: u64 = 8;

/// Base XIP flash address on RP2040 (`0x1000_0000`) distinguishing Mask Boot ROM (`< 0x1000_0000`) from application code.
pub const RP2040_FLASH_BASE_ADDR: u32 = 0x1000_0000;

/// Cortex-M Flash Patch / Breakpoint Control Register (`FP_CTRL` / `BP_CTRL`) address.
pub const CORTEX_M_BP_CTRL_ADDR: u64 = 0xE000_2000;

/// `FP_CTRL` / `BP_CTRL` write value (`KEY = 1 << 1 | ENABLE = 1`) enabling hardware comparators.
pub const CORTEX_M_BP_CTRL_KEY_ENABLE: u32 = 0x0000_0003;

/// Base address of Cortex-M Breakpoint Comparator Register 0 (`FP_COMP0` / `BP_COMP0`).
pub const CORTEX_M_BP_COMP0_ADDR: u64 = 0xE000_2008;

/// Exclusive upper address bound for ARMv6-M BPU v1 hardware comparators (`0x2000_0000`).
pub const ARMV6M_HW_BP_LIMIT: u64 = 0x2000_0000;

/// ARM DCRSR register number for Frame Pointer (`r7`).
pub const DCRSR_R7: u16 = 7;

/// ARM DCRSR register number for Stack Pointer (`sp` / `r13`).
pub const DCRSR_SP: u16 = 13;

/// ARM DCRSR register number for Link Register (`lr` / `r14`).
pub const DCRSR_LR: u16 = 14;

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

/// Build the canonical ARM Cortex-M GDB `memory-map` region descriptors marking Code/XIP (`< 0x2000_0000`)
/// as `rom` (forcing hardware FPB breakpoints `Z1`) and SRAM/MMIO (`>= 0x2000_0000`) as `ram`.
pub fn build_arm_cortex_m_memory_regions() -> Vec<MemoryRegionDescriptor> {
    vec![
        MemoryRegionDescriptor {
            kind: MemoryRegionKind::Rom,
            start: CORTEX_M_ROM_START,
            length: CORTEX_M_ROM_LENGTH,
        },
        MemoryRegionDescriptor {
            kind: MemoryRegionKind::Ram,
            start: CORTEX_M_SRAM_START,
            length: CORTEX_M_SRAM_LENGTH,
        },
        MemoryRegionDescriptor {
            kind: MemoryRegionKind::Ram,
            start: CORTEX_M_PERIPH_START,
            length: CORTEX_M_PERIPH_LENGTH,
        },
    ]
}

/// Build a single-sourced ARM Cortex-M [`TargetRegisterLayout`] (`target.xml` + `memory-map` XML) rendered dynamically via Rinja.
pub fn build_arm_cortex_m_register_layout(
    arch: ProcessorArch,
    has_fpu: bool,
) -> TargetRegisterLayout {
    let mut core_regs = Vec::with_capacity(17);
    for idx in 0..=12u32 {
        core_regs.push(RegisterDescriptor {
            name: format!("r{}", idx),
            bitsize: 32,
            gdb_regnum: idx,
            gdb_alias_regnum: None,
            hw_reg_id: idx as u16,
            reg_type: "uint32",
            reg_group: "general",
            is_pc: false,
        });
    }
    core_regs.push(RegisterDescriptor {
        name: "sp".to_string(),
        bitsize: 32,
        gdb_regnum: 13,
        gdb_alias_regnum: None,
        hw_reg_id: 13,
        reg_type: "data_ptr",
        reg_group: "general",
        is_pc: false,
    });
    core_regs.push(RegisterDescriptor {
        name: "lr".to_string(),
        bitsize: 32,
        gdb_regnum: 14,
        gdb_alias_regnum: None,
        hw_reg_id: 14,
        reg_type: "int",
        reg_group: "general",
        is_pc: false,
    });
    core_regs.push(RegisterDescriptor {
        name: "pc".to_string(),
        bitsize: 32,
        gdb_regnum: 15,
        gdb_alias_regnum: None,
        hw_reg_id: DCRSR_PC,
        reg_type: "code_ptr",
        reg_group: "general",
        is_pc: true,
    });
    core_regs.push(RegisterDescriptor {
        name: "xpsr".to_string(),
        bitsize: 32,
        gdb_regnum: GDB_REGNUM_XPSR,
        gdb_alias_regnum: Some(16),
        hw_reg_id: DCRSR_XPSR,
        reg_type: "uint32",
        reg_group: "general",
        is_pc: false,
    });

    let sys_regs = vec![
        RegisterDescriptor {
            name: "msp".to_string(),
            bitsize: 32,
            gdb_regnum: GDB_REGNUM_MSP,
            gdb_alias_regnum: Some(17),
            hw_reg_id: DCRSR_MSP,
            reg_type: "data_ptr",
            reg_group: "system",
            is_pc: false,
        },
        RegisterDescriptor {
            name: "psp".to_string(),
            bitsize: 32,
            gdb_regnum: GDB_REGNUM_PSP,
            gdb_alias_regnum: Some(18),
            hw_reg_id: DCRSR_PSP,
            reg_type: "data_ptr",
            reg_group: "system",
            is_pc: false,
        },
    ];

    let mut features = vec![
        RegisterFeatureGroup {
            name: "org.gnu.gdb.arm.m-profile",
            registers: core_regs,
        },
        RegisterFeatureGroup {
            name: "org.gnu.gdb.arm.m-system",
            registers: sys_regs,
        },
    ];

    if has_fpu {
        let mut fpu_regs = Vec::with_capacity(33);
        for idx in 0..32u32 {
            fpu_regs.push(RegisterDescriptor {
                name: format!("s{}", idx),
                bitsize: 32,
                gdb_regnum: GDB_REGNUM_S0_BASE + idx,
                gdb_alias_regnum: None,
                hw_reg_id: DCRSR_S0_BASE + idx as u16,
                reg_type: "ieee_single",
                reg_group: "float",
                is_pc: false,
            });
        }
        fpu_regs.push(RegisterDescriptor {
            name: "fpscr".to_string(),
            bitsize: 32,
            gdb_regnum: GDB_REGNUM_FPSCR,
            gdb_alias_regnum: None,
            hw_reg_id: DCRSR_FPSCR,
            reg_type: "int",
            reg_group: "float",
            is_pc: false,
        });
        features.push(RegisterFeatureGroup {
            name: "org.gnu.gdb.arm.vfp",
            registers: fpu_regs,
        });
    }

    TargetRegisterLayout::new(arch, features, build_arm_cortex_m_memory_regions())
}

/// Compute `(dirty_gpr_mask, dirty_xpsr)` for a 16-bit Thumb instruction (`hw < 0xE800`) stepped
/// with `DHCSR.C_MASKINTS = 1` from `pc` to `next_pc`.
///
/// Bit `r` (`0..=14`) in `dirty_gpr_mask` is `1` when core register `r` (`r0..r12, sp, lr`) may
/// have been modified by `hw`. Returns `None` for 32-bit Thumb-2 prefixes (`>= 0xE800`), system/trap
/// instructions, `EXC_RETURN`, or unexpected control-flow jumps so the caller refreshes all registers.
pub fn thumb16_dirty_core_regs(
    hw: u16,
    pc: u32,
    next_pc: u32,
    prev_xpsr: Option<u32>,
) -> Option<(u32, bool)> {
    const THUMB32_PREFIX_MIN: u16 = 0xE800;
    const XPSR_IPSR_MASK: u32 = 0x0000_01FF;
    const XPSR_IT_MASK: u32 = 0x0600_FC00;
    const NON_CODE_PC_MIN: u32 = 0xF000_0000;

    if hw >= THUMB32_PREFIX_MIN || next_pc >= NON_CODE_PC_MIN {
        return None;
    }
    let in_it_block = prev_xpsr.map(|x| (x & XPSR_IT_MASK) != 0).unwrap_or(true);
    let in_handler = prev_xpsr.map(|x| (x & XPSR_IPSR_MASK) != 0).unwrap_or(true);
    let seq_next = next_pc == pc.wrapping_add(2);

    match hw >> 12 {
        0x0 | 0x1 => {
            if !seq_next {
                return None;
            }
            Some((1u32 << (hw & 0x7), true))
        }
        0x2 | 0x3 => {
            if !seq_next {
                return None;
            }
            let is_cmp_imm = (hw & 0xF800) == 0x2800;
            let mask = if is_cmp_imm {
                0
            } else {
                1u32 << ((hw >> 8) & 0x7)
            };
            Some((mask, true))
        }
        0x4 => {
            if (hw & 0x0800) != 0 {
                // `LDR Rt, [PC, #imm8]`
                return seq_next.then_some((1u32 << ((hw >> 8) & 0x7), in_it_block));
            }
            if (hw & 0x0C00) == 0 {
                // 16-bit ALU data-processing (`Rdn = hw & 0x7`)
                return seq_next.then_some((1u32 << (hw & 0x7), true));
            }
            match (hw >> 8) & 0x3 {
                0x0 => {
                    // `ADD Rdn, Rm` (high registers)
                    let rdn = ((hw & 0x0080) >> 4) | (hw & 0x7);
                    if rdn == DCRSR_PC {
                        None
                    } else {
                        seq_next.then_some((1u32 << rdn, in_it_block))
                    }
                }
                0x1 => {
                    // `CMP Rn, Rm` (high registers)
                    seq_next.then_some((0, true))
                }
                0x2 => {
                    // `MOV Rd, Rm` (high registers)
                    let rd = ((hw & 0x0080) >> 4) | (hw & 0x7);
                    if rd == DCRSR_PC {
                        (!in_handler).then_some((0, in_it_block))
                    } else {
                        seq_next.then_some((1u32 << rd, in_it_block))
                    }
                }
                _ => {
                    // `BX Rm` / `BLX Rm`
                    if in_handler {
                        None
                    } else {
                        let mask = if (hw & 0x0080) != 0 {
                            1u32 << DCRSR_LR
                        } else {
                            0
                        };
                        Some((mask, in_it_block))
                    }
                }
            }
        }
        0x5 => {
            if !seq_next {
                return None;
            }
            let is_store = (hw & 0x0E00) <= 0x0400;
            let mask = if is_store { 0 } else { 1u32 << (hw & 0x7) };
            Some((mask, in_it_block))
        }
        0x6..=0x8 => {
            if !seq_next {
                return None;
            }
            let is_store = (hw & 0x0800) == 0;
            let mask = if is_store { 0 } else { 1u32 << (hw & 0x7) };
            Some((mask, in_it_block))
        }
        0x9 => {
            if !seq_next {
                return None;
            }
            let is_store = (hw & 0x0800) == 0;
            let mask = if is_store {
                0
            } else {
                1u32 << ((hw >> 8) & 0x7)
            };
            Some((mask, in_it_block))
        }
        0xA => seq_next.then_some((1u32 << ((hw >> 8) & 0x7), in_it_block)),
        0xB => {
            if (hw & 0xFF00) == 0xB000 || (hw & 0xFE00) == 0xB400 {
                // `ADD/SUB SP, #imm` or `PUSH {reglist, lr?}`
                seq_next.then_some((1u32 << DCRSR_SP, in_it_block))
            } else if (hw & 0xFE00) == 0xBC00 {
                // `POP {reglist, pc?}`
                let writes_pc = (hw & 0x0100) != 0;
                if writes_pc && in_handler {
                    None
                } else if !writes_pc && !seq_next {
                    None
                } else {
                    Some((((hw as u32) & 0x00FF) | (1u32 << DCRSR_SP), in_it_block))
                }
            } else if (hw & 0xF500) == 0xB100 {
                // `CBZ` / `CBNZ`
                Some((0, in_it_block))
            } else if (hw & 0xFF00) == 0xB200 || (hw & 0xFF00) == 0xBA00 {
                // `SXTH`/`SXTB`/`UXTH`/`UXTB` or `REV`/`REV16`/`REVSH`
                seq_next.then_some((1u32 << (hw & 0x7), in_it_block))
            } else {
                None
            }
        }
        0xC => {
            if !seq_next {
                return None;
            }
            let rn_bit = 1u32 << ((hw >> 8) & 0x7);
            if (hw & 0x0800) == 0 {
                // `STMIA Rn!, {reglist}`
                Some((rn_bit, in_it_block))
            } else {
                // `LDMIA Rn!, {reglist}`
                Some((((hw as u32) & 0x00FF) | rn_bit, in_it_block))
            }
        }
        0xD => {
            if (hw & 0x0E00) == 0x0E00 {
                // `UDF` (`0xDExx`) or `SVC` (`0xDFxx`)
                None
            } else {
                // `B<cond> label`
                Some((0, in_it_block))
            }
        }
        0xE => {
            // Unconditional 16-bit `B label` (`0xE000..0xE7FF`)
            Some((0, in_it_block))
        }
        _ => None,
    }
}

/// Return `true` if the ARM hardware FPB/BPU for `arch` can match instruction fetches at `addr`.
pub const fn supports_arm_hw_breakpoint_at(arch: ProcessorArch, addr: u64) -> bool {
    match arch {
        ProcessorArch::Armv6m => addr < ARMV6M_HW_BP_LIMIT,
        ProcessorArch::Armv7m | ProcessorArch::Armv8m => addr <= u32::MAX as u64,
    }
}

/// Low-level ARM hardware FPB and memory primitive operations used by [`ArmBreakpointManager`].
pub trait ArmHardwareOps {
    /// Processor architecture variant (`Armv6m`, `Armv7m`, `Armv8m`) for `core_idx`.
    fn core_arch(&self, core_idx: usize) -> ProcessorArch;

    /// Read `buf.len()` raw target bytes at `addr`.
    fn raw_read_memory(&mut self, core_idx: usize, addr: u64, buf: &mut [u8])
        -> Result<(), String>;

    /// Write `data` raw bytes to target memory at `addr`.
    fn raw_write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String>;

    /// Read raw 32-bit `PC` (`DCRSR_PC`) on `core_idx`.
    fn raw_read_pc(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Write raw 32-bit `PC` (`DCRSR_PC`) on halted `core_idx`.
    fn raw_write_pc(&mut self, core_idx: usize, pc: u32) -> Result<(), String>;

    /// Return `true` if `core_idx` is currently halted.
    fn raw_core_halted(&mut self, core_idx: usize) -> Result<bool, String>;

    /// Single-step one hardware instruction on `core_idx` and return the resulting `PC`.
    fn raw_step_core(&mut self, core_idx: usize) -> Result<u32, String>;

    /// Set or clear `DHCSR.C_MASKINTS` on halted `core_idx` so pending interrupts do not preempt single-stepping.
    fn set_interrupt_mask(&mut self, core_idx: usize, masked: bool) -> Result<(), String>;

    /// Resume execution on `core_idx`.
    fn raw_resume_core(&mut self, core_idx: usize) -> Result<(), String>;

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

    /// Verify live `BP_CTRL` / `BP_COMP` registers on `core_idx` and re-arm any cleared slots.
    fn ensure_hw_breakpoints_active(
        &mut self,
        core_idx: usize,
        slots: &[Option<u64>],
    ) -> Result<bool, String>;

    /// Release secondary core from Mask Boot ROM (`PC < 0x1000_0000`) if halted after `PSM.frce_off`.
    fn release_secondary_bootrom_if_halted(&mut self) {}
}

/// Per-core ARM Cortex-M hardware FPB comparator and SRAM `BKPT #0` software breakpoint table.
#[derive(Clone, Debug)]
pub struct ArmCoreBreakpoints {
    /// Shadow table of hardware FPB comparator addresses (`None` when comparator unit is free).
    pub hw_slots: Vec<Option<u64>>,
    /// Original 2-byte Thumb instruction words replaced by `BKPT #0` (`0xBE00`) in Cortex-M SRAM.
    pub sw_ram_patches: BTreeMap<u64, [u8; 2]>,
}

impl ArmCoreBreakpoints {
    /// Construct a new ARM per-core breakpoint table with `hw_unit_count` hardware comparator slots.
    pub fn new(hw_unit_count: usize) -> Self {
        Self {
            hw_slots: vec![None; hw_unit_count],
            sw_ram_patches: BTreeMap::new(),
        }
    }

    /// Return `true` if at least one hardware FPB comparator slot is currently allocated.
    pub fn has_active_hw_breakpoints(&self) -> bool {
        self.hw_slots.iter().any(Option::is_some)
    }

    /// Mask active SRAM `BKPT #0` software patches in `buf` read from `[addr, addr + buf.len())`.
    pub fn mask_memory_read(&self, addr: u64, buf: &mut [u8]) {
        let end_addr = addr + buf.len() as u64;
        for (&bp_addr, orig) in &self.sw_ram_patches {
            for (byte_idx, &orig_byte) in orig.iter().enumerate() {
                let target_byte_addr = bp_addr + byte_idx as u64;
                if target_byte_addr >= addr && target_byte_addr < end_addr {
                    buf[(target_byte_addr - addr) as usize] = orig_byte;
                }
            }
        }
    }
}

/// Multi-core ARM breakpoint and RP2040 `PSM.frce_off` post-reset FPB re-arming manager.
#[derive(Clone, Debug)]
pub struct ArmBreakpointManager {
    /// Per-core breakpoint tables indexed by `core_idx`.
    pub cores: Vec<ArmCoreBreakpoints>,
    /// Optional Core 0 post-`PSM.frce_off.proc1` instruction address used to re-arm Core 1 FPB before FIFO jump.
    pub core1_post_reset_trap_addr: Option<u64>,
    /// Currently armed Core 0 comparator slot holding `core1_post_reset_trap_addr`, if active.
    pub armed_core0_trap_slot: Option<usize>,
}

impl ArmBreakpointManager {
    /// Create a multi-core ARM breakpoint manager from per-core comparator counts.
    pub fn new(hw_units_per_core: &[usize], core1_post_reset_trap_addr: Option<u64>) -> Self {
        Self {
            cores: hw_units_per_core
                .iter()
                .map(|&units| ArmCoreBreakpoints::new(units))
                .collect(),
            core1_post_reset_trap_addr: core1_post_reset_trap_addr.map(|a| a & !1),
            armed_core0_trap_slot: None,
        }
    }

    /// Insert an ARM hardware FPB or Cortex-M SRAM `BKPT #0` breakpoint on `core_idx`.
    pub fn insert_breakpoint<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
        raw_addr: u64,
        allow_sw_ram: bool,
    ) -> Result<bool, String> {
        let addr = raw_addr & !1;
        let arch = hw.core_arch(core_idx);
        let Some(table) = self.cores.get_mut(core_idx) else {
            return Ok(false);
        };
        if table.hw_slots.contains(&Some(addr)) || table.sw_ram_patches.contains_key(&addr) {
            return Ok(true);
        }

        if supports_arm_hw_breakpoint_at(arch, addr) {
            // If Core 0's temporary post-reset trap occupies the only free slot, reclaim it for user breakpoints.
            if core_idx == 0 && table.hw_slots.iter().all(Option::is_some) {
                if let (Some(trap_slot), Some(trap_addr)) = (
                    self.armed_core0_trap_slot.take(),
                    self.core1_post_reset_trap_addr,
                ) {
                    let _ = hw.clear_hw_breakpoint_slot(0, trap_slot, trap_addr);
                }
            }
            let reserved_trap = if core_idx == 0 {
                self.armed_core0_trap_slot
            } else {
                None
            };
            let free_slot = table
                .hw_slots
                .iter()
                .enumerate()
                .position(|(idx, entry)| entry.is_none() && Some(idx) != reserved_trap)
                .or_else(|| table.hw_slots.iter().position(Option::is_none));

            if let Some(slot_idx) = free_slot {
                if core_idx == 0 && self.armed_core0_trap_slot == Some(slot_idx) {
                    self.armed_core0_trap_slot = None;
                }
                if core_idx > 0 {
                    // Secondary core may still be held in hardware reset until Core 0 boots it;
                    // record the comparator slot in shadow state and arm Core 0's post-reset trap
                    // so the hardware comparator is programmed once Core 1 comes out of reset.
                    let _ = hw.set_hw_breakpoint_slot(core_idx, slot_idx, addr);
                } else {
                    hw.set_hw_breakpoint_slot(core_idx, slot_idx, addr)?;
                }
                self.cores[core_idx].hw_slots[slot_idx] = Some(addr);
                self.sync_core1_trap(hw);
                return Ok(true);
            }
        }

        if allow_sw_ram && (CORTEX_M_SRAM_START..CORTEX_M_SRAM_END).contains(&addr) {
            let mut orig = [0u8; 2];
            hw.raw_read_memory(core_idx, addr, &mut orig)?;
            hw.raw_write_memory(core_idx, addr, &THUMB_BKPT_0_LE)?;
            self.cores[core_idx].sw_ram_patches.insert(addr, orig);
            return Ok(true);
        }

        Ok(false)
    }

    /// Remove an ARM hardware FPB or SRAM `BKPT #0` breakpoint on `core_idx`.
    pub fn remove_breakpoint<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
        raw_addr: u64,
    ) -> Result<(), String> {
        let addr = raw_addr & !1;
        let Some(table) = self.cores.get_mut(core_idx) else {
            return Ok(());
        };
        let mut slots_to_clear = Vec::new();
        for (slot_idx, entry) in table.hw_slots.iter_mut().enumerate() {
            if *entry == Some(addr) {
                *entry = None;
                slots_to_clear.push(slot_idx);
            }
        }
        let patch = table.sw_ram_patches.remove(&addr);
        for slot_idx in slots_to_clear {
            let _ = hw.clear_hw_breakpoint_slot(core_idx, slot_idx, addr);
        }
        if let Some(orig) = patch {
            hw.raw_write_memory(core_idx, addr, &orig)?;
        }
        self.sync_core1_trap(hw);
        Ok(())
    }

    /// Clear all hardware comparators and restore all SRAM `BKPT #0` patches on `core_idx`.
    pub fn clear_all<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
    ) -> Result<(), String> {
        let Some(table) = self.cores.get_mut(core_idx) else {
            return Ok(());
        };
        let mut slots = Vec::new();
        for (slot_idx, entry) in table.hw_slots.iter_mut().enumerate() {
            if let Some(addr) = entry.take() {
                slots.push((slot_idx, addr));
            }
        }
        let patches: Vec<(u64, [u8; 2])> = std::mem::take(&mut table.sw_ram_patches)
            .into_iter()
            .collect();
        for (slot_idx, addr) in slots {
            let _ = hw.clear_hw_breakpoint_slot(core_idx, slot_idx, addr);
        }
        for (addr, orig) in patches {
            let _ = hw.raw_write_memory(core_idx, addr, &orig);
        }
        self.sync_core1_trap(hw);
        Ok(())
    }

    /// Return `true` if `core_idx` has a registered hardware or SRAM software breakpoint at `raw_addr`.
    pub fn has_breakpoint_at(&self, core_idx: usize, raw_addr: u64) -> bool {
        let addr = raw_addr & !1;
        let Some(table) = self.cores.get(core_idx) else {
            return false;
        };
        table.hw_slots.contains(&Some(addr)) || table.sw_ram_patches.contains_key(&addr)
    }

    /// Re-arm hardware FPB comparators on `core_idx` if cleared by an MCU or subsystem reset.
    pub fn ensure_armed<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
    ) -> Result<bool, String> {
        if core_idx == 0 {
            self.armed_core0_trap_slot = None;
        }
        let slots = match self.cores.get(core_idx) {
            Some(t) if t.has_active_hw_breakpoints() => t.hw_slots.clone(),
            _ => {
                self.sync_core1_trap(hw);
                return Ok(false);
            }
        };
        let rearmed = hw.ensure_hw_breakpoints_active(core_idx, &slots)?;
        self.sync_core1_trap(hw);
        Ok(rearmed)
    }

    /// Return `true` if `core_idx` has any active user hardware or SRAM software breakpoint.
    pub fn has_any_user_breakpoints(&self, core_idx: usize) -> bool {
        self.cores
            .get(core_idx)
            .map(|t| t.has_active_hw_breakpoints() || !t.sw_ram_patches.is_empty())
            .unwrap_or(false)
    }

    /// Step over an active SRAM `BKPT #0` patch at `PC` on `core_idx`, if `PC` matches one.
    pub fn step_over_sw_bkpt_if_needed<H: ArmHardwareOps + ?Sized>(
        &self,
        hw: &mut H,
        core_idx: usize,
    ) -> Result<Option<u32>, String> {
        let pc = (hw.raw_read_pc(core_idx)? & !1) as u64;
        let orig_opt = self
            .cores
            .get(core_idx)
            .and_then(|t| t.sw_ram_patches.get(&pc).copied());
        if let Some(orig) = orig_opt {
            let mut insn_buf = [0u8; 4];
            let _ = hw.raw_read_memory(core_idx, pc, &mut insn_buf);
            insn_buf[0..2].copy_from_slice(&orig);
            if let Some(sleep_len) = decode_arm_thumb_sleep_hint_len(&insn_buf) {
                let next_pc = (pc as u32).wrapping_add(sleep_len) & !1;
                hw.raw_write_pc(core_idx, next_pc)?;
                return Ok(Some(next_pc));
            }
            hw.raw_write_memory(core_idx, pc, &orig)?;
            let _ = hw.set_interrupt_mask(core_idx, true);
            let step_res = hw.raw_step_core(core_idx);
            let _ = hw.set_interrupt_mask(core_idx, false);
            let _ = hw.raw_write_memory(core_idx, pc, &THUMB_BKPT_0_LE);
            let next_pc = step_res?;
            return Ok(Some(next_pc & !1));
        }
        Ok(None)
    }

    /// Single-step one instruction on `core_idx` with `DHCSR.C_MASKINTS` enabled while temporarily
    /// disarming any hardware FPB comparator or SRAM `BKPT #0` patch matching the current `PC`,
    /// and advancing deterministically past `WFE`/`WFI` sleep-hint instructions without hanging.
    pub fn step_core<H: ArmHardwareOps + ?Sized>(
        &self,
        hw: &mut H,
        core_idx: usize,
    ) -> Result<u32, String> {
        let pc = (hw.raw_read_pc(core_idx)? & !1) as u64;
        let orig_sw_patch = self
            .cores
            .get(core_idx)
            .and_then(|t| t.sw_ram_patches.get(&pc).copied());

        let mut insn_buf = [0u8; 4];
        if hw.raw_read_memory(core_idx, pc, &mut insn_buf).is_ok() {
            if let Some(orig) = orig_sw_patch {
                insn_buf[0..2].copy_from_slice(&orig);
            }
            if let Some(sleep_len) = decode_arm_thumb_sleep_hint_len(&insn_buf) {
                let next_pc = (pc as u32).wrapping_add(sleep_len) & !1;
                hw.raw_write_pc(core_idx, next_pc)?;
                return Ok(next_pc);
            }
        }

        let mut disarmed_slots: Vec<usize> = Vec::new();
        if let Some(table) = self.cores.get(core_idx) {
            for (slot_idx, entry) in table.hw_slots.iter().enumerate() {
                if *entry == Some(pc) {
                    let _ = hw.clear_hw_breakpoint_slot(core_idx, slot_idx, pc);
                    disarmed_slots.push(slot_idx);
                }
            }
        }
        if core_idx == 0 && self.core1_post_reset_trap_addr == Some(pc) {
            if let Some(trap_slot) = self.armed_core0_trap_slot {
                if !disarmed_slots.contains(&trap_slot) {
                    let _ = hw.clear_hw_breakpoint_slot(0, trap_slot, pc);
                    disarmed_slots.push(trap_slot);
                }
            }
        }

        if let Some(orig) = orig_sw_patch {
            hw.raw_write_memory(core_idx, pc, &orig)?;
        }

        let _ = hw.set_interrupt_mask(core_idx, true);
        let step_res = hw.raw_step_core(core_idx);
        let _ = hw.set_interrupt_mask(core_idx, false);

        if orig_sw_patch.is_some() {
            let _ = hw.raw_write_memory(core_idx, pc, &THUMB_BKPT_0_LE);
        }
        for slot_idx in disarmed_slots {
            let _ = hw.set_hw_breakpoint_slot(core_idx, slot_idx, pc);
        }

        let next_pc = step_res?;
        Ok(next_pc & !1)
    }

    /// Prepare `core_idx` for resuming execution from Debug Halt, stepping past any active
    /// `PC` breakpoint or `WFE`/`WFI` sleep-hint instruction when user breakpoints are armed.
    pub fn prepare_resume<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
    ) -> Result<(), String> {
        if hw.raw_core_halted(core_idx).unwrap_or(false) {
            if let Ok(raw_pc) = hw.raw_read_pc(core_idx) {
                let pc = (raw_pc & !1) as u64;
                let has_trap_at_pc = self.has_breakpoint_at(core_idx, pc)
                    || (core_idx == 0 && self.core1_post_reset_trap_addr == Some(pc));
                if has_trap_at_pc {
                    let _ = self.step_core(hw, core_idx);
                } else if self.has_any_user_breakpoints(core_idx) {
                    let mut insn_buf = [0u8; 4];
                    if hw.raw_read_memory(core_idx, pc, &mut insn_buf).is_ok() {
                        if let Some(sleep_len) = decode_arm_thumb_sleep_hint_len(&insn_buf) {
                            let next_pc = (pc as u32).wrapping_add(sleep_len) & !1;
                            let _ = hw.raw_write_pc(core_idx, next_pc);
                        }
                    }
                }
            }
        }
        let _ = self.ensure_armed(hw, core_idx);
        Ok(())
    }

    /// Synchronize the optional Core 0 post-`PSM.frce_off` helper trap whenever Core 1 has breakpoints.
    pub fn sync_core1_trap<H: ArmHardwareOps + ?Sized>(&mut self, hw: &mut H) {
        let Some(trap_addr) = self.core1_post_reset_trap_addr else {
            return;
        };
        if self.cores.len() < 2 {
            return;
        }
        let c1_needs = self.cores[1].has_active_hw_breakpoints();
        if !c1_needs {
            if let Some(slot_idx) = self.armed_core0_trap_slot.take() {
                let _ = hw.clear_hw_breakpoint_slot(0, slot_idx, trap_addr);
            }
            return;
        }
        if let Some(existing_slot) = self.armed_core0_trap_slot {
            if self.cores[0]
                .hw_slots
                .get(existing_slot)
                .copied()
                .flatten()
                .is_some()
            {
                self.armed_core0_trap_slot = None;
            } else {
                return;
            }
        }
        if self.cores[0].hw_slots.contains(&Some(trap_addr)) {
            return;
        }
        if let Some(free_slot) = self.cores[0].hw_slots.iter().rposition(Option::is_none) {
            if hw.set_hw_breakpoint_slot(0, free_slot, trap_addr).is_ok() {
                self.armed_core0_trap_slot = Some(free_slot);
            }
        }
    }

    /// Service background maintenance for `core_idx`; returns `true` if Core 0 hit the internal Core 1 re-arm trap.
    pub fn poll_maintenance<H: ArmHardwareOps + ?Sized>(
        &mut self,
        hw: &mut H,
        core_idx: usize,
    ) -> bool {
        let serviced_trap = self.service_if_core0_hit_trap(hw);
        if core_idx >= 1 {
            if let Some(c1) = self.cores.get(core_idx) {
                if c1.has_active_hw_breakpoints() {
                    let slots = c1.hw_slots.clone();
                    let _ = hw.ensure_hw_breakpoints_active(core_idx, &slots);
                }
            }
        }
        serviced_trap && core_idx == 0
    }

    fn service_if_core0_hit_trap<H: ArmHardwareOps + ?Sized>(&mut self, hw: &mut H) -> bool {
        let (Some(trap_addr), Some(slot_idx)) =
            (self.core1_post_reset_trap_addr, self.armed_core0_trap_slot)
        else {
            return false;
        };
        if self.cores.len() < 2 {
            return false;
        }
        if !hw.raw_core_halted(0).unwrap_or(false) {
            return false;
        }
        let pc = (hw.raw_read_pc(0).unwrap_or(0) & !1) as u64;
        if pc != trap_addr {
            return false;
        }
        if self.cores[0].hw_slots.contains(&Some(trap_addr)) {
            return false;
        }
        let mut rearm_succeeded = true;
        if self.cores[1].has_active_hw_breakpoints() {
            let c1_slots = self.cores[1].hw_slots.clone();
            rearm_succeeded = hw.ensure_hw_breakpoints_active(1, &c1_slots).is_ok();
        }
        hw.release_secondary_bootrom_if_halted();
        if rearm_succeeded {
            let _ = hw.clear_hw_breakpoint_slot(0, slot_idx, trap_addr);
            self.armed_core0_trap_slot = None;
        } else {
            // Step Core 0 once past `trap_addr` while keeping `armed_core0_trap_slot` active so
            // subsequent iterations of `spawn_core1` (`cmds = [0, 0, 1, ...]`) re-trap and retry.
            let _ = self.step_core(hw, 0);
        }
        let _ = hw.raw_resume_core(0);
        true
    }
}

/// Hardware `probe_rs::Session` implementation of [`DebugBackend`] for ARM Cortex-M targets.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub mod probe_backend {
    use super::*;
    use probe_rs::{CoreType, MemoryInterface as _, RegisterId, Session};
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

    const SECONDARY_FPB_REARM_MAX_ATTEMPTS: usize = 5;
    const SECONDARY_FPB_REARM_RETRY_DELAY_MS: u64 = 1;

    fn with_session_core<T, F>(session: &mut Session, core_idx: usize, op: F) -> Result<T, String>
    where
        F: FnOnce(&mut probe_rs::Core<'_>) -> Result<T, String>,
    {
        let err_msg = match session.core(core_idx) {
            Ok(mut core) => return op(&mut core),
            Err(err) => format!("{:?}", err),
        };
        if core_idx > 0 {
            let _ = session.core(0);
        }
        Err(err_msg)
    }

    struct SessionHwAccess<'a> {
        session: &'a mut Session,
        core_archs: &'a [ProcessorArch],
        is_rp2040: bool,
    }

    impl ArmHardwareOps for SessionHwAccess<'_> {
        fn core_arch(&self, core_idx: usize) -> ProcessorArch {
            self.core_archs
                .get(core_idx)
                .copied()
                .unwrap_or(ProcessorArch::Armv6m)
        }

        fn raw_read_memory(
            &mut self,
            core_idx: usize,
            addr: u64,
            buf: &mut [u8],
        ) -> Result<(), String> {
            if buf.is_empty() {
                return Ok(());
            }
            with_session_core(self.session, core_idx, |core| {
                let head_len = ((4 - (addr as usize & 3)) & 3).min(buf.len());
                if head_len > 0 {
                    core.read_8(addr, &mut buf[..head_len])
                        .map_err(|e| format!("{:?}", e))?;
                }
                let body_bytes = &mut buf[head_len..];
                let word_count = body_bytes.len() / 4;
                if word_count > 0 {
                    let mut words = vec![0u32; word_count];
                    core.read_32(addr + head_len as u64, &mut words)
                        .map_err(|e| format!("{:?}", e))?;
                    for (idx, &w) in words.iter().enumerate() {
                        let off = idx * 4;
                        body_bytes[off..off + 4].copy_from_slice(&w.to_le_bytes());
                    }
                }
                let tail_offset = head_len + word_count * 4;
                if tail_offset < buf.len() {
                    core.read_8(addr + tail_offset as u64, &mut buf[tail_offset..])
                        .map_err(|e| format!("{:?}", e))?;
                }
                Ok(())
            })
        }

        fn raw_write_memory(
            &mut self,
            core_idx: usize,
            addr: u64,
            data: &[u8],
        ) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                core.write_8(addr, data).map_err(|e| format!("{:?}", e))?;
                core.flush().map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_read_pc(&mut self, core_idx: usize) -> Result<u32, String> {
            with_session_core(self.session, core_idx, |core| {
                core.read_core_reg::<u32>(RegisterId(DCRSR_PC))
                    .map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_write_pc(&mut self, core_idx: usize, pc: u32) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                if !core.core_halted().unwrap_or(false) {
                    let _ = core.halt(Duration::from_millis(50));
                }
                core.write_core_reg(RegisterId(DCRSR_PC), pc & !1)
                    .map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
            with_session_core(self.session, core_idx, |core| {
                core.core_halted().map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_step_core(&mut self, core_idx: usize) -> Result<u32, String> {
            with_session_core(self.session, core_idx, |core| match core.step() {
                Ok(info) => Ok((info.pc as u32) & !1),
                Err(step_err) => {
                    // If `C_STEP` ever enters sleep or stalls, force Debug Halt immediately so
                    // `C_MASKINTS` / `C_STEP` never remain latched on the target core.
                    if let Ok(halted_info) = core.halt(Duration::from_millis(50)) {
                        Ok((halted_info.pc as u32) & !1)
                    } else {
                        Err(format!("{:?}", step_err))
                    }
                }
            })
        }

        fn set_interrupt_mask(&mut self, core_idx: usize, masked: bool) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                let dhcsr_val = if masked {
                    CORTEX_M_DHCSR_DBGKEY_HALT_MASKINTS
                } else {
                    CORTEX_M_DHCSR_DBGKEY_HALT
                };
                core.write_word_32(CORTEX_M_DHCSR_ADDR, dhcsr_val)
                    .map_err(|e| format!("{:?}", e))?;
                core.flush().map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_resume_core(&mut self, core_idx: usize) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                // Write `DHCSR = DBGKEY | C_DEBUGEN` directly rather than calling `probe_rs::Core::run()`,
                // because `probe-rs` `armv6m::run()` unconditionally invokes `self.step()?` with
                // Clear `C_MASKINTS` while `C_HALT = 1` first (ARMv6-M requires `C_HALT == 1`
                // whenever modifying `C_MASKINTS`), then release `C_HALT` via `DHCSR` without calling
                // `probe_rs::Core::run()`, which writes `C_MASKINTS = 1` and hangs on `WFE`/`WFI` (`WORM-LITHE-ENCHYTRAEID-678`).
                if core.core_halted().unwrap_or(false) {
                    let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_HALT);
                }
                core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE)
                    .map_err(|e| format!("{:?}", e))?;
                core.flush().map_err(|e| format!("{:?}", e))?;
                let _ = core.status();
                Ok(())
            })
        }

        fn set_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            let program_slot = |session: &mut Session| -> Result<(), String> {
                with_session_core(session, core_idx, |core| {
                    if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                        if (dhcsr & 1) == 0 {
                            let _ = core
                                .write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                        }
                    }
                    core.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE)
                        .map_err(|e| format!("{:?}", e))?;
                    core.set_hw_breakpoint_unit(slot_idx, addr)
                        .map_err(|e| format!("{:?}", e))?;
                    core.flush().map_err(|e| format!("{:?}", e))
                })
            };
            match program_slot(self.session) {
                Ok(()) => Ok(()),
                Err(_) if core_idx == 0 => {
                    let _ = self.session.core(0);
                    program_slot(self.session)
                }
                Err(err) => Err(err),
            }
        }

        fn clear_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                let _ = core.clear_hw_breakpoint(addr);
                let _ = core.write_word_32(comp_addr, 0);
                let _ = core.flush();
                Ok(())
            })
        }

        fn ensure_hw_breakpoints_active(
            &mut self,
            core_idx: usize,
            slots: &[Option<u64>],
        ) -> Result<bool, String> {
            if !slots.iter().any(Option::is_some) {
                return Ok(false);
            }
            let max_attempts = if core_idx > 0 {
                SECONDARY_FPB_REARM_MAX_ATTEMPTS
            } else {
                1
            };
            let mut last_err = String::new();

            for attempt in 0..max_attempts {
                if attempt > 0 {
                    thread::sleep(Duration::from_millis(SECONDARY_FPB_REARM_RETRY_DELAY_MS));
                }
                let res = with_session_core(self.session, core_idx, |core| {
                    let bp_ctrl = core
                        .read_word_32(CORTEX_M_BP_CTRL_ADDR)
                        .map_err(|e| format!("{:?}", e))?;

                    let mut all_armed = (bp_ctrl & 1) != 0;
                    if all_armed {
                        for (slot_idx, entry) in slots.iter().enumerate() {
                            if entry.is_some() {
                                let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                                let comp_val = core
                                    .read_word_32(comp_addr)
                                    .map_err(|e| format!("{:?}", e))?;
                                if (comp_val & 1) == 0 {
                                    all_armed = false;
                                    break;
                                }
                            }
                        }
                    }
                    if all_armed {
                        return Ok(false);
                    }

                    if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                        if (dhcsr & 1) == 0 {
                            let _ = core
                                .write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
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
                    core.flush().map_err(|e| format!("{:?}", e))?;

                    // Read back every active FPB comparator to verify hardware latched `ENABLE` (`bit 0`).
                    for (slot_idx, entry) in slots.iter().enumerate() {
                        if entry.is_some() {
                            let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                            let comp_val = core
                                .read_word_32(comp_addr)
                                .map_err(|e| format!("{:?}", e))?;
                            if (comp_val & 1) == 0 {
                                return Err(format!(
                                    "FPB comparator slot {} on Core {} read back unarmed (0x{:08x})",
                                    slot_idx, core_idx, comp_val
                                ));
                            }
                        }
                    }
                    Ok(true)
                });
                match res {
                    Ok(rearmed) => {
                        if core_idx > 0 {
                            let _ = self.session.core(0);
                        }
                        return Ok(rearmed);
                    }
                    Err(err) => {
                        last_err = err;
                    }
                }
            }
            Err(last_err)
        }

        fn release_secondary_bootrom_if_halted(&mut self) {
            if !self.is_rp2040 || self.core_archs.len() < 2 {
                return;
            }
            let _ = with_session_core(self.session, 1, |core1| {
                if core1.core_halted().unwrap_or(false) {
                    let c1_pc = core1
                        .read_core_reg::<u32>(RegisterId(DCRSR_PC))
                        .unwrap_or(0)
                        & !1;
                    if c1_pc < RP2040_FLASH_BASE_ADDR {
                        if let Ok(demcr) = core1.read_word_32(CORTEX_M_DEMCR_ADDR) {
                            if (demcr & CORTEX_M_DEMCR_VC_CORERESET_MASK) != 0 {
                                let _ = core1.write_word_32(
                                    CORTEX_M_DEMCR_ADDR,
                                    demcr & !CORTEX_M_DEMCR_VC_CORERESET_MASK,
                                );
                            }
                        }
                        let _ =
                            core1.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                        let _ = core1.flush();
                        let _ = core1.status();
                    }
                }
                Ok(())
            });
            let _ = self.session.core(0);
        }
    }

    fn map_probe_core_type(ct: CoreType) -> Result<ProcessorArch, String> {
        match ct {
            CoreType::Armv6m => Ok(ProcessorArch::Armv6m),
            CoreType::Armv7m | CoreType::Armv7em => Ok(ProcessorArch::Armv7m),
            CoreType::Armv8m => Ok(ProcessorArch::Armv8m),
            other => Err(format!(
                "Unsupported non-ARM processor core type {:?} in ArmProbeDebugBackend",
                other
            )),
        }
    }

    #[derive(Clone, Debug)]
    struct ElfRomSegment {
        vaddr: u64,
        data: Vec<u8>,
    }

    fn load_elf_rom_segments(elf_path: Option<&Path>) -> Vec<ElfRomSegment> {
        const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
        const ELFCLASS32: u8 = 1;
        const ELFDATA2LSB: u8 = 1;
        const PT_LOAD: u32 = 1;
        const PF_W: u32 = 0x2;
        const ELF32_EHDR_MIN_LEN: usize = 52;
        const ELF32_PHDR_MIN_LEN: usize = 32;

        let Some(path) = elf_path else {
            return Vec::new();
        };
        let Ok(bytes) = std::fs::read(path) else {
            return Vec::new();
        };
        if bytes.len() < ELF32_EHDR_MIN_LEN
            || &bytes[0..4] != ELF_MAGIC
            || bytes[4] != ELFCLASS32
            || bytes[5] != ELFDATA2LSB
        {
            return Vec::new();
        }
        let phoff = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]) as usize;
        let phentsize = u16::from_le_bytes([bytes[42], bytes[43]]) as usize;
        let phnum = u16::from_le_bytes([bytes[44], bytes[45]]) as usize;
        if phentsize < ELF32_PHDR_MIN_LEN {
            return Vec::new();
        }

        let mut segments = Vec::new();
        for idx in 0..phnum {
            let Some(off) = phoff.checked_add(idx.saturating_mul(phentsize)) else {
                break;
            };
            if off.saturating_add(ELF32_PHDR_MIN_LEN) > bytes.len() {
                break;
            }
            let hdr = &bytes[off..off + ELF32_PHDR_MIN_LEN];
            let p_type = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
            let p_offset = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) as usize;
            let p_vaddr = u32::from_le_bytes([hdr[8], hdr[9], hdr[10], hdr[11]]) as u64;
            let p_filesz = u32::from_le_bytes([hdr[16], hdr[17], hdr[18], hdr[19]]) as usize;
            let p_memsz = u32::from_le_bytes([hdr[20], hdr[21], hdr[22], hdr[23]]) as usize;
            let p_flags = u32::from_le_bytes([hdr[24], hdr[25], hdr[26], hdr[27]]);

            if p_type == PT_LOAD
                && (p_flags & PF_W) == 0
                && p_filesz > 0
                && p_filesz == p_memsz
                && p_vaddr < CORTEX_M_SRAM_START
                && p_offset.saturating_add(p_filesz) <= bytes.len()
            {
                segments.push(ElfRomSegment {
                    vaddr: p_vaddr,
                    data: bytes[p_offset..p_offset + p_filesz].to_vec(),
                });
            }
        }
        segments
    }

    /// Concrete ARM Cortex-M `probe-rs` target session implementing [`DebugBackend`].
    pub struct ArmProbeDebugBackend {
        session: Session,
        chip: String,
        core_archs: Vec<ProcessorArch>,
        core_layouts: Vec<TargetRegisterLayout>,
        core0_reset_addr: Option<u64>,
        core1_reset_addr: Option<u64>,
        bp_mgr: ArmBreakpointManager,
        halted_reg_cache: Vec<BTreeMap<u16, u32>>,
        core1_left_bootrom: bool,
        rom_segments: Vec<ElfRomSegment>,
    }

    impl ArmProbeDebugBackend {
        fn halt_core_at_entry(
            session: &mut Session,
            core_idx: usize,
            entry_addr: Option<u64>,
        ) -> Result<u32, String> {
            let mut core = session
                .core(core_idx)
                .map_err(|e| format!("Failed to access Core {}: {:?}", core_idx, e))?;
            let info = core
                .reset_and_halt(Duration::from_millis(500))
                .map_err(|e| format!("Failed to reset_and_halt Core {}: {:?}", core_idx, e))?;
            let halted_pc = (info.pc as u32) & !1;

            let Some(raw_rh) = entry_addr else {
                return Ok(halted_pc);
            };
            let target_pc = (raw_rh as u32) & !1;
            if halted_pc == target_pc {
                return Ok(halted_pc);
            }

            // On chips with a Mask Boot ROM (e.g. RP2040 halting at 0x0000_00EA before boot2 enables XIP),
            // set a temporary hardware comparator on the highest FPB unit for `Reset_Handler`, run through
            // boot2, and clear that unit cleanly.
            let temp_slot =
                (core.available_breakpoint_units().unwrap_or(4) as usize).saturating_sub(1);
            if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                if (dhcsr & 1) == 0 {
                    let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                }
            }
            let _ = core.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE);
            let _ = core.flush();
            core.set_hw_breakpoint_unit(temp_slot, target_pc as u64)
                .map_err(|e| format!("Failed to arm Reset_Handler breakpoint: {:?}", e))?;
            core.run().map_err(|e| {
                format!("Failed to run Core {} to Reset_Handler: {:?}", core_idx, e)
            })?;

            let deadline = std::time::Instant::now() + Duration::from_millis(500);
            while std::time::Instant::now() < deadline {
                if core.core_halted().unwrap_or(false) {
                    break;
                }
                thread::sleep(Duration::from_millis(2));
            }
            if !core.core_halted().unwrap_or(false) {
                let _ = core.halt(Duration::from_millis(100));
            }
            let temp_comp_addr = CORTEX_M_BP_COMP0_ADDR + (temp_slot as u64) * 4;
            if core.clear_hw_breakpoint(target_pc as u64).is_err() {
                let _ = core.write_word_32(temp_comp_addr, 0);
                let _ = core.flush();
            }
            let final_pc = core
                .read_core_reg::<u32>(RegisterId(DCRSR_PC))
                .unwrap_or(halted_pc)
                & !1;
            Ok(final_pc)
        }

        /// Construct an [`ArmProbeDebugBackend`] from an already-attached `probe-rs` [`Session`] and optionally reset or reset-and-halt Core 0.
        pub fn from_session(
            mut session: Session,
            chip: &str,
            reset: bool,
            halt_on_reset: bool,
            core0_reset_addr: Option<u64>,
            core1_reset_addr: Option<u64>,
            elf_path: Option<&Path>,
        ) -> Result<Self, String> {
            let listed_cores = session.list_cores();
            let cores = listed_cores.len();
            if halt_on_reset {
                Self::halt_core_at_entry(&mut session, 0, core0_reset_addr)?;
            } else if reset {
                let mut core0 = session
                    .core(0)
                    .map_err(|e| format!("Failed to access Core 0: {:?}", e))?;
                core0
                    .reset()
                    .map_err(|e| format!("Failed to reset Core 0: {:?}", e))?;
                let _ = core0.run();
                thread::sleep(Duration::from_millis(25));
            }

            let mut core_archs = Vec::with_capacity(cores);
            let mut core_layouts = Vec::with_capacity(cores);
            let mut bp_units = Vec::with_capacity(cores);
            let (core0_arch, core0_fpu, core0_units) = {
                let mut core0 = session
                    .core(0)
                    .map_err(|e| format!("Failed to inspect Core 0: {:?}", e))?;
                let arch = map_probe_core_type(core0.core_type())?;
                let has_fpu = core0.fpu_support().unwrap_or(false);
                let units = core0.available_breakpoint_units().unwrap_or(4) as usize;
                (arch, has_fpu, units)
            };
            core_archs.push(core0_arch);
            core_layouts.push(build_arm_cortex_m_register_layout(core0_arch, core0_fpu));
            bp_units.push(core0_units);

            for &(idx, core_type) in listed_cores.iter().skip(1) {
                let arch = map_probe_core_type(core_type)?;
                let default_units = match arch {
                    ProcessorArch::Armv6m => 4,
                    ProcessorArch::Armv7m | ProcessorArch::Armv8m => 8,
                };
                // Secondary cores may be held in hardware reset at attach/reset time; avoid
                // sending SWD Multidrop TARGETSEL right after reset, or fall back cleanly if
                // the secondary core DAP is not yet powered up.
                let (has_fpu, units) = if reset || halt_on_reset {
                    (false, core0_units.min(default_units))
                } else {
                    let inspected = with_session_core(&mut session, idx, |sec_core| {
                        let fpu = sec_core.fpu_support().unwrap_or(false);
                        let u = sec_core
                            .available_breakpoint_units()
                            .unwrap_or(default_units as u32)
                            as usize;
                        Ok((fpu, u))
                    });
                    let _ = session.core(0);
                    inspected.unwrap_or((false, core0_units.min(default_units)))
                };
                core_archs.push(arch);
                core_layouts.push(build_arm_cortex_m_register_layout(arch, has_fpu));
                bp_units.push(units);
            }

            // On RP2040, `embassy_rp::multicore::spawn_core1` pulses `PSM.frce_off.proc1` (wiping Core 1's
            // BPU comparators) right before draining the SIO FIFO to launch `Core1_Reset_Handler`.
            let rp2040_psm_trap = if chip.eq_ignore_ascii_case("rp2040") {
                elf_path.and_then(|p| {
                    tool_common::resolve_breakpoint_target(p, "multicore.rs:247").ok()
                })
            } else {
                None
            };

            let mut rom_segments = load_elf_rom_segments(elf_path);
            if !rom_segments.is_empty() {
                let verified = with_session_core(&mut session, 0, |core0| {
                    for seg in &rom_segments {
                        let check_len = seg.data.len().min(16);
                        if check_len > 0 {
                            let mut actual = vec![0u8; check_len];
                            if core0.read_8(seg.vaddr, &mut actual).is_err()
                                || actual.as_slice() != &seg.data[..check_len]
                            {
                                return Ok(false);
                            }
                        }
                    }
                    Ok(true)
                })
                .unwrap_or(false);
                if !verified {
                    rom_segments.clear();
                }
            }
            let core_count = core_archs.len();
            let mut backend = Self {
                session,
                chip: chip.to_string(),
                core_archs,
                core_layouts,
                core0_reset_addr,
                core1_reset_addr,
                bp_mgr: ArmBreakpointManager::new(&bp_units, rp2040_psm_trap),
                halted_reg_cache: vec![BTreeMap::new(); core_count],
                core1_left_bootrom: false,
                rom_segments,
            };
            backend.configure_target_debug_peripherals();
            Ok(backend)
        }

        fn try_read_rom(&self, addr: u64, buf: &mut [u8]) -> bool {
            let Some(end) = addr.checked_add(buf.len() as u64) else {
                return false;
            };
            for seg in &self.rom_segments {
                let seg_end = seg.vaddr.saturating_add(seg.data.len() as u64);
                if addr >= seg.vaddr && end <= seg_end {
                    let start_off = (addr - seg.vaddr) as usize;
                    buf.copy_from_slice(&seg.data[start_off..start_off + buf.len()]);
                    return true;
                }
            }
            false
        }

        fn clear_reg_cache(&mut self, core_idx: usize) {
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.clear();
            }
        }

        fn clear_all_reg_caches(&mut self) {
            for cache in &mut self.halted_reg_cache {
                cache.clear();
            }
        }

        /// Configure chip-specific debug-pause peripherals (e.g. RP2040 `TIMER.DBGPAUSE = DBG0`) and
        /// clear lingering `DEMCR.VC_CORERESET` vector catches so Core 0 `.await` timers and `spawn_core1`
        /// FIFO handshakes never deadlock or trigger stuck-task resets during dual-core GDB sessions.
        fn configure_target_debug_peripherals(&mut self) {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let _ = with_session_core(&mut self.session, 0, |core0| {
                if is_rp2040 {
                    let _ = core0.write_word_32(
                        RP2040_TIMER_DBGPAUSE_ADDR,
                        RP2040_TIMER_DBGPAUSE_CORE0_ONLY,
                    );
                }
                if let Ok(demcr) = core0.read_word_32(CORTEX_M_DEMCR_ADDR) {
                    if (demcr & CORTEX_M_DEMCR_VC_CORERESET_MASK) != 0 {
                        let _ = core0.write_word_32(
                            CORTEX_M_DEMCR_ADDR,
                            demcr & !CORTEX_M_DEMCR_VC_CORERESET_MASK,
                        );
                    }
                }
                let _ = core0.flush();
                Ok(())
            });
        }

        /// Phase 1 of [`DebugBackend::step_core`]: delegate single-stepping to [`ArmBreakpointManager`]
        /// whenever software `BKPT #0` patches are active in SRAM so patched halfwords are temporarily
        /// restored before stepping and re-inserted afterward.
        fn step_core_with_sw_patches(&mut self, core_idx: usize) -> Result<u32, String> {
            self.clear_reg_cache(core_idx);
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let next_pc = {
                let mut hw = SessionHwAccess {
                    session: &mut self.session,
                    core_archs: &self.core_archs,
                    is_rp2040,
                };
                self.bp_mgr.step_core(&mut hw, core_idx)?
            };
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(DCRSR_PC, next_pc);
            }
            Ok(next_pc)
        }

        /// Phase 2 of [`DebugBackend::step_core`]: resolve the starting Thumb-normalized `PC` (reusing
        /// `cached_pc` when available) and fetch the 4-byte instruction window at `PC` from the host-side
        /// ELF `.text`/`.rodata` ROM cache or target memory.
        fn fetch_step_pc_and_insn(
            core: &mut probe_rs::Core<'_>,
            cached_pc: Option<u32>,
            rom_segs: &[ElfRomSegment],
        ) -> Result<(u64, Option<[u8; 4]>), String> {
            let pc = match cached_pc {
                Some(c_pc) => (c_pc & !1) as u64,
                None => {
                    (core
                        .read_core_reg::<u32>(RegisterId(DCRSR_PC))
                        .map_err(|e| format!("{:?}", e))?
                        & !1) as u64
                }
            };

            let mut insn_buf = [0u8; 4];
            if let Some(end) = pc.checked_add(4) {
                for seg in rom_segs {
                    let seg_end = seg.vaddr.saturating_add(seg.data.len() as u64);
                    if pc >= seg.vaddr && end <= seg_end {
                        let off = (pc - seg.vaddr) as usize;
                        insn_buf.copy_from_slice(&seg.data[off..off + 4]);
                        return Ok((pc, Some(insn_buf)));
                    }
                }
            }
            let got_insn = core.read_8(pc, &mut insn_buf).ok().map(|()| insn_buf);
            Ok((pc, got_insn))
        }

        /// Phase 3 of [`DebugBackend::step_core`]: either deterministically advance `DCRSR_PC` past an
        /// ARM Thumb/Thumb-2 `WFE`/`WFI` sleep hint (avoiding `CoreStatus::Sleeping` lockups when
        /// `DHCSR.C_MASKINTS = 1`) or disarm matching FPB comparators, pulse `DHCSR.C_STEP | C_MASKINTS`,
        /// read the resulting `PC`, and re-arm the FPB comparators.
        ///
        /// Returns `(final_pc, bypassed_sleep_hint)`.
        fn advance_sleep_hint_or_hw_step(
            core: &mut probe_rs::Core<'_>,
            core_idx: usize,
            is_rp2040: bool,
            pc: u64,
            insn_opt: Option<[u8; 4]>,
            bp_mgr_ref: &ArmBreakpointManager,
        ) -> Result<(u32, bool), String> {
            if let Some(insn_buf) = insn_opt {
                if let Some(sleep_len) = decode_arm_thumb_sleep_hint_len(&insn_buf) {
                    let target_pc = (pc as u32).wrapping_add(sleep_len) & !1;
                    if !core.core_halted().unwrap_or(false) {
                        let _ = core.halt(Duration::from_millis(50));
                    }
                    if is_rp2040 && core_idx == 0 {
                        // Allow the RP2040 1 MHz hardware timer to advance briefly while stepping
                        // across `WFE`/`WFI` so pending `embassy_time` alarms mature and wake tasks
                        // on the next `Executor::poll()` instead of spinning indefinitely at `wfe()`.
                        let _ = core.write_word_32(RP2040_TIMER_DBGPAUSE_ADDR, 0);
                        let _ = core.flush();
                        thread::sleep(Duration::from_millis(WFE_STEP_TIMER_ADVANCE_MS));
                        let _ = core.write_word_32(
                            RP2040_TIMER_DBGPAUSE_ADDR,
                            RP2040_TIMER_DBGPAUSE_CORE0_ONLY,
                        );
                    }
                    core.write_core_reg(RegisterId(DCRSR_PC), target_pc)
                        .map_err(|e| format!("{:?}", e))?;
                    return Ok((target_pc, true));
                }
            }

            // Temporarily disarm any hardware FPB comparator at `pc` so `C_STEP` advances instead of re-trapping.
            let mut disarmed_slots: Vec<usize> = Vec::new();
            if let Some(table) = bp_mgr_ref.cores.get(core_idx) {
                for (slot_idx, entry) in table.hw_slots.iter().enumerate() {
                    if *entry == Some(pc) {
                        if core.clear_hw_breakpoint(pc).is_err() {
                            let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                            let _ = core.write_word_32(comp_addr, 0);
                            let _ = core.flush();
                        }
                        disarmed_slots.push(slot_idx);
                    }
                }
            }
            if core_idx == 0 && bp_mgr_ref.core1_post_reset_trap_addr == Some(pc) {
                if let Some(trap_slot) = bp_mgr_ref.armed_core0_trap_slot {
                    if !disarmed_slots.contains(&trap_slot) {
                        if core.clear_hw_breakpoint(pc).is_err() {
                            let comp_addr = CORTEX_M_BP_COMP0_ADDR + (trap_slot as u64) * 4;
                            let _ = core.write_word_32(comp_addr, 0);
                            let _ = core.flush();
                        }
                        disarmed_slots.push(trap_slot);
                    }
                }
            }

            let step_res = (|| -> Result<u32, String> {
                let _ =
                    core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_HALT_MASKINTS);
                core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_STEP_MASKINTS)
                    .map_err(|e| format!("{:?}", e))?;
                let dhcsr = core
                    .read_word_32(CORTEX_M_DHCSR_ADDR)
                    .map_err(|e| format!("{:?}", e))?;
                if (dhcsr & CORTEX_M_DHCSR_S_HALT_MASK) == 0
                    && core
                        .wait_for_core_halted(Duration::from_millis(50))
                        .is_err()
                {
                    let halted_info = core
                        .halt(Duration::from_millis(50))
                        .map_err(|e| format!("{:?}", e))?;
                    return Ok((halted_info.pc as u32) & !1);
                }
                core.write_word_32(CORTEX_M_DCRSR_ADDR, DCRSR_PC as u32)
                    .map_err(|e| format!("{:?}", e))?;
                let new_pc = core
                    .read_word_32(CORTEX_M_DCRDR_ADDR)
                    .map_err(|e| format!("{:?}", e))?;
                Ok(new_pc & !1)
            })();
            // Always clear `C_MASKINTS` while `C_HALT == 1` so subsequent `continue` runs with IRQs unmasked!
            let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_HALT);

            for slot_idx in disarmed_slots {
                let _ = core.set_hw_breakpoint_unit(slot_idx, pc);
            }
            let _ = core.flush();
            Ok((step_res?, false))
        }

        /// Phase 4 of [`DebugBackend::step_core`]: assemble all expedited GDB `T05` general-purpose
        /// registers (`r0..r12`, `sp`, `lr`, `xpsr`), preserving architecturally untouched cached
        /// values across 16-bit Thumb steps and fetching only modified/uncached registers over SWD.
        fn refresh_expedited_step_regs(
            core: &mut probe_rs::Core<'_>,
            pc: u32,
            final_pc: u32,
            insn_opt: Option<[u8; 4]>,
            bypassed_sleep_hint: bool,
            mut prev_cache: BTreeMap<u16, u32>,
        ) -> BTreeMap<u16, u32> {
            const EXPEDITED_CORE_REG_IDS: [u16; 16] = [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, DCRSR_SP, DCRSR_LR, DCRSR_XPSR,
            ];
            prev_cache.remove(&DCRSR_PC);

            if !bypassed_sleep_hint {
                let dirty = insn_opt.and_then(|insn_buf| {
                    let hw = u16::from_le_bytes([insn_buf[0], insn_buf[1]]);
                    thumb16_dirty_core_regs(hw, pc, final_pc, prev_cache.get(&DCRSR_XPSR).copied())
                });
                match dirty {
                    Some((dirty_gpr_mask, dirty_xpsr)) => {
                        for reg_id in 0..=14u16 {
                            if (dirty_gpr_mask & (1u32 << reg_id)) != 0 {
                                prev_cache.remove(&reg_id);
                            }
                        }
                        if (dirty_gpr_mask & (1u32 << DCRSR_SP)) != 0 {
                            prev_cache.remove(&DCRSR_MSP);
                            prev_cache.remove(&DCRSR_PSP);
                        }
                        if dirty_xpsr {
                            prev_cache.remove(&DCRSR_XPSR);
                        }
                    }
                    None => {
                        prev_cache.clear();
                    }
                }
            }

            for reg_id in EXPEDITED_CORE_REG_IDS {
                if prev_cache.contains_key(&reg_id) {
                    continue;
                }
                if core
                    .write_word_32(CORTEX_M_DCRSR_ADDR, reg_id as u32)
                    .is_ok()
                {
                    if let Ok(val) = core.read_word_32(CORTEX_M_DCRDR_ADDR) {
                        prev_cache.insert(reg_id, val);
                    }
                }
            }
            prev_cache
        }
    }

    impl DebugBackend for ArmProbeDebugBackend {
        fn chip_name(&self) -> &str {
            &self.chip
        }

        fn core_count(&self) -> usize {
            self.core_archs.len()
        }

        fn core_register_layout(&mut self, core_idx: usize) -> TargetRegisterLayout {
            self.core_layouts
                .get(core_idx)
                .cloned()
                .unwrap_or_else(|| build_arm_cortex_m_register_layout(ProcessorArch::Armv6m, false))
        }

        fn core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let halted = {
                let mut hw = SessionHwAccess {
                    session: &mut self.session,
                    core_archs: &self.core_archs,
                    is_rp2040,
                };
                hw.raw_core_halted(core_idx)?
            };
            if !halted {
                self.clear_reg_cache(core_idx);
            }
            Ok(halted)
        }

        fn halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            self.clear_reg_cache(core_idx);
            let pc = with_session_core(&mut self.session, core_idx, |core| {
                let info = core
                    .halt(Duration::from_millis(250))
                    .map_err(|e| format!("{:?}", e))?;
                Ok((info.pc as u32) & !1)
            })?;
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(DCRSR_PC, pc);
            }
            if core_idx == 1 && pc >= RP2040_FLASH_BASE_ADDR {
                self.core1_left_bootrom = true;
            }
            Ok(pc)
        }

        fn resume_core(&mut self, core_idx: usize) -> Result<(), String> {
            self.clear_reg_cache(core_idx);
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let check_bootrom = core_idx == 0 && is_rp2040 && !self.core1_left_bootrom;
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            if check_bootrom {
                // If Core 1 was halted in Mask Boot ROM (`PC < 0x1000_0000`) by a Dual-Core GDB attach
                // before Core 0 calls `spawn_core1`, release Core 1's Boot ROM FIFO loop (and clear
                // `DEMCR.VC_CORERESET`) so Core 0's `bootstrap_core1` handshake never deadlocks!
                hw.release_secondary_bootrom_if_halted();
            }
            let _ = self.bp_mgr.prepare_resume(&mut hw, core_idx);
            match hw.raw_resume_core(core_idx) {
                Ok(()) => Ok(()),
                Err(_) if core_idx > 0 => Ok(()),
                Err(e) => Err(e),
            }
        }

        fn step_core(&mut self, core_idx: usize) -> Result<u32, String> {
            // Phase 1: Delegate to `ArmBreakpointManager` when SRAM `BKPT #0` patches must be swapped around the step.
            let has_sw_patches = self
                .bp_mgr
                .cores
                .get(core_idx)
                .map(|t| !t.sw_ram_patches.is_empty())
                .unwrap_or(false);
            if has_sw_patches {
                return self.step_core_with_sw_patches(core_idx);
            }

            // Snapshot pre-step register cache before invalidating so unmodified registers can be preserved.
            let prev_cache = self
                .halted_reg_cache
                .get(core_idx)
                .cloned()
                .unwrap_or_default();
            let cached_pc = prev_cache.get(&DCRSR_PC).copied();
            self.clear_reg_cache(core_idx);

            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let rom_segs = &self.rom_segments;
            let bp_mgr_ref = &self.bp_mgr;
            let (next_pc, mut updated_cache) =
                with_session_core(&mut self.session, core_idx, |core| {
                    // Phase 2: Resolve current `PC` and fetch the 4-byte instruction window (ELF ROM cache or target RAM).
                    let (pc, insn_opt) = Self::fetch_step_pc_and_insn(core, cached_pc, rom_segs)?;

                    // Phase 3: Advance deterministically past `WFE`/`WFI` or execute a masked hardware `C_STEP`.
                    let (final_pc, bypassed_sleep_hint) = Self::advance_sleep_hint_or_hw_step(
                        core, core_idx, is_rp2040, pc, insn_opt, bp_mgr_ref,
                    )?;

                    // Phase 4: Refresh expedited `T05` general registers (`r0..r12, sp, lr, xpsr`) so GDB never needs `$g`.
                    let cache_after = Self::refresh_expedited_step_regs(
                        core,
                        pc as u32,
                        final_pc,
                        insn_opt,
                        bypassed_sleep_hint,
                        prev_cache,
                    );
                    Ok((final_pc, cache_after))
                })?;

            updated_cache.insert(DCRSR_PC, next_pc);
            if let Some(slot) = self.halted_reg_cache.get_mut(core_idx) {
                *slot = updated_cache;
            }
            Ok(next_pc)
        }

        fn step_range_core(
            &mut self,
            core_idx: usize,
            start_addr: u32,
            end_addr: u32,
            max_steps: usize,
        ) -> Result<u32, String> {
            let norm_start = start_addr & !1;
            let norm_end = end_addr & !1;
            // Perform the first step through `bp_mgr.step_core` so any breakpoint or SRAM patch
            // at the starting `PC` is cleanly disarmed, stepped over, and re-armed.
            let mut cur_pc = self.step_core(core_idx)? & !1;
            let total_limit = max_steps.max(1);
            if total_limit <= 1
                || cur_pc < norm_start
                || cur_pc >= norm_end
                || self.bp_mgr.has_breakpoint_at(core_idx, cur_pc as u64)
            {
                return Ok(cur_pc);
            }

            let has_sw_patches = self
                .bp_mgr
                .cores
                .get(core_idx)
                .map(|t| !t.sw_ram_patches.is_empty())
                .unwrap_or(false);
            if has_sw_patches {
                for _ in 1..total_limit {
                    if cur_pc < norm_start
                        || cur_pc >= norm_end
                        || self.bp_mgr.has_breakpoint_at(core_idx, cur_pc as u64)
                    {
                        break;
                    }
                    cur_pc = self.step_core(core_idx)? & !1;
                }
                return Ok(cur_pc);
            }

            // Fast hardware range-stepping path inside a single `session.core(core_idx)` borrow!
            const EXPEDITED_PREFETCH_REGS: &[u16] = &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, DCRSR_SP, DCRSR_LR, DCRSR_XPSR,
            ];
            self.clear_reg_cache(core_idx);
            let rom_segs = &self.rom_segments;
            let bp_mgr_ref = &self.bp_mgr;
            let (final_pc, prefetched) = with_session_core(&mut self.session, core_idx, |core| {
                let mut pc = cur_pc;
                let mut insn_buf = [0u8; 4];
                for _ in 1..total_limit {
                    if pc < norm_start
                        || pc >= norm_end
                        || bp_mgr_ref.has_breakpoint_at(core_idx, pc as u64)
                        || (core_idx == 0
                            && bp_mgr_ref.core1_post_reset_trap_addr == Some(pc as u64))
                    {
                        break;
                    }
                    let pc_u64 = pc as u64;
                    let mut got_insn = false;
                    if let Some(end) = pc_u64.checked_add(4) {
                        for seg in rom_segs {
                            let seg_end = seg.vaddr.saturating_add(seg.data.len() as u64);
                            if pc_u64 >= seg.vaddr && end <= seg_end {
                                let off = (pc_u64 - seg.vaddr) as usize;
                                insn_buf.copy_from_slice(&seg.data[off..off + 4]);
                                got_insn = true;
                                break;
                            }
                        }
                    }
                    if !got_insn {
                        got_insn = core.read_8(pc_u64, &mut insn_buf).is_ok();
                    }
                    if got_insn {
                        if let Some(sleep_len) = decode_arm_thumb_sleep_hint_len(&insn_buf) {
                            let next_pc = pc.wrapping_add(sleep_len) & !1;
                            core.write_core_reg(RegisterId(DCRSR_PC), next_pc)
                                .map_err(|e| format!("{:?}", e))?;
                            pc = next_pc;
                            continue;
                        }
                    }
                    core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_STEP_MASKINTS)
                        .map_err(|e| format!("{:?}", e))?;
                    let dhcsr = core
                        .read_word_32(CORTEX_M_DHCSR_ADDR)
                        .map_err(|e| format!("{:?}", e))?;
                    if (dhcsr & CORTEX_M_DHCSR_S_HALT_MASK) == 0
                        && core
                            .wait_for_core_halted(Duration::from_millis(50))
                            .is_err()
                    {
                        if let Ok(halted_info) = core.halt(Duration::from_millis(50)) {
                            pc = (halted_info.pc as u32) & !1;
                            break;
                        }
                    }
                    core.write_word_32(CORTEX_M_DCRSR_ADDR, DCRSR_PC as u32)
                        .map_err(|e| format!("{:?}", e))?;
                    let raw_next = core
                        .read_word_32(CORTEX_M_DCRDR_ADDR)
                        .map_err(|e| format!("{:?}", e))?;
                    pc = raw_next & !1;
                }
                let _ = core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_HALT);
                let mut regs = Vec::with_capacity(EXPEDITED_PREFETCH_REGS.len());
                for &reg_id in EXPEDITED_PREFETCH_REGS {
                    if core
                        .write_word_32(CORTEX_M_DCRSR_ADDR, reg_id as u32)
                        .is_ok()
                    {
                        if let Ok(v) = core.read_word_32(CORTEX_M_DCRDR_ADDR) {
                            regs.push((reg_id, v));
                        }
                    }
                }
                Ok((pc, regs))
            })?;
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(DCRSR_PC, final_pc);
                for (reg_id, val) in prefetched {
                    cache.insert(reg_id, val);
                }
            }
            Ok(final_pc)
        }

        fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            self.clear_all_reg_caches();
            self.core1_left_bootrom = false;
            let entry = if core_idx == 0 {
                self.core0_reset_addr
            } else {
                self.core1_reset_addr
            };
            let pc = Self::halt_core_at_entry(&mut self.session, core_idx, entry)?;
            self.configure_target_debug_peripherals();
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(DCRSR_PC, pc);
            }
            Ok(pc)
        }

        fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String> {
            self.clear_all_reg_caches();
            self.core1_left_bootrom = false;
            {
                let mut core = self.session.core(core_idx).map_err(|e| {
                    format!("Failed to access Core {} for reset: {:?}", core_idx, e)
                })?;
                core.reset()
                    .map_err(|e| format!("Failed to reset Core {}: {:?}", core_idx, e))?;
                // Clear `C_HALT` / `C_STEP` / `C_MASKINTS` directly via `DHCSR` while keeping `C_DEBUGEN`
                // enabled so hardware breakpoints remain active without invoking `probe-rs` `armv6m::run()`'s
                // masked pre-step, and propagate any SWD write/flush/status error explicitly.
                core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE)
                    .map_err(|e| {
                        format!(
                            "Failed to write DHCSR after Core {} reset: {:?}",
                            core_idx, e
                        )
                    })?;
                core.flush().map_err(|e| {
                    format!("Failed to flush SWD after Core {} reset: {:?}", core_idx, e)
                })?;
                core.status().map_err(|e| {
                    format!(
                        "Failed to synchronize Core {} status after reset: {:?}",
                        core_idx, e
                    )
                })?;
            }
            self.configure_target_debug_peripherals();
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            self.bp_mgr.ensure_armed(&mut hw, core_idx)?;
            Ok(())
        }

        fn read_core_reg(&mut self, core_idx: usize, hw_reg_id: u16) -> Result<u32, String> {
            if let Some(val) = self
                .halted_reg_cache
                .get(core_idx)
                .and_then(|c| c.get(&hw_reg_id).copied())
            {
                return Ok(val);
            }
            let val = with_session_core(&mut self.session, core_idx, |core| {
                core.read_core_reg::<u32>(RegisterId(hw_reg_id))
                    .map_err(|e| format!("{:?}", e))
            })?;
            if core_idx == 1 && hw_reg_id == DCRSR_PC && (val & !1) >= RP2040_FLASH_BASE_ADDR {
                self.core1_left_bootrom = true;
            }
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(hw_reg_id, val);
            }
            Ok(val)
        }

        fn read_core_regs(&mut self, core_idx: usize, hw_reg_ids: &[u16]) -> Vec<u32> {
            let mut results = vec![0u32; hw_reg_ids.len()];
            let mut missing: Vec<(usize, u16)> = Vec::new();
            // `halted_reg_cache` is intentionally populated sparsely during halts and single-steps:
            // `halt_core` / `reset_and_halt_core` seed only `PC`, and `step_core` / `step_range_core`
            // prefetch only the expedited `T05` frame-unwind registers (`PC`, `r7`, `sp`, `lr`) to avoid
            // 19+ SWD `DCRSR` round-trips per instruction. When GDB or an IDE subsequently requests the
            // full register bank (`$g`, e.g. opening the VS Code Registers view or evaluating locals in
            // `r0..r6`, `r8..r12`, `xpsr`, `msp`, `psp`, or FPU `s0..s31`), those uncached registers
            // miss here (`cache.get(&reg_id) == None`) and are fetched on-demand below and memoized for
            // the remainder of the current halt.
            if let Some(cache) = self.halted_reg_cache.get(core_idx) {
                for (idx, &reg_id) in hw_reg_ids.iter().enumerate() {
                    if let Some(&cached) = cache.get(&reg_id) {
                        results[idx] = cached;
                    } else {
                        missing.push((idx, reg_id));
                    }
                }
            } else {
                for (idx, &reg_id) in hw_reg_ids.iter().enumerate() {
                    missing.push((idx, reg_id));
                }
            }

            if !missing.is_empty() {
                let fetched = with_session_core(&mut self.session, core_idx, |core| {
                    let mut out = Vec::with_capacity(missing.len());
                    // Secondary cores held in `PSM` hardware reset prior to `spawn_core1` (or cores
                    // queried while briefly running) cannot service `DCRSR` reads; check `core_halted()`
                    // once and issue direct `DCRSR`+`DCRDR` MMIO reads (avoiding 3x USB round-trips per
                    // register in `core.read_core_reg`), falling back to `0` if not halted.
                    let is_halted = core.core_halted().unwrap_or(false);
                    for &(idx, reg_id) in &missing {
                        if is_halted
                            && core
                                .write_word_32(CORTEX_M_DCRSR_ADDR, reg_id as u32)
                                .is_ok()
                        {
                            if let Ok(val) = core.read_word_32(CORTEX_M_DCRDR_ADDR) {
                                out.push((idx, reg_id, Some(val)));
                                continue;
                            }
                        }
                        out.push((idx, reg_id, None));
                    }
                    Ok(out)
                });
                if let Ok(items) = fetched {
                    for (idx, reg_id, opt_val) in items {
                        if let Some(val) = opt_val {
                            results[idx] = val;
                            if core_idx == 1
                                && reg_id == DCRSR_PC
                                && (val & !1) >= RP2040_FLASH_BASE_ADDR
                            {
                                self.core1_left_bootrom = true;
                            }
                            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                                cache.insert(reg_id, val);
                            }
                        }
                    }
                }
            }
            results
        }

        fn write_core_reg(
            &mut self,
            core_idx: usize,
            hw_reg_id: u16,
            value: u32,
        ) -> Result<(), String> {
            with_session_core(&mut self.session, core_idx, |core| {
                core.write_core_reg(RegisterId(hw_reg_id), value)
                    .map_err(|e| format!("{:?}", e))
            })?;
            if let Some(cache) = self.halted_reg_cache.get_mut(core_idx) {
                cache.insert(
                    hw_reg_id,
                    if hw_reg_id == DCRSR_PC {
                        value & !1
                    } else {
                        value
                    },
                );
            }
            Ok(())
        }

        fn read_memory(
            &mut self,
            core_idx: usize,
            addr: u64,
            buf: &mut [u8],
        ) -> Result<(), String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            hw.raw_read_memory(core_idx, addr, buf)
        }

        fn read_memory_masked(
            &mut self,
            core_idx: usize,
            addr: u64,
            buf: &mut [u8],
        ) -> Result<(), String> {
            if !self.try_read_rom(addr, buf) {
                self.read_memory(core_idx, addr, buf)?;
            }
            if let Some(table) = self.bp_mgr.cores.get(core_idx) {
                table.mask_memory_read(addr, buf);
            }
            Ok(())
        }

        fn write_memory(&mut self, core_idx: usize, addr: u64, data: &[u8]) -> Result<(), String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            hw.raw_write_memory(core_idx, addr, data)
        }

        fn insert_breakpoint(
            &mut self,
            core_idx: usize,
            addr: u64,
            allow_sw_ram: bool,
        ) -> Result<bool, String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            self.bp_mgr
                .insert_breakpoint(&mut hw, core_idx, addr, allow_sw_ram)
        }

        fn remove_breakpoint(&mut self, core_idx: usize, addr: u64) -> Result<(), String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            self.bp_mgr.remove_breakpoint(&mut hw, core_idx, addr)
        }

        fn clear_all_breakpoints(&mut self, core_idx: usize) -> Result<(), String> {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            self.bp_mgr.clear_all(&mut hw, core_idx)
        }

        fn has_active_breakpoint_at(&self, core_idx: usize, addr: u64) -> bool {
            self.bp_mgr.has_breakpoint_at(core_idx, addr)
        }

        fn poll_background_core_maintenance(&mut self, core_idx: usize) -> bool {
            let is_rp2040 = self.chip.eq_ignore_ascii_case("rp2040");
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
                is_rp2040,
            };
            self.bp_mgr.poll_maintenance(&mut hw, core_idx)
        }
    }
}
