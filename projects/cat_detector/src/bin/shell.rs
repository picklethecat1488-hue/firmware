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
use {
    defmt_rtt as _, embassy_executor::Spawner, embassy_rp::uart::UartTx,
    embedded_cli::cli::CliBuilder,
};

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
use embedded_io::Write as IoWrite;

/// Helper struct to write formatted strings directly to UART.
#[cfg(all(target_arch = "arm", target_os = "none"))]
struct UartWriter<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> {
    uart: UartTx<'d, T, M>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> embedded_io::ErrorType
    for UartWriter<'d, T, M>
{
    type Error = core::convert::Infallible;
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, T: embassy_rp::uart::Instance, M: embassy_rp::uart::Mode> IoWrite
    for UartWriter<'d, T, M>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, core::convert::Infallible> {
        let _ = self.uart.blocking_write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
}

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
    let writer = UartWriter { uart: tx };

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

    // Register system time function for the panic handler
    app::set_time_fn(app::system_time);

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

    let mut processor =
        app::shell_controller::ShellController::<_, 4, _, _, _, (), (), (), (), _>::new(
            app::MOTOR_CHANNEL.sender(),
            app::SYSTEM_CHANNEL.sender(),
            unsafe { Some(BOARD_I2C.unwrap()) },
            unsafe { Some(BOARD_MOTOR.unwrap()) },
            unsafe { Some(PANIC_FLASH.as_mut().unwrap()) },
            None,
            None,
            None,
            None,
            None,
            None,
            temp_sensor_ptr,
            app::STORAGE_PARTITION_START,
            app::STORAGE_PARTITION_END,
        );

    // Run the main input loop feeding bytes to the embedded-cli processor
    loop {
        let mut rx_byte = [0u8; 1];
        if rx.blocking_read(&mut rx_byte).is_ok() {
            let _ = cli.process_byte::<CliCommand, _>(rx_byte[0], &mut processor);
        }
    }
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
