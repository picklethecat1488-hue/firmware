//! LED controller to drive NeoPixel or RGB indicators.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use model::interfaces::LedDriver;
use model::types::SystemLedState;

/// A controller that manages status indicator LEDs.
pub struct LedController<D> {
    driver: D,
    current_state: SystemLedState,
}

impl<D: LedDriver> LedController<D> {
    /// Creates a new LedController instance.
    pub const fn new(driver: D) -> Self {
        Self {
            driver,
            current_state: SystemLedState::Off,
        }
    }

    /// Gets the currently set LED state pattern.
    pub fn current_state(&self) -> SystemLedState {
        self.current_state
    }

    /// Sets and executes the LED color pattern.
    pub async fn set_pattern(&mut self, pattern: SystemLedState) -> Result<(), D::Error> {
        self.current_state = pattern;
        match pattern {
            SystemLedState::Off => {
                self.driver.set_color(0, 0, 0)?;
            }
            SystemLedState::SolidGreen => {
                self.driver.set_color(0, 128, 0)?;
            }
            SystemLedState::SolidBlue => {
                self.driver.set_color(0, 0, 64)?;
            }
            SystemLedState::SolidYellow => {
                self.driver.set_color(128, 128, 0)?;
            }
            SystemLedState::SolidOrange => {
                self.driver.set_color(128, 64, 0)?;
            }
            SystemLedState::BlinksRedFourTimes => {
                for _ in 0..4 {
                    self.driver.set_color(255, 0, 0)?;
                    embassy_time::Timer::after_millis(150).await;
                    self.driver.set_color(0, 0, 0)?;
                    embassy_time::Timer::after_millis(150).await;
                }
            }
            SystemLedState::BlinksRedOncePerThirtySeconds => {
                self.driver.set_color(255, 0, 0)?;
                embassy_time::Timer::after_millis(500).await;
                self.driver.set_color(0, 0, 0)?;
            }
        }
        Ok(())
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
                                let _ = self.driver.set_color(255, 0, 0);
                            } else {
                                let _ = self.driver.set_color(0, 0, 0);
                            }
                        }
                    }
                }
                SystemLedState::BlinksRedFourTimes => {
                    // One-shot blinking pattern: Blink 4 times.
                    for _ in 0..4 {
                        let _ = self.driver.set_color(255, 0, 0);
                        embassy_time::Timer::after_millis(150).await;
                        let _ = self.driver.set_color(0, 0, 0);
                        embassy_time::Timer::after_millis(150).await;
                    }
                    // After blinking 4 times, reset state to Off or wait for next command
                    state = SystemLedState::Off;
                    self.current_state = state;
                }
                _ => {
                    // Set color statically
                    match state {
                        SystemLedState::Off => {
                            let _ = self.driver.set_color(0, 0, 0);
                        }
                        SystemLedState::SolidGreen => {
                            let _ = self.driver.set_color(0, 128, 0);
                        }
                        SystemLedState::SolidBlue => {
                            let _ = self.driver.set_color(0, 0, 64);
                        }
                        SystemLedState::SolidYellow => {
                            let _ = self.driver.set_color(128, 128, 0);
                        }
                        SystemLedState::SolidOrange => {
                            let _ = self.driver.set_color(128, 64, 0);
                        }
                        _ => {}
                    }
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
