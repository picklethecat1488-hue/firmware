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
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let entropy = app::get_hw_entropy();
    let micros = app::system_time();
    app::handle_panic_with_sizes::<
        { app::FLASH_SIZE },
        { app::STACK_TOP },
        { app::FLASH_START },
        { app::FLASH_END },
        { app::FLASH_WRITE_SIZE },
        { app::FLASH_ERASE_SIZE },
    >(entropy, micros, info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::fmt::Write as FmtWrite;

// UartWriter definition removed (now provided by firmware_lib::shell::UartWriter)

#[cfg(all(target_arch = "arm", target_os = "none"))]
use app::CliCommand;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_I2C: Option<
    *mut embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Blocking>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut BOARD_MOTOR: Option<
    *mut peripherals::motor::GpioMotor<embassy_rp::gpio::Flex<'static>>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
static mut PANIC_FLASH: Option<
    embassy_rp::flash::Flash<
        'static,
        embassy_rp::peripherals::FLASH,
        embassy_rp::flash::Blocking,
        { app::FLASH_SIZE },
    >,
> = None;

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

    // Extract the motor control pin from the board configuration array
    let motor_pin = board.gpio_pins[app::LED_PIN as usize]
        .take()
        .expect("Motor pin must be available");

    let mut motor = peripherals::motor::GpioMotor::new(motor_pin);

    unsafe {
        BOARD_I2C = Some(&mut board.i2c as *mut _ as *mut _);
        BOARD_MOTOR = Some(&mut motor as *mut _);
    }

    // Split the UART into TX and RX parts to satisfy the borrow checker
    let (tx, mut rx) = board.uart.split();
    let writer = app::uart::UartWriter::new(tx);

    let mut cli = CliBuilder::default()
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

    // Initialize the modular panic handler
    let panic_flash = unsafe {
        PANIC_FLASH = Some(embassy_rp::flash::Flash::new_blocking(board.flash));
        PANIC_FLASH.as_mut().unwrap()
    };
    app::init_panic_handler(
        panic_flash,
        app::STORAGE_PARTITION_START..app::STORAGE_PARTITION_END,
    );

    let temp_sensor_ptr = board.temp_sensor.as_mut().map(|s| s as *mut _);

    let pointers = app::shell_controller::ShellControllerPointers {
        i2c_ptr: unsafe { Some(BOARD_I2C.unwrap()) },
        motor_ptr: unsafe { Some(BOARD_MOTOR.unwrap()) },
        flash_ptr: unsafe { Some(PANIC_FLASH.as_mut().unwrap()) },
        battery_ctrl_ptr: None,
        thermal_ctrl_ptr: None,
        sensor_north_ctrl_ptr: None,
        sensor_east_ctrl_ptr: None,
        sensor_west_ctrl_ptr: None,
        motor_ctrl_ptr: None,
        temp_sensor_ptr,
    };

    let mut processor =
        app::shell_controller::ShellController::<_, 4, _, _, _, (), (), (), (), _>::new(
            app::MOTOR_CHANNEL.sender(),
            app::SYSTEM_CHANNEL.sender(),
            pointers,
            app::STORAGE_PARTITION_START,
            app::STORAGE_PARTITION_END,
        );

    // Run the main input loop feeding bytes to the embedded-cli processor
    app::uart::run_uart_shell_loop::<_, _, CliCommand, _, _, _, _, _>(
        &mut cli,
        &mut rx,
        &mut processor,
    );
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
