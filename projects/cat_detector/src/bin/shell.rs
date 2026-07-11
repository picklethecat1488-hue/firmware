//! Standalone interactive hardware bringup serial console shell.
//!
//! Provides a real-time command interface over UART0 for sending one-way commands
//! to controllers (fountain, thermal, power) using the embedded-cli parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cat_detector as app;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {embassy_executor::Spawner, embedded_cli::cli::CliBuilder};

#[cfg(all(target_arch = "arm", target_os = "none"))]
use controller::shell_controller::{ShellController, ShellControllerPointers};

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic_with_sizes::<
        { app::FLASH_SIZE },
        { app::STACK_TOP },
        { app::FLASH_START },
        { app::FLASH_END },
        { app::FLASH_WRITE_SIZE },
        { app::FLASH_ERASE_SIZE },
    >(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::fmt::Write as FmtWrite;

#[cfg(all(target_arch = "arm", target_os = "none"))]
controller::declare_shell_commands! {
    CatDetectorCli (CatDetectorCliProcessor) {
        Motor,
        Sensor,
        Fs,
        System,
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
type I2cBus =
    embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
type MotorDevice =
    peripherals::l9110s::L9110s<embassy_rp::gpio::Flex<'static>, embassy_rp::gpio::Flex<'static>>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
type FlashDevice = embassy_rp::flash::Flash<
    'static,
    embassy_rp::peripherals::FLASH,
    embassy_rp::flash::Blocking,
    { app::FLASH_SIZE },
>;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_I2C: Option<*mut I2cBus> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_MOTOR: Option<*mut MotorDevice> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut PANIC_FLASH: Option<FlashDevice> = None;

/// Main application entry point for the bringup shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _ = spawner;
    let p = embassy_rp::init(Default::default());

    // Configure hardware stack guard using Cortex-M MPU
    app::configure_mpu_stack_guard();

    // Initialize board peripherals using the unified board configuration
    let mut board = app::Board::init(p);

    unsafe {
        BOARD_I2C = Some(&mut board.i2c as *mut _ as *mut _);
        BOARD_MOTOR = Some(&mut board.motor as *mut _);
    }

    let writer = firmware_lib::rtt::RttTxWriter;

    let mut cli: embedded_cli::cli::Cli<
        firmware_lib::rtt::RttTxWriter,
        core::convert::Infallible,
        _,
        _,
    > = CliBuilder::default()
        .writer(writer)
        .prompt("\r\nshell> ")
        .build()
        .map_err(|_| ())
        .unwrap();

    // Print welcome text using the CLI's internal writer
    let banner = r#"
       |\      _,,,---,,_
 Zzz   /,`.-'`'    -.  ;-;;,_
      |,4-  ) )-,_. ,\ (  `'-'
     '---''(_/--'  `-'\_)  
"#;
    let _ = cli.write(|writer| {
        let _ = core::writeln!(writer, "{}", banner);
        let _ = core::writeln!(writer, "Type 'help' to print usage.");
        Ok(())
    });

    // Declare statically to avoid stack allocation and stack overflow
    static mut FS_BUF: [u8; 4096] = [0u8; 4096];

    // Initialize the modular panic handler
    let panic_flash = unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(board.flash));
        PANIC_FLASH.as_mut().unwrap()
    };
    let fs_buf = unsafe { &mut FS_BUF };
    app::init_panic_handler(
        panic_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
        fs_buf,
    );

    let temp_sensor_ptr = board.temp_sensor.as_mut().map(|s| s as *mut _);

    let i2c_buses = unsafe {
        &[controller::NamedDevice {
            name: "default",
            device: BOARD_I2C.unwrap(),
        }]
    };
    let motors = unsafe {
        &[controller::NamedDevice {
            name: "default",
            device: BOARD_MOTOR.unwrap(),
        }]
    };
    let flash_partitions = unsafe {
        &[controller::NamedPartition {
            name: "default",
            partition: controller::FlashPartition {
                flash_ptr: PANIC_FLASH.as_mut().unwrap() as *mut _,
                start_address: app::STORAGE_PARTITION_START,
                end_address: app::STORAGE_PARTITION_END,
            },
        }]
    };
    let temp_sensors: &[controller::NamedDevice<_>] = if let Some(sensor) = temp_sensor_ptr {
        &[controller::NamedDevice {
            name: "default",
            device: sensor,
        }]
    } else {
        &[]
    };

    struct AppConfig;
    controller::impl_shell_config! {
        AppConfig {
            MutexRaw: embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            I2c = I2cBus,
            Motor = MotorDevice,
            Flash = FlashDevice,
            TempSensor = cat_detector::Rp2040TempSensor,
        }
    }

    let pointers = ShellControllerPointers::<AppConfig> {
        i2c_buses,
        motors,
        flash_partitions,
        temp_sensors,
        ..Default::default()
    };

    let mut processor = ShellController::<AppConfig>::new(pointers);

    // Run the main input loop feeding bytes to the embedded-cli processor over RTT
    let mut local_proc = CatDetectorCliProcessor::new(&mut processor);
    firmware_lib::rtt::run_rtt_shell_loop::<CatDetectorCli, _, _, _>(&mut cli, &mut local_proc);
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
