//! ARM Cortex-M (`ARMv6-M`, `ARMv7-M`, `ARMv8-M`) register descriptors, hardware FPB + SRAM
//! software `BKPT #0` management, and `probe-rs` SWD debug backend.

use crate::arch::{ProcessorArch, RegisterDescriptor, RegisterFeatureGroup, TargetRegisterLayout};
use crate::server::DebugBackend;
use std::collections::BTreeMap;

/// Start address of Cortex-M SRAM region (`0x2000_0000`).
pub const CORTEX_M_SRAM_START: u64 = 0x2000_0000;

/// Exclusive end address of Cortex-M SRAM region (`0x4000_0000`).
pub const CORTEX_M_SRAM_END: u64 = 0x4000_0000;

/// Little-endian encoding of 16-bit Thumb `BKPT #0` (`0xBE00`).
pub const THUMB_BKPT_0_LE: [u8; 2] = [0x00, 0xBE];

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

/// Build a single-sourced ARM Cortex-M [`TargetRegisterLayout`] rendered dynamically via Rinja.
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
        is_pc: false,
    });
    core_regs.push(RegisterDescriptor {
        name: "lr".to_string(),
        bitsize: 32,
        gdb_regnum: 14,
        gdb_alias_regnum: None,
        hw_reg_id: 14,
        reg_type: "int",
        is_pc: false,
    });
    core_regs.push(RegisterDescriptor {
        name: "pc".to_string(),
        bitsize: 32,
        gdb_regnum: 15,
        gdb_alias_regnum: None,
        hw_reg_id: DCRSR_PC,
        reg_type: "code_ptr",
        is_pc: true,
    });
    core_regs.push(RegisterDescriptor {
        name: "xpsr".to_string(),
        bitsize: 32,
        gdb_regnum: GDB_REGNUM_XPSR,
        gdb_alias_regnum: Some(16),
        hw_reg_id: DCRSR_XPSR,
        reg_type: "uint32",
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
            is_pc: false,
        },
        RegisterDescriptor {
            name: "psp".to_string(),
            bitsize: 32,
            gdb_regnum: GDB_REGNUM_PSP,
            gdb_alias_regnum: Some(18),
            hw_reg_id: DCRSR_PSP,
            reg_type: "data_ptr",
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
            is_pc: false,
        });
        features.push(RegisterFeatureGroup {
            name: "org.gnu.gdb.arm.vfp",
            registers: fpu_regs,
        });
    }

    TargetRegisterLayout::new(arch, features)
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

    /// Return `true` if `core_idx` is currently halted.
    fn raw_core_halted(&mut self, core_idx: usize) -> Result<bool, String>;

    /// Single-step one hardware instruction on `core_idx` and return the resulting `PC`.
    fn raw_step_core(&mut self, core_idx: usize) -> Result<u32, String>;

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
            hw.raw_write_memory(core_idx, pc, &orig)?;
            let step_res = hw.raw_step_core(core_idx);
            let _ = hw.raw_write_memory(core_idx, pc, &THUMB_BKPT_0_LE);
            let next_pc = step_res?;
            return Ok(Some(next_pc & !1));
        }
        Ok(None)
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
        if self.cores[1].has_active_hw_breakpoints() {
            let c1_slots = self.cores[1].hw_slots.clone();
            let _ = hw.ensure_hw_breakpoints_active(1, &c1_slots);
        }
        let _ = hw.clear_hw_breakpoint_slot(0, slot_idx, trap_addr);
        self.armed_core0_trap_slot = None;
        let _ = hw.raw_resume_core(0);
        true
    }
}

/// Hardware `probe_rs::Session` implementation of [`DebugBackend`] for ARM Cortex-M targets.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub mod probe_backend {
    use super::*;
    use probe_rs::probe::list::Lister;
    use probe_rs::probe::DebugProbeSelector;
    use probe_rs::{CoreType, MemoryInterface as _, Permissions, RegisterId, Session};
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

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

        fn raw_core_halted(&mut self, core_idx: usize) -> Result<bool, String> {
            with_session_core(self.session, core_idx, |core| {
                core.core_halted().map_err(|e| format!("{:?}", e))
            })
        }

        fn raw_step_core(&mut self, core_idx: usize) -> Result<u32, String> {
            with_session_core(self.session, core_idx, |core| {
                let info = core.step().map_err(|e| format!("{:?}", e))?;
                Ok((info.pc as u32) & !1)
            })
        }

        fn raw_resume_core(&mut self, core_idx: usize) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                if !core.core_halted().unwrap_or(true) {
                    return Ok(());
                }
                core.run().map_err(|e| format!("{:?}", e))
            })
        }

        fn set_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                if let Ok(dhcsr) = core.read_word_32(CORTEX_M_DHCSR_ADDR) {
                    if (dhcsr & 1) == 0 {
                        let _ =
                            core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
                    }
                }
                let _ = core.write_word_32(CORTEX_M_BP_CTRL_ADDR, CORTEX_M_BP_CTRL_KEY_ENABLE);
                let _ = core.flush();
                core.set_hw_breakpoint_unit(slot_idx, addr)
                    .map_err(|e| format!("{:?}", e))
            })
        }

        fn clear_hw_breakpoint_slot(
            &mut self,
            core_idx: usize,
            slot_idx: usize,
            addr: u64,
        ) -> Result<(), String> {
            with_session_core(self.session, core_idx, |core| {
                if core.clear_hw_breakpoint(addr).is_err() {
                    let comp_addr = CORTEX_M_BP_COMP0_ADDR + (slot_idx as u64) * 4;
                    let _ = core.write_word_32(comp_addr, 0);
                    let _ = core.flush();
                }
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
            with_session_core(self.session, core_idx, |core| {
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
                        let _ =
                            core.write_word_32(CORTEX_M_DHCSR_ADDR, CORTEX_M_DHCSR_DBGKEY_ENABLE);
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
            })
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

    /// Concrete ARM Cortex-M `probe-rs` target session implementing [`DebugBackend`].
    pub struct ArmProbeDebugBackend {
        session: Session,
        chip: String,
        core_archs: Vec<ProcessorArch>,
        core_layouts: Vec<TargetRegisterLayout>,
        core0_reset_addr: Option<u64>,
        core1_reset_addr: Option<u64>,
        bp_mgr: ArmBreakpointManager,
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

        /// Open a SWD debug probe session for an ARM `chip` and optionally reset or reset-and-halt Core 0.
        #[allow(clippy::too_many_arguments)]
        pub fn attach(
            chip: &str,
            probe_selector: Option<&str>,
            speed_khz: u32,
            reset: bool,
            halt_on_reset: bool,
            core0_reset_addr: Option<u64>,
            core1_reset_addr: Option<u64>,
            elf_path: Option<&Path>,
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

            Ok(Self {
                session,
                chip: chip.to_string(),
                core_archs,
                core_layouts,
                core0_reset_addr,
                core1_reset_addr,
                bp_mgr: ArmBreakpointManager::new(&bp_units, rp2040_psm_trap),
            })
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
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            hw.raw_core_halted(core_idx)
        }

        fn halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            with_session_core(&mut self.session, core_idx, |core| {
                let info = core
                    .halt(Duration::from_millis(250))
                    .map_err(|e| format!("{:?}", e))?;
                Ok((info.pc as u32) & !1)
            })
        }

        fn resume_core(&mut self, core_idx: usize) -> Result<(), String> {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            if hw.raw_core_halted(core_idx).unwrap_or(false) {
                let _ = self.bp_mgr.step_over_sw_bkpt_if_needed(&mut hw, core_idx);
            }
            let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
            match hw.raw_resume_core(core_idx) {
                Ok(()) => Ok(()),
                Err(_) if core_idx > 0 => Ok(()),
                Err(e) => Err(e),
            }
        }

        fn step_core(&mut self, core_idx: usize) -> Result<u32, String> {
            if !self.core_halted(core_idx).unwrap_or(false) {
                let _ = self.halt_core(core_idx);
            }
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            if let Some(stepped_pc) = self.bp_mgr.step_over_sw_bkpt_if_needed(&mut hw, core_idx)? {
                return Ok(stepped_pc);
            }
            hw.raw_step_core(core_idx)
        }

        fn reset_and_halt_core(&mut self, core_idx: usize) -> Result<u32, String> {
            let entry = if core_idx == 0 {
                self.core0_reset_addr
            } else {
                self.core1_reset_addr
            };
            let pc = Self::halt_core_at_entry(&mut self.session, core_idx, entry)?;
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
            Ok(pc)
        }

        fn reset_and_run_core(&mut self, core_idx: usize) -> Result<(), String> {
            {
                let mut core = self
                    .session
                    .core(core_idx)
                    .map_err(|e| format!("{:?}", e))?;
                core.reset().map_err(|e| format!("{:?}", e))?;
                core.run().map_err(|e| format!("{:?}", e))?;
            }
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            let _ = self.bp_mgr.ensure_armed(&mut hw, core_idx);
            Ok(())
        }

        fn read_core_reg(&mut self, core_idx: usize, hw_reg_id: u16) -> Result<u32, String> {
            with_session_core(&mut self.session, core_idx, |core| {
                core.read_core_reg::<u32>(RegisterId(hw_reg_id))
                    .map_err(|e| format!("{:?}", e))
            })
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
            })
        }

        fn read_memory(
            &mut self,
            core_idx: usize,
            addr: u64,
            buf: &mut [u8],
        ) -> Result<(), String> {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
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
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            hw.raw_write_memory(core_idx, addr, data)
        }

        fn insert_breakpoint(
            &mut self,
            core_idx: usize,
            addr: u64,
            allow_sw_ram: bool,
        ) -> Result<bool, String> {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            self.bp_mgr
                .insert_breakpoint(&mut hw, core_idx, addr, allow_sw_ram)
        }

        fn remove_breakpoint(&mut self, core_idx: usize, addr: u64) -> Result<(), String> {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            self.bp_mgr.remove_breakpoint(&mut hw, core_idx, addr)
        }

        fn clear_all_breakpoints(&mut self, core_idx: usize) -> Result<(), String> {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            self.bp_mgr.clear_all(&mut hw, core_idx)
        }

        fn poll_background_core_maintenance(&mut self, core_idx: usize) -> bool {
            let mut hw = SessionHwAccess {
                session: &mut self.session,
                core_archs: &self.core_archs,
            };
            self.bp_mgr.poll_maintenance(&mut hw, core_idx)
        }
    }
}
