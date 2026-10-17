//! Concrete driver implementation for the TMC6300 three-phase BLDC motor driver.

#![deny(missing_docs)]

use crate::tracing;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use model::interfaces::{Motor, Tickable};
use model::types::MotorSpeed;

/// Number of steps in standard 3-phase commutation sequence.
pub const COMMUTATION_STEPS: usize = 6;

/// Maximum duty cycle divisor steps for software PWM modulation.
pub const PWM_DUTY_CYCLE_DIVISOR: u8 = 10;

/// Maximum rated peak current for the TMC6300 driver in milliamperes (2A).
pub const MAX_TMC6300_CURRENT_MA: i32 = 2000;

/// Target PWM frequency for PIO duty cycling in Hz (~25 kHz).
pub const PIO_PWM_FREQUENCY_HZ: u32 = 25_000;

/// Number of cycles in one PWM period (resolution).
pub const PWM_PERIOD_CYCLES: u8 = 100;

/// Helper to construct a 32-bit PWM command word for the PIO state machine.
///
/// Bit layout:
/// - Bits 0..7: 6-bit phase mask (GP16..GP21)
/// - Bits 8..15: on-count cycles (duty cycle)
/// - Bits 16..23: off-count cycles (period - duty cycle)
/// - Bits 24..31: unused (0)
pub const fn make_pwm_word(phase_mask: u8, on_count: u8, off_count: u8) -> u32 {
    (phase_mask as u32) | ((on_count as u32) << 8) | ((off_count as u32) << 16)
}

/// Returns the 6-bit phase mask for the given speed and commutation step.
///
/// Ensures strict break-before-make and zero shoot-through across all steps:
/// - Bit 0: UH (GP16)
/// - Bit 1: UL (GP17)
/// - Bit 2: VH (GP18)
/// - Bit 3: VL (GP19)
/// - Bit 4: WH (GP20)
/// - Bit 5: WL (GP21)
pub fn phase_mask(speed: i8, step: u8) -> u8 {
    let step = (step as usize) % COMMUTATION_STEPS;
    if speed > 0 {
        match step {
            0 => 0b001001, // UH High, VL Low, W Float
            1 => 0b100001, // UH High, WL Low, V Float
            2 => 0b100100, // VH High, WL Low, U Float
            3 => 0b000110, // VH High, UL Low, W Float
            4 => 0b010010, // WH High, UL Low, V Float
            _ => 0b011000, // WH High, VL Low, U Float
        }
    } else if speed < 0 {
        match step {
            0 => 0b011000, // U Float, VL Low, WH High
            1 => 0b010010, // UL Low, V Float, WH High
            2 => 0b000110, // UL Low, VH High, W Float
            3 => 0b100100, // U Float, VH High, WL Low
            4 => 0b100001, // UH High, V Float, WL Low
            _ => 0b001001, // UH High, VL Low, W Float
        }
    } else {
        0b000000
    }
}

/// Phase state for 3-phase BLDC commutation.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum PhaseState {
    /// Both high-side and low-side FETs off (floating / high-impedance).
    Float,
    /// High-side FET on, low-side FET off (connected to VM).
    High,
    /// Low-side FET on, high-side FET off (connected to GND).
    Low,
}

/// Driver for the SparkFun TMC6300 low-voltage three-phase brushless (BLDC) motor driver.
///
/// Controls the TMC6300 using 6-PWM control across six half-bridge phase inputs:
/// - Phase U: High-side (`UH`), Low-side (`UL`)
/// - Phase V: High-side (`VH`), Low-side (`VL`)
/// - Phase W: High-side (`WH`), Low-side (`WL`)
///
/// Monitors the active-high hardware diagnostic fault pin (`DIAG`).
pub struct Tmc6300<PUH, PUL, PVH, PVL, PWH, PWL, PD> {
    pin_uh: PUH,
    pin_ul: PUL,
    pin_vh: PVH,
    pin_vl: PVL,
    pin_wh: PWH,
    pin_wl: PWL,
    pin_diag: PD,
    speed: i8,
    commutation_step: u8,
    tick_counter: u8,
}

impl<
        PUH: OutputPin,
        PUL: OutputPin,
        PVH: OutputPin,
        PVL: OutputPin,
        PWH: OutputPin,
        PWL: OutputPin,
        PD: InputPin,
    > Tmc6300<PUH, PUL, PVH, PVL, PWH, PWL, PD>
where
    PUH::Error: core::fmt::Debug,
    PUL::Error: core::fmt::Debug,
    PVH::Error: core::fmt::Debug,
    PVL::Error: core::fmt::Debug,
    PWH::Error: core::fmt::Debug,
    PWL::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    /// Creates a new TMC6300 motor driver instance with 6-PWM phase pins and diagnostic pin.
    pub fn new(
        pin_uh: PUH,
        pin_ul: PUL,
        pin_vh: PVH,
        pin_vl: PVL,
        pin_wh: PWH,
        pin_wl: PWL,
        pin_diag: PD,
    ) -> Self {
        Self {
            pin_uh,
            pin_ul,
            pin_vh,
            pin_vl,
            pin_wh,
            pin_wl,
            pin_diag,
            speed: 0,
            commutation_step: 0,
            tick_counter: 0,
        }
    }

    /// Checks if the hardware diagnostic pin (`DIAG`) is actively asserted (high).
    ///
    /// The TMC6300 asserts DIAG high during overtemperature shutdown, short-circuit,
    /// or undervoltage fault conditions.
    pub fn is_fault_active(&mut self) -> Result<bool, PD::Error> {
        self.pin_diag.is_high()
    }

    /// Sets the state of Phase U enforcing software break-before-make.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_phase_u(
        &mut self,
        state: PhaseState,
    ) -> Tmc6300Result<
        PUH::Error,
        PUL::Error,
        PVH::Error,
        PVL::Error,
        PWH::Error,
        PWL::Error,
        PD::Error,
    > {
        match state {
            PhaseState::Float => {
                self.pin_uh.set_low().map_err(Tmc6300Error::PinUh)?;
                self.pin_ul.set_low().map_err(Tmc6300Error::PinUl)?;
            }
            PhaseState::High => {
                self.pin_ul.set_low().map_err(Tmc6300Error::PinUl)?;
                self.pin_uh.set_high().map_err(Tmc6300Error::PinUh)?;
            }
            PhaseState::Low => {
                self.pin_uh.set_low().map_err(Tmc6300Error::PinUh)?;
                self.pin_ul.set_high().map_err(Tmc6300Error::PinUl)?;
            }
        }
        Ok(())
    }

    /// Sets the state of Phase V enforcing software break-before-make.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_phase_v(
        &mut self,
        state: PhaseState,
    ) -> Tmc6300Result<
        PUH::Error,
        PUL::Error,
        PVH::Error,
        PVL::Error,
        PWH::Error,
        PWL::Error,
        PD::Error,
    > {
        match state {
            PhaseState::Float => {
                self.pin_vh.set_low().map_err(Tmc6300Error::PinVh)?;
                self.pin_vl.set_low().map_err(Tmc6300Error::PinVl)?;
            }
            PhaseState::High => {
                self.pin_vl.set_low().map_err(Tmc6300Error::PinVl)?;
                self.pin_vh.set_high().map_err(Tmc6300Error::PinVh)?;
            }
            PhaseState::Low => {
                self.pin_vh.set_low().map_err(Tmc6300Error::PinVh)?;
                self.pin_vl.set_high().map_err(Tmc6300Error::PinVl)?;
            }
        }
        Ok(())
    }

    /// Sets the state of Phase W enforcing software break-before-make.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_phase_w(
        &mut self,
        state: PhaseState,
    ) -> Tmc6300Result<
        PUH::Error,
        PUL::Error,
        PVH::Error,
        PVL::Error,
        PWH::Error,
        PWL::Error,
        PD::Error,
    > {
        match state {
            PhaseState::Float => {
                self.pin_wh.set_low().map_err(Tmc6300Error::PinWh)?;
                self.pin_wl.set_low().map_err(Tmc6300Error::PinWl)?;
            }
            PhaseState::High => {
                self.pin_wl.set_low().map_err(Tmc6300Error::PinWl)?;
                self.pin_wh.set_high().map_err(Tmc6300Error::PinWh)?;
            }
            PhaseState::Low => {
                self.pin_wh.set_low().map_err(Tmc6300Error::PinWh)?;
                self.pin_wl.set_high().map_err(Tmc6300Error::PinWl)?;
            }
        }
        Ok(())
    }

    /// Applies the 3-phase states for phases U, V, and W.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_phases(
        &mut self,
        u: PhaseState,
        v: PhaseState,
        w: PhaseState,
    ) -> Tmc6300Result<
        PUH::Error,
        PUL::Error,
        PVH::Error,
        PVL::Error,
        PWH::Error,
        PWL::Error,
        PD::Error,
    > {
        self.set_phase_u(u)?;
        self.set_phase_v(v)?;
        self.set_phase_w(w)?;
        Ok(())
    }

    /// Applies the active commutation step based on direction.
    ///
    /// # Errors
    ///
    /// Returns [`Tmc6300Error::ZeroSpeed`] if speed is exactly zero, or pin error if GPIO toggle fails.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    pub fn apply_commutation(
        &mut self,
    ) -> Tmc6300Result<
        PUH::Error,
        PUL::Error,
        PVH::Error,
        PVL::Error,
        PWH::Error,
        PWL::Error,
        PD::Error,
    > {
        let step = (self.commutation_step as usize) % COMMUTATION_STEPS;
        if self.speed > 0 {
            match step {
                0 => self.set_phases(PhaseState::High, PhaseState::Low, PhaseState::Float),
                1 => self.set_phases(PhaseState::High, PhaseState::Float, PhaseState::Low),
                2 => self.set_phases(PhaseState::Float, PhaseState::High, PhaseState::Low),
                3 => self.set_phases(PhaseState::Low, PhaseState::High, PhaseState::Float),
                4 => self.set_phases(PhaseState::Low, PhaseState::Float, PhaseState::High),
                _ => self.set_phases(PhaseState::Float, PhaseState::Low, PhaseState::High),
            }
        } else if self.speed < 0 {
            match step {
                0 => self.set_phases(PhaseState::Float, PhaseState::Low, PhaseState::High),
                1 => self.set_phases(PhaseState::Low, PhaseState::Float, PhaseState::High),
                2 => self.set_phases(PhaseState::Low, PhaseState::High, PhaseState::Float),
                3 => self.set_phases(PhaseState::Float, PhaseState::High, PhaseState::Low),
                4 => self.set_phases(PhaseState::High, PhaseState::Float, PhaseState::Low),
                _ => self.set_phases(PhaseState::High, PhaseState::Low, PhaseState::Float),
            }
        } else {
            Err(Tmc6300Error::ZeroSpeed)
        }
    }
}

impl<
        PUH: OutputPin,
        PUL: OutputPin,
        PVH: OutputPin,
        PVL: OutputPin,
        PWH: OutputPin,
        PWL: OutputPin,
        PD: InputPin,
    > Motor for Tmc6300<PUH, PUL, PVH, PVL, PWH, PWL, PD>
where
    PUH::Error: core::fmt::Debug,
    PUL::Error: core::fmt::Debug,
    PVH::Error: core::fmt::Debug,
    PVL::Error: core::fmt::Debug,
    PWH::Error: core::fmt::Debug,
    PWL::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PinError<PUH, PUL, PVH, PVL, PWH, PWL, PD>;

    /// Sets the motor speed and energizes the driver phases if speed is non-zero.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace", skip(speed))]
    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        let speed_raw = speed.get();
        self.speed = speed_raw;
        self.tick_counter = 0;

        if speed_raw != 0 {
            self.apply_commutation()?;
        } else {
            self.stop()?;
        }
        Ok(())
    }

    /// Stops the motor and places all phase outputs into float (low-power standby mode).
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.tick_counter = 0;
        self.commutation_step = 0;

        self.set_phases(PhaseState::Float, PhaseState::Float, PhaseState::Float)?;
        Ok(())
    }
}

impl<
        PUH: OutputPin,
        PUL: OutputPin,
        PVH: OutputPin,
        PVL: OutputPin,
        PWH: OutputPin,
        PWL: OutputPin,
        PD: InputPin,
    > Tickable for Tmc6300<PUH, PUL, PVH, PVL, PWH, PWL, PD>
where
    PUH::Error: core::fmt::Debug,
    PUL::Error: core::fmt::Debug,
    PVH::Error: core::fmt::Debug,
    PVL::Error: core::fmt::Debug,
    PWH::Error: core::fmt::Debug,
    PWL::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PinError<PUH, PUL, PVH, PVL, PWH, PWL, PD>;

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    async fn tick(&mut self) -> Result<(), Self::Error> {
        // Check diagnostic fault pin first
        if self.pin_diag.is_high().map_err(Tmc6300Error::PinDiag)? {
            tracing::error!("TMC6300 DIAG pin asserted: driver fault detected");
            self.stop()?;
            Err(Tmc6300Error::DriverFault)
        } else {
            let speed = self.speed;
            let abs_speed = speed.abs();
            if abs_speed != 0 {
                // Advance commutation step
                self.commutation_step = (self.commutation_step + 1) % (COMMUTATION_STEPS as u8);

                // Apply PWM duty cycle modulation via tick counter
                if abs_speed >= 100 {
                    self.apply_commutation()?;
                } else {
                    self.tick_counter = (self.tick_counter + 1) % PWM_DUTY_CYCLE_DIVISOR;
                    let threshold = (abs_speed / 10) as u8;
                    if self.tick_counter < threshold {
                        self.apply_commutation()?;
                    } else {
                        // Low duty cycle window: de-energize phases (Float)
                        self.set_phases(PhaseState::Float, PhaseState::Float, PhaseState::Float)?;
                    }
                }
            }
            Ok(())
        }
    }
}

/// Concrete error type alias for TMC6300 driver operations given pin types.
pub type Tmc6300PinError<PUH, PUL, PVH, PVL, PWH, PWL, PD> = Tmc6300Error<
    <PUH as ErrorType>::Error,
    <PUL as ErrorType>::Error,
    <PVH as ErrorType>::Error,
    <PVL as ErrorType>::Error,
    <PWH as ErrorType>::Error,
    <PWL as ErrorType>::Error,
    <PD as ErrorType>::Error,
>;

/// Result alias for TMC6300 driver operations.
pub type Tmc6300Result<EUH, EUL, EVH, EVL, EWH, EWL, ED> =
    Result<(), Tmc6300Error<EUH, EUL, EVH, EVL, EWH, EWL, ED>>;

/// Errors returned by the TMC6300 motor driver.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum Tmc6300Error<EUH, EUL, EVH, EVL, EWH, EWL, ED> {
    /// Error controlling Phase U High pin.
    PinUh(EUH),
    /// Error controlling Phase U Low pin.
    PinUl(EUL),
    /// Error controlling Phase V High pin.
    PinVh(EVH),
    /// Error controlling Phase V Low pin.
    PinVl(EVL),
    /// Error controlling Phase W High pin.
    PinWh(EWH),
    /// Error controlling Phase W Low pin.
    PinWl(EWL),
    /// Error reading the diagnostic (DIAG) pin.
    PinDiag(ED),
    /// Hardware driver fault reported via DIAG assertion.
    DriverFault,
    /// Commutation error when commanded speed is zero.
    ZeroSpeed,
}

/// Errors returned by the TMC6300 PIO motor driver.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
pub enum Tmc6300PioError<ED> {
    /// Error reading the diagnostic (DIAG) pin.
    PinDiag(ED),
    /// Hardware driver fault reported via DIAG assertion.
    DriverFault,
    /// Commutation error when commanded speed is zero.
    ZeroSpeed,
}

/// Pin bundle for the 6-PWM phase control inputs of the TMC6300 BLDC driver.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct Tmc6300PhasePins<PUH, PUL, PVH, PVL, PWH, PWL> {
    /// High-side phase U control pin.
    pub pin_uh: PUH,
    /// Low-side phase U control pin.
    pub pin_ul: PUL,
    /// High-side phase V control pin.
    pub pin_vh: PVH,
    /// Low-side phase V control pin.
    pub pin_vl: PVL,
    /// High-side phase W control pin.
    pub pin_wh: PWH,
    /// Low-side phase W control pin.
    pub pin_wl: PWL,
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
use {
    embassy_rp::pio::{
        Common, Config, Direction, FifoJoin, Instance, InterruptHandler, LoadedProgram, Pio,
        PioPin, ShiftConfig, ShiftDirection, StateMachine,
    },
    fixed::types::U24F8,
};

/// Assembles the 6-pin BLDC PWM duty-cycling PIO program.
///
/// Ensures valid instruction wrapping within the program bounds (offsets 0..=9),
/// with a 100% duty cycle cleanly skipping the off-phase pin de-energization and
/// looping directly back to wrap target without jumping past wrap source into uninitialized SRAM.
pub fn assemble_tmc6300_pio_program() -> pio::Program<32> {
    let mut a: pio::Assembler<32> = pio::Assembler::new();
    let mut wrap_target = a.label();
    let mut wrap_source = a.label();
    let mut skip_on = a.label();
    let mut on_loop = a.label();
    let mut off_loop = a.label();

    a.bind(&mut wrap_target);
    // Non-blocking pull: pop word from TX FIFO into OSR, or copy X into OSR if empty
    a.pull(false, false);
    // Save word into X so next non-blocking pull preserves it
    a.mov(
        pio::MovDestination::X,
        pio::MovOperation::None,
        pio::MovSource::OSR,
    );
    // Shift lower 8 bits (6-bit phase mask) into output pins (GP16..GP21)
    a.out(pio::OutDestination::PINS, 8);
    // Shift next 8 bits (on_count) into Y
    a.out(pio::OutDestination::Y, 8);
    // If on_count == 0, skip ON delay loop
    a.jmp(pio::JmpCondition::YIsZero, &mut skip_on);
    a.bind(&mut on_loop);
    a.jmp(pio::JmpCondition::YDecNonZero, &mut on_loop);
    a.bind(&mut skip_on);
    // Shift next 8 bits (off_count) into Y
    a.out(pio::OutDestination::Y, 8);
    // If off_count == 0, skip OFF delay loop (jump directly to wrap_target)
    a.jmp(pio::JmpCondition::YIsZero, &mut wrap_target);
    // Turn OFF all phase pins (write 0 to OUT pins)
    a.mov(
        pio::MovDestination::PINS,
        pio::MovOperation::None,
        pio::MovSource::NULL,
    );
    a.bind(&mut off_loop);
    a.jmp(pio::JmpCondition::YDecNonZero, &mut off_loop);
    a.bind(&mut wrap_source);

    a.assemble_with_wrap(wrap_source, wrap_target)
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
/// PIO program for 6-pin BLDC PWM duty-cycling.
pub struct PioTmc6300Program<'a, PIO: Instance> {
    prg: LoadedProgram<'a, PIO>,
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
impl<'a, PIO: Instance> PioTmc6300Program<'a, PIO> {
    /// Loads the 6-pin BLDC PWM duty-cycling program into the given PIO block.
    pub fn new(common: &mut Common<'a, PIO>) -> Self {
        let prg = assemble_tmc6300_pio_program();
        let prg = common.load_program(&prg);
        Self { prg }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
/// Driver for the SparkFun TMC6300 BLDC motor driver using RP2040 PIO state machine.
pub struct Tmc6300Pio<'d, PIO: Instance, const SM: usize, PD: InputPin> {
    sm: StateMachine<'d, PIO, SM>,
    pin_diag: PD,
    speed: i8,
    commutation_step: u8,
    duty_cycle: u8,
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
impl<'d, PD: InputPin> Tmc6300Pio<'d, embassy_rp::peripherals::PIO1, 0, PD>
where
    PD::Error: core::fmt::Debug,
{
    /// Creates and configures the TMC6300 PIO driver using PIO1 and state machine 0.
    #[allow(static_mut_refs, clippy::missing_transmute_annotations)]
    pub fn new_pio<PUH, PUL, PVH, PVL, PWH, PWL, IRQ>(
        pio: embassy_rp::peripherals::PIO1,
        pins: Tmc6300PhasePins<PUH, PUL, PVH, PVL, PWH, PWL>,
        pin_diag: PD,
        irq: IRQ,
    ) -> Self
    where
        PUH: PioPin + 'd,
        PUL: PioPin + 'd,
        PVH: PioPin + 'd,
        PVL: PioPin + 'd,
        PWH: PioPin + 'd,
        PWL: PioPin + 'd,
        IRQ: embassy_rp::interrupt::typelevel::Binding<
                <embassy_rp::peripherals::PIO1 as Instance>::Interrupt,
                InterruptHandler<embassy_rp::peripherals::PIO1>,
            > + 'd,
    {
        static mut PIO_COMMON: Option<Common<'static, embassy_rp::peripherals::PIO1>> = None;
        static mut PIO_PROGRAM: Option<PioTmc6300Program<'static, embassy_rp::peripherals::PIO1>> =
            None;

        let Pio { common, sm0, .. } = Pio::new(pio, irq);

        unsafe {
            PIO_COMMON = Some(core::mem::transmute(common));
            let common_ref: &mut Common<'d, embassy_rp::peripherals::PIO1> =
                core::mem::transmute(PIO_COMMON.as_mut().unwrap());
            let program = PioTmc6300Program::new(common_ref);
            PIO_PROGRAM = Some(core::mem::transmute(program));
            let program_ref: &PioTmc6300Program<'d, embassy_rp::peripherals::PIO1> =
                core::mem::transmute(PIO_PROGRAM.as_ref().unwrap());
            Self::new(common_ref, sm0, pins, pin_diag, program_ref)
        }
    }
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Tmc6300Pio<'d, PIO, SM, PD>
where
    PD::Error: core::fmt::Debug,
{
    /// Creates a new TMC6300 PIO driver using pre-allocated state machine and program.
    pub fn new<PUH, PUL, PVH, PVL, PWH, PWL>(
        common: &mut Common<'d, PIO>,
        mut sm: StateMachine<'d, PIO, SM>,
        pins: Tmc6300PhasePins<PUH, PUL, PVH, PVL, PWH, PWL>,
        pin_diag: PD,
        program: &PioTmc6300Program<'d, PIO>,
    ) -> Self
    where
        PUH: PioPin + 'd,
        PUL: PioPin + 'd,
        PVH: PioPin + 'd,
        PVL: PioPin + 'd,
        PWH: PioPin + 'd,
        PWL: PioPin + 'd,
    {
        let mut cfg = Config::default();

        let p16 = common.make_pio_pin(pins.pin_uh);
        let p17 = common.make_pio_pin(pins.pin_ul);
        let p18 = common.make_pio_pin(pins.pin_vh);
        let p19 = common.make_pio_pin(pins.pin_vl);
        let p20 = common.make_pio_pin(pins.pin_wh);
        let p21 = common.make_pio_pin(pins.pin_wl);

        let pins = [&p16, &p17, &p18, &p19, &p20, &p21];
        cfg.set_out_pins(&pins);

        cfg.use_program(&program.prg, &[]);

        let clock_freq = U24F8::from_num(embassy_rp::clocks::clk_sys_freq() / 1000);
        let target_freq =
            U24F8::from_num((PIO_PWM_FREQUENCY_HZ / 1000) * (PWM_PERIOD_CYCLES as u32));
        cfg.clock_divider = clock_freq / target_freq;

        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: false,
            threshold: 32,
            direction: ShiftDirection::Right,
        };

        sm.set_pin_dirs(Direction::Out, &pins);
        sm.set_config(&cfg);
        sm.set_enable(true);

        sm.tx().push(make_pwm_word(0, 0, PWM_PERIOD_CYCLES));

        Self {
            sm,
            pin_diag,
            speed: 0,
            commutation_step: 0,
            duty_cycle: 0,
        }
    }

    /// Checks if the hardware diagnostic pin (`DIAG`) is actively asserted (high).
    pub fn is_fault_active(&mut self) -> Result<bool, PD::Error> {
        self.pin_diag.is_high()
    }

    /// Returns the active commanded speed (-100..100).
    pub fn speed(&self) -> i8 {
        self.speed
    }

    /// Returns the active PWM duty cycle percentage (0..100).
    pub fn duty_cycle(&self) -> u8 {
        self.duty_cycle
    }

    /// Returns the current commutation step index (0..5).
    pub fn commutation_step(&self) -> u8 {
        self.commutation_step
    }
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Motor for Tmc6300Pio<'d, PIO, SM, PD>
where
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PioError<PD::Error>;

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace", skip(speed))]
    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        let speed_raw = speed.get();
        self.speed = speed_raw;
        if speed_raw != 0 {
            self.duty_cycle = speed_raw.unsigned_abs().min(100);
        } else {
            self.stop()?;
        }
        Ok(())
    }

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.duty_cycle = 0;
        self.commutation_step = 0;
        let word = make_pwm_word(0, 0, PWM_PERIOD_CYCLES);
        self.sm.tx().push(word);
        Ok(())
    }
}

#[cfg(all(target_arch = "arm", target_os = "none", target_has_embassy_rp))]
impl<'d, PIO: Instance, const SM: usize, PD: InputPin> Tickable for Tmc6300Pio<'d, PIO, SM, PD>
where
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PioError<PD::Error>;

    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    async fn tick(&mut self) -> Result<(), Self::Error> {
        if self.pin_diag.is_high().map_err(Tmc6300PioError::PinDiag)? {
            tracing::error!("TMC6300 PIO DIAG pin asserted: driver fault detected");
            self.stop()?;
            Err(Tmc6300PioError::DriverFault)
        } else {
            let speed = self.speed;
            if speed != 0 {
                self.commutation_step = (self.commutation_step + 1) % (COMMUTATION_STEPS as u8);
                let mask = phase_mask(speed, self.commutation_step);
                let word =
                    make_pwm_word(mask, self.duty_cycle, PWM_PERIOD_CYCLES - self.duty_cycle);
                self.sm.tx().push(word);
            }
            Ok(())
        }
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none", target_has_embassy_rp)))]
/// Mock PIO-driven TMC6300 motor driver for host testing.
pub struct Tmc6300Pio<PD = ()> {
    pin_diag: PD,
    speed: i8,
    commutation_step: u8,
    duty_cycle: u8,
    last_word: u32,
}

#[cfg(not(all(target_arch = "arm", target_os = "none", target_has_embassy_rp)))]
impl<PD: InputPin> Tmc6300Pio<PD>
where
    PD::Error: core::fmt::Debug,
{
    /// Creates a new mock TMC6300 PIO driver with diagnostic pin.
    pub fn new(pin_diag: PD) -> Self {
        Self {
            pin_diag,
            speed: 0,
            commutation_step: 0,
            duty_cycle: 0,
            last_word: make_pwm_word(0, 0, PWM_PERIOD_CYCLES),
        }
    }

    /// Creates a dummy TMC6300 PIO driver from hardware pins and IRQ.
    pub fn new_pio<PUH, PUL, PVH, PVL, PWH, PWL, IRQ, PIO>(
        _pio: PIO,
        _pins: Tmc6300PhasePins<PUH, PUL, PVH, PVL, PWH, PWL>,
        pin_diag: PD,
        _irq: IRQ,
    ) -> Self {
        Self::new(pin_diag)
    }

    /// Checks if the hardware diagnostic pin (`DIAG`) is actively asserted (high).
    pub fn is_fault_active(&mut self) -> Result<bool, PD::Error> {
        self.pin_diag.is_high()
    }

    /// Returns the active commanded speed (-100..100).
    pub fn speed(&self) -> i8 {
        self.speed
    }

    /// Returns the active PWM duty cycle percentage (0..100).
    pub fn duty_cycle(&self) -> u8 {
        self.duty_cycle
    }

    /// Returns the current commutation step index (0..5).
    pub fn commutation_step(&self) -> u8 {
        self.commutation_step
    }

    /// Returns the last 32-bit PWM command word sent to PIO.
    pub fn last_word(&self) -> u32 {
        self.last_word
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none", target_has_embassy_rp)))]
impl<PD: InputPin> Motor for Tmc6300Pio<PD>
where
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PioError<PD::Error>;

    fn set_speed(&mut self, speed: MotorSpeed) -> Result<(), Self::Error> {
        let raw = speed.get();
        self.speed = raw;
        if raw != 0 {
            self.duty_cycle = raw.unsigned_abs().min(100);
            let mask = phase_mask(self.speed, self.commutation_step);
            self.last_word =
                make_pwm_word(mask, self.duty_cycle, PWM_PERIOD_CYCLES - self.duty_cycle);
        } else {
            self.stop()?;
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.duty_cycle = 0;
        self.commutation_step = 0;
        self.last_word = make_pwm_word(0, 0, PWM_PERIOD_CYCLES);
        Ok(())
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none", target_has_embassy_rp)))]
impl<PD: InputPin> Tickable for Tmc6300Pio<PD>
where
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300PioError<PD::Error>;

    async fn tick(&mut self) -> Result<(), Self::Error> {
        if self.pin_diag.is_high().map_err(Tmc6300PioError::PinDiag)? {
            tracing::error!("TMC6300 PIO DIAG pin asserted: driver fault detected");
            self.stop()?;
            Err(Tmc6300PioError::DriverFault)
        } else {
            let raw = self.speed;
            if raw != 0 {
                self.commutation_step = (self.commutation_step + 1) % (COMMUTATION_STEPS as u8);
                let mask = phase_mask(raw, self.commutation_step);
                self.last_word =
                    make_pwm_word(mask, self.duty_cycle, PWM_PERIOD_CYCLES - self.duty_cycle);
            }
            Ok(())
        }
    }
}
