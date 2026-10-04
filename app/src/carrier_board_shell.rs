//! Carrier Board 2.0 Diagnostic & Bringup Shell Binary.
//!
//! Provides a minimal entry point that renders the Carrier Board 2.0
//! logo banner defined in the hardware project.

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
    defmt::info!("{}", cat_detector::carrier_board::CARRIER_BOARD_BANNER);

    loop {
        cortex_m::asm::wfi();
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
fn main() {
    print!("{}", cat_detector::carrier_board::CARRIER_BOARD_BANNER);
}
