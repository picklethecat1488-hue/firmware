//! Battery status and telemetry controller.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use model::interfaces::FuelGauge;

/// Current operating state of the battery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    /// Battery voltage is normal.
    Ok,
    /// Battery voltage is low.
    Low,
}

/// A controller that periodically monitors battery status.
pub struct BatteryController<'a, M: RawMutex, B, Cmd = ()> {
    battery: &'a Mutex<M, B>,
    state: BatteryState,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    update_fn: Option<fn(u8, bool) -> Cmd>,
}

impl<'a, M: RawMutex, B: FuelGauge, Cmd: Clone> BatteryController<'a, M, B, Cmd> {
    /// Creates a new battery controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>) -> Self {
        Self {
            battery,
            state: BatteryState::Ok,
            system_tx: None,
            update_fn: None,
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
            let _ = tx.try_send(f(soc, charging));
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
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(2000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => match cmd {
                    BatteryCommand::CheckStatus => {
                        let _ = self.update(Some(&telemetry_tx)).await;
                    }
                },
                Err(_timeout) => {
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
