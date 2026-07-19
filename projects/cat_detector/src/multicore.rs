//! Multicore and execution support for Core 1 bring-up, CLI routing, and panic handling.

#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use crate::{
    MOTOR_CHANNEL, SENSOR_EAST_CHANNEL, SENSOR_NORTH_CHANNEL, SENSOR_WEST_CHANNEL,
    TELEMETRY_CHANNEL,
};
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_executor::Spawner;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use embassy_sync::channel::Channel;

#[cfg(all(target_arch = "arm", target_os = "none"))]
struct SyncExecutor(embassy_executor::raw::Executor);

#[cfg(all(target_arch = "arm", target_os = "none"))]
unsafe impl Sync for SyncExecutor {}

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut CORE1_STACK: embassy_rp::multicore::Stack<4096> = embassy_rp::multicore::Stack::new();

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut EXECUTOR_CORE1: Option<SyncExecutor> = None;

/// Global pointer to the active MotorController on Core 1 (populated during startup).
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut MOTOR_CTRL_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active North SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_NORTH_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active East SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_EAST_PTR: *mut () = core::ptr::null_mut();

/// Global pointer to the active West SensorController on Core 1.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[allow(dead_code)]
pub static mut SENSOR_WEST_PTR: *mut () = core::ptr::null_mut();

/// Core 1 execution commands sent from Core 0 shell or orchestrator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Core1Command {
    /// Request Core 1 to panic.
    Panic,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Global channel for sending commands to Core 1 tasks.
pub static CORE1_COMMAND_CHANNEL: Channel<CriticalSectionRawMutex, Core1Command, 4> =
    Channel::new();

#[cfg(all(target_arch = "arm", target_os = "none"))]
type MotorType =
    controller::motor_controller::MotorController<crate::MotorDevice, crate::CurrentSensorDevice>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
type SensorType = controller::sensor_controller::SensorController<
    'static,
    crate::ProximitySensorDevice,
    CriticalSectionRawMutex,
    crate::DataReadyPinType,
    crate::SystemCommand,
    controller::sensor_controller::ProximityReader,
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
async fn bootstrap_core1_task(
    spawner: Spawner,
    mut motor: MotorType,
    mut sensors: (SensorType, SensorType, SensorType),
) {
    // Spawn the Core 1 command task
    spawner
        .spawn(core1_command_task(CORE1_COMMAND_CHANNEL.receiver()))
        .unwrap();

    unsafe {
        MOTOR_CTRL_PTR = &mut motor as *mut _ as *mut ();
        SENSOR_NORTH_PTR = &mut sensors.0 as *mut _ as *mut ();
        SENSOR_EAST_PTR = &mut sensors.1 as *mut _ as *mut ();
        SENSOR_WEST_PTR = &mut sensors.2 as *mut _ as *mut ();
    }

    controller::spawn_controllers! {
        spawner,
        telemetry: TELEMETRY_CHANNEL,
        controllers: {
            Motor(motor, MOTOR_CHANNEL), generics: (crate::MotorDevice, crate::CurrentSensorDevice),
            Sensor(sensors.0, SENSOR_NORTH_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.1, SENSOR_EAST_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
            Sensor(sensors.2, SENSOR_WEST_CHANNEL), generics: (crate::ProximitySensorDevice, crate::DataReadyPinType, crate::SystemCommand),
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
#[allow(clippy::never_loop)]
async fn core1_command_task(
    rx: embassy_sync::channel::Receiver<'static, CriticalSectionRawMutex, Core1Command, 4>,
) {
    loop {
        let cmd = rx.receive().await;
        match cmd {
            Core1Command::Panic => {
                panic!("Simulated Core 1 panic");
            }
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Boots Core 1 and starts the RAM executor with the given controllers.
pub fn boot_core1(
    core1: embassy_rp::peripherals::CORE1,
    motor: MotorType,
    sensors: (SensorType, SensorType, SensorType),
) {
    unsafe {
        EXECUTOR_CORE1 = Some(SyncExecutor(embassy_executor::raw::Executor::new(
            !0 as *mut (),
        )));
        firmware_lib::panic_handler::CORE1_STACK_TOP =
            core::ptr::addr_of!(CORE1_STACK) as u32 + 4096;
    }

    let executor_c1 = unsafe { EXECUTOR_CORE1.as_ref().unwrap() };

    embassy_rp::multicore::spawn_core1(core1, unsafe { &mut CORE1_STACK }, move || {
        let spawner_c1 = executor_c1.0.spawner();

        spawner_c1
            .spawn(bootstrap_core1_task(spawner_c1, motor, sensors))
            .unwrap();

        loop {
            unsafe {
                executor_c1.0.poll();
                defmt::trace!("ctx=cpu_idle_c1 parent=0 span_enter: CPU Idle Core 1");
                cortex_m::asm::wfe();
                defmt::trace!("cpu_idle_c1 span_exit: CPU Idle Core 1");
            }
        }
    });
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
/// Handle a panic, performing multicore checks, resets, and delegating to flash writer.
pub fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    let cpuid = unsafe { core::ptr::read_volatile(0xd0000000 as *const u32) };
    if cpuid == 1 {
        // Force Core 0 (proc0) into reset to prevent concurrent flash operations
        const PSM_FRCE_OFF: *mut u32 = 0x40010004 as *mut u32;
        unsafe {
            let val = core::ptr::read_volatile(PSM_FRCE_OFF);
            core::ptr::write_volatile(PSM_FRCE_OFF, val | (1 << 15));
        }
    }

    let actual_stack_top = if cpuid == 1 {
        let top = unsafe { firmware_lib::panic_handler::CORE1_STACK_TOP };
        if top != 0 {
            top
        } else {
            crate::STACK_TOP
        }
    } else {
        crate::STACK_TOP
    };

    crate::handle_panic_with_sizes::<
        { crate::FLASH_SIZE },
        { crate::FLASH_START },
        { crate::FLASH_END },
        { crate::FLASH_WRITE_SIZE },
        { crate::FLASH_ERASE_SIZE },
    >(info, actual_stack_top, cpuid);
}
