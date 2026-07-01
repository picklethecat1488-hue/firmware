//! Battery status and telemetry controller.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use model::interfaces::FuelGauge;

/// Trait for waiting on a battery alert pin.
#[allow(async_fn_in_trait)]
pub trait BatteryAlertPin {
    /// Wait for the alert pin to go low (active state).
    async fn wait_for_alert(&mut self);
}

/// A dummy mock implementation of BatteryAlertPin that waits forever.
pub struct DummyAlertPin;

impl BatteryAlertPin for DummyAlertPin {
    async fn wait_for_alert(&mut self) {
        // Sleep forever to let the periodic timeout drive updates
        embassy_time::Timer::after_secs(3600 * 24).await;
    }
}

/// Current operating state of the battery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    /// Battery voltage is normal.
    Ok,
    /// Battery voltage is low.
    Low,
}

/// A controller that periodically monitors battery status and wakes on alerts.
pub struct BatteryController<'a, M: RawMutex, B, Pin = DummyAlertPin, Cmd = ()> {
    battery: &'a Mutex<M, B>,
    state: BatteryState,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    update_fn: Option<fn(u8, bool) -> Cmd>,
    alert_pin: Option<Pin>,
}

impl<'a, M: RawMutex, B: FuelGauge, Cmd: Clone + core::fmt::Debug>
    BatteryController<'a, M, B, DummyAlertPin, Cmd>
{
    /// Creates a new battery controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>) -> Self {
        Self {
            battery,
            state: BatteryState::Ok,
            system_tx: None,
            update_fn: None,
            alert_pin: None,
        }
    }

    /// Creates a new battery controller with system notification capabilities.
    pub fn new_with_system(
        battery: &'a Mutex<M, B>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        update_fn: fn(u8, bool) -> Cmd,
    ) -> Self {
        Self {
            battery,
            state: BatteryState::Ok,
            system_tx: Some(system_tx),
            update_fn: Some(update_fn),
            alert_pin: None,
        }
    }
}

impl<'a, M: RawMutex, B: FuelGauge, Pin: BatteryAlertPin, Cmd: Clone + core::fmt::Debug>
    BatteryController<'a, M, B, Pin, Cmd>
{
    /// Creates a new battery controller with system notification and alert pin support.
    pub fn new_with_system_and_alert(
        battery: &'a Mutex<M, B>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        update_fn: fn(u8, bool) -> Cmd,
        alert_pin: Pin,
    ) -> Self {
        Self {
            battery,
            state: BatteryState::Ok,
            system_tx: Some(system_tx),
            update_fn: Some(update_fn),
            alert_pin: Some(alert_pin),
        }
    }

    /// Gets the current state of the battery.
    pub fn state(&self) -> BatteryState {
        self.state
    }

    /// Updates the battery status by locking and reading the peripheral.
    pub async fn update(
        &mut self,
        telemetry_tx: Option<
            &embassy_sync::channel::Sender<
                '_,
                CriticalSectionRawMutex,
                model::telemetry::TelemetryRecord,
                16,
            >,
        >,
    ) -> Result<(), B::Error> {
        let (voltage, soc) = {
            let mut bat = self.battery.lock().await;
            let v = bat.read_voltage_mv()?;
            let soc = bat.read_state_of_charge().unwrap_or(100);
            (v, soc)
        };
        let charging = false;

        if voltage < 3500 {
            self.state = BatteryState::Low;
        } else {
            self.state = BatteryState::Ok;
        }

        if let (Some(tx), Some(f)) = (&self.system_tx, self.update_fn) {
            tx.try_send(f(soc, charging)).unwrap();
        }

        if let Some(tx) = telemetry_tx {
            let battery_state = match self.state {
                BatteryState::Ok => model::types::BatteryState::Ok,
                BatteryState::Low => model::types::BatteryState::Low,
            };
            let status = model::types::BatteryStatus::VolTempState(voltage, 25000, battery_state);
            let _ = tx.try_send(model::telemetry::TelemetryRecord::Battery(status));
            let _ = tx.try_send(model::telemetry::TelemetryRecord::FuelGauge(
                model::types::FuelGaugeTelemetry::VolSoc(voltage, soc),
            ));
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "Battery Controller: Voltage is {} mV, State: {:?}",
            voltage,
            defmt::Debug2Format(&self.state)
        );

        Ok(())
    }

    /// Starts the controller's main infinite run loop, processing commands.
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, BatteryCommand, 4>,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            16,
        >,
    ) -> ! {
        loop {
            let rx_fut = command_rx.receive();
            let alert_fut = async {
                if let Some(ref mut pin) = self.alert_pin {
                    pin.wait_for_alert().await;
                } else {
                    core::future::pending::<()>().await;
                }
            };
            let timeout_fut = embassy_time::Timer::after(embassy_time::Duration::from_millis(2000));

            match embassy_futures::select::select3(rx_fut, alert_fut, timeout_fut).await {
                // Command received from system shell/console
                embassy_futures::select::Either3::First(cmd) => match cmd {
                    BatteryCommand::CheckStatus => {
                        let _ = self.update(Some(&telemetry_tx)).await;
                    }
                },
                // Fuel gauge alert interrupt triggered
                embassy_futures::select::Either3::Second(_) => {
                    let _ = self.update(Some(&telemetry_tx)).await;
                }
                // Periodic update interval elapsed
                embassy_futures::select::Either3::Third(_) => {
                    let _ = self.update(Some(&telemetry_tx)).await;
                }
            }
        }
    }
}

/// One-way commands sent to the Battery Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryCommand {
    /// Force battery status query and print telemetry logs
    CheckStatus,
}
