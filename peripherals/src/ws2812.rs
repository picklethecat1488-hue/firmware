//! Custom WS2812 NeoPixel driver using embassy-rp PIO.

#![deny(missing_docs)]

use model::interfaces::LedDriver;
use model::types::PeripheralError;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    embassy_rp::pio::{Common, Instance, PioPin, StateMachine},
    embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program},
    embassy_rp::Peripheral,
    smart_leds_trait::RGB8,
};

/// Driver for WS2812 addressable LED.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub struct Ws2812<'d, PIO: Instance, const SM: usize> {
    driver: PioWs2812<'d, PIO, SM, 1>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, PIO: Instance, const SM: usize> Ws2812<'d, PIO, SM> {
    /// Creates a new Ws2812 driver instance wrapping the embassy PioWs2812 driver.
    pub fn new(
        common: &mut Common<'d, PIO>,
        sm: StateMachine<'d, PIO, SM>,
        dma: impl Peripheral<P = impl embassy_rp::dma::Channel> + 'd,
        pin: impl PioPin + 'd,
        program: &PioWs2812Program<'d, PIO>,
    ) -> Self {
        let driver = PioWs2812::new(common, sm, dma, pin, program);
        Self { driver }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, PIO: Instance, const SM: usize> LedDriver for Ws2812<'d, PIO, SM> {
    type Error = PeripheralError;

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        let colors = [RGB8 { r, g, b }];
        embassy_futures::block_on(self.driver.write(&colors));
        Ok(())
    }
}

/// Dummy Ws2812 driver for host-compilation.
#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub struct Ws2812 {
    _dummy: (),
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl Ws2812 {
    /// Creates a new dummy Ws2812 instance.
    pub const fn new() -> Self {
        Self { _dummy: () }
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl Default for Ws2812 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
impl LedDriver for Ws2812 {
    type Error = PeripheralError;

    fn set_color(&mut self, _r: u8, _g: u8, _b: u8) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Macro to initialize a WS2812 LED driver during boot.
#[macro_export]
macro_rules! init_ws2812 {
    ($pio:expr, $dma:expr, $pin:expr, $boot_status:expr) => {{
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            use embassy_rp::pio::Pio;
            use embassy_rp::pio_programs::ws2812::PioWs2812Program;
            let Pio {
                mut common, sm0, ..
            } = Pio::new($pio, Irqs);
            let program = PioWs2812Program::new(&mut common);
            $crate::ws2812::Ws2812::new(&mut common, sm0, $dma, $pin, &program)
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        {
            let _ = $boot_status;
            $crate::ws2812::Ws2812::new()
        }
    }};
}
