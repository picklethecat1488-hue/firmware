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
