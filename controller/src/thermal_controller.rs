//! Thermal monitoring and regulation controller.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
use embassy_sync::mutex::Mutex;
use model::interfaces::TemperatureSensor;

/// Current thermal status of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    /// System temperature is normal.
    Normal,
    /// System is overheating.
    Overheating,
}

/// A controller that periodically monitors system temperature from battery sensors.
pub struct ThermalController<'a, M: RawMutex, B, Cmd = ()> {
    battery: &'a Mutex<M, B>,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    shutdown_cmd: Option<Cmd>,
    state: ThermalState,
}

impl<'a, M: RawMutex, B: TemperatureSensor, Cmd: Clone + core::fmt::Debug>
    ThermalController<'a, M, B, Cmd>
{
    /// Creates a new thermal controller referencing a shared battery peripheral without shutdown coordination.
    pub fn new(battery: &'a Mutex<M, B>) -> Self {
        Self {
            battery,
            system_tx: None,
            shutdown_cmd: None,
            state: ThermalState::Normal,
        }
    }

    /// Creates a new thermal controller with safety shutdown capabilities.
    pub fn new_with_shutdown(
        battery: &'a Mutex<M, B>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        shutdown_cmd: Cmd,
    ) -> Self {
        Self {
            battery,
            system_tx: Some(system_tx),
            shutdown_cmd: Some(shutdown_cmd),
            state: ThermalState::Normal,
        }
    }

    /// Gets the current state of the thermal system.
    pub fn state(&self) -> ThermalState {
        self.state
    }

    /// Updates the thermal status by locking and reading the peripheral.
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
        let temp = {
            let mut bat = self.battery.lock().await;
            bat.read_temperature_milli_c()?
        };

        if temp > 45000 {
            self.state = ThermalState::Overheating;
        } else {
            self.state = ThermalState::Normal;
        }

        // Critical threshold check: shut down system if temp > 60°C (60000 mC)
        if temp > 60000 {
            if let (Some(tx), Some(cmd)) = (&self.system_tx, &self.shutdown_cmd) {
                tx.try_send(cmd.clone()).unwrap();
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::error!("Thermal Controller: Critical temperature exceeded ({} mC). Dispatching safety shutdown.", temp);
            }
        }

        if let Some(tx) = telemetry_tx {
            let overheating = self.state == ThermalState::Overheating;
            let status = model::types::ThermalStatus::TempOverheating(temp, overheating);
            let _ = tx.try_send(model::telemetry::TelemetryRecord::Thermal(status));
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        defmt::info!(
            "Thermal Controller: Temp is {} mC, State: {:?}",
            temp,
            defmt::Debug2Format(&self.state)
        );

        Ok(())
    }

    /// Starts the controller's main infinite run loop, processing commands.
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, M, ThermalCommand, 4>,
        telemetry_tx: embassy_sync::channel::Sender<
            'static,
            CriticalSectionRawMutex,
            model::telemetry::TelemetryRecord,
            16,
        >,
    ) -> ! {
        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1500),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => match cmd {
                    ThermalCommand::CheckTemp => {
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

/// One-way commands sent to the Thermal Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalCommand {
    /// Force thermal status query and print telemetry logs
    CheckTemp,
}
