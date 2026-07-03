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
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), derive(defmt::Format))]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BatteryState {
    /// Battery voltage is normal.
    Ok,
    /// Battery voltage is low.
    Low,
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
impl core::fmt::Debug for BatteryState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("BatteryState")
    }
}

/// A controller that periodically monitors battery status and wakes on alerts.
pub struct BatteryController<'a, M: RawMutex, B, C, Pin = DummyAlertPin, Cmd = ()> {
    battery: &'a Mutex<M, B>,
    charger: &'a Mutex<M, C>,
    state: BatteryState,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    update_fn: Option<fn(u8, model::types::ChargeState) -> Cmd>,
    alert_pin: Option<Pin>,
}

impl<
        'a,
        M: RawMutex,
        B: FuelGauge,
        C: model::interfaces::ChargeStatus,
        Cmd: Clone + core::fmt::Debug,
    > BatteryController<'a, M, B, C, DummyAlertPin, Cmd>
{
    /// Creates a new battery controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>, charger: &'a Mutex<M, C>) -> Self {
        Self {
            battery,
            charger,
            state: BatteryState::Ok,
            system_tx: None,
            update_fn: None,
            alert_pin: None,
        }
    }

    /// Creates a new battery controller with system notification capabilities.
    pub fn new_with_system(
        battery: &'a Mutex<M, B>,
        charger: &'a Mutex<M, C>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        update_fn: fn(u8, model::types::ChargeState) -> Cmd,
    ) -> Self {
        Self {
            battery,
            charger,
            state: BatteryState::Ok,
            system_tx: Some(system_tx),
            update_fn: Some(update_fn),
            alert_pin: None,
        }
    }
}

impl<
        'a,
        M: RawMutex,
        B: FuelGauge,
        C: model::interfaces::ChargeStatus,
        Pin: BatteryAlertPin,
        Cmd: Clone + core::fmt::Debug,
    > BatteryController<'a, M, B, C, Pin, Cmd>
{
    /// Creates a new battery controller with system notification and alert pin support.
    pub fn new_with_system_and_alert(
        battery: &'a Mutex<M, B>,
        charger: &'a Mutex<M, C>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        update_fn: fn(u8, model::types::ChargeState) -> Cmd,
        alert_pin: Pin,
    ) -> Self {
        Self {
            battery,
            charger,
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
        let charger_state = {
            let mut chg = self.charger.lock().await;
            chg.get_charge_state()
                .unwrap_or(model::types::ChargeState::DoneOrStandbyOrUnplugged)
        };

        if voltage < 3500 {
            self.state = BatteryState::Low;
        } else {
            self.state = BatteryState::Ok;
        }

        if let (Some(tx), Some(f)) = (&self.system_tx, self.update_fn) {
            tx.try_send(f(soc, charger_state)).unwrap();
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
            let _ = tx.try_send(model::telemetry::TelemetryRecord::ChargerState(
                charger_state,
            ));
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "Battery Controller: Voltage is {} mV, State: {:?}",
            voltage,
            self.state
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
        // Configure alerts on boot (3.0V low threshold, 4.2V high threshold, 10% SOC empty alert, enable 1% SOC change alert)
        {
            let mut bat = self.battery.lock().await;
            let _ = bat.configure_alerts(3000, 4200, 10, true);
        }

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
                    let mut is_voltage_alert = false;
                    let mut is_soc_alert = false;
                    {
                        let mut bat = self.battery.lock().await;
                        if let Ok((v_alert, soc_alert)) = bat.check_and_clear_alerts() {
                            is_voltage_alert = v_alert;
                            is_soc_alert = soc_alert;
                        }
                    }

                    if is_voltage_alert {
                        // Put the system into PowerOff/PowerDown mode by treating it like a critical battery alert
                        self.state = BatteryState::Low;
                        if let (Some(tx), Some(f)) = (&self.system_tx, self.update_fn) {
                            // SOC = 0, charging = false triggers battery_critical and SystemCommand::PowerDown in SystemController
                            let _ = tx.try_send(f(
                                0,
                                model::types::ChargeState::DoneOrStandbyOrUnplugged,
                            ));
                        }
                    } else if is_soc_alert {
                        let _ = self.update(Some(&telemetry_tx)).await;
                    } else {
                        // Default fallback
                        let _ = self.update(Some(&telemetry_tx)).await;
                    }
                }
                // Periodic update interval elapsed
                embassy_futures::select::Either3::Third(_) => {
                    let _ = self.update(Some(&telemetry_tx)).await;
                }
            }
        }
    }
}

impl<'a, M: RawMutex, B: FuelGauge, C: model::interfaces::ChargeStatus, Pin, Cmd>
    crate::BlockingBatteryReader for BatteryController<'a, M, B, C, Pin, Cmd>
{
    fn read_battery_blocking(&self) -> Option<(u32, u8)> {
        if let Ok(mut bat) = self.battery.try_lock() {
            if let (Ok(v), Ok(soc)) = (bat.read_voltage_mv(), bat.read_state_of_charge()) {
                return Some((v, soc));
            }
        }
        None
    }
}

/// One-way commands sent to the Battery Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryCommand {
    /// Force battery status query and print telemetry logs
    CheckStatus,
}
