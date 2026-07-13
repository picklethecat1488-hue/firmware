//! Thermal monitoring and regulation controller.

#![deny(missing_docs)]

use crate::types::ThermalState;
use crate::{BlockingThermalReader, Sender, TelemetrySender, ThermalReceiver};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use firmware_lib::subcommand_enum;
use model::interfaces::TemperatureSensor;
use model::types::PeripheralError;
use peripherals::ToPeripheralError;

/// A controller that periodically monitors system temperature from temperature sensors.
pub struct ThermalController<'a, M: RawMutex, B> {
    temp: &'a Mutex<M, B>,
    thermal_tx: Option<Sender<'a, M, crate::types::ThermalUpdateAction, 4>>,
    state: ThermalState,
    overheating_temp_milli_c: i32,
    critical_temp_milli_c: i32,
    hysteresis_temp_milli_c: i32,
    first_update: bool,
}

impl<'a, M: RawMutex, B: TemperatureSensor> ThermalController<'a, M, B> {
    /// Creates a new thermal controller referencing a shared temperature peripheral without shutdown coordination.
    pub fn new(temp: &'a Mutex<M, B>) -> Self {
        Self {
            temp,
            thermal_tx: None,
            state: ThermalState::Normal,
            overheating_temp_milli_c: 45000,
            critical_temp_milli_c: 60000,
            hysteresis_temp_milli_c: 2000,
            first_update: true,
        }
    }

    /// Creates a new thermal controller with safety shutdown capabilities.
    pub fn new_with_shutdown(
        temp: &'a Mutex<M, B>,
        thermal_tx: Sender<'a, M, crate::types::ThermalUpdateAction, 4>,
    ) -> Self {
        Self {
            temp,
            thermal_tx: Some(thermal_tx),
            state: ThermalState::Normal,
            overheating_temp_milli_c: 45000,
            critical_temp_milli_c: 60000,
            hysteresis_temp_milli_c: 2000,
            first_update: true,
        }
    }

    /// Creates a new thermal controller with safety shutdown and boot trap clearing capabilities.
    pub fn new_with_shutdown_and_trap(
        temp: &'a Mutex<M, B>,
        thermal_tx: Sender<'a, M, crate::types::ThermalUpdateAction, 4>,
    ) -> Self {
        Self::new_with_shutdown(temp, thermal_tx)
    }

    /// Gets the current state of the thermal system.
    pub fn state(&self) -> ThermalState {
        self.state
    }

    /// Gets the overheating temperature threshold in milli-degrees Celsius.
    pub fn overheating_temp_milli_c(&self) -> i32 {
        self.overheating_temp_milli_c
    }

    /// Sets the overheating temperature threshold in milli-degrees Celsius.
    pub fn set_overheating_temp_milli_c(&mut self, temp: i32) {
        self.overheating_temp_milli_c = temp;
    }

    /// Gets the critical temperature threshold in milli-degrees Celsius.
    pub fn critical_temp_milli_c(&self) -> i32 {
        self.critical_temp_milli_c
    }

    /// Sets the critical temperature threshold in milli-degrees Celsius.
    pub fn set_critical_temp_milli_c(&mut self, temp: i32) {
        self.critical_temp_milli_c = temp;
    }

    /// Gets the hysteresis temperature range in milli-degrees Celsius.
    pub fn hysteresis_temp_milli_c(&self) -> i32 {
        self.hysteresis_temp_milli_c
    }

    /// Sets the hysteresis temperature range in milli-degrees Celsius.
    pub fn set_hysteresis_temp_milli_c(&mut self, val: i32) {
        self.hysteresis_temp_milli_c = val;
    }

    /// Updates the thermal status by locking and reading the peripheral.
    pub async fn update<TC: model::telemetry::TelemetryClient<(i32, ThermalState)>>(
        &mut self,
        telemetry_client: Option<&mut TC>,
    ) -> Result<(), B::Error> {
        let temp = {
            let mut sensor = self.temp.lock().await;
            match sensor.read_temperature_milli_c() {
                Ok(t) => t,
                Err(e) => {
                    let safe_temp = 25000; // 25°C
                    if self.first_update {
                        self.first_update = false;
                        if let Some(tx) = &self.thermal_tx {
                            let _ = tx.try_send(crate::types::ThermalUpdateAction::ClearBootTrap);
                        }
                    }
                    if let Some(client) = telemetry_client {
                        client.report((safe_temp, ThermalState::Normal));
                    }
                    return Err(e);
                }
            }
        };

        match self.state {
            ThermalState::Normal => {
                if temp > self.overheating_temp_milli_c {
                    self.state = ThermalState::Overheating;
                }
            }
            ThermalState::Overheating => {
                if temp < self.overheating_temp_milli_c - self.hysteresis_temp_milli_c {
                    self.state = ThermalState::Normal;
                }
            }
        }

        // Critical threshold check: shut down system if temp > critical_temp_milli_c
        if temp > self.critical_temp_milli_c {
            if let Some(tx) = &self.thermal_tx {
                tx.try_send(crate::types::ThermalUpdateAction::AlertTriggered)
                    .unwrap();
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("Thermal Controller: Critical temperature exceeded ({} mC). Dispatching safety shutdown.", temp);
            }
        } else if self.first_update {
            self.first_update = false;
            if let Some(tx) = &self.thermal_tx {
                let _ = tx.try_send(crate::types::ThermalUpdateAction::ClearBootTrap);
            }
        }

        if let Some(client) = telemetry_client {
            client.report((temp, self.state));
        }

        Ok(())
    }

    /// Starts the controller's main infinite run loop, processing commands.
    pub async fn run(
        mut self,
        command_rx: ThermalReceiver<M, 4>,
        telemetry_tx: TelemetrySender<
            CriticalSectionRawMutex,
            { crate::telemetry_controller::CHANNEL_CAPACITY },
        >,
    ) -> ! {
        let mut telemetry_client =
            crate::telemetry_controller::ThermalTelemetryClient::new(Some(telemetry_tx));
        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1500),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => match cmd {
                    ThermalCommand::CheckTemp => {
                        let _ = self.update(Some(&mut telemetry_client)).await;
                    }
                },
                Err(_timeout) => {
                    let _ = self.update(Some(&mut telemetry_client)).await;
                }
            }
        }
    }
}

impl<'a, M: RawMutex, B: TemperatureSensor> crate::BlockingThermalReader
    for ThermalController<'a, M, B>
where
    B::Error: ToPeripheralError,
{
    fn read_temperature_blocking(&self) -> Result<i32, PeripheralError> {
        if let Ok(mut guard) = self.temp.try_lock() {
            guard
                .read_temperature_milli_c()
                .map_err(|e| e.to_peripheral_error())
        } else {
            Err(PeripheralError::DeviceNotAvailable)
        }
    }
}

/// One-way commands sent to the Thermal Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalCommand {
    /// Force thermal status query and print telemetry logs
    CheckTemp,
}

subcommand_enum! {
    /// Thermal subcommands for CLI processing.
    pub enum ThermalSubcommand {
        /// Read external temperature sensor
        Status,
        /// Read MCU temperature sensor
        Mcu,
    }
    "Invalid thermal subcommand. Expected: status, mcu"
}

/// Processes thermal-specific CLI subcommands.
pub fn handle_thermal_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<ThermalSubcommand>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing thermal subcommand")?;

    match cmd {
        ThermalSubcommand::Status => {
            let thermal_ctrl = resolver.resolve_thermal(None)?;
            let temp = thermal_ctrl
                .read_temperature_blocking()
                .map_err(|_| "Direct thermal reading failed")?;
            let _ = core::writeln!(
                writer,
                "\r\nDirect thermal reading (ThermalController): {}.{:03} C",
                temp / 1000,
                (temp.abs() % 1000)
            );
            Ok(())
        }
        ThermalSubcommand::Mcu => {
            let sensor = resolver.resolve_temp_sensor(None)?;
            let temp = sensor
                .read_temperature_milli_c()
                .map_err(|_| "Direct system temperature reading failed")?;
            let _ = core::writeln!(
                writer,
                "\r\nDirect system temperature reading (RP2040): {}.{:03} C",
                temp / 1000,
                (temp.abs() % 1000)
            );
            Ok(())
        }
    }
}

/// Standard config implementation for ThermalFeature.
pub struct ThermalFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Thermal channel sender
    pub thermal_tx: Option<crate::ThermalSender<MutexRaw, N>>,
    /// Thermal manager for checking alerts
    pub thermal_manager: core::cell::RefCell<firmware_lib::ThermalManager>,
    /// Overheating temperature threshold in milli-Celsius
    pub overheating_temp_milli_c: i32,
    /// Critical temperature threshold in milli-Celsius
    pub critical_temp_milli_c: i32,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> ThermalFeatureConfig<MutexRaw, N> {
    /// Creates a new `ThermalFeatureConfig`.
    pub fn new(thermal_tx: Option<crate::ThermalSender<MutexRaw, N>>) -> Self {
        Self {
            thermal_tx,
            thermal_manager: core::cell::RefCell::new(firmware_lib::ThermalManager::new()),
            overheating_temp_milli_c: 45000,
            critical_temp_milli_c: 60000,
        }
    }

    /// Creates a new `ThermalFeatureConfig` with custom thresholds.
    pub fn new_with_thresholds(
        thermal_tx: Option<crate::ThermalSender<MutexRaw, N>>,
        overheating_temp_milli_c: i32,
        critical_temp_milli_c: i32,
    ) -> Self {
        Self {
            thermal_tx,
            thermal_manager: core::cell::RefCell::new(firmware_lib::ThermalManager::new()),
            overheating_temp_milli_c,
            critical_temp_milli_c,
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize> crate::SystemFeature<MutexRaw, N>
    for ThermalFeatureConfig<MutexRaw, N>
{
    fn default_boot_trap_mask(&self) -> u32 {
        if self.thermal_tx.is_some() {
            firmware_lib::BootTrapReason::Thermal as u32
        } else {
            0
        }
    }

    fn thermal_overheating_temp_threshold(&self) -> i32 {
        self.overheating_temp_milli_c
    }

    fn thermal_critical_temp_threshold(&self) -> i32 {
        self.critical_temp_milli_c
    }

    fn thermal_critical(&self) -> bool {
        self.thermal_manager.borrow().thermal_critical()
    }

    fn on_alert_triggered(&self, _status: model::types::SystemStatus) {
        self.thermal_manager.borrow_mut().set_thermal_critical(true);
    }

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        crossed_tick: bool,
        _status: model::types::SystemStatus,
        support: crate::DeviceSupport,
        _wake_locks: u32,
    ) {
        if crossed_tick && support.thermal {
            if let Some(ref thermal_tx) = self.thermal_tx {
                let _ = thermal_tx.try_send(crate::ThermalCommand::CheckTemp);
            }
        }
    }
}
