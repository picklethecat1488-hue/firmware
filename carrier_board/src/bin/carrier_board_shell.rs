//! Carrier Board 2.0 Diagnostic & Bringup Shell Binary.
//!
//! Provides a trivial "Hello World" entry point that initializes the
//! board state, performs sanity checks on the hardware configuration,
//! and outputs system telemetry over the debug console.

#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_rt::entry;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use panic_probe as _;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use platform as _;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[entry]
fn main() -> ! {
    defmt::info!("======================================================");
    defmt::info!("  CARRIER BOARD 2.0 // BRINGUP & DIAGNOSTIC SHELL");
    defmt::info!("  NXP MCX N947 Dual-Core Arm Cortex-M33 (no_std)");
    defmt::info!("======================================================");
    defmt::info!("{}", carrier_board::hello_world());
    defmt::info!(
        "Hardware self-test: {}",
        carrier_board::run_bringup_self_test()
    );

    loop {
        cortex_m::asm::wfi();
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {
    println!("{}", carrier_board::get_bringup_banner());
    println!("{}", carrier_board::hello_world());
    println!();
    println!("Platform Microcontroller: {}", carrier_board::MCU_ARCH);
    println!(
        "Internal Flash Size:       {} KB",
        carrier_board::FLASH_SIZE_BYTES / 1024
    );
    println!(
        "Internal SRAM Size:        {} KB (ECC Protected)",
        carrier_board::SRAM_SIZE_BYTES / 1024
    );
    println!(
        "External NAND Flash:       {} MB (FlexSPI Port A)",
        carrier_board::EXT_NAND_SIZE_BYTES / (1024 * 1024)
    );
    println!();
    println!("Canonical Components Populated:");
    for comp in carrier_board::CANONICAL_COMPONENTS {
        println!(
            "  - [{:>3}] {:<24} | {:<42} | {}",
            comp.designator, comp.part_number, comp.description, comp.interface
        );
    }
    println!();
    let passed = carrier_board::run_bringup_self_test();
    if passed {
        println!("[PASS] Carrier Board 2.0 configuration self-test successful.");
    } else {
        println!("[FAIL] Carrier Board 2.0 configuration self-test failed!");
        std::process::exit(1);
    }
}
