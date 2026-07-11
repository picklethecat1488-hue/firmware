//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]

pub use crate::gesture_detector::ProximityEvent;
use crate::{BatteryCommand, MotorCommand, SensorCommand, ThermalCommand};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
use firmware_lib::{
    BatteryManager, BatteryUpdateAction, PeriodicTimer, PowerManager, ThermalManager,
};
use model::types::MotorSpeed;

/// One-way commands to control the global system state and notify it of events.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SystemCommand {
    /// Notify system of activity, resetting inactivity timer and waking up if asleep.
    ActivityDetected,
    /// Thermal safety or motor stall alert occurred.
    AlertTriggered,
    /// Battery level updates from the fuel gauge.
    BatteryUpdate {
        /// Battery capacity percentage (0-100).
        state_of_charge: u8,
        /// Charger state: Charging, Discharging, Fault, etc.
        charger_state: ChargeState,
    },
    /// High-level gesture detected.
    Gesture(Gesture),
    /// The system status/power state changed.
    StateChanged {
        /// The previous system status.
        from: SystemStatus,
        /// The new system status.
        to: SystemStatus,
    },
    /// A battery action was triggered and processed.
    BatteryAction(BatteryUpdateAction),
}

impl crate::battery_controller::FromBatteryUpdate for SystemCommand {
    fn from_battery_update(state_of_charge: u8, charger_state: ChargeState) -> Self {
        SystemCommand::BatteryUpdate {
            state_of_charge,
            charger_state,
        }
    }
}

use model::types::{
    BootReason, ChargeState, Gesture, SystemLedState, SystemStatus, TelemetryRecord,
};

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
        LOW_BATTERY_SOC_THRESHOLD > 0,
        "Low battery threshold be nonzero"
    );
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
    const M: usize,
    const T_CAP: usize,
> {
    /// System command channel sender
    pub system_tx: Sender<'static, MutexRaw, SystemCommand, 4>,
    /// Motor channel sender
    pub motor_tx: Option<Sender<'static, MutexRaw, MotorCommand, N>>,
    /// Sensor channel senders
    pub sensor_txs: [Sender<'static, MutexRaw, SensorCommand, N>; M],
    /// Battery channel sender
    pub battery_tx: Option<Sender<'static, MutexRaw, BatteryCommand, N>>,
    /// Thermal channel sender
    pub thermal_tx: Option<Sender<'static, MutexRaw, ThermalCommand, N>>,
    /// LED channel sender
    pub led_tx: Option<Sender<'static, MutexRaw, SystemLedState, N>>,
    /// Telemetry channel sender
    pub telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, T_CAP>,
}

/// Controller responsible for tracking global status and coordinating other subsystems.
pub struct SystemController<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    const M: usize,
    const T_CAP: usize = { crate::telemetry_controller::CHANNEL_CAPACITY },
> {
    /// Subsystem manager for power, transitions, and timers
    pub power_manager: PowerManager<MutexRaw, T_CAP>,
    /// Subsystem manager for battery thresholds and status
    pub battery_manager: BatteryManager,
    /// Subsystem manager for thermal monitoring alerts
    pub thermal_manager: ThermalManager,
    motor_tx: Option<Sender<'static, MutexRaw, MotorCommand, N>>,
    sensor_txs: [Sender<'static, MutexRaw, SensorCommand, N>; M],
    battery_tx: Option<Sender<'static, MutexRaw, BatteryCommand, N>>,
    thermal_tx: Option<Sender<'static, MutexRaw, ThermalCommand, N>>,
    led_tx: Option<Sender<'static, MutexRaw, SystemLedState, N>>,
}

/// Concrete type alias for SystemController using CriticalSectionRawMutex.
pub type AppSystemController<
    const M: usize,
    const T_CAP: usize = { crate::telemetry_controller::CHANNEL_CAPACITY },
> = SystemController<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, 4, M, T_CAP>;

impl<MutexRaw: RawMutex + 'static, const N: usize, const M: usize, const T_CAP: usize>
    SystemController<MutexRaw, N, M, T_CAP>
{
    /// Creates a new SystemController instance.
    pub fn new(
        channels: SystemControllerChannels<MutexRaw, N, M, T_CAP>,
        boot_reason: BootReason,
    ) -> Self {
        let power_manager = PowerManager::new(channels.telemetry_tx, boot_reason);
        let mut battery_manager = BatteryManager::new(
            10, // critical_soc_threshold default
            2,  // soc_hysteresis default
            LOW_BATTERY_SOC_THRESHOLD,
            MID_BATTERY_SOC_THRESHOLD,
            HIGH_BATTERY_SOC_THRESHOLD,
        );
        if battery_manager.critical_soc_threshold() >= LOW_BATTERY_SOC_THRESHOLD {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::error!("Critical SoC threshold must be lower than the low battery threshold");
            battery_manager.set_critical_soc_threshold(LOW_BATTERY_SOC_THRESHOLD - 1);
        }
        let thermal_manager = ThermalManager::new();

        Self {
            power_manager,
            battery_manager,
            thermal_manager,
            motor_tx: channels.motor_tx,
            sensor_txs: channels.sensor_txs,
            battery_tx: channels.battery_tx,
            thermal_tx: channels.thermal_tx,
            led_tx: channels.led_tx,
        }
    }

    /// Sets the current system status.
    pub fn set_status(
        &mut self,
        status: SystemStatus,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        if let Some(prev) = self.power_manager.set_status(
            status,
            self.battery_manager.battery_critical(),
            self.thermal_manager.thermal_critical(),
        )? {
            let _ = self.handle_command(SystemCommand::StateChanged {
                from: prev,
                to: status,
            });
        }
        Ok(())
    }

    /// Handles battery status updates and updates the internal critical flag.
    pub fn update_battery_status(
        &mut self,
        state_of_charge: u8,
        charger_state: ChargeState,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        let action = self.battery_manager.update_battery_status(
            state_of_charge,
            charger_state,
            self.power_manager.status(),
            self.power_manager.boot_power_down(),
        );

        if let Some(act) = action {
            self.handle_command(SystemCommand::BatteryAction(act))?;
        }
        Ok(())
    }

    /// Handles an incoming SystemCommand.
    pub fn handle_command(
        &mut self,
        cmd: SystemCommand,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        match cmd {
            SystemCommand::ActivityDetected => {
                self.power_manager.set_inactive_ms(0);
                if self.power_manager.status() == SystemStatus::Sleep
                    && !self.battery_manager.battery_critical()
                    && !self.thermal_manager.thermal_critical()
                {
                    self.set_status(SystemStatus::Active)?;
                }
            }
            SystemCommand::AlertTriggered => {
                self.thermal_manager.set_thermal_critical(true);
                if self.power_manager.status() != SystemStatus::PowerDown {
                    if self.power_manager.status() != SystemStatus::Sleep {
                        self.power_manager.clear_wake_locks();
                        self.set_status(SystemStatus::Sleep)?;
                    } else if let Some(ref led_tx) = self.led_tx {
                        let _ = led_tx.try_send(SystemLedState::BlinksRedFourTimes);
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!("SystemController: Alert triggered. LED indicator set to RED.");
            }
            SystemCommand::BatteryUpdate {
                state_of_charge,
                charger_state,
            } => {
                self.update_battery_status(state_of_charge, charger_state)?;
            }
            SystemCommand::BatteryAction(action) => match action {
                BatteryUpdateAction::GoToPowerDown => {
                    self.power_manager.clear_wake_locks();
                    self.set_status(SystemStatus::PowerDown)?;
                }
                BatteryUpdateAction::ClearBootTrap => {
                    self.power_manager.set_boot_power_down(false);
                    self.set_status(SystemStatus::Active)?;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::info!(
                        "SystemController: exiting PowerDown state. Waking up to Active mode."
                    );
                }
                BatteryUpdateAction::ReportSoC => {
                    if let Some(ref led_tx) = self.led_tx {
                        if self.battery_manager.battery_critical() {
                            let _ = led_tx.try_send(SystemLedState::BlinksRedOncePerThirtySeconds);
                        } else if self.power_manager.status() == SystemStatus::PowerDown {
                            let led = if self.battery_manager.charger_connected() {
                                self.battery_manager.get_soc_led_state()
                            } else {
                                SystemLedState::Off
                            };
                            let _ = led_tx.try_send(led);
                        } else if self.power_manager.status() == SystemStatus::Active {
                            let _ = led_tx.try_send(self.battery_manager.get_soc_led_state());
                        }
                    }
                }
            },
            SystemCommand::Gesture(gesture) => {
                let current_status = self.power_manager.status();
                match gesture {
                    Gesture::DualLongPress => {
                        self.power_manager
                            .log_gesture_telemetry(Gesture::DualLongPress);
                        if current_status == SystemStatus::PowerDown {
                            if !self.battery_manager.charger_connected() {
                                self.set_status(SystemStatus::Active)?;
                            }
                        } else {
                            self.power_manager.clear_wake_locks();
                            self.set_status(SystemStatus::PowerDown)?;
                        }
                    }
                    Gesture::ProximityDetected if current_status != SystemStatus::PowerDown => {
                        self.power_manager
                            .log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityDetected));
                        if self.power_manager.status() == SystemStatus::Sleep
                            && !self.battery_manager.battery_critical()
                            && !self.thermal_manager.thermal_critical()
                        {
                            self.set_status(SystemStatus::Active)?;
                        }
                        if self.power_manager.status() == SystemStatus::Active {
                            self.power_manager.acquire_wake_lock(None);
                        }
                    }
                    Gesture::ProximityNotDetected if current_status != SystemStatus::PowerDown => {
                        self.power_manager
                            .log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityNotDetected));
                        if self.power_manager.status() == SystemStatus::Active {
                            self.power_manager.release_wake_lock(None);
                        }
                    }
                    _ => {}
                }
            }
            SystemCommand::StateChanged { from: _, to } => match to {
                SystemStatus::Active => {
                    self.power_manager.reset_on_wake();
                    if let Some(ref led_tx) = self.led_tx {
                        let _ = led_tx.try_send(self.battery_manager.get_soc_led_state());
                    }
                    if let Some(ref motor_tx) = self.motor_tx {
                        let _ = motor_tx.try_send(MotorCommand::SetSpeed(MotorSpeed::MAX));
                    }
                }
                SystemStatus::Sleep | SystemStatus::PowerDown => {
                    if let Some(ref motor_tx) = self.motor_tx {
                        let _ = motor_tx.try_send(MotorCommand::Stop);
                    }
                    if let Some(ref led_tx) = self.led_tx {
                        let led = if to == SystemStatus::Sleep {
                            if self.thermal_manager.thermal_critical() {
                                SystemLedState::BlinksRedFourTimes
                            } else {
                                SystemLedState::SolidBlue
                            }
                        } else if self.battery_manager.battery_critical() {
                            SystemLedState::BlinksRedOncePerThirtySeconds
                        } else if self.battery_manager.charger_connected() {
                            self.battery_manager.get_soc_led_state()
                        } else {
                            SystemLedState::Off
                        };
                        let _ = led_tx.try_send(led);
                    }
                    if let Some(ref battery_tx) = self.battery_tx {
                        let _ = battery_tx.try_send(BatteryCommand::UpdateWakeLocks(0));
                    }
                }
            },
        }
        Ok(())
    }

    /// Ticks the inactivity timer and active mode duration timer by a specified duration in milliseconds.
    /// Returns true if the 1-second system tick boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        let crossed = self.power_manager.tick_ms(ms);
        if crossed {
            if let Some(ref battery_tx) = self.battery_tx {
                let _ = battery_tx.try_send(BatteryCommand::UpdateWakeLocks(
                    self.power_manager.wake_locks(),
                ));
            }
            // Sleep after inactivity timeout
            if self.power_manager.inactive_ms() >= INACTIVITY_TIMEOUT_SECONDS * 1000 {
                let _ = self.set_status(SystemStatus::Sleep);
            }
        }
        crossed
    }

    /// Main execution loop.
    pub async fn run(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, SystemCommand, N>,
        gesture_rx: embassy_sync::channel::Receiver<'static, MutexRaw, Gesture, 4>,
    ) -> ! {
        // Initialize LED to Off (as we start in PowerDown)
        if let Some(ref led_tx) = self.led_tx {
            led_tx.try_send(SystemLedState::Off).unwrap();
        }
        self.power_manager
            .log_telemetry(TelemetryRecord::System(SystemStatus::PowerDown));

        let mut timer = PeriodicTimer::new(embassy_time::Duration::from_millis(1000));
        loop {
            if let Some(elapsed_ms) = timer.expired_and_reset() {
                let crossed_tick = self.tick_ms(elapsed_ms);
                // Coordinate periodic telemetry reads across other controllers on the system tick
                if crossed_tick {
                    if let Some(ref battery_tx) = self.battery_tx {
                        let _ = battery_tx.try_send(BatteryCommand::CheckStatus);
                    }
                    if let Some(ref thermal_tx) = self.thermal_tx {
                        let _ = thermal_tx.try_send(ThermalCommand::CheckTemp);
                    }
                }
                if self.power_manager.status() == SystemStatus::Active {
                    for sensor_tx in &self.sensor_txs {
                        let _ = sensor_tx.try_send(SensorCommand::ReadSensors);
                    }
                }
                continue;
            }

            let remaining_ms = timer.remaining_ms();

            let recv_fut = async {
                use embassy_futures::select::{select, Either};
                match select(command_rx.receive(), gesture_rx.receive()).await {
                    Either::First(cmd) => Either::First(cmd),
                    Either::Second(gesture) => Either::Second(gesture),
                }
            };

            match embassy_time::with_timeout(
                embassy_time::Duration::from_millis(remaining_ms as u64),
                recv_fut,
            )
            .await
            {
                Ok(embassy_futures::select::Either::First(cmd)) => {
                    // Handle project-specific command from system command channel
                    let _ = self.handle_command(cmd);
                }
                Ok(embassy_futures::select::Either::Second(gesture)) => {
                    // Delegate generic gesture detection event to the command handler
                    let _ = self.handle_command(SystemCommand::Gesture(gesture));
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
        $system_rx:expr,
        $gesture_rx:expr,
        $m:expr
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                mut controller: $crate::system_controller::AppSystemController<$m>,
                system_rx: $crate::Receiver<$crate::system_controller::SystemCommand, 4>,
                gesture_rx: $crate::Receiver<model::types::Gesture, 4>,
            ) {
                controller.run(system_rx, gesture_rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $system_rx, $gesture_rx))
            .unwrap();
    };
}

/// System-specific CLI commands
#[derive(Debug, embedded_cli::Command, Clone, Copy, PartialEq, Eq)]
pub enum SystemCliCommand {
    /// Simulate activity event
    Activity,
    /// Trigger a panic to test the crash dump / panic flow
    Crash,
}

/// Processes system-specific CLI commands
pub fn process_system_command<W: embedded_io::Write<Error = E>, E: embedded_io::Error>(
    system_ctrl: &mut impl crate::BlockingSystemWriter,
    _writer: &mut embedded_cli::writer::Writer<'_, W, E>,
    cmd: SystemCliCommand,
) -> Result<(), &'static str> {
    match cmd {
        SystemCliCommand::Activity => system_ctrl
            .record_activity()
            .map_err(|_| "Failed to record system activity"),
        SystemCliCommand::Crash => {
            panic!("Simulated crash dump flow");
        }
    }
}
