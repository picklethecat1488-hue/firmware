//! RP2040 platform support traits and concrete implementations.

use crate::types::{CpuId, MulticoreStack};

/// Trait to manage platform multicore initialization and execution.
pub trait PlatformMulticore {
    /// Gets the CPU ID of the calling core.
    fn current_core_id(&self) -> CpuId;

    /// Boots a secondary core with the specified stack and entry point.
    ///
    /// # Safety
    /// This function must be called only from Core 0 and only once per target core.
    unsafe fn spawn_core<const SIZE: usize>(
        &self,
        core_id: CpuId,
        stack: &'static mut MulticoreStack<SIZE>,
        entry: fn() -> !,
    ) -> Result<(), &'static str>;

    /// Initialize the executor for the specified core.
    ///
    /// # Safety
    /// This function must be called only once and before spawning any tasks on that core.
    unsafe fn init_executor(&self, core_id: CpuId);

    /// Run the executor loop for the specified core.
    /// This function never returns.
    ///
    /// # Safety
    /// This function must be called from the main thread of the corresponding core.
    unsafe fn run_executor(&self, cpu_id: CpuId) -> !;

    /// Get the spawner for the specified core.
    ///
    /// # Safety
    /// This must be called only after the executor for the specified core is initialized.
    unsafe fn spawner(&self, core_id: CpuId) -> embassy_executor::Spawner;
}

/// Trait for platform-wide panic handling.
pub trait PlatformPanic {
    /// Handles a panic on the calling core.
    ///
    /// This captures register states, writes a crash dump to storage,
    /// and resets the system.
    fn handle_panic(&self, info: &core::panic::PanicInfo) -> !;
}

/// Trait for platform I2C bus recovery.
pub trait PlatformI2cRecovery {
    /// Perform a bus recovery sequence to free stuck devices on the bus.
    ///
    /// # Safety
    /// This function steals the pins and therefore must only be called when the I2C peripheral is disabled.
    unsafe fn recover_i2c_bus(&self) -> Result<(), &'static str>;
}

/// Trait for sharing I2C access safely across tasks and cores.
pub trait PlatformI2cAccess {
    /// The error type associated with this I2C bus.
    type Error: embedded_hal::i2c::Error;

    /// The type of I2C bus implementation returned.
    type I2c<'a>: embedded_hal::i2c::I2c<Error = Self::Error>
    where
        Self: 'a;

    /// Get a shared reference to the I2C bus.
    fn get_i2c(&self) -> Self::I2c<'_>;
}

// --- Target Concrete Implementations ---

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct SyncExecutor(embassy_executor::raw::Executor);
#[cfg(all(target_arch = "arm", target_os = "none"))]
unsafe impl Sync for SyncExecutor {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut EXECUTOR_CORE0: Option<SyncExecutor> = None;
#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut EXECUTOR_CORE1: Option<SyncExecutor> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Concrete RP2040 multicore support implementation.
pub struct Rp2040Multicore;

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl PlatformMulticore for Rp2040Multicore {
    fn current_core_id(&self) -> CpuId {
        let sio_base = 0xd0000000usize;
        let cpuid_val = unsafe { core::ptr::read_volatile(sio_base as *const u32) };
        match cpuid_val {
            0 => CpuId::Core0,
            1 => CpuId::Core1,
            _ => panic!("Unknown CPU ID"),
        }
    }

    unsafe fn spawn_core<const SIZE: usize>(
        &self,
        core_id: CpuId,
        stack: &'static mut MulticoreStack<SIZE>,
        entry: fn() -> !,
    ) -> Result<(), &'static str> {
        match core_id {
            CpuId::Core1 => {
                let core1 = embassy_rp::peripherals::CORE1::steal();
                let embassy_stack = unsafe {
                    &mut *(core::ptr::addr_of_mut!(stack.mem)
                        as *mut embassy_rp::multicore::Stack<SIZE>)
                };
                embassy_rp::multicore::spawn_core1(core1, embassy_stack, entry);
                Ok(())
            }
            _ => Err("Invalid target core for spawn"),
        }
    }

    unsafe fn init_executor(&self, core_id: CpuId) {
        match core_id {
            CpuId::Core0 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE0);
                let invalid_ptr = usize::MAX as *mut ();
                *ptr = Some(SyncExecutor(embassy_executor::raw::Executor::new(
                    invalid_ptr,
                )));
            }
            CpuId::Core1 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE1);
                let invalid_ptr = usize::MAX as *mut ();
                *ptr = Some(SyncExecutor(embassy_executor::raw::Executor::new(
                    invalid_ptr,
                )));
            }
        }
    }

    unsafe fn run_executor(&self, cpu_id: CpuId) -> ! {
        use crate::system::CpuScheduler as _;
        match cpu_id {
            CpuId::Core0 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE0);
                if (*ptr).is_none() {
                    self.init_executor(CpuId::Core0);
                }
                let executor_static: &'static embassy_executor::raw::Executor =
                    &(*ptr).as_ref().unwrap().0;
                executor_static.run_loop(cpu_id);
            }
            CpuId::Core1 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE1);
                if let Some(ref mut executor) = *ptr {
                    let executor_static: &'static embassy_executor::raw::Executor = &executor.0;
                    executor_static.run_loop(cpu_id);
                } else {
                    loop {
                        cortex_m::asm::wfe();
                    }
                }
            }
        }
    }

    unsafe fn spawner(&self, core_id: CpuId) -> embassy_executor::Spawner {
        match core_id {
            CpuId::Core0 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE0);
                if (*ptr).is_none() {
                    self.init_executor(CpuId::Core0);
                }
                (*ptr).as_ref().unwrap().0.spawner()
            }
            CpuId::Core1 => {
                let ptr = core::ptr::addr_of_mut!(EXECUTOR_CORE1);
                (*ptr).as_ref().unwrap().0.spawner()
            }
        }
    }
}

/// Concrete RP2040 panic support implementation.
pub struct Rp2040Panic<
    const FLASH_SIZE: usize,
    const FLASH_START: u32,
    const FLASH_END: u32,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
> {
    /// Core 0 stack top address.
    pub core0_stack_top: u32,
    /// Shared atomic reference to the Core 1 stack top address.
    pub core1_stack_top: &'static core::sync::atomic::AtomicU32,
}

impl<
        const FLASH_SIZE: usize,
        const FLASH_START: u32,
        const FLASH_END: u32,
        const WRITE_SIZE: usize,
        const ERASE_SIZE: usize,
    > PlatformPanic for Rp2040Panic<FLASH_SIZE, FLASH_START, FLASH_END, WRITE_SIZE, ERASE_SIZE>
{
    fn handle_panic(&self, info: &core::panic::PanicInfo) -> ! {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let sio_base = 0xd0000000usize;
            let cpuid_val = unsafe { core::ptr::read_volatile(sio_base as *const u32) };
            let (cpuid, stack_top) = match cpuid_val {
                0 => (CpuId::Core0, self.core0_stack_top),
                1 => (
                    CpuId::Core1,
                    self.core1_stack_top
                        .load(core::sync::atomic::Ordering::Relaxed),
                ),
                _ => loop {
                    cortex_m::asm::nop();
                },
            };
            crate::panic_handler::handle_panic_with_sizes::<
                FLASH_SIZE,
                FLASH_START,
                FLASH_END,
                WRITE_SIZE,
                ERASE_SIZE,
            >(info, cpuid, stack_top);
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        {
            let _ = info;
            panic!("Mock panic on host");
        }
    }
}

/// Concrete RP2040 I2C Recovery implementation.
pub struct Rp2040I2cRecovery {
    /// The GPIO pin number used for I2C SDA.
    pub sda_pin: u8,
    /// The GPIO pin number used for I2C SCL.
    pub scl_pin: u8,
}

impl PlatformI2cRecovery for Rp2040I2cRecovery {
    unsafe fn recover_i2c_bus(&self) -> Result<(), &'static str> {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            use embassy_rp::gpio::{Flex, Pull};
            let mut scl = Flex::new(unsafe { embassy_rp::gpio::AnyPin::steal(self.scl_pin) });
            let mut sda = Flex::new(unsafe { embassy_rp::gpio::AnyPin::steal(self.sda_pin) });

            scl.set_as_output();
            scl.set_high();

            sda.set_as_input();
            sda.set_pull(Pull::Up);

            // Give pull-up resistor time to charge bus settle (~8 microseconds)
            embassy_time::block_for(embassy_time::Duration::from_micros(8));

            for _ in 0..16 {
                if sda.is_high() {
                    break;
                }
                scl.set_low();
                embassy_time::block_for(embassy_time::Duration::from_micros(400));
                scl.set_high();
                embassy_time::block_for(embassy_time::Duration::from_micros(400));
            }

            scl.set_pull(Pull::Up);
            sda.set_pull(Pull::Up);
            Ok(())
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        {
            Ok(())
        }
    }
}

/// Concrete RP2040 shared I2C Access wrapper implementation.
#[cfg(all(target_arch = "arm", target_os = "none"))]
impl PlatformI2cAccess
    for &'static embassy_sync::blocking_mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        core::cell::RefCell<crate::i2c::SafeI2c>,
    >
{
    type Error = embassy_rp::i2c::Error;
    type I2c<'a>
        = crate::i2c::SharedI2cWrapper<'a>
    where
        Self: 'a;

    fn get_i2c(&self) -> Self::I2c<'_> {
        crate::i2c::SharedI2cWrapper::new(self)
    }
}

/// Helper macro to generate multicore booting boilerplate for the RP2040 platform.
///
/// This macro defines the Core 1 stack, the atomic `CORE1_STACK_TOP` pointer,
/// the secondary entry point function, and the `boot_core1` bootstrapper.
#[macro_export]
macro_rules! boot_multicore {
    ($board:ty, $stack_size:expr) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Core 1 default stack top address.
        pub const CORE1_DEFAULT_STACK_TOP: u32 = 0x2004_0000;

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        static mut CORE1_STACK: $crate::types::MulticoreStack<$stack_size> =
            $crate::types::MulticoreStack::new();

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Core 1 stack top address.
        pub static CORE1_STACK_TOP: core::sync::atomic::AtomicU32 =
            core::sync::atomic::AtomicU32::new(CORE1_DEFAULT_STACK_TOP);

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Core 1 stack bottom address.
        pub static CORE1_STACK_BOTTOM: core::sync::atomic::AtomicU32 =
            core::sync::atomic::AtomicU32::new(0);

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        fn core1_entry() -> ! {
            unsafe {
                core::arch::asm!(
                    "movs r0, #0",
                    "mov lr, r0",
                    "ldr r0, ={entry}",
                    "bx r0",
                    entry = sym core1_entry_point,
                    options(noreturn)
                );
            }
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        fn core1_entry_point() -> ! {
            unsafe {
                <$board>::run_executor($crate::types::CpuId::Core1);
            }
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Boots Core 1 and starts the RAM executor.
        pub fn boot_core1(_core1: embassy_rp::peripherals::CORE1) {
            use $crate::rp2040::PlatformMulticore as _;
            let stack_ptr = core::ptr::addr_of_mut!(CORE1_STACK);
            let stack_top = unsafe { (*stack_ptr).stack_top() };
            let stack_bottom = unsafe { (*stack_ptr).stack_bottom() };
            CORE1_STACK_TOP.store(stack_top, core::sync::atomic::Ordering::Release);
            CORE1_STACK_BOTTOM.store(stack_bottom, core::sync::atomic::Ordering::Release);

            unsafe {
                <$board>::init_executor_core1();
                let _ = $crate::rp2040::Rp2040Multicore.spawn_core(
                    $crate::types::CpuId::Core1,
                    &mut *stack_ptr,
                    core1_entry,
                );
            }
        }
    };
}

/// Helper macro to declare the RP2040 platform panic handler instance and panic callback override.
#[macro_export]
macro_rules! define_panic_handler {
    ($stack_top:expr, $flash_size:expr, $flash_start:expr, $flash_end:expr, $flash_write_size:expr, $flash_erase_size:expr) => {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Global panic handler instance implementing PlatformPanic.
        pub static PANIC_HANDLER: $crate::rp2040::Rp2040Panic<
            $flash_size,
            $flash_start,
            $flash_end,
            $flash_write_size,
            $flash_erase_size,
        > = $crate::rp2040::Rp2040Panic {
            core0_stack_top: $stack_top,
            core1_stack_top: &CORE1_STACK_TOP,
        };

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        /// Handle a panic, performing multicore checks, resets, and delegating to flash writer.
        pub fn handle_panic(info: &core::panic::PanicInfo) -> ! {
            use $crate::rp2040::PlatformPanic as _;
            PANIC_HANDLER.handle_panic(info);
        }
    };
}
