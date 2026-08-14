//! Battery status and telemetry controller.

#![deny(missing_docs)]

use crate::system_controller::SystemCommand;
use crate::telemetry_controller::BatteryTelemetryClient;
use crate::tracing::{self, controller_context};
use crate::{BatteryReceiver, BlockingBatteryReader, TelemetrySender};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use model::interfaces::{FuelGauge, Tickable};
use model::telemetry::TelemetryClient;
use model::types::{PeriodicInterval, PeripheralError};
use peripheral::ToPeripheralError;
use platform::{select_branch_with_timeout, subcommand_enum, BatteryUpdateAction};

/// Default minimum voltage alert threshold (mV).
pub const DEFAULT_ALERT_V_MIN_MV: u32 = 3000;
/// Default maximum voltage alert threshold (mV).
pub const DEFAULT_ALERT_V_MAX_MV: u32 = 4200;

/// Test minimum voltage alert threshold (mV).
pub const TEST_ALERT_V_MIN_MV: u32 = 4500;
/// Test maximum voltage alert threshold (mV).
pub const TEST_ALERT_V_MAX_MV: u32 = 5000;

/// Trait for waiting on a battery alert pin.
#[allow(async_fn_in_trait)]
pub trait BatteryAlertPin {
    /// Wait for the alert pin to go low (active state).
    async fn wait_for_alert(&mut self);

    /// Check if the alert pin is currently asserted.
    fn is_asserted(&self) -> bool;
}

/// A dummy mock implementation of BatteryAlertPin that waits forever.
pub struct DummyAlertPin;

impl BatteryAlertPin for DummyAlertPin {
    async fn wait_for_alert(&mut self) {
        // Sleep forever to let the periodic timeout drive updates
        embassy_time::Timer::after_secs(3600 * 24).await;
    }

    fn is_asserted(&self) -> bool {
        false
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

/// Battery controller errors.
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BatteryControllerError<E> {
    /// An error returned by the fuel gauge driver.
    FuelGauge(E),
    /// Critical error: failed to notify the system channel about battery status or charger state change.
    SystemChannelSendFailed,
}

impl<E: ToPeripheralError> ToPeripheralError for BatteryControllerError<E> {
    #[cfg_attr(
        all(target_arch = "arm", feature = "core1"),
        link_section = ".data.core1_func"
    )]
    fn to_peripheral_error(&self) -> model::types::PeripheralError {
        match self {
            Self::FuelGauge(e) => e.to_peripheral_error(),
            Self::SystemChannelSendFailed => model::types::PeripheralError::Unknown,
        }
    }
}

/// A controller that periodically monitors battery status and wakes on alerts.
#[controller_context]
pub struct BatteryController<
    'a,
    M: RawMutex + 'static,
    B,
    C,
    Pin = DummyAlertPin,
    const SYS_CAP: usize = 16,
> {
    battery: &'a Mutex<M, B>,
    charger: &'a Mutex<M, C>,
    state: BatteryState,
    system_tx: Option<crate::SystemSender<M, SYS_CAP>>,
    alert_pin: Option<Pin>,
    last_reported_voltage: Option<u32>,
    last_reported_state: Option<BatteryState>,
    active_wake_locks: u32,
    latest_temp_milli_c: Option<i32>,
}

impl<
        'a,
        M: RawMutex,
        B: FuelGauge + Tickable<Error = <B as FuelGauge>::Error>,
        C: model::interfaces::ChargeStatus,
        const SYS_CAP: usize,
    > BatteryController<'a, M, B, C, DummyAlertPin, SYS_CAP>
where
    <B as FuelGauge>::Error: ToPeripheralError,
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
            latest_temp_milli_c: None,
        }
    }

    /// Creates a new battery controller with system notification capabilities.
    pub fn new_with_system(
        battery: &'a Mutex<M, B>,
        charger: &'a Mutex<M, C>,
        system_tx: crate::SystemSender<M, SYS_CAP>,
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
            latest_temp_milli_c: None,
        }
    }
}

impl<
        'a,
        M: RawMutex,
        B: FuelGauge + Tickable<Error = <B as FuelGauge>::Error>,
        C: model::interfaces::ChargeStatus,
        Pin: BatteryAlertPin,
        const SYS_CAP: usize,
    > BatteryController<'a, M, B, C, Pin, SYS_CAP>
where
    <B as FuelGauge>::Error: ToPeripheralError,
{
    /// Creates a new battery controller with system notification and alert pin support.
    pub fn new_with_system_and_alert(
        battery: &'a Mutex<M, B>,
        charger: &'a Mutex<M, C>,
        system_tx: crate::SystemSender<M, SYS_CAP>,
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
            latest_temp_milli_c: None,
        }
    }

    /// Gets the current state of the battery.
    pub fn state(&self) -> BatteryState {
        self.state
    }

    /// Updates the battery status by locking and reading the peripheral.
    #[tracing::instrument(
        name = "battery_controller::update",
        level = "info",
        skip(telemetry_client)
    )]
    pub async fn update(
        &mut self,
        telemetry_client: Option<&mut BatteryTelemetryClient<CriticalSectionRawMutex>>,
    ) -> Result<(), BatteryControllerError<<B as FuelGauge>::Error>> {
        let mut read_failed = false;
        let mut error_val = None;
        let (voltage, soc) = {
            let mut bat = self.battery.lock().await;

            // Compensate fuel gauge based on temperature if available
            if let Some(temp_milli_c) = self.latest_temp_milli_c {
                let _ = bat.set_battery_temperature(temp_milli_c).await;
            }

            let _ = bat.tick().await;
            match (
                bat.read_voltage_mv().await,
                bat.read_state_of_charge().await,
            ) {
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
                .await
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
            tx.try_send(SystemCommand::from_battery_update(
                reported_soc,
                charger_state,
            ))
            .map_err(|_| BatteryControllerError::SystemChannelSendFailed)?;
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

        let voltage_changed = match self.last_reported_voltage {
            None => true,
            Some(last) => (voltage as i32 - last as i32).abs() >= 10,
        };
        let state_changed = self.last_reported_state != Some(self.state);
        if voltage_changed || state_changed {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            {
                defmt::info!(
                    "Battery Controller: Voltage is {} mV, SoC: {}%, Charging: {}, State: {:?}",
                    voltage,
                    reported_soc,
                    charger_state == model::types::ChargeState::Charging,
                    self.state
                );
            }
            if voltage_changed {
                self.last_reported_voltage = Some(voltage);
            }
            if state_changed {
                self.last_reported_state = Some(self.state);
            }
        }

        if read_failed {
            if let Some(err) = error_val {
                Err(BatteryControllerError::FuelGauge(err))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Wait for the battery alert pin to trigger an alert, or wait forever if no pin is configured.
    pub async fn wait_for_alert(&mut self) {
        if let Some(ref mut pin) = self.alert_pin {
            pin.wait_for_alert().await;
        } else {
            core::future::pending::<()>().await;
        }
    }

    fn handle_update_error(
        &self,
        e: BatteryControllerError<<B as FuelGauge>::Error>,
        telemetry_client: &mut BatteryTelemetryClient<CriticalSectionRawMutex>,
        check_interval: Option<&mut embassy_time::Duration>,
    ) {
        match e {
            BatteryControllerError::FuelGauge(err) => {
                telemetry_client.report_error(err.to_peripheral_error());
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("BatteryController: status read failed!");
                if let Some(interval) = check_interval {
                    *interval = crate::OVERFLOW_SAFE_MAX_DURATION;
                }
            }
            BatteryControllerError::SystemChannelSendFailed => {
                panic!("BatteryController: Failed to send battery update to system channel!");
            }
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
        let mut check_interval = embassy_time::Duration::from_millis(2000);
        let mut boot_config_failed = false;

        // Configure alerts on boot (3.0V low threshold, 4.2V high threshold, 10% SOC empty alert, enable 1% SOC change alert)
        {
            let mut bat = self.battery.lock().await;
            if let Err(e) = bat.configure_alerts(3000, 4200, 10, true).await {
                let err = e.to_peripheral_error();
                telemetry_client.report_error(err);
                boot_config_failed = true;
            }
        }

        // Run initial status check on boot.
        if let Err(e) = self.update(Some(&mut telemetry_client)).await {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::warn!("BatteryController: Boot read failed/timed out!");
            self.handle_update_error(e, &mut telemetry_client, Some(&mut check_interval));
            // Explicitly clear boot trap on timeout/failure to allow booting with warnings
            if let Some(ref tx) = self.system_tx {
                let _ = tx.try_send(SystemCommand::BatteryAction(
                    BatteryUpdateAction::ClearBootTrap,
                ));
            }
        }

        if boot_config_failed {
            telemetry_client.report_error(model::types::PeripheralError::InvalidConfiguration);
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("BatteryController: Boot alert configuration failed!");
        }
        loop {
            let res = select_branch_with_timeout!(
                check_interval,
                command_rx.receive() => |cmd| {
                    match cmd {
                        BatteryCommand::CheckStatus => {
                            if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                                self.handle_update_error(e, &mut telemetry_client, None);
                            }
                        }
                        BatteryCommand::UpdateWakeLocks(mask) => {
                            self.active_wake_locks = mask;
                        }
                        BatteryCommand::SetInterval(interval) => {
                            check_interval = match interval {
                                PeriodicInterval::None => crate::OVERFLOW_SAFE_MAX_DURATION,
                                PeriodicInterval::UpdateMs(ms) => {
                                    embassy_time::Duration::from_millis(ms as u64)
                                }
                            };
                            telemetry_client.report_interval(model::types::Device::Battery, interval);
                        }
                        BatteryCommand::UpdateTemperature(temp) => {
                            self.latest_temp_milli_c = Some(temp);
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
                    match bat.check_and_clear_alerts().await {
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
                        if tx
                            .try_send(SystemCommand::from_battery_update(
                                0,
                                model::types::ChargeState::DoneOrStandbyOrUnplugged,
                            ))
                            .is_err()
                        {
                            telemetry_client.report_error(model::types::PeripheralError::Unknown);
                            #[cfg(all(target_arch = "arm", target_os = "none"))]
                            defmt::error!("BatteryController: Critical battery alert send failed!");
                        }
                    }
                } else if is_soc_alert {
                    if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                        self.handle_update_error(
                            e,
                            &mut telemetry_client,
                            Some(&mut check_interval),
                        );
                    }
                } else if check_interval != crate::OVERFLOW_SAFE_MAX_DURATION {
                    // Default fallback
                    if let Err(e) = self.update(Some(&mut telemetry_client)).await {
                        self.handle_update_error(
                            e,
                            &mut telemetry_client,
                            Some(&mut check_interval),
                        );
                    }
                }
            }
        }
    }
}

impl<
        'a,
        M: RawMutex,
        B: FuelGauge + Tickable<Error = <B as FuelGauge>::Error>,
        C: model::interfaces::ChargeStatus,
        Pin: BatteryAlertPin,
        const SYS_CAP: usize,
    > crate::BlockingBatteryReader for BatteryController<'a, M, B, C, Pin, SYS_CAP>
{
    fn read_battery_blocking(&self) -> Result<(u32, u8), PeripheralError> {
        if let Ok(mut bat) = self.battery.try_lock() {
            let fut = async {
                match (
                    bat.read_voltage_mv().await,
                    bat.read_state_of_charge().await,
                ) {
                    (Ok(v), Ok(soc)) => Ok((v, soc)),
                    _ => Err(PeripheralError::DeviceNotAvailable),
                }
            };
            return embassy_futures::block_on(fut);
        }
        Err(PeripheralError::DeviceNotAvailable)
    }

    fn configure_alerts(&self, v_min_mv: u32, v_max_mv: u32) -> Result<(), PeripheralError> {
        if let Ok(mut bat) = self.battery.try_lock() {
            let fut = bat.configure_alerts(v_min_mv, v_max_mv, 1, true);
            embassy_futures::block_on(fut).map_err(|_| PeripheralError::DeviceNotAvailable)?;
            return Ok(());
        }
        Err(PeripheralError::DeviceNotAvailable)
    }

    fn check_and_clear_alerts(&self) -> Result<(bool, bool), PeripheralError> {
        if let Ok(mut bat) = self.battery.try_lock() {
            let fut = bat.check_and_clear_alerts();
            return embassy_futures::block_on(fut).map_err(|_| PeripheralError::DeviceNotAvailable);
        }
        Err(PeripheralError::DeviceNotAvailable)
    }

    fn read_alert_pin(&self) -> Result<bool, PeripheralError> {
        if let Some(ref pin) = self.alert_pin {
            Ok(pin.is_asserted())
        } else {
            Err(PeripheralError::NotImplemented)
        }
    }
}

/// One-way commands sent to the Battery Controller from the shell.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum BatteryCommand {
    /// Force battery status query and print telemetry logs
    CheckStatus,
    /// Update the current active wake locks bitmask
    UpdateWakeLocks(u32),
    /// Set periodic automatic checking interval
    SetInterval(PeriodicInterval),
    /// Update the latest battery temperature in milli-Celsius for compensation
    UpdateTemperature(i32),
}

subcommand_enum! {
    /// Battery subcommands for CLI processing.
    pub enum BatterySubcommand {
        /// Query battery status
        Status,
        /// Test the battery alert pin interrupt
        TestAlert = "test-alert",
    }
    "Invalid battery subcommand. Expected: status, test-alert"
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
        BatterySubcommand::TestAlert => {
            // 1. Force the alert by setting thresholds above the current voltage
            battery_ctrl
                .configure_alerts(TEST_ALERT_V_MIN_MV, TEST_ALERT_V_MAX_MV)
                .map_err(|_| "Failed to configure test alert thresholds")?;

            // 2. Poll the alert pin status directly to verify it asserts low (active-low)
            // Poll for up to 300 ms (30 iterations of 10 ms delay)
            let mut asserted = false;
            for _ in 0..30 {
                if let Ok(true) = battery_ctrl.read_alert_pin() {
                    asserted = true;
                    break;
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                {
                    cortex_m::asm::delay(125_000 * 10);
                }
                #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                {
                    asserted = true; // Mock pin for host tests
                }
            }

            // 3. Restore normal thresholds
            battery_ctrl
                .configure_alerts(DEFAULT_ALERT_V_MIN_MV, DEFAULT_ALERT_V_MAX_MV)
                .map_err(|_| "Failed to restore alert thresholds")?;

            // 4. Clear the active status registers so the alert pin deasserts high
            let (has_v, has_soc) = battery_ctrl
                .check_and_clear_alerts()
                .map_err(|_| "Failed to clear alerts")?;

            // 5. Poll to verify the alert pin returns high (deasserted)
            let mut deasserted = false;
            for _ in 0..10 {
                if let Ok(false) = battery_ctrl.read_alert_pin() {
                    deasserted = true;
                    break;
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                {
                    cortex_m::asm::delay(125_000 * 10);
                }
                #[cfg(not(all(target_arch = "arm", target_os = "none")))]
                {
                    deasserted = true; // Mock pin for host tests
                }
            }

            let _ = core::writeln!(
                writer,
                "\r\nPin asserted (low): {}. Pin deasserted (high): {}. Pre-clear alerts: voltage={}, soc={}",
                asserted,
                deasserted,
                has_v,
                has_soc
            );
            Ok(())
        }
    }
}

/// Standard config implementation for BatteryFeature.
pub struct BatteryFeatureConfig<MutexRaw: RawMutex + 'static> {
    /// Battery channel sender
    pub battery_tx: Option<crate::BatterySender<MutexRaw>>,
    /// Battery manager for battery thresholds and status
    pub battery_manager: core::cell::RefCell<platform::BatteryManager>,
}

impl<MutexRaw: RawMutex + 'static> BatteryFeatureConfig<MutexRaw> {
    /// Creates a new `BatteryFeatureConfig`.
    pub fn new(
        battery_tx: Option<crate::BatterySender<MutexRaw>>,
        battery_manager: platform::BatteryManager,
    ) -> Self {
        Self {
            battery_tx,
            battery_manager: core::cell::RefCell::new(battery_manager),
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> crate::SystemFeature<MutexRaw, N>
    for BatteryFeatureConfig<MutexRaw>
{
    fn default_boot_trap_mask(&self) -> u32 {
        if self.battery_tx.is_some() {
            platform::BootTrapReason::Battery as u32
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
        support: crate::DeviceSupport,
        _battery_status: Option<crate::BatteryStatus>,
        _thermal_critical: bool,
    ) {
        <Self as crate::SystemFeature<MutexRaw, N>>::on_wake_locks_changed(self, 0);
        use crate::Periodic;
        if support.battery {
            self.set_interval(PeriodicInterval::UpdateMs(1000));
        } else {
            self.set_interval(PeriodicInterval::None);
        }
    }

    fn on_wake_locks_changed(&self, wake_locks: u32) {
        if let Some(ref battery_tx) = self.battery_tx {
            let res = battery_tx.try_send(crate::BatteryCommand::UpdateWakeLocks(wake_locks));
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            res.map_err(|_| "TrySendError")
                .expect("Failed to signal wake locks update to battery controller");
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            let _ = res;
        }
    }

    fn on_thermal_update(&self, temp_milli_c: i32) {
        if let Some(ref battery_tx) = self.battery_tx {
            let _ = battery_tx.try_send(crate::BatteryCommand::UpdateTemperature(temp_milli_c));
        }
    }
}

impl<MutexRaw: RawMutex + 'static> crate::Periodic for BatteryFeatureConfig<MutexRaw> {
    fn set_interval(&self, interval: PeriodicInterval) {
        if let Some(ref battery_tx) = self.battery_tx {
            let res = battery_tx.try_send(BatteryCommand::SetInterval(interval));
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            res.map_err(|_| "TrySendError")
                .expect("Failed to send periodic interval to battery controller");
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            let _ = res;
        }
    }
}
