//! Thermal monitoring and regulation controller.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use peripherals::battery::Battery;

/// Current thermal status of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    /// System temperature is normal.
    Normal,
    /// System is overheating.
    Overheating,
}

/// A controller that periodically monitors system temperature from battery sensors.
pub struct ThermalController<'a, M: RawMutex, B> {
    battery: &'a Mutex<M, B>,
    state: ThermalState,
}

impl<'a, M: RawMutex, B: Battery> ThermalController<'a, M, B> {
    /// Creates a new thermal controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>) -> Self {
        Self {
            battery,
            state: ThermalState::Normal,
        }
    }

    /// Gets the current state of the thermal system.
    pub fn state(&self) -> ThermalState {
        self.state
    }

    /// Updates the thermal status by locking and reading the peripheral.
    pub async fn update(&mut self) -> Result<(), B::Error> {
        let temp = {
            let mut bat = self.battery.lock().await;
            bat.read_temperature_milli_c()?
        };

        if temp > 45000 {
            self.state = ThermalState::Overheating;
        } else {
            self.state = ThermalState::Normal;
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
                        let _ = self.update().await;
                    }
                },
                Err(_timeout) => {
                    let _ = self.update().await;
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
