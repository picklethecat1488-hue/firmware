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

/// A controller that periodically monitors system temperature from temperature sensors.
pub struct ThermalController<'a, M: RawMutex, B, Cmd = ()> {
    temp: &'a Mutex<M, B>,
    system_tx: Option<embassy_sync::channel::Sender<'a, M, Cmd, 4>>,
    shutdown_cmd: Option<Cmd>,
    state: ThermalState,
    overheating_temp_milli_c: i32,
    critical_temp_milli_c: i32,
    hysteresis_temp_milli_c: i32,
}

impl<'a, M: RawMutex, B: TemperatureSensor, Cmd: Clone + core::fmt::Debug>
    ThermalController<'a, M, B, Cmd>
{
    /// Creates a new thermal controller referencing a shared temperature peripheral without shutdown coordination.
    pub fn new(temp: &'a Mutex<M, B>) -> Self {
        Self {
            temp,
            system_tx: None,
            shutdown_cmd: None,
            state: ThermalState::Normal,
            overheating_temp_milli_c: 45000,
            critical_temp_milli_c: 60000,
            hysteresis_temp_milli_c: 2000,
        }
    }

    /// Creates a new thermal controller with safety shutdown capabilities.
    pub fn new_with_shutdown(
        temp: &'a Mutex<M, B>,
        system_tx: embassy_sync::channel::Sender<'a, M, Cmd, 4>,
        shutdown_cmd: Cmd,
    ) -> Self {
        Self {
            temp,
            system_tx: Some(system_tx),
            shutdown_cmd: Some(shutdown_cmd),
            state: ThermalState::Normal,
            overheating_temp_milli_c: 45000,
            critical_temp_milli_c: 60000,
            hysteresis_temp_milli_c: 2000,
        }
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
            let mut sensor = self.temp.lock().await;
            sensor.read_temperature_milli_c()?
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

impl<'a, M: RawMutex, B: TemperatureSensor, Cmd: Clone + core::fmt::Debug>
    crate::BlockingThermalReader for ThermalController<'a, M, B, Cmd>
{
    fn read_temperature_blocking(&self) -> Option<i32> {
        if let Ok(mut guard) = self.temp.try_lock() {
            guard.read_temperature_milli_c().ok()
        } else {
            None
        }
    }
}

/// One-way commands sent to the Thermal Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalCommand {
    /// Force thermal status query and print telemetry logs
    CheckTemp,
}
