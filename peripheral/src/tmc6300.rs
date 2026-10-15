//! Concrete driver implementation for the TMC6300 three-phase BLDC motor driver.

#![deny(missing_docs)]

use crate::tracing;
use embedded_hal::digital::{InputPin, OutputPin};
use model::interfaces::{Motor, Tickable};
use model::types::MotorSpeed;

/// Number of steps in standard 3-phase commutation sequence.
pub const COMMUTATION_STEPS: usize = 6;

/// Driver for the SparkFun TMC6300 low-voltage three-phase brushless (BLDC) motor driver.
///
/// Controls the TMC6300 using an active-high enable pin (`EN`), three half-bridge phase inputs
/// (`IN1` -> U, `IN2` -> V, `IN3` -> W), and monitors the active-high diagnostic fault pin (`DIAG`).
pub struct Tmc6300<PE, P1, P2, P3, PD> {
    pin_en: PE,
    pin_in1: P1,
    pin_in2: P2,
    pin_in3: P3,
    pin_diag: PD,
    speed: i8,
    commutation_step: u8,
    tick_counter: u8,
}

impl<PE: OutputPin, P1: OutputPin, P2: OutputPin, P3: OutputPin, PD: InputPin>
    Tmc6300<PE, P1, P2, P3, PD>
where
    PE::Error: core::fmt::Debug,
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
    P3::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    /// Creates a new TMC6300 motor driver instance.
    pub fn new(pin_en: PE, pin_in1: P1, pin_in2: P2, pin_in3: P3, pin_diag: PD) -> Self {
        Self {
            pin_en,
            pin_in1,
            pin_in2,
            pin_in3,
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

    /// Applies the 3-phase state for the given commutation step.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn set_phase_outputs(
        &mut self,
        u: bool,
        v: bool,
        w: bool,
    ) -> Tmc6300Result<PE::Error, P1::Error, P2::Error, P3::Error, PD::Error> {
        if u {
            self.pin_in1.set_high().map_err(Tmc6300Error::PinIn1)?;
        } else {
            self.pin_in1.set_low().map_err(Tmc6300Error::PinIn1)?;
        }

        if v {
            self.pin_in2.set_high().map_err(Tmc6300Error::PinIn2)?;
        } else {
            self.pin_in2.set_low().map_err(Tmc6300Error::PinIn2)?;
        }

        if w {
            self.pin_in3.set_high().map_err(Tmc6300Error::PinIn3)?;
        } else {
            self.pin_in3.set_low().map_err(Tmc6300Error::PinIn3)?;
        }

        Ok(())
    }

    /// Applies the active commutation step based on direction.
    #[allow(clippy::type_complexity)]
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    fn apply_commutation(
        &mut self,
    ) -> Tmc6300Result<PE::Error, P1::Error, P2::Error, P3::Error, PD::Error> {
        let step = if self.speed >= 0 {
            self.commutation_step % 6
        } else {
            (5 - (self.commutation_step % 6)) % 6
        };

        match step {
            0 => self.set_phase_outputs(true, false, false),
            1 => self.set_phase_outputs(true, true, false),
            2 => self.set_phase_outputs(false, true, false),
            3 => self.set_phase_outputs(false, true, true),
            4 => self.set_phase_outputs(false, false, true),
            _ => self.set_phase_outputs(true, false, true),
        }
    }
}

impl<PE: OutputPin, P1: OutputPin, P2: OutputPin, P3: OutputPin, PD: InputPin> Motor
    for Tmc6300<PE, P1, P2, P3, PD>
where
    PE::Error: core::fmt::Debug,
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
    P3::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300Error<PE::Error, P1::Error, P2::Error, P3::Error, PD::Error>;

    /// Sets the motor speed and enables the driver if speed is non-zero.
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
            // Enable driver power stage
            self.pin_en.set_high().map_err(Tmc6300Error::PinEn)?;
            self.apply_commutation()?;
        } else {
            self.stop()?;
        }
        Ok(())
    }

    /// Stops the motor and places the TMC6300 into low-power standby mode (< 1 µA).
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn stop(&mut self) -> Result<(), Self::Error> {
        self.speed = 0;
        self.tick_counter = 0;
        self.commutation_step = 0;

        // Disable enable pin to put TMC6300 into standby
        self.pin_en.set_low().map_err(Tmc6300Error::PinEn)?;
        self.set_phase_outputs(false, false, false)?;
        Ok(())
    }
}

impl<PE: OutputPin, P1: OutputPin, P2: OutputPin, P3: OutputPin, PD: InputPin> Tickable
    for Tmc6300<PE, P1, P2, P3, PD>
where
    PE::Error: core::fmt::Debug,
    P1::Error: core::fmt::Debug,
    P2::Error: core::fmt::Debug,
    P3::Error: core::fmt::Debug,
    PD::Error: core::fmt::Debug,
{
    type Error = Tmc6300Error<PE::Error, P1::Error, P2::Error, P3::Error, PD::Error>;

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
                    self.tick_counter = (self.tick_counter + 1) % 10;
                    let threshold = (abs_speed / 10) as u8;
                    if self.tick_counter < threshold {
                        self.pin_en.set_high().map_err(Tmc6300Error::PinEn)?;
                        self.apply_commutation()?;
                    } else {
                        // Low duty cycle window: de-energize phases
                        self.set_phase_outputs(false, false, false)?;
                    }
                }
            }
            Ok(())
        }
    }
}

/// Result alias for TMC6300 driver operations.
pub type Tmc6300Result<EE, E1, E2, E3, ED> = Result<(), Tmc6300Error<EE, E1, E2, E3, ED>>;

/// Errors returned by the TMC6300 motor driver.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum Tmc6300Error<EE, E1, E2, E3, ED> {
    /// Error controlling the enable (EN) pin.
    PinEn(EE),
    /// Error controlling the Phase U input (IN1) pin.
    PinIn1(E1),
    /// Error controlling the Phase V input (IN2) pin.
    PinIn2(E2),
    /// Error controlling the Phase W input (IN3) pin.
    PinIn3(E3),
    /// Error reading the diagnostic (DIAG) pin.
    PinDiag(ED),
    /// Hardware driver fault reported via DIAG assertion.
    DriverFault,
}
