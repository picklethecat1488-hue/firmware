//! LED controller to drive NeoPixel or RGB indicators.

#![deny(missing_docs)]

use crate::telemetry_controller::LedTelemetryClient;
use crate::{LedReceiver, TelemetrySender};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use model::interfaces::LedDriver;
use model::telemetry::TelemetryClient;
use model::types::PeripheralError;
use model::types::SystemLedState;
use peripherals::ToPeripheralError;

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

impl<D: LedDriver> LedController<D>
where
    <D as LedDriver>::Error: ToPeripheralError,
{
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
    async fn fade_to(
        &mut self,
        from: (u8, u8, u8),
        to: (u8, u8, u8),
    ) -> Result<(), PeripheralError> {
        for step in 1..=FADE_STEPS {
            let r = (from.0 as i32 + (to.0 as i32 - from.0 as i32) * step / FADE_STEPS) as u8;
            let g = (from.1 as i32 + (to.1 as i32 - from.1 as i32) * step / FADE_STEPS) as u8;
            let b = (from.2 as i32 + (to.2 as i32 - from.2 as i32) * step / FADE_STEPS) as u8;
            self.driver
                .set_color(r, g, b)
                .map_err(|e| e.to_peripheral_error())?;
            sleep_ms(FADE_DELAY_MS).await;
        }
        Ok(())
    }

    /// Updates the LED color based on the target pattern, applying fade transitions if enabled.
    pub async fn update_color(
        &mut self,
        pattern: SystemLedState,
        use_fade: bool,
    ) -> Result<(), PeripheralError> {
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
                self.driver
                    .set_color(to.0, to.1, to.2)
                    .map_err(|e| e.to_peripheral_error())?;
            }
            self.current_color = to;
        }
        Ok(())
    }

    /// Sets and executes the LED color pattern.
    #[crate::tracing::instrument(
        name = "led_controller::set_pattern",
        level = "info",
        skip(pattern)
    )]
    pub async fn set_pattern(&mut self, pattern: SystemLedState) -> Result<(), PeripheralError> {
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
        command_rx: LedReceiver<M, SIZE>,
        telemetry_tx: TelemetrySender<
            CriticalSectionRawMutex,
            { crate::telemetry_controller::CHANNEL_CAPACITY },
        >,
    ) -> ! {
        let mut telemetry_client = LedTelemetryClient::new(Some(telemetry_tx));
        let mut state = SystemLedState::Off;
        let mut blink_timer = embassy_time::Instant::now();
        let mut led_on = false;

        // Log the initial state
        telemetry_client.report(state);

        loop {
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
                                if let Err(e) = self
                                    .update_color(
                                        SystemLedState::BlinksRedOncePerThirtySeconds,
                                        false,
                                    )
                                    .await
                                {
                                    telemetry_client.report_error(e);
                                }
                            } else if let Err(e) =
                                self.update_color(SystemLedState::Off, false).await
                            {
                                telemetry_client.report_error(e);
                            }
                        }
                    }
                }
                SystemLedState::BlinksRedFourTimes => {
                    // One-shot blinking pattern: Blink 4 times.
                    for _ in 0..4 {
                        if let Err(e) = self
                            .update_color(SystemLedState::BlinksRedFourTimes, false)
                            .await
                        {
                            telemetry_client.report_error(e);
                        }
                        sleep_ms(150).await;
                        if let Err(e) = self.update_color(SystemLedState::Off, false).await {
                            telemetry_client.report_error(e);
                        }
                        sleep_ms(150).await;
                    }
                    // After blinking 4 times, reset state to Off or wait for next command
                    state = SystemLedState::Off;
                    self.current_state = state;
                }
                _ => {
                    // Set color statically (fading if transitioning to/from Off)
                    let use_fade = matches!(
                        (self.current_state, state),
                        (SystemLedState::Off, SystemLedState::SolidGreen)
                            | (SystemLedState::Off, SystemLedState::SolidBlue)
                            | (SystemLedState::Off, SystemLedState::SolidYellow)
                            | (SystemLedState::Off, SystemLedState::SolidOrange)
                            | (SystemLedState::SolidGreen, SystemLedState::Off)
                            | (SystemLedState::SolidBlue, SystemLedState::Off)
                            | (SystemLedState::SolidYellow, SystemLedState::Off)
                            | (SystemLedState::SolidOrange, SystemLedState::Off)
                    );
                    if let Err(e) = self.update_color(state, use_fade).await {
                        telemetry_client.report_error(e);
                    }
                    let new_cmd = command_rx.receive().await;
                    state = new_cmd;
                    self.current_state = state;
                }
            }

            telemetry_client.report(state);
        }
    }
}

/// Standard config implementation for LedFeature.
pub struct LedFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// LED channel sender
    pub led_tx: Option<crate::LedSender<MutexRaw, N>>,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> LedFeatureConfig<MutexRaw, N> {
    /// Creates a new `LedFeatureConfig`.
    pub fn new(led_tx: Option<crate::LedSender<MutexRaw, N>>) -> Self {
        Self { led_tx }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> crate::SystemFeature<MutexRaw, N>
    for LedFeatureConfig<MutexRaw, N>
{
    fn on_init(&self) {
        if let Some(ref led_tx) = self.led_tx {
            let _ = led_tx.try_send(SystemLedState::Off);
        }
    }

    fn on_state_changed(
        &self,
        _from: model::types::SystemStatus,
        to: model::types::SystemStatus,
        support: crate::DeviceSupport,
        battery_status: Option<crate::BatteryStatus>,
        thermal_critical: bool,
    ) {
        if let Some(ref led_tx) = self.led_tx {
            let led = if support.led {
                if to == model::types::SystemStatus::Active {
                    battery_status
                        .map(|s| s.soc_led_state)
                        .unwrap_or(SystemLedState::Off)
                } else if thermal_critical {
                    SystemLedState::BlinksRedFourTimes
                } else {
                    SystemLedState::SolidBlue
                }
            } else if battery_status.map(|s| s.battery_critical).unwrap_or(false) {
                SystemLedState::BlinksRedOncePerThirtySeconds
            } else if battery_status.map(|s| s.charger_connected).unwrap_or(false) {
                battery_status
                    .map(|s| s.soc_led_state)
                    .unwrap_or(SystemLedState::Off)
            } else {
                SystemLedState::Off
            };
            let _ = led_tx.try_send(led);
        }
    }

    fn on_battery_action(
        &self,
        action: firmware_lib::BatteryUpdateAction,
        status: model::types::SystemStatus,
        battery_status: Option<crate::BatteryStatus>,
    ) {
        if action == firmware_lib::BatteryUpdateAction::ReportSoC {
            if let Some(ref led_tx) = self.led_tx {
                if battery_status.map(|s| s.battery_critical).unwrap_or(false) {
                    let _ = led_tx.try_send(SystemLedState::BlinksRedOncePerThirtySeconds);
                } else if status == model::types::SystemStatus::PowerDown {
                    let led = if battery_status.map(|s| s.charger_connected).unwrap_or(false) {
                        battery_status
                            .map(|s| s.soc_led_state)
                            .unwrap_or(SystemLedState::Off)
                    } else {
                        SystemLedState::Off
                    };
                    let _ = led_tx.try_send(led);
                } else if status == model::types::SystemStatus::Active {
                    if let Some(s) = battery_status {
                        let _ = led_tx.try_send(s.soc_led_state);
                    }
                }
            }
        }
    }

    fn on_alert_triggered(&self, status: model::types::SystemStatus) {
        if status == model::types::SystemStatus::Sleep {
            if let Some(ref led_tx) = self.led_tx {
                let _ = led_tx.try_send(SystemLedState::BlinksRedFourTimes);
            }
        }
    }
}
