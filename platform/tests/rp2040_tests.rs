#![cfg(feature = "rp2040")]

use platform::rp2040::{PlatformI2cRecovery, PlatformMulticore, Rp2040I2cRecovery, Rp2040Panic};
use platform::types::{CpuId, MulticoreStack};
use std::sync::atomic::AtomicU32;

#[test]
fn test_multicore_stack_layout() {
    let stack: MulticoreStack<1024> = MulticoreStack::new();
    let base_addr = &stack as *const _ as u32;
    let expected_top = base_addr + 4096;
    assert_eq!(stack.stack_top(), expected_top);
    assert_eq!(stack.stack_bottom(), base_addr);
}

#[test]
fn test_multicore_stack_default() {
    let stack: MulticoreStack<1024> = Default::default();
    let base_addr = &stack as *const _ as u32;
    let expected_top = base_addr + 4096;
    assert_eq!(stack.stack_top(), expected_top);
    assert_eq!(stack.stack_bottom(), base_addr);
}

#[test]
fn test_host_mock_i2c_recovery() {
    let recovery = Rp2040I2cRecovery {
        sda_pin: 12,
        scl_pin: 13,
    };
    let result = unsafe { recovery.recover_i2c_bus() };
    assert!(result.is_ok());
}

static CORE1_STACK_TOP_MOCK: AtomicU32 = AtomicU32::new(0x20040000);

struct MockMulticore;

impl PlatformMulticore for MockMulticore {
    fn current_core_id(&self) -> CpuId {
        CpuId::Core0
    }

    unsafe fn spawn_core<const SIZE: usize>(
        &self,
        core_id: CpuId,
        stack: &'static mut MulticoreStack<SIZE>,
        entry: fn() -> !,
    ) -> Result<(), &'static str> {
        let _ = core_id;
        let _ = stack;
        let _ = entry;
        Ok(())
    }

    unsafe fn init_executor(&self, core_id: CpuId) {
        let _ = core_id;
    }

    unsafe fn run_executor(&self, cpu_id: CpuId) -> ! {
        let _ = cpu_id;
        panic!("MOCKED EXECUTOR");
    }

    unsafe fn spawner(&self, core_id: CpuId) -> embassy_executor::Spawner {
        let _ = core_id;
        panic!("MOCKED SPAWNER");
    }
}

#[test]
fn test_multicore_trait_implementation() {
    let mc = MockMulticore;
    assert!(mc.current_core_id() == CpuId::Core0);
}

#[test]
fn test_panic_handler_instantiation() {
    let panic_handler = Rp2040Panic::<1024, 0x10000000, 0x10100000, 1, 4096> {
        core0_stack_top: 0x20042000,
        core1_stack_top: &CORE1_STACK_TOP_MOCK,
    };
    assert_eq!(panic_handler.core0_stack_top, 0x20042000);
}
