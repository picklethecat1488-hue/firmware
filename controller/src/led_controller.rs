//! LED controller to drive NeoPixel or RGB indicators.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use model::interfaces::LedDriver;
use model::types::SystemLedState;

#[cfg(all(target_arch = "arm", target_os = "none"))]
async fn sleep_ms(ms: u32) {
    embassy_time::Timer::after_millis(ms as u64).await;
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
async fn sleep_ms(_ms: u32) {}

const FADE_STEPS: i32 = 10;
const FADE_DELAY_MS: u32 = 20;

/// A controller that manages status indicator LEDs.
pub struct LedController<D> {
    driver: D,
    current_state: SystemLedState,
    current_color: (u8, u8, u8),
}

impl<D: LedDriver> LedController<D> {
    /// Creates a new LedController instance.
    pub const fn new(driver: D) -> Self {
        Self {
            driver,
            current_state: SystemLedState::Off,
            current_color: (0, 0, 0),
        }
    }

    /// Gets the currently set LED state pattern.
    pub fn current_state(&self) -> SystemLedState {
        self.current_state
    }

    /// Fades the LED color from one RGB color to another.
    async fn fade_to(&mut self, from: (u8, u8, u8), to: (u8, u8, u8)) -> Result<(), D::Error> {
        for step in 1..=FADE_STEPS {
            let r = (from.0 as i32 + (to.0 as i32 - from.0 as i32) * step / FADE_STEPS) as u8;
            let g = (from.1 as i32 + (to.1 as i32 - from.1 as i32) * step / FADE_STEPS) as u8;
            let b = (from.2 as i32 + (to.2 as i32 - from.2 as i32) * step / FADE_STEPS) as u8;
            self.driver.set_color(r, g, b)?;
            sleep_ms(FADE_DELAY_MS).await;
        }
        Ok(())
    }

    /// Updates the LED color based on the target pattern, applying fade transitions if enabled.
    pub async fn update_color(
        &mut self,
        pattern: SystemLedState,
        use_fade: bool,
    ) -> Result<(), D::Error> {
        let from = self.current_color;
        let to = match pattern {
            SystemLedState::Off => (0, 0, 0),
            SystemLedState::SolidGreen => (0, 128, 0),
            SystemLedState::SolidBlue => (0, 0, 64),
            SystemLedState::SolidYellow => (128, 128, 0),
            SystemLedState::SolidOrange => (128, 64, 0),
            SystemLedState::BlinksRedFourTimes => (255, 0, 0),
            SystemLedState::BlinksRedOncePerThirtySeconds => (255, 0, 0),
        };

        if from != to {
            if use_fade && (from == (0, 0, 0) || to == (0, 0, 0)) {
                self.fade_to(from, to).await?;
            } else {
                self.driver.set_color(to.0, to.1, to.2)?;
            }
            self.current_color = to;
        }
        Ok(())
    }

    /// Sets and executes the LED color pattern.
    pub async fn set_pattern(&mut self, pattern: SystemLedState) -> Result<(), D::Error> {
        let use_fade = matches!(
            (self.current_state, pattern),
            (SystemLedState::Off, SystemLedState::SolidGreen)
                | (SystemLedState::Off, SystemLedState::SolidBlue)
                | (SystemLedState::Off, SystemLedState::SolidYellow)
                | (SystemLedState::Off, SystemLedState::SolidOrange)
                | (SystemLedState::SolidGreen, SystemLedState::Off)
                | (SystemLedState::SolidBlue, SystemLedState::Off)
                | (SystemLedState::SolidYellow, SystemLedState::Off)
                | (SystemLedState::SolidOrange, SystemLedState::Off)
        );
        self.current_state = pattern;
        self.update_color(pattern, use_fade).await
    }

    /// Runs the controller's command processing loop.
    pub async fn run<M: RawMutex, const SIZE: usize>(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, SystemLedState, SIZE>,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            16,
        >,
    ) -> ! {
        let mut state = SystemLedState::Off;
        let mut blink_timer = embassy_time::Instant::now();
        let mut led_on = false;

        // Log the initial state
        let _ = telemetry_tx.try_send(model::telemetry::TelemetryRecord::Led(state));

        loop {
            let prev_state = state;

            match state {
                SystemLedState::BlinksRedOncePerThirtySeconds => {
                    let now = embassy_time::Instant::now();
                    let next_change = if led_on {
                        blink_timer + embassy_time::Duration::from_millis(500)
                    } else {
                        blink_timer + embassy_time::Duration::from_secs(30)
                    };

                    let delay = if next_change > now {
                        next_change - now
                    } else {
                        embassy_time::Duration::from_millis(0)
                    };

                    match embassy_time::with_timeout(delay, command_rx.receive()).await {
                        Ok(new_cmd) => {
                            state = new_cmd;
                            led_on = false;
                            self.current_state = state;
                        }
                        Err(_timeout) => {
                            led_on = !led_on;
                            blink_timer = embassy_time::Instant::now();
                            if led_on {
                                let _ = self
                                    .update_color(
                                        SystemLedState::BlinksRedOncePerThirtySeconds,
                                        false,
                                    )
                                    .await;
                            } else {
                                let _ = self.update_color(SystemLedState::Off, false).await;
                            }
                        }
                    }
                }
                SystemLedState::BlinksRedFourTimes => {
                    // One-shot blinking pattern: Blink 4 times.
                    for _ in 0..4 {
                        let _ = self
                            .update_color(SystemLedState::BlinksRedFourTimes, false)
                            .await;
                        sleep_ms(150).await;
                        let _ = self.update_color(SystemLedState::Off, false).await;
                        sleep_ms(150).await;
                    }
                    // After blinking 4 times, reset state to Off or wait for next command
                    state = SystemLedState::Off;
                    self.current_state = state;
                }
                _ => {
                    // Set color statically (fading if transitioning to/from Off)
                    let use_fade = matches!(
                        (prev_state, state),
                        (SystemLedState::Off, SystemLedState::SolidGreen)
                            | (SystemLedState::Off, SystemLedState::SolidBlue)
                            | (SystemLedState::Off, SystemLedState::SolidYellow)
                            | (SystemLedState::Off, SystemLedState::SolidOrange)
                            | (SystemLedState::SolidGreen, SystemLedState::Off)
                            | (SystemLedState::SolidBlue, SystemLedState::Off)
                            | (SystemLedState::SolidYellow, SystemLedState::Off)
                            | (SystemLedState::SolidOrange, SystemLedState::Off)
                    );
                    let _ = self.update_color(state, use_fade).await;
                    let new_cmd = command_rx.receive().await;
                    state = new_cmd;
                    self.current_state = state;
                }
            }

            if state != prev_state {
                let _ = telemetry_tx.try_send(model::telemetry::TelemetryRecord::Led(state));
            }
        }
    }
}
