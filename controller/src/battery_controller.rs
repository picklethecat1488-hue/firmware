//! Battery status and telemetry controller.

#![deny(missing_docs)]

use crate::telemetry_controller::BatteryTelemetryClient;
use crate::{BatteryReceiver, BlockingBatteryReader, Sender, TelemetrySender};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use firmware_lib::{select_branch_with_timeout, subcommand_enum, BatteryUpdateAction};
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
/// A trait to convert state of charge and charger status to a system command.
pub trait FromBatteryUpdate {
    /// Constructs a command from state of charge and charge state.
    fn from_battery_update(state_of_charge: u8, charger_state: model::types::ChargeState) -> Self;
}

impl FromBatteryUpdate for () {
    fn from_battery_update(
        _state_of_charge: u8,
        _charger_state: model::types::ChargeState,
    ) -> Self {
    }
}

/// A controller that periodically monitors battery status and wakes on alerts.
pub struct BatteryController<'a, M: RawMutex, B, C, Pin = DummyAlertPin, Cmd = ()> {
    battery: &'a Mutex<M, B>,
    charger: &'a Mutex<M, C>,
    state: BatteryState,
    system_tx: Option<Sender<'a, M, Cmd, 4>>,
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
        Cmd: FromBatteryUpdate + Clone + core::fmt::Debug,
    > BatteryController<'a, M, B, C, DummyAlertPin, Cmd>
{
    /// Creates a new battery controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>, charger: &'a Mutex<M, C>) -> Self {
        Self {
            battery,
            charger,
            state: BatteryState::Ok,
            system_tx: None,
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
        system_tx: Sender<'a, M, Cmd, 4>,
    ) -> Self {
        Self {
            battery,
            charger,
            state: BatteryState::Ok,
            system_tx: Some(system_tx),
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
        Cmd: FromBatteryUpdate + Clone + core::fmt::Debug,
    > BatteryController<'a, M, B, C, Pin, Cmd>
where
    <B as FuelGauge>::Error: ToPeripheralError,
{
    /// Creates a new battery controller with system notification and alert pin support.
    pub fn new_with_system_and_alert(
        battery: &'a Mutex<M, B>,
        charger: &'a Mutex<M, C>,
        system_tx: Sender<'a, M, Cmd, 4>,
        alert_pin: Pin,
    ) -> Self {
        Self {
            battery,
            charger,
            state: BatteryState::Ok,
            system_tx: Some(system_tx),
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

        let reported_soc = if read_failed {
            self.state = BatteryState::Ok;
            100
        } else {
            if voltage < 3500 {
                self.state = BatteryState::Low;
            } else {
                self.state = BatteryState::Ok;
            }
            soc
        };

        if let Some(ref tx) = self.system_tx {
            let _ = tx.try_send(Cmd::from_battery_update(reported_soc, charger_state));
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

    /// Wait for the battery alert pin to trigger an alert, or wait forever if no pin is configured.
    pub async fn wait_for_alert(&mut self) {
        if let Some(ref mut pin) = self.alert_pin {
            pin.wait_for_alert().await;
        } else {
            core::future::pending::<()>().await;
        }
    }

    /// Starts the controller's main infinite run loop, processing commands.
    pub async fn run(
        mut self,
        command_rx: BatteryReceiver<M, 4>,
        telemetry_tx: TelemetrySender<
            CriticalSectionRawMutex,
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
            let res = select_branch_with_timeout!(
                embassy_time::Duration::from_millis(2000),
                command_rx.receive() => |cmd| {
                    match cmd {
                        BatteryCommand::CheckStatus => {
                            if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                                let err = e.to_peripheral_error();
                                telemetry_client.report_error(err);
                            }
                        }
                        BatteryCommand::UpdateWakeLocks(mask) => {
                            self.active_wake_locks = mask;
                        }
                    }
                    Some(())
                },
                self.wait_for_alert() => || {
                    None
                },
            );

            if res.is_none() {
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
                    if let Some(ref tx) = self.system_tx {
                        // SOC = 0, charging = false triggers battery_critical and SystemCommand::PowerDown in SystemController
                        let _ = tx.try_send(Cmd::from_battery_update(
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

subcommand_enum! {
    /// Battery subcommands for CLI processing.
    pub enum BatterySubcommand {
        /// Query battery status
        Status,
    }
    "Invalid battery subcommand. Expected: status"
}

/// Processes battery-specific CLI subcommands.
pub fn handle_battery_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<BatterySubcommand>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let battery_ctrl = resolver.resolve_battery(None)?;
    let cmd = subcommand.ok_or("Missing battery subcommand")?;

    match cmd {
        BatterySubcommand::Status => {
            let (v, soc) = battery_ctrl
                .read_battery_blocking()
                .map_err(|_| "Failed to read battery")?;
            let _ = core::writeln!(
                writer,
                "\r\nBattery Status:\r\n  Voltage: {} mV\r\n  SoC: {}%",
                v,
                soc
            );
            Ok(())
        }
    }
}

/// Standard config implementation for BatteryFeature.
pub struct BatteryFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Battery channel sender
    pub battery_tx: Option<crate::BatterySender<MutexRaw, N>>,
    /// Battery manager for battery thresholds and status
    pub battery_manager: core::cell::RefCell<firmware_lib::BatteryManager>,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> BatteryFeatureConfig<MutexRaw, N> {
    /// Creates a new `BatteryFeatureConfig`.
    pub fn new(
        battery_tx: Option<crate::BatterySender<MutexRaw, N>>,
        battery_manager: firmware_lib::BatteryManager,
    ) -> Self {
        Self {
            battery_tx,
            battery_manager: core::cell::RefCell::new(battery_manager),
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> crate::SystemFeature<MutexRaw, N>
    for BatteryFeatureConfig<MutexRaw, N>
{
    fn default_boot_trap_mask(&self) -> u32 {
        if self.battery_tx.is_some() {
            firmware_lib::BootTrapReason::Battery as u32
        } else {
            0
        }
    }

    fn on_init(&self) {
        let mut bm = self.battery_manager.borrow_mut();
        let low_threshold = bm.low_soc_threshold();
        if bm.critical_soc_threshold() >= low_threshold {
            bm.set_critical_soc_threshold(low_threshold - 1);
        }
    }

    fn on_battery_update(
        &self,
        state_of_charge: u8,
        charger_state: model::types::ChargeState,
        status: model::types::SystemStatus,
        is_boot_trapped: bool,
    ) -> Option<(Option<BatteryUpdateAction>, crate::BatteryStatus)> {
        let mut bm = self.battery_manager.borrow_mut();
        let action =
            bm.update_battery_status(state_of_charge, charger_state, status, is_boot_trapped);
        let battery_critical = bm.battery_critical();
        let charger_connected = bm.charger_connected();
        let soc_led_state = bm.get_soc_led_state();
        Some((
            action,
            crate::BatteryStatus {
                battery_critical,
                charger_connected,
                soc_led_state,
            },
        ))
    }

    fn on_state_changed(
        &self,
        _from: model::types::SystemStatus,
        _to: model::types::SystemStatus,
        _support: crate::DeviceSupport,
        _battery_status: Option<crate::BatteryStatus>,
        _thermal_critical: bool,
    ) {
        if let Some(ref battery_tx) = self.battery_tx {
            let _ = battery_tx.try_send(crate::BatteryCommand::UpdateWakeLocks(0));
        }
    }

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        crossed_tick: bool,
        _status: model::types::SystemStatus,
        support: crate::DeviceSupport,
        wake_locks: u32,
    ) {
        if crossed_tick && support.battery {
            if let Some(ref battery_tx) = self.battery_tx {
                let _ = battery_tx.try_send(crate::BatteryCommand::UpdateWakeLocks(wake_locks));
                let _ = battery_tx.try_send(crate::BatteryCommand::CheckStatus);
            }
        }
    }
}
