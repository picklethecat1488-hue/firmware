//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]

use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use model::types::{SystemLedState, SystemStatus};

/// One-way commands to control the global system state and notify it of events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCommand {
    /// Transition the system to Active state.
    Wake,
    /// Transition the system to low-power Sleep state.
    Sleep,
    /// Notify system of activity, resetting inactivity timer and waking up if asleep.
    ActivityDetected,
    /// Low water warning or thermal safety alert occurred.
    AlertTriggered,
    /// Battery level updates from the fuel gauge.
    BatteryUpdate {
        /// Battery capacity percentage (0-100).
        state_of_charge: u8,
        /// Charger status (whether currently charging).
        charging: bool,
    },
}

/// Controller responsible for tracking global status and coordinating other subsystems.
pub struct SystemController<MutexRaw: RawMutex + 'static, const N: usize> {
    status: SystemStatus,
    inactivity_seconds: u32,
    motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    sensor_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
    thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
    led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> SystemController<MutexRaw, N> {
    /// Creates a new SystemController instance.
    pub const fn new(
        motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
        sensor_tx: Sender<'static, MutexRaw, SensorCommand, N>,
        battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
        thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
        led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    ) -> Self {
        Self {
            status: SystemStatus::Active,
            inactivity_seconds: 0,
            motor_tx,
            sensor_tx,
            battery_tx,
            thermal_tx,
            led_tx,
        }
    }

    /// Gets the current system status.
    pub fn status(&self) -> SystemStatus {
        self.status
    }

    /// Handles an incoming SystemCommand.
    pub fn handle_command(&mut self, cmd: SystemCommand) {
        match cmd {
            SystemCommand::Wake => {
                if self.status != SystemStatus::Active {
                    self.status = SystemStatus::Active;
                    self.inactivity_seconds = 0;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::info!("SystemController: waking up to Active mode.");
                    let _ = self.led_tx.try_send(SystemLedState::Rgb(0, 128, 0));
                    // Active green
                }
            }
            SystemCommand::Sleep => {
                if self.status != SystemStatus::Sleep {
                    self.status = SystemStatus::Sleep;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::info!("SystemController: entering low-power Sleep mode.");
                    // Stop motor to preserve energy in sleep state
                    let _ = self.motor_tx.try_send(MotorCommand::Stop);
                    let _ = self.led_tx.try_send(SystemLedState::Rgb(0, 0, 64));
                    // Sleep dim blue
                }
            }
            SystemCommand::ActivityDetected => {
                self.inactivity_seconds = 0;
                if self.status == SystemStatus::Sleep {
                    self.handle_command(SystemCommand::Wake);
                }
            }
            SystemCommand::AlertTriggered => {
                // Trigger warning alert (Red LED indicator)
                let _ = self.led_tx.try_send(SystemLedState::Rgb(255, 0, 0));
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::warn!("SystemController: Alert triggered. LED indicator set to RED.");
            }
            SystemCommand::BatteryUpdate {
                state_of_charge,
                charging,
            } => {
                if charging {
                    let _ = self.led_tx.try_send(SystemLedState::Rgb(128, 128, 0));
                // Charging yellow
                } else if state_of_charge < 20 {
                    let _ = self.led_tx.try_send(SystemLedState::Rgb(128, 64, 0));
                // Battery low orange
                } else if self.status == SystemStatus::Active {
                    let _ = self.led_tx.try_send(SystemLedState::Rgb(0, 128, 0));
                    // Active green
                }
            }
        }
    }

    /// Ticks the inactivity timer (called once per second).
    pub fn tick(&mut self) {
        if self.status == SystemStatus::Active {
            self.inactivity_seconds += 1;
            // Sleep after 30 seconds of inactivity
            if self.inactivity_seconds >= 30 {
                self.handle_command(SystemCommand::Sleep);
            }
        }
    }

    /// Main execution loop.
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, SystemCommand, N>,
    ) -> ! {
        // Initialize LED to green at start
        let _ = self.led_tx.try_send(SystemLedState::Rgb(0, 128, 0));

        loop {
            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(1000),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => {
                    self.handle_command(cmd);
                }
                Err(_timeout) => {
                    self.tick();
                    // Coordinate periodic telemetry reads across other controllers
                    let _ = self.battery_tx.try_send(BatteryCommand::CheckStatus);
                    let _ = self.thermal_tx.try_send(ThermalCommand::CheckTemp);
                    if self.status == SystemStatus::Active {
                        let _ = self.sensor_tx.try_send(SensorCommand::ReadSensors);
                    }
                }
            }
        }
    }
}

/// A macro to define and spawn the System Controller task.
///
/// Generates the task definition, then spawns it on the provided Embassy spawner.
#[macro_export]
macro_rules! run_system_task {
    (
        $spawner:expr,
        $task_module:ident,
        $controller:expr,
        $rx:expr
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                controller: $crate::system_controller::SystemController<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    4,
                >,
                rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::system_controller::SystemCommand,
                    4,
                >,
            ) {
                controller.run(rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $rx))
            .unwrap();
    };
}

#[cfg(test)]
#[path = "system_controller_test.rs"]
mod tests;
