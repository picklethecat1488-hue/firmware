//! Custom WS2812 NeoPixel driver using embassy-rp PIO.

#![deny(missing_docs)]

use model::interfaces::LedDriver;
use model::types::PeripheralError;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use {
    embassy_rp::pio::{Common, Instance, PioPin, StateMachine},
    embassy_rp::pio_programs::ws2812::PioWs2812Program,
    fixed::types::U24F8,
};

#[cfg(all(target_arch = "arm", target_os = "none"))]
const T1: u8 = 2; // start bit
#[cfg(all(target_arch = "arm", target_os = "none"))]
const T2: u8 = 5; // data bit
#[cfg(all(target_arch = "arm", target_os = "none"))]
const T3: u8 = 3; // stop bit
#[cfg(all(target_arch = "arm", target_os = "none"))]
const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

/// Driver for WS2812 addressable LED.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub struct Ws2812<'d, PIO: Instance, const SM: usize> {
    sm: StateMachine<'d, PIO, SM>,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, PIO: Instance, const SM: usize> Ws2812<'d, PIO, SM> {
    /// Creates a new Ws2812 driver instance using PIO state machine.
    pub fn new(
        common: &mut Common<'d, PIO>,
        mut sm: StateMachine<'d, PIO, SM>,
        pin: impl PioPin + 'd,
        program: &PioWs2812Program<'d, PIO>,
    ) -> Self {
        use embassy_rp::pio::{Config, FifoJoin, ShiftConfig, ShiftDirection};

        let mut cfg = Config::default();
        let out_pin = common.make_pio_pin(pin);
        cfg.set_out_pins(&[&out_pin]);
        cfg.set_set_pins(&[&out_pin]);

        // PioWs2812Program is a newtype struct wrapping LoadedProgram.
        // Since LoadedProgram is its only field, they have the same memory layout.
        // We transmute the reference to bypass the private field access restriction.
        let loaded_program: &embassy_rp::pio::LoadedProgram<'d, PIO> =
            unsafe { core::mem::transmute(program) };
        cfg.use_program(loaded_program, &[&out_pin]);

        let clock_freq = U24F8::from_num(embassy_rp::clocks::clk_sys_freq() / 1000);
        let ws2812_freq = U24F8::from_num(800);
        let bit_freq = ws2812_freq * CYCLES_PER_BIT;
        cfg.clock_divider = clock_freq / bit_freq;

        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 24,
            direction: ShiftDirection::Left,
        };

        sm.set_config(&cfg);
        sm.set_enable(true);

        let mut driver = Self { sm };
        let _ = driver.set_color(0, 0, 0);
        driver
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl<'d, PIO: Instance, const SM: usize> LedDriver for Ws2812<'d, PIO, SM> {
    type Error = PeripheralError;

    fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!("ws2812::set_color({}, {}, {}) start", r, g, b);

        let word = (u32::from(g) << 24) | (u32::from(r) << 16) | (u32::from(b) << 8);
        self.sm.tx().push(word);

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!("ws2812::set_color pushed");

        // WS2812 bit frequency is 800 kHz.
        // 1 bit period = 1.25 microseconds (1250 ns).
        // WS2812 protocol requires at least a 50 microsecond reset period (40 bit times).
        // We wait for 44 bit periods (55 microseconds) to ensure a robust reset.
        let bit_period_ns = 1250; // 1.25 microseconds in nanoseconds
        let reset_duration = embassy_time::Duration::from_nanos(44 * bit_period_ns);
        embassy_time::block_for(reset_duration);

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!("ws2812::set_color end");

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
    ($pio:expr, $pin:expr, $boot_status:expr) => {{
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            use embassy_rp::pio::Pio;
            use embassy_rp::pio_programs::ws2812::PioWs2812Program;

            static mut PIO_COMMON: Option<
                embassy_rp::pio::Common<'static, embassy_rp::peripherals::PIO0>,
            > = None;
            static mut PIO_PROGRAM: Option<
                embassy_rp::pio_programs::ws2812::PioWs2812Program<
                    'static,
                    embassy_rp::peripherals::PIO0,
                >,
            > = None;

            let Pio { common, sm0, .. } = Pio::new($pio, Irqs);

            unsafe {
                PIO_COMMON = Some(core::mem::transmute(common));
                let common_ref = PIO_COMMON.as_mut().unwrap();
                let program = PioWs2812Program::new(common_ref);
                PIO_PROGRAM = Some(core::mem::transmute(program));
                let program_ref = PIO_PROGRAM.as_ref().unwrap();
                $crate::ws2812::Ws2812::new(common_ref, sm0, $pin, program_ref)
            }
        }
        #[cfg(not(all(target_arch = "arm", target_os = "none")))]
        {
            let _ = $boot_status;
            $crate::ws2812::Ws2812::new()
        }
    }};
}
