//! Concrete driver implementation for digital latching Hall effect speed sensor.

#![deny(missing_docs)]

use crate::tracing;
use embedded_hal::digital::InputPin;
use model::interfaces::SpeedSensor;

/// Default number of Hall pulses per mechanical revolution (6 magnets = 3 pole pairs).
pub const DEFAULT_PULSES_PER_REV: u8 = 3;

/// Default measurement window interval for calculating live RPM in milliseconds (100ms = 10Hz calculation rate).
pub const DEFAULT_RPM_CALC_INTERVAL_MS: u32 = 100;

/// Default timeout in milliseconds with zero detected pulses before motor is considered stopped (0 RPM).
pub const DEFAULT_STOPPED_TIMEOUT_MS: u32 = 500;

/// Driver for a digital latching Hall effect speed sensor (e.g., AH3577, US1881).
///
/// Measures rotational speed contactlessly by counting digital output transitions
/// triggered by alternating magnet poles on the rotating impeller drive hub.
pub struct HallSensor<P> {
    pin: P,
    last_state: bool,
    pulse_count: u32,
    current_rpm: u32,
    pulses_per_rev: u8,
    last_calc_instant: Option<embassy_time::Instant>,
    last_pulse_instant: Option<embassy_time::Instant>,
}

impl<P: InputPin> HallSensor<P> {
    /// Creates a new Hall sensor driver instance with the default 3 pulses per revolution.
    pub fn new(pin: P) -> Self {
        Self::with_pulses_per_rev(pin, DEFAULT_PULSES_PER_REV)
    }

    /// Creates a new Hall sensor driver instance with a custom pulse-per-revolution count.
    pub fn with_pulses_per_rev(pin: P, pulses_per_rev: u8) -> Self {
        Self {
            pin,
            last_state: false,
            pulse_count: 0,
            current_rpm: 0,
            pulses_per_rev: if pulses_per_rev == 0 {
                1
            } else {
                pulses_per_rev
            },
            last_calc_instant: None,
            last_pulse_instant: None,
        }
    }

    /// Polls the Hall sensor pin, detecting rising edges and updating the pulse count.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    pub fn poll_state(&mut self) -> Result<(), P::Error> {
        let is_high = self.pin.is_high()?;
        if is_high && !self.last_state {
            self.pulse_count = self.pulse_count.saturating_add(1);
            self.last_pulse_instant = Some(embassy_time::Instant::now());
        }
        self.last_state = is_high;
        Ok(())
    }

    /// Calculates and updates the current RPM given elapsed milliseconds since the last calculation.
    ///
    /// Clears the accumulated pulse counter upon calculation if new pulses were registered.
    /// If no pulses were registered, preserves the current RPM reading.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    pub fn update_rpm(&mut self, elapsed_ms: u32) {
        if elapsed_ms > 0 && self.pulse_count > 0 {
            let ppr = self.pulses_per_rev as u64;
            let elapsed = elapsed_ms as u64;
            self.current_rpm = ((self.pulse_count as u64 * 60_000) / (ppr * elapsed)) as u32;
            self.pulse_count = 0;
            self.last_calc_instant = Some(embassy_time::Instant::now());
        } else if elapsed_ms > 0 {
            self.last_calc_instant = Some(embassy_time::Instant::now());
        }
    }

    /// Overrides or directly sets the current measured RPM value (useful for testing).
    pub fn set_rpm(&mut self, rpm: u32) {
        self.current_rpm = rpm;
    }

    /// Returns the timestamp of the most recently detected pulse edge.
    pub fn last_pulse_instant(&self) -> Option<embassy_time::Instant> {
        self.last_pulse_instant
    }

    /// Resets the accumulated pulse counter, calculated RPM, and timing state.
    pub fn reset(&mut self) {
        self.pulse_count = 0;
        self.current_rpm = 0;
        self.last_pulse_instant = None;
        self.last_calc_instant = None;
    }

    /// Returns the number of accumulated pulses since the last RPM calculation.
    pub fn pulse_count(&self) -> u32 {
        self.pulse_count
    }

    /// Returns the configured pulses per revolution.
    pub fn pulses_per_rev(&self) -> u8 {
        self.pulses_per_rev
    }
}

impl<P: InputPin> SpeedSensor for HallSensor<P> {
    type Error = P::Error;

    /// Reads the current measured rotational speed in RPM.
    ///
    /// Periodically computes RPM every [`DEFAULT_RPM_CALC_INTERVAL_MS`] based on accumulated Hall pulse edges,
    /// retaining the active RPM reading between slow pulses and resetting to 0 only after [`DEFAULT_STOPPED_TIMEOUT_MS`]
    /// of pulse inactivity.
    #[cfg_attr(
        all(target_arch = "arm", feature = "motor-core"),
        link_section = ".data.core1_func"
    )]
    #[tracing::instrument(core1 = "core1", level = "trace")]
    fn read_rpm(&mut self) -> Result<u32, Self::Error> {
        self.poll_state()?;
        let now = embassy_time::Instant::now();
        if let Some(last) = self.last_calc_instant {
            let elapsed = now.duration_since(last).as_millis() as u32;
            if self.pulse_count > 0 && elapsed >= DEFAULT_RPM_CALC_INTERVAL_MS {
                self.update_rpm(elapsed);
            } else if let Some(last_pulse) = self.last_pulse_instant {
                if now.duration_since(last_pulse).as_millis() as u32 >= DEFAULT_STOPPED_TIMEOUT_MS {
                    self.current_rpm = 0;
                    self.last_calc_instant = Some(now);
                }
            } else if elapsed >= DEFAULT_STOPPED_TIMEOUT_MS {
                self.current_rpm = 0;
                self.last_calc_instant = Some(now);
            }
        } else {
            self.last_calc_instant = Some(now);
        }
        Ok(self.current_rpm)
    }
}
