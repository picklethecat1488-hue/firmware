//! Standalone interactive hardware bringup serial console shell for Carrier Board 2.0.
//!
//! Provides a real-time command interface over RTT / high-speed console UART for dispatching
//! diagnostic commands to Carrier Board 2.0 controllers using the `embedded-cli` parser.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]
#![deny(missing_docs)]
#![allow(static_mut_refs)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cat_detector::carrier_board as app;

board::carrier_board::emit_project_metadata!();

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
    CarrierBoardCli (CarrierBoardCliProcessor)
}

/// Main asynchronous task driving the Carrier Board 2.0 bringup shell on Core 0.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[embassy_executor::task]
async fn carrier_shell_main_task(spawner: Spawner) {
    platform::rtt::set_rtt_boot_state(platform::rtt::TargetBootState::Booting);
    // Initialize board peripherals using the unified Carrier Board 2.0 BSP configuration
    let mut board = app::Board::init().await;

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

    // Print welcome banner using the CLI's internal writer
    let _ = cli.write(|writer| {
        let _ = core::writeln!(writer, "{}", app::CARRIER_BOARD_BANNER);
        let _ = core::writeln!(writer, "Type 'help' to print usage.");
        Ok(())
    });

    // Retrieve peripherals and store board in global OnceLock static
    let peripherals = board.take_peripherals();
    if app::BOARD.set(board).is_err() {
        panic!("Failed to set Carrier Board BOARD static");
    }
    let board_ref = app::BOARD.get().unwrap();

    // Initialize board subcontrollers
    let mut controllers = app::init_controllers(board_ref, peripherals).await;

    // Initialize filesystem storage configuration
    let fs_cfg = app::get_filesystem_config();

    core_monitor::init_core(
        Some(spawner),
        core_monitor::CpuId::Core0,
        app::Board::CORE_MONITOR_TIMEOUT_MS,
        app::Board::CORE_MONITOR_WARN_PCT,
        false,
    );

    // Bind shell controller pointers using the declarative board/app helper
    app::declare_shell_pointers!(fs_cfg.buffer, controllers, board_ref, pointers);

    let mut processor = ShellController::<app::CarrierBoardShellConfig>::new(pointers);

    let mut local_proc = CarrierBoardCliProcessor::new(&mut processor);

    platform::rtt::set_rtt_boot_state(platform::rtt::TargetBootState::Ready);
    platform::run_rtt_shell_loop!(&mut cli, &mut local_proc, CarrierBoardCli, "\r\nshell> ");
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[cortex_m_rt::entry]
fn main() -> ! {
    let spawner_c0 = app::spawner_core0();
    spawner_c0
        .spawn(carrier_shell_main_task(spawner_c0))
        .unwrap();
    app::run_executor(platform::types::CpuId::Core0);
}

/// Host entry point printing the Carrier Board 2.0 banner during host diagnostics.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {
    print!("{}", cat_detector::carrier_board::CARRIER_BOARD_BANNER);
}
