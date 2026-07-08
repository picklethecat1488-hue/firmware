//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]
#![allow(clippy::collapsible_match)]
use controller::battery_controller::BatteryCommand;
use controller::motor_controller::MotorCommand;
use controller::sensor_controller::SensorCommand;
use controller::thermal_controller::ThermalCommand;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::channel::Sender;
pub use firmware_lib::gesture_detector::ProximityEvent;
use firmware_lib::system::BatteryUpdateAction;
pub use model::types::SystemCommand;
use model::types::{BootReason, Gesture, SystemLedState, SystemStatus, TelemetryRecord};

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
    const T_CAP: usize,
> {
    /// System command channel sender
    pub system_tx: Sender<'static, MutexRaw, SystemCommand, 4>,
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
pub struct SystemController<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    const T_CAP: usize = { controller::telemetry_controller::CHANNEL_CAPACITY },
> {
    state_manager: firmware_lib::system::SystemStateManager<MutexRaw, T_CAP>,
    motor_tx: Sender<'static, MutexRaw, MotorCommand, N>,
    sensor_north_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_east_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    sensor_west_tx: Sender<'static, MutexRaw, SensorCommand, N>,
    battery_tx: Sender<'static, MutexRaw, BatteryCommand, N>,
    thermal_tx: Sender<'static, MutexRaw, ThermalCommand, N>,
    led_tx: Sender<'static, MutexRaw, SystemLedState, N>,
    _system_tx: Sender<'static, MutexRaw, SystemCommand, 4>,
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
        boot_reason: BootReason,
    ) -> Self {
        let state_manager = firmware_lib::system::SystemStateManager::new(
            10, // critical_soc_threshold default
            2,  // soc_hysteresis default
            LOW_BATTERY_SOC_THRESHOLD,
            MID_BATTERY_SOC_THRESHOLD,
            HIGH_BATTERY_SOC_THRESHOLD,
            channels.telemetry_tx,
            Some(channels.system_tx),
            boot_reason,
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
            _system_tx: channels.system_tx,
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

    fn handle_state_changed(&mut self, _from: SystemStatus, to: SystemStatus) {
        match to {
            SystemStatus::Active => {
                self.state_manager.reset_on_wake();
                let _ = self.led_tx.try_send(self.get_soc_led_state());
                let _ = self.motor_tx.try_send(MotorCommand::SetSpeed(100));
            }
            SystemStatus::Sleep | SystemStatus::PowerDown => {
                let _ = self.motor_tx.try_send(MotorCommand::Stop);
                let led = if to == SystemStatus::Sleep {
                    if self.thermal_critical() {
                        SystemLedState::BlinksRedFourTimes
                    } else {
                        SystemLedState::SolidBlue
                    }
                } else if self.battery_critical() {
                    SystemLedState::BlinksRedOncePerThirtySeconds
                } else if self.charger_connected() {
                    self.get_soc_led_state()
                } else {
                    SystemLedState::Off
                };
                let _ = self.led_tx.try_send(led);
                let _ = self.battery_tx.try_send(BatteryCommand::UpdateWakeLocks(0));
            }
        }
    }

    /// Handles an incoming SystemCommand.
    pub fn handle_command(&mut self, cmd: SystemCommand) {
        match cmd {
            SystemCommand::ActivityDetected => {
                self.set_inactive_ms(0);
                if self.status() == SystemStatus::Sleep
                    && !self.battery_critical()
                    && !self.thermal_critical()
                {
                    let _ = self.set_status(SystemStatus::Active);
                }
            }
            SystemCommand::AlertTriggered => {
                self.set_thermal_critical(true);
                if self.status() != SystemStatus::PowerDown {
                    if self.status() != SystemStatus::Sleep {
                        self.state_manager.clear_wake_locks();
                        let _ = self.set_status(SystemStatus::Sleep);
                    } else {
                        let _ = self.led_tx.try_send(SystemLedState::BlinksRedFourTimes);
                    }
                }
                #[cfg(all(target_arch = "arm", target_os = "none"))]
                defmt::info!("SystemController: Alert triggered. LED indicator set to RED.");
            }
            SystemCommand::BatteryUpdate {
                state_of_charge,
                charger_state,
            } => {
                if self.critical_soc_threshold() >= LOW_BATTERY_SOC_THRESHOLD {
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::error!(
                        "Critical SoC threshold must be lower than the low battery threshold"
                    );
                    self.set_critical_soc_threshold(LOW_BATTERY_SOC_THRESHOLD - 1);
                }
                let charging = charger_state == model::types::ChargeState::Charging;
                let is_fault = charger_state == model::types::ChargeState::RecoverableFault
                    || charger_state == model::types::ChargeState::NonRecoverableFault;

                match self.update_battery_status(state_of_charge, charging, is_fault) {
                    BatteryUpdateAction::GoToPowerDown => {
                        self.state_manager.clear_wake_locks();
                        let _ = self.set_status(SystemStatus::PowerDown);
                    }
                    BatteryUpdateAction::ClearBootTrap => {
                        self.set_boot_power_down(false);
                        let _ = self.set_status(SystemStatus::Active);
                        #[cfg(all(target_arch = "arm", target_os = "none"))]
                        defmt::info!(
                            "SystemController: exiting PowerDown state. Waking up to Active mode."
                        );
                    }
                    BatteryUpdateAction::ReportSoC => {
                        if self.battery_critical() {
                            let _ = self
                                .led_tx
                                .try_send(SystemLedState::BlinksRedOncePerThirtySeconds);
                        } else if self.status() == SystemStatus::PowerDown {
                            let led = if self.charger_connected() {
                                self.get_soc_led_state()
                            } else {
                                SystemLedState::Off
                            };
                            let _ = self.led_tx.try_send(led);
                        } else if self.status() == SystemStatus::Active {
                            let _ = self.led_tx.try_send(self.get_soc_led_state());
                        }
                    }
                    BatteryUpdateAction::NoAction => {}
                }
            }
            SystemCommand::Gesture(gesture) => {
                let current_status = self.status();
                match gesture {
                    Gesture::DualLongPress => {
                        self.log_gesture_telemetry(Gesture::DualLongPress);
                        if current_status == SystemStatus::PowerDown {
                            if !self.charger_connected() {
                                let _ = self.set_status(SystemStatus::Active);
                            }
                        } else {
                            self.state_manager.clear_wake_locks();
                            let _ = self.set_status(SystemStatus::PowerDown);
                        }
                    }
                    Gesture::ProximityDetected if current_status != SystemStatus::PowerDown => {
                        self.log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityDetected));
                        if self.status() == SystemStatus::Sleep
                            && !self.battery_critical()
                            && !self.thermal_critical()
                        {
                            let _ = self.set_status(SystemStatus::Active);
                        }
                        if self.status() == SystemStatus::Active {
                            self.acquire_wake_lock(None);
                        }
                    }
                    Gesture::ProximityNotDetected if current_status != SystemStatus::PowerDown => {
                        self.log_telemetry(TelemetryRecord::Gesture(Gesture::ProximityNotDetected));
                        if self.status() == SystemStatus::Active {
                            self.release_wake_lock(None);
                        }
                    }
                    _ => {}
                }
            }
            SystemCommand::StateChanged { from, to } => {
                self.handle_state_changed(from, to);
            }
        }
    }

    /// Ticks the inactivity timer and active mode duration timer by a specified duration in milliseconds.
    /// Returns true if the 1-second system tick boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        let crossed = self.state_manager.tick_ms(ms);
        if crossed {
            let _ = self.battery_tx.try_send(BatteryCommand::UpdateWakeLocks(
                self.state_manager.wake_locks(),
            ));
            // Sleep after inactivity timeout
            if self.inactive_ms() >= INACTIVITY_TIMEOUT_SECONDS * 1000 {
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
        self.led_tx.try_send(SystemLedState::Off).unwrap();
        self.log_telemetry(TelemetryRecord::System(SystemStatus::PowerDown));

        let mut last_tick_time = embassy_time::Instant::now();
        loop {
            let timeout_duration = embassy_time::Duration::from_millis(1000);
            let now = embassy_time::Instant::now();
            let elapsed_ms = now.duration_since(last_tick_time).as_millis() as u32;
            let remaining_ms = if elapsed_ms >= timeout_duration.as_millis() as u32 {
                0
            } else {
                (timeout_duration.as_millis() as u32) - elapsed_ms
            };

            if remaining_ms == 0 {
                last_tick_time = now;
                let crossed_tick = self.tick_ms(elapsed_ms);
                // Coordinate periodic telemetry reads across other controllers on the system tick
                if crossed_tick {
                    self.battery_tx
                        .try_send(BatteryCommand::CheckStatus)
                        .unwrap();
                    self.thermal_tx.try_send(ThermalCommand::CheckTemp).unwrap();
                }
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
                    self.handle_command(cmd);
                }
                Ok(embassy_futures::select::Either::Second(gesture)) => {
                    // Delegate generic gesture detection event to the command handler
                    self.handle_command(SystemCommand::Gesture(gesture));
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
        $gesture_rx:expr
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                mut controller: $crate::system_controller::SystemController<
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    4,
                >,
                system_rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    $crate::system_controller::SystemCommand,
                    4,
                >,
                gesture_rx: embassy_sync::channel::Receiver<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    model::types::Gesture,
                    4,
                >,
            ) {
                controller.run(system_rx, gesture_rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $system_rx, $gesture_rx))
            .unwrap();
    };
}
