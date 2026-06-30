//! Battery status and telemetry controller.

#![deny(missing_docs)]

use embassy_sync::blocking_mutex::raw::RawMutex;
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
pub struct BatteryController<'a, M: RawMutex, B> {
    battery: &'a Mutex<M, B>,
    state: BatteryState,
}

impl<'a, M: RawMutex, B: FuelGauge> BatteryController<'a, M, B> {
    /// Creates a new battery controller referencing a shared battery peripheral.
    pub fn new(battery: &'a Mutex<M, B>) -> Self {
        Self {
            battery,
            state: BatteryState::Ok,
        }
    }

    /// Gets the current state of the battery.
    pub fn state(&self) -> BatteryState {
        self.state
    }

    /// Updates the battery status by locking and reading the peripheral.
    pub async fn update(&mut self) -> Result<(), B::Error> {
        let voltage = {
            let mut bat = self.battery.lock().await;
            bat.read_voltage_mv()?
        };

        if voltage < 3500 {
            self.state = BatteryState::Low;
        } else {
            self.state = BatteryState::Ok;
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

/// One-way commands sent to the Battery Controller from the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryCommand {
    /// Force battery status query and print telemetry logs
    CheckStatus,
}
