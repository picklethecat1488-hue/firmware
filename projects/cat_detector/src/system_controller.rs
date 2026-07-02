//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]
#![allow(clippy::collapsible_match)]
use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use firmware_lib::gesture_detector::GestureDetector;
use model::types::{Direction, Gesture, SystemLedState, SystemStatus, TelemetryRecord};

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
        /// Charger state.
        charger_state: model::types::ChargeState,
    },
    /// Proximity telemetry update from individual ToF sensors.
    SensorUpdate {
        /// Sensor direction (North, East, West).
        direction: Direction,
        /// Measured distance in mm.
        distance_mm: u16,
    },
}

/// The default inactivity timeout in seconds before transitioning to Sleep.
pub const INACTIVITY_TIMEOUT_SECONDS: u32 = 30;
/// The state of charge threshold under which battery is considered low.
pub const LOW_BATTERY_SOC_THRESHOLD: u8 = 20;
/// The state of charge threshold under which battery is considered medium.
pub const MID_BATTERY_SOC_THRESHOLD: u8 = 21;
/// The state of charge threshold under which battery is considered high.
pub const HIGH_BATTERY_SOC_THRESHOLD: u8 = 80;

const _: () = {
    assert!(
        LOW_BATTERY_SOC_THRESHOLD < MID_BATTERY_SOC_THRESHOLD,
        "Low battery threshold must be lower than the mid battery threshold"
    );
    assert!(
        MID_BATTERY_SOC_THRESHOLD < HIGH_BATTERY_SOC_THRESHOLD,
        "Mid battery threshold must be lower than the high battery threshold"
    );
};

/// Group of channels for coordinating control tasks from SystemController.
pub struct SystemControllerChannels<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    const T_CAP: usize,
> {
    /// Motor channel sender
    pub motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    /// Sensor North channel sender
    pub sensor_north_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    /// Sensor East channel sender
    pub sensor_east_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    /// Sensor West channel sender
    pub sensor_west_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    /// Battery channel sender
    pub battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
    /// Thermal channel sender
    pub thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
    /// LED channel sender
    pub led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    /// Telemetry channel sender
    pub telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, T_CAP>,
}

/// Controller responsible for tracking global status and coordinating other subsystems.
/// Controller responsible for tracking global status and coordinating other subsystems.
pub struct SystemController<MutexRaw: RawMutex + 'static, const N: usize, const T_CAP: usize = 16> {
    state_manager: firmware_lib::system::SystemStateManager<MutexRaw, T_CAP>,
    motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    sensor_north_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_east_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_west_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
    thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
    led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    /// Distance reading from the North sensor.
    pub distance_north: u16,
    /// Distance reading from the East sensor.
    pub distance_east: u16,
    /// Distance reading from the West sensor.
    pub distance_west: u16,
    gesture_detector: GestureDetector,
    proximity_active: bool,
    /// Proximity detection threshold in mm.
    pub proximity_threshold_mm: u16,
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const T_CAP: usize> core::ops::Deref
    for SystemController<MutexRaw, N, T_CAP>
{
    type Target = firmware_lib::system::SystemStateManager<MutexRaw, T_CAP>;

    fn deref(&self) -> &Self::Target {
        &self.state_manager
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const T_CAP: usize> core::ops::DerefMut
    for SystemController<MutexRaw, N, T_CAP>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state_manager
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const T_CAP: usize>
    SystemController<MutexRaw, N, T_CAP>
{
    /// Creates a new SystemController instance.
    pub fn new(
        channels: SystemControllerChannels<MutexRaw, N, T_CAP>,
        proximity_threshold_mm: u16,
    ) -> Self {
        let state_manager = firmware_lib::system::SystemStateManager::new(
            10, // critical_soc_threshold default
            2,  // soc_hysteresis default
            LOW_BATTERY_SOC_THRESHOLD,
            MID_BATTERY_SOC_THRESHOLD,
            HIGH_BATTERY_SOC_THRESHOLD,
            channels.telemetry_tx,
        );

        Self {
            state_manager,
            motor_tx: channels.motor_tx,
            sensor_north_tx: channels.sensor_north_tx,
            sensor_east_tx: channels.sensor_east_tx,
            sensor_west_tx: channels.sensor_west_tx,
            battery_tx: channels.battery_tx,
            thermal_tx: channels.thermal_tx,
            led_tx: channels.led_tx,
            distance_north: 1000,
            distance_east: 1000,
            distance_west: 1000,
            gesture_detector: GestureDetector::new(20, proximity_threshold_mm),
            proximity_active: false,
            proximity_threshold_mm,
        }
    }

    /// Gets the current system status.
    pub fn status(&self) -> SystemStatus {
        self.state_manager.status()
    }

    /// Determines the LED state based on current battery state of charge.
    pub fn get_soc_led_state(&self) -> SystemLedState {
        self.state_manager.get_soc_led_state()
    }

    /// Updates the gesture detector with the current system time in microseconds.
    pub fn update_gesture(&mut self, current_time_us: u64) {
        let current_status = self.status();
        match self.gesture_detector.update(
            Gesture::Proximity(self.distance_north, self.distance_east, self.distance_west),
            current_time_us,
        ) {
            Some(Gesture::DualLongPress) => {
                self.log_gesture_telemetry(Gesture::DualLongPress);
                if current_status == SystemStatus::PowerDown {
                    if !self.charger_connected() {
                        self.set_status(SystemStatus::Active);
                        self.reset_on_wake();
                        self.led_tx.try_send(self.get_soc_led_state()).unwrap();
                        self.gesture_detector.reset();
                    }
                } else {
                    self.handle_command(SystemCommand::PowerDown);
                }
            }
            Some(Gesture::ProximityDetected) if current_status != SystemStatus::PowerDown => {
                self.log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityDetected));
                self.proximity_active = true;
                self.set_inactivity_seconds(0);
                if self.status() == SystemStatus::Sleep {
                    self.handle_command(SystemCommand::Wake);
                }
                if self.status() == SystemStatus::Active
                    && !self.battery_critical()
                    && !self.thermal_critical()
                {
                    self.motor_tx.try_send(MotorCommand::SetSpeed(100)).unwrap();
                }
            }
            Some(Gesture::ProximityNotDetected) if current_status != SystemStatus::PowerDown => {
                self.log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityNotDetected));
                self.proximity_active = false;
            }
            Some(Gesture::ProximityDetected) => {
                if current_status != SystemStatus::PowerDown {
                    self.log_gesture_telemetry(Gesture::ProximityDetected);
                    self.proximity_active = true;
                    self.set_inactivity_seconds(0);
                    if self.status() == SystemStatus::Sleep {
                        self.handle_command(SystemCommand::Wake);
                    }
                    if self.status() == SystemStatus::Active
                        && !self.battery_critical()
                        && !self.thermal_critical()
                    {
                        self.motor_tx.try_send(MotorCommand::SetSpeed(100)).unwrap();
                    }
                }
            }
            Some(Gesture::ProximityNotDetected) => {
                if current_status != SystemStatus::PowerDown {
                    self.log_gesture_telemetry(Gesture::ProximityNotDetected);
                    self.proximity_active = false;
                }
            }

            _ => {}
        }
    }

    /// Handles an incoming SystemCommand.
    pub fn handle_command(&mut self, cmd: SystemCommand) {
        match cmd {
            SystemCommand::Wake => {
                if let Some(next) = firmware_lib::system::transition_wake(
                    self.status(),
                    self.battery_critical(),
                    self.thermal_critical(),
                    self.boot_power_down(),
                ) {
                    self.set_status(next);
                    self.reset_on_wake();
                    self.led_tx.try_send(self.get_soc_led_state()).unwrap();
                }
            }
            SystemCommand::Sleep => {
                if let Some(next) = firmware_lib::system::transition_sleep(
                    self.status(),
                    self.time_in_active(),
                    INACTIVITY_TIMEOUT_SECONDS,
                    self.battery_critical(),
                    self.thermal_critical(),
                ) {
                    self.set_status(next);
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    crate::log_info!("SystemController: entering low-power Sleep mode.");
                    self.motor_tx.try_send(MotorCommand::Stop).unwrap();
                    self.led_tx.try_send(SystemLedState::SolidBlue).unwrap();
                }
            }
            SystemCommand::PowerDown => {
                if let Some(next) = firmware_lib::system::transition_power_down(self.status()) {
                    self.set_status(next);
                    self.motor_tx.try_send(MotorCommand::Stop).unwrap();
                    let led = if self.charger_connected() {
                        self.get_soc_led_state()
                    } else {
                        SystemLedState::Off
                    };
                    self.led_tx.try_send(led).unwrap();
                    self.gesture_detector.reset();
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    crate::log_info!("SystemController: entering PowerDown state. Motor locked.");
                }
            }
            SystemCommand::ActivityDetected => {
                self.set_inactivity_seconds(0);
                if self.status() == SystemStatus::Sleep {
                    self.handle_command(SystemCommand::Wake);
                }
            }
            SystemCommand::AlertTriggered => {
                self.set_thermal_critical(true);
                self.led_tx
                    .try_send(SystemLedState::BlinksRedFourTimes)
                    .unwrap();
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                crate::log_info!("SystemController: Alert triggered. LED indicator set to RED.");
                if self.status() != SystemStatus::PowerDown {
                    self.handle_command(SystemCommand::Sleep);
                }
            }
            SystemCommand::BatteryUpdate {
                state_of_charge,
                charger_state,
            } => {
                assert!(
                    self.critical_soc_threshold() < LOW_BATTERY_SOC_THRESHOLD,
                    "Critical SoC threshold must be lower than the low battery threshold"
                );
                let charging = charger_state == model::types::ChargeState::Charging;
                let is_fault = charger_state == model::types::ChargeState::RecoverableFault
                    || charger_state == model::types::ChargeState::NonRecoverableFault;

                let _changed = self.update_battery_status(state_of_charge, charging, is_fault);

                if self.battery_critical() {
                    self.led_tx
                        .try_send(SystemLedState::BlinksRedOncePerThirtySeconds)
                        .unwrap();
                    if self.status() != SystemStatus::PowerDown {
                        self.handle_command(SystemCommand::PowerDown);
                    } else {
                        self.motor_tx.try_send(MotorCommand::Stop).unwrap();
                    }
                } else {
                    let should_exit_power_down = self.status() == SystemStatus::PowerDown
                        && self.boot_power_down()
                        && !charging;

                    if should_exit_power_down {
                        self.set_status(SystemStatus::Active);
                        self.reset_on_wake();
                        self.led_tx.try_send(self.get_soc_led_state()).unwrap();
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        crate::log_info!(
                            "SystemController: exiting PowerDown state. Waking up to Active mode."
                        );
                    } else if self.status() == SystemStatus::PowerDown {
                        let led = if charging {
                            self.get_soc_led_state()
                        } else {
                            SystemLedState::Off
                        };
                        self.led_tx.try_send(led).unwrap();
                    } else {
                        self.set_boot_power_down(false);
                        if charging {
                            self.handle_command(SystemCommand::PowerDown);
                        } else if self.status() == SystemStatus::Active {
                            self.led_tx.try_send(self.get_soc_led_state()).unwrap();
                        }
                    }
                }
            }
            SystemCommand::SensorUpdate {
                direction,
                distance_mm,
            } => {
                match direction {
                    Direction::North => self.distance_north = distance_mm,
                    Direction::East => self.distance_east = distance_mm,
                    Direction::West => self.distance_west = distance_mm,
                }

                self.log_proximity_telemetry(distance_mm, self.proximity_threshold_mm);

                self.update_gesture(crate::system_time());
            }
        }
    }

    /// Ticks the inactivity timer and active mode duration timer (called once per second).
    pub fn tick(&mut self) {
        self.tick_ms(1000);
    }

    /// Ticks the inactivity timer and active mode duration timer by a specified duration in milliseconds.
    pub fn tick_ms(&mut self, ms: u32) {
        if self.state_manager.tick_ms(ms) {
            // Stay in Active state as long as proximity is detected
            if self.proximity_active {
                self.set_inactivity_seconds(0);
            } else {
                let current_inactivity = self.inactivity_seconds();
                self.set_inactivity_seconds(current_inactivity + 1);
            }

            // Sleep after inactivity timeout
            if self.inactivity_seconds() >= INACTIVITY_TIMEOUT_SECONDS {
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
        self.led_tx.try_send(SystemLedState::Off).unwrap();
        self.log_telemetry(TelemetryRecord::System(SystemStatus::PowerDown));

        let mut last_tick_time = embassy_time::Instant::now();
        loop {
            let timeout_duration = if self.proximity_active {
                embassy_time::Duration::from_millis(200)
            } else {
                embassy_time::Duration::from_millis(1000)
            };
            let now = embassy_time::Instant::now();
            let elapsed_ms = now.duration_since(last_tick_time).as_millis() as u32;
            let remaining_ms = if elapsed_ms >= timeout_duration.as_millis() as u32 {
                0
            } else {
                (timeout_duration.as_millis() as u32) - elapsed_ms
            };

            if remaining_ms == 0 {
                last_tick_time = now;
                self.tick_ms(elapsed_ms);
                // Coordinate periodic telemetry reads across other controllers
                self.battery_tx
                    .try_send(BatteryCommand::CheckStatus)
                    .unwrap();
                self.thermal_tx.try_send(ThermalCommand::CheckTemp).unwrap();
                if self.status() == SystemStatus::Active {
                    self.sensor_north_tx
                        .try_send(SensorCommand::ReadSensors)
                        .unwrap();
                    self.sensor_east_tx
                        .try_send(SensorCommand::ReadSensors)
                        .unwrap();
                    self.sensor_west_tx
                        .try_send(SensorCommand::ReadSensors)
                        .unwrap();
                }
                continue;
            }

            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(remaining_ms as u64),
                command_rx.receive(),
            )
            .await
            {
                Ok(cmd) => {
                    self.handle_command(cmd);
                }
                Err(_timeout) => {
                    // Timeout occurred
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
