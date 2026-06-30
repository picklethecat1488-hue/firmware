//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]

use firmware_lib::gesture_detector::GestureDetector;
use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use model::types::{Gesture, SystemLedState, SystemStatus};

/// One-way commands to control the global system state and notify it of events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCommand {
    /// Transition the system to Active state.
    Wake,
    /// Transition the system to low-power Sleep state.
    Sleep,
    /// Transition the system to PowerDown state.
    PowerDown,
    /// Notify system of activity, resetting inactivity timer and waking up if asleep.
    ActivityDetected,
    /// Thermal safety or motor stall alert occurred.
    AlertTriggered,
    /// Battery level updates from the fuel gauge.
    BatteryUpdate {
        /// Battery capacity percentage (0-100).
        state_of_charge: u8,
        /// Charger status (whether currently charging).
        charging: bool,
    },
    /// Proximity telemetry update from individual ToF sensors.
    SensorUpdate {
        /// Sensor ID (0 = North, 1 = East, 2 = West).
        sensor_id: u8,
        /// Measured distance in mm.
        distance_mm: u16,
    },
}

/// Controller responsible for tracking global status and coordinating other subsystems.
pub struct SystemController<MutexRaw: RawMutex + 'static, const N: usize> {
    status: SystemStatus,
    inactivity_seconds: u32,
    motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    sensor_north_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_east_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_west_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
    thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
    led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    distance_north: u16,
    distance_east: u16,
    distance_west: u16,
    time_in_active: u32,
    battery_critical: bool,
    thermal_critical: bool,
    gesture_detector: GestureDetector,
    proximity_active: bool,
    boot_power_down: bool,
}

impl<MutexRaw: RawMutex + 'static, const N: usize> SystemController<MutexRaw, N> {
    /// Creates a new SystemController instance.
    pub const fn new(
        motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
        sensor_north_tx: Sender<'static, MutexRaw, SensorCommand, N>,
        sensor_east_tx: Sender<'static, MutexRaw, SensorCommand, N>,
        sensor_west_tx: Sender<'static, MutexRaw, SensorCommand, N>,
        battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
        thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
        led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    ) -> Self {
        Self {
            status: SystemStatus::PowerDown,
            inactivity_seconds: 0,
            motor_tx,
            sensor_north_tx,
            sensor_east_tx,
            sensor_west_tx,
            battery_tx,
            thermal_tx,
            led_tx,
            distance_north: 1000,
            distance_east: 1000,
            distance_west: 1000,
            time_in_active: 0,
            battery_critical: true,
            thermal_critical: false,
            gesture_detector: GestureDetector::new(100),
            proximity_active: false,
            boot_power_down: true,
        }
    }

    /// Gets the current system status.
    pub fn status(&self) -> SystemStatus {
        self.status
    }

    /// Updates the gesture detector with the current system time in microseconds.
    pub fn update_gesture(&mut self, current_time_us: u64) {
        if self.status != SystemStatus::PowerDown {
            match self.gesture_detector.update(
                Gesture::Proximity(self.distance_north, self.distance_east, self.distance_west),
                current_time_us,
            ) {
                Some(Gesture::DualLongPress) => {
                    self.handle_command(SystemCommand::PowerDown);
                }
                Some(Gesture::ProximityDetected) => {
                    self.proximity_active = true;
                    self.inactivity_seconds = 0;
                    if self.status == SystemStatus::Sleep {
                        self.handle_command(SystemCommand::Wake);
                    }
                    if self.status == SystemStatus::Active && !self.battery_critical {
                        let _ = self.motor_tx.try_send(MotorCommand::SetSpeed(100));
                    }
                }
                Some(Gesture::ProximityNotDetected) => {
                    self.proximity_active = false;
                }
                _ => {}
            }
        }
    }

    /// Handles an incoming SystemCommand.
    pub fn handle_command(&mut self, cmd: SystemCommand) {
        match cmd {
            SystemCommand::Wake => {
                if !self.battery_critical
                    && !self.thermal_critical
                    && self.status != SystemStatus::Active
                    && !self.boot_power_down
                    && self.status != SystemStatus::PowerDown
                {
                    self.status = SystemStatus::Active;
                    self.inactivity_seconds = 0;
                    self.time_in_active = 0;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    crate::log_info!("SystemController: waking up to Active mode.");
                    let _ = self.led_tx.try_send(SystemLedState::SolidGreen);
                }
            }
            SystemCommand::Sleep => {
                let can_sleep =
                    self.time_in_active >= 30 || self.battery_critical || self.thermal_critical;
                if can_sleep
                    && self.status != SystemStatus::Sleep
                    && self.status != SystemStatus::PowerDown
                {
                    self.status = SystemStatus::Sleep;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    crate::log_info!("SystemController: entering low-power Sleep mode.");
                    let _ = self.motor_tx.try_send(MotorCommand::Stop);
                    let _ = self.led_tx.try_send(SystemLedState::SolidBlue);
                }
            }
            SystemCommand::PowerDown => {
                if self.status != SystemStatus::PowerDown {
                    self.status = SystemStatus::PowerDown;
                    let _ = self.motor_tx.try_send(MotorCommand::Stop);
                    let _ = self.led_tx.try_send(SystemLedState::Off);
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    crate::log_info!("SystemController: entering PowerDown state. Motor locked.");
                }
            }
            SystemCommand::ActivityDetected => {
                self.inactivity_seconds = 0;
                if self.status == SystemStatus::Sleep {
                    self.handle_command(SystemCommand::Wake);
                }
            }
            SystemCommand::AlertTriggered => {
                self.thermal_critical = true;
                let _ = self.led_tx.try_send(SystemLedState::BlinksRedFourTimes);
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                crate::log_info!("SystemController: Alert triggered. LED indicator set to RED.");
                if self.status != SystemStatus::PowerDown {
                    self.handle_command(SystemCommand::Sleep);
                }
            }
            SystemCommand::BatteryUpdate {
                state_of_charge,
                charging,
            } => {
                if state_of_charge < 10 && !charging {
                    self.battery_critical = true;
                    let _ = self
                        .led_tx
                        .try_send(SystemLedState::BlinksRedOncePerThirtySeconds);
                    if self.status != SystemStatus::PowerDown {
                        self.handle_command(SystemCommand::PowerDown);
                    } else {
                        let _ = self.motor_tx.try_send(MotorCommand::Stop);
                    }
                } else {
                    self.battery_critical = false;

                    let should_exit_power_down = if self.status == SystemStatus::PowerDown {
                        if self.boot_power_down {
                            true
                        } else {
                            charging
                        }
                    } else {
                        false
                    };

                    if should_exit_power_down {
                        self.status = SystemStatus::Active;
                        self.boot_power_down = false;
                        self.inactivity_seconds = 0;
                        self.time_in_active = 0;
                        if charging {
                            let _ = self.led_tx.try_send(SystemLedState::SolidYellow);
                        } else {
                            let _ = self.led_tx.try_send(SystemLedState::SolidGreen);
                        }
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        crate::log_info!(
                            "SystemController: exiting PowerDown state. Waking up to Active mode."
                        );
                    } else if self.status != SystemStatus::PowerDown {
                        self.boot_power_down = false;
                        if charging {
                            let _ = self.led_tx.try_send(SystemLedState::SolidYellow);
                        } else if state_of_charge < 20 {
                            let _ = self.led_tx.try_send(SystemLedState::SolidOrange);
                        } else if self.status == SystemStatus::Active {
                            let _ = self.led_tx.try_send(SystemLedState::SolidGreen);
                        }
                    }
                }
            }
            SystemCommand::SensorUpdate {
                sensor_id,
                distance_mm,
            } => {
                match sensor_id {
                    0 => self.distance_north = distance_mm,
                    1 => self.distance_east = distance_mm,
                    2 => self.distance_west = distance_mm,
                    _ => {}
                }

                self.update_gesture(crate::system_time());
            }
        }
    }

    /// Ticks the inactivity timer and active mode duration timer (called once per second).
    pub fn tick(&mut self) {
        if self.status == SystemStatus::Active {
            self.time_in_active += 1;

            // Stay in Active state as long as proximity is detected
            if self.proximity_active {
                self.inactivity_seconds = 0;
            } else {
                self.inactivity_seconds += 1;
            }

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
        // Initialize LED to Off (as we start in PowerDown)
        let _ = self.led_tx.try_send(SystemLedState::Off);

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
                        let _ = self.sensor_north_tx.try_send(SensorCommand::ReadSensors);
                        let _ = self.sensor_east_tx.try_send(SensorCommand::ReadSensors);
                        let _ = self.sensor_west_tx.try_send(SensorCommand::ReadSensors);
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
