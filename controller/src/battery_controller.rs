//! Battery status and telemetry controller.

#![deny(missing_docs)]

use crate::telemetry_controller::BatteryTelemetryClient;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use model::interfaces::FuelGauge;
use model::telemetry::TelemetryClient;
use model::types::PeripheralError;
use peripherals::ToPeripheralError;

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
    last_reported_voltage: Option<u32>,
    last_reported_state: Option<BatteryState>,
    active_wake_locks: u32,
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
            last_reported_voltage: None,
            last_reported_state: None,
            active_wake_locks: 0,
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
            last_reported_voltage: None,
            last_reported_state: None,
            active_wake_locks: 0,
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
where
    <B as FuelGauge>::Error: ToPeripheralError,
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
            last_reported_voltage: None,
            last_reported_state: None,
            active_wake_locks: 0,
        }
    }

    /// Gets the current state of the battery.
    pub fn state(&self) -> BatteryState {
        self.state
    }

    /// Updates the battery status by locking and reading the peripheral.
    pub async fn update(
        &mut self,
        telemetry_client: Option<
            &mut BatteryTelemetryClient<
                '_,
                CriticalSectionRawMutex,
                { crate::telemetry_controller::CHANNEL_CAPACITY },
            >,
        >,
    ) -> Result<(), B::Error> {
        let mut read_failed = false;
        let mut error_val = None;
        let (voltage, soc) = {
            let mut bat = self.battery.lock().await;
            match (bat.read_voltage_mv(), bat.read_state_of_charge()) {
                (Ok(v), Ok(s)) => (v, s),
                (Err(e), _) | (_, Err(e)) => {
                    read_failed = true;
                    error_val = Some(e);
                    (0, 0)
                }
            }
        };
        let charger_state = {
            let mut chg = self.charger.lock().await;
            chg.get_charge_state()
                .unwrap_or(model::types::ChargeState::DoneOrStandbyOrUnplugged)
        };

        if read_failed || voltage < 3500 {
            self.state = BatteryState::Low;
        } else {
            self.state = BatteryState::Ok;
        }

        if let (Some(tx), Some(f)) = (&self.system_tx, self.update_fn) {
            let _ = tx.try_send(f(soc, charger_state));
        }

        if let Some(client) = telemetry_client {
            let battery_state = if read_failed {
                model::types::BatteryState::Critical
            } else {
                match self.state {
                    BatteryState::Ok => model::types::BatteryState::Ok,
                    BatteryState::Low => model::types::BatteryState::Low,
                }
            };
            let status = model::types::BatteryStatus::VolTempState(
                voltage,
                25000,
                battery_state,
                self.active_wake_locks,
            );
            client.report(status);
            client.report(model::types::FuelGaugeTelemetry::VolSoc(voltage, soc));
            client.report(charger_state);
            if let Some(ref err) = error_val {
                client.report_error(err.to_peripheral_error());
            }
        }

        let voltage_changed = self.last_reported_voltage != Some(voltage);
        let state_changed = self.last_reported_state != Some(self.state);
        if voltage_changed || state_changed {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!(
                "Battery Controller: Voltage is {} mV, State: {:?}",
                voltage,
                self.state
            );
            self.last_reported_voltage = Some(voltage);
            self.last_reported_state = Some(self.state);
        }

        if read_failed {
            if let Some(err) = error_val {
                return Err(err);
            }
        }

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
            { crate::telemetry_controller::CHANNEL_CAPACITY },
        >,
    ) -> ! {
        let mut telemetry_client = BatteryTelemetryClient::new(Some(telemetry_tx));
        // Configure alerts on boot (3.0V low threshold, 4.2V high threshold, 10% SOC empty alert, enable 1% SOC change alert)
        {
            let mut bat = self.battery.lock().await;
            if let Err(e) = bat.configure_alerts(3000, 4200, 10, true) {
                let err = e.to_peripheral_error();
                telemetry_client.report_error(err);
            }
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
                        if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                            let err = e.to_peripheral_error();
                            telemetry_client.report_error(err);
                        }
                    }
                    BatteryCommand::UpdateWakeLocks(mask) => {
                        self.active_wake_locks = mask;
                    }
                },
                // Fuel gauge alert interrupt triggered
                embassy_futures::select::Either3::Second(_) => {
                    let mut is_voltage_alert = false;
                    let mut is_soc_alert = false;
                    {
                        let mut bat = self.battery.lock().await;
                        match bat.check_and_clear_alerts() {
                            Ok((v_alert, soc_alert)) => {
                                is_voltage_alert = v_alert;
                                is_soc_alert = soc_alert;
                            }
                            Err(e) => {
                                let err = e.to_peripheral_error();
                                telemetry_client.report_error(err);
                            }
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
                        if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                            let err = e.to_peripheral_error();
                            telemetry_client.report_error(err);
                        }
                    } else {
                        // Default fallback
                        if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                            let err = e.to_peripheral_error();
                            telemetry_client.report_error(err);
                        }
                    }
                }
                // Periodic update interval elapsed
                embassy_futures::select::Either3::Third(_) => {
                    if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                        let err = e.to_peripheral_error();
                        telemetry_client.report_error(err);
                    }
                }
            }
        }
    }
}

impl<'a, M: RawMutex, B: FuelGauge, C: model::interfaces::ChargeStatus, Pin, Cmd>
    crate::BlockingBatteryReader for BatteryController<'a, M, B, C, Pin, Cmd>
{
    fn read_battery_blocking(&self) -> Result<(u32, u8), PeripheralError> {
        if let Ok(mut bat) = self.battery.try_lock() {
            if let (Ok(v), Ok(soc)) = (bat.read_voltage_mv(), bat.read_state_of_charge()) {
                return Ok((v, soc));
            }
        }
        Err(PeripheralError::DeviceNotAvailable)
    }
}

/// One-way commands sent to the Battery Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryCommand {
    /// Force battery status query and print telemetry logs
    CheckStatus,
    /// Update the current active wake locks bitmask
    UpdateWakeLocks(u32),
}
