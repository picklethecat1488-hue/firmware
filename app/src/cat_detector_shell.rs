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
use {embassy_executor::Spawner, embedded_cli::cli::CliBuilder, platform::core_monitor};

#[cfg(all(target_arch = "arm", target_os = "none"))]
use controller::shell_controller::ShellController;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    app::handle_panic(info);
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use core::fmt::Write as FmtWrite;

#[cfg(all(target_arch = "arm", target_os = "none"))]
controller::declare_active_shell_commands! {
    CatDetectorCli (CatDetectorCliProcessor)
}

/// Static holder for LED Sender to resolve lifetimes.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub static mut LED_SENDER: Option<
    controller::LedSender<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>,
> = None;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
async fn led_task(led_ctrl: controller::led_controller::LedController<app::LedDevice>) {
    led_ctrl
        .run::<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4>(
            app::LED_CHANNEL.receiver(),
            app::TELEMETRY_CHANNEL.sender(),
        )
        .await;
}

/// Main application entry point for the bringup shell.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Initialize board peripherals using the unified board configuration
    let mut board = app::Board::init(p).await;

    let writer = platform::rtt::RttTxWriter;

    let mut cli: embedded_cli::cli::Cli<
        platform::rtt::RttTxWriter,
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

    // Retrieve peripherals and store board in global OnceLock static
    let peripherals = board.take_peripherals();
    if app::BOARD.set(board).is_err() {
        panic!("Failed to set BOARD static");
    }
    let board_ref = app::BOARD.get().unwrap();

    // Initialize board peripherals and subcontrollers
    let mut controllers = app::init_controllers(board_ref, peripherals).await;

    // Initialize panic diagnostics and filesystem storage
    let fs_cfg = app::get_filesystem_config();

    core_monitor::init_core(
        Some(spawner),
        core_monitor::CpuId::Core0,
        app::CORE_MONITOR_TIMEOUT_MS,
        app::CORE_MONITOR_WARN_PCT,
        false,
    );

    // Retrieve pointers using the platformitized helper
    app::declare_shell_pointers!(fs_cfg.buffer, controllers, board_ref, pointers);

    let mut processor = ShellController::<app::CatDetectorShellConfig>::new(pointers);

    let mut local_proc = CatDetectorCliProcessor::new(&mut processor);

    platform::run_rtt_shell_loop!(&mut cli, &mut local_proc, CatDetectorCli);
}

/// Dummy host entry point to satisfy Cargo compilation requirements.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {}
