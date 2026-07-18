//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]

use crate::tracing;
pub use firmware_lib::gesture_detector::ProximityEvent;

use crate::system_feature::FeatureList;
use crate::types::{
    BatteryStatus, Device, DeviceSupport, GestureAction, ProximityAction, ThermalUpdateAction,
};
use crate::{BlockingSystemWriter, PeripheralError, Sender};
use core::fmt::Write as _;
use embassy_sync::blocking_mutex::raw::RawMutex;
use firmware_lib::{
    select_branch_with_timeout, subcommand_enum, transition_thermal_update, BatteryUpdateAction,
    BootTrapMask, BootTrapReason, PeriodicTimer, PowerManager,
};

/// One-way commands to control the global system state and notify it of events.
#[derive(Clone, Copy, PartialEq, Eq)]
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
    /// Raw sensor proximity update from a ToF sensor.
    ProximityUpdate {
        /// The direction of the sensor that updated.
        direction: Direction,
        /// The distance in mm measured by the sensor.
        distance_mm: u16,
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

impl core::fmt::Debug for SystemCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ActivityDetected => write!(f, "ActivityDetected"),
            Self::AlertTriggered => write!(f, "AlertTriggered"),
            Self::BatteryUpdate {
                state_of_charge,
                charger_state,
            } => f
                .debug_struct("BatteryUpdate")
                .field("state_of_charge", state_of_charge)
                .field("charger_state", charger_state)
                .finish(),
            Self::ProximityUpdate {
                direction,
                distance_mm,
            } => f
                .debug_struct("ProximityUpdate")
                .field("direction", direction)
                .field("distance_mm", distance_mm)
                .finish(),
            Self::Gesture(g) => f.debug_tuple("Gesture").field(g).finish(),
            Self::StateChanged { from, to } => f
                .debug_struct("StateChanged")
                .field("from", from)
                .field("to", to)
                .finish(),
            Self::BatteryAction(a) => f.debug_tuple("BatteryAction").field(a).finish(),
        }
    }
}

impl crate::battery_controller::FromBatteryUpdate for SystemCommand {
    fn from_battery_update(state_of_charge: u8, charger_state: ChargeState) -> Self {
        SystemCommand::BatteryUpdate {
            state_of_charge,
            charger_state,
        }
    }
}

impl crate::sensor_controller::FromProximityUpdate for SystemCommand {
    fn from_proximity_update(metadata: crate::types::SensorMetadata, distance_mm: u16) -> Self {
        SystemCommand::ProximityUpdate {
            direction: metadata.direction,
            distance_mm,
        }
    }
}

use model::types::{BootReason, ChargeState, Direction, Gesture, SystemStatus, TelemetryRecord};

/// A set of features and event hooks for customizing the system controller's behavior.
pub trait SystemFeatureSet<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Tuple/list of features active in this feature set.
    type Features: FeatureList<MutexRaw, N>;

    /// Returns a reference to the active features.
    fn features(&self) -> &Self::Features;

    /// Returns the inactivity timeout in seconds before entering Sleep.
    fn inactivity_timeout_seconds(&self) -> u32 {
        30
    }

    /// Returns the thermal overheating threshold in milli-Celsius.
    fn thermal_overheating_temp_threshold(&self) -> i32 {
        self.features().thermal_overheating_temp_threshold()
    }

    /// Returns the thermal critical threshold in milli-Celsius.
    fn thermal_critical_temp_threshold(&self) -> i32 {
        self.features().thermal_critical_temp_threshold()
    }

    /// Map an incoming gesture to a system action.
    fn map_gesture(&self, gesture: Gesture, status: SystemStatus) -> GestureAction {
        self.features().map_gesture(gesture, status)
    }

    /// Returns true if the device should be active in the given system state.
    fn device_supported_in_state(&self, device: Device, state: SystemStatus) -> bool {
        match device {
            Device::Motor => state == SystemStatus::Active,
            Device::Sensors => state == SystemStatus::Active,
            Device::Led => state == SystemStatus::Active || state == SystemStatus::Sleep,
            Device::Battery => true,
            Device::Thermal => true,
        }
    }

    /// Returns the combined DeviceSupport configuration in the given state.
    fn get_device_support(&self, state: SystemStatus) -> DeviceSupport {
        DeviceSupport {
            motor: self.device_supported_in_state(Device::Motor, state),
            battery: self.device_supported_in_state(Device::Battery, state),
            proximity: self.device_supported_in_state(Device::Sensors, state),
            led: self.device_supported_in_state(Device::Led, state),
            thermal: self.device_supported_in_state(Device::Thermal, state),
        }
    }
}

/// Controller responsible for tracking global status and coordinating other subsystems.
pub struct SystemController<
    MutexRaw: RawMutex + 'static,
    F: SystemFeatureSet<MutexRaw, N>,
    const N: usize = 4,
    const T_CAP: usize = { crate::telemetry_controller::CHANNEL_CAPACITY },
> {
    /// Subsystem manager for power, transitions, and timers
    pub power_manager: PowerManager<MutexRaw, T_CAP>,
    /// The app-defined feature set containing event hooks and channel configurations
    pub feature_set: F,
    /// Current battery status summary.
    pub battery_status: Option<BatteryStatus>,
    /// Countdown in seconds for logging active boot traps.
    boot_trap_log_countdown: u8,
}

impl<
        MutexRaw: RawMutex + 'static,
        F: SystemFeatureSet<MutexRaw, N>,
        const N: usize,
        const T_CAP: usize,
    > SystemController<MutexRaw, F, N, T_CAP>
{
    /// Creates a new SystemController instance.
    pub fn new(
        feature_set: F,
        telemetry_tx: Sender<'static, MutexRaw, TelemetryRecord, T_CAP>,
        boot_reason: BootReason,
    ) -> Self {
        let mut power_manager = PowerManager::new(telemetry_tx, boot_reason);
        let default_mask = feature_set.features().default_boot_trap_mask();
        power_manager
            .set_boot_trap_mask(BootTrapMask::from_raw(default_mask))
            .unwrap();

        let mut ctrl = Self {
            power_manager,
            feature_set,
            battery_status: None,
            boot_trap_log_countdown: 0,
        };

        if !ctrl.power_manager.is_boot_trapped() {
            let _ = ctrl.set_status_internal(SystemStatus::Active);
        }

        ctrl
    }

    fn log_boot_trap_cleared(&self, _cleared: BootTrapReason) {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let cleared_str = match _cleared {
                BootTrapReason::Battery => "Battery",
                BootTrapReason::Thermal => "Thermal",
            };
            defmt::info!("SystemController: cleared boot trap: {}", cleared_str);
            if self.power_manager.is_boot_trapped() {
                self.log_active_boot_traps();
            } else {
                defmt::info!("SystemController: all boot traps cleared. exiting PowerDown state. Waking up to Active mode.");
            }
        }
    }

    /// Returns true if the battery is critical.
    pub fn battery_critical(&self) -> bool {
        self.battery_status
            .map(|s| s.battery_critical)
            .unwrap_or(false)
    }

    /// Returns true if the charger is connected.
    pub fn charger_connected(&self) -> bool {
        self.battery_status
            .map(|s| s.charger_connected)
            .unwrap_or(false)
    }

    /// Returns true if thermal critical alert is active.
    pub fn thermal_critical(&self) -> bool {
        self.feature_set.features().thermal_critical()
    }

    /// Sets the current system status.
    #[tracing::instrument(level = "trace", skip(status))]
    pub fn set_status(
        &mut self,
        status: SystemStatus,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        self.set_status_internal(status)
    }

    fn set_status_internal(
        &mut self,
        status: SystemStatus,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        let battery_crit = self.battery_critical();
        let thermal_crit = self.thermal_critical();
        if let Some(prev) = self
            .power_manager
            .set_status(status, battery_crit, thermal_crit)?
        {
            let _ = self.handle_command(SystemCommand::StateChanged {
                from: prev,
                to: status,
            });
        }
        Ok(())
    }

    /// Updates the battery status and processes any resulting state transition actions.
    #[tracing::instrument(level = "trace", skip(charger_state))]
    pub fn update_battery_status(
        &mut self,
        state_of_charge: u8,
        charger_state: ChargeState,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        let res = self.feature_set.features().on_battery_update(
            state_of_charge,
            charger_state,
            self.power_manager.status(),
            self.power_manager.is_boot_trapped(),
        );

        if let Some((action, status)) = res {
            self.battery_status = Some(status);
            if let Some(act) = action {
                self.handle_command(SystemCommand::BatteryAction(act))?;
            }
        }
        Ok(())
    }

    /// Handles an incoming SystemCommand.
    #[tracing::instrument(level = "trace", skip(cmd))]
    pub fn handle_command(
        &mut self,
        cmd: SystemCommand,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        match cmd {
            SystemCommand::ActivityDetected => {
                self.power_manager.set_inactive_ms(0);
                if self.power_manager.status() == SystemStatus::Sleep
                    && !self.battery_critical()
                    && !self.thermal_critical()
                {
                    self.set_status(SystemStatus::Active)?;
                }
            }
            SystemCommand::AlertTriggered => {
                let current_status = self.power_manager.status();
                self.feature_set
                    .features()
                    .on_alert_triggered(current_status);
                if current_status != SystemStatus::PowerDown
                    && current_status != SystemStatus::Sleep
                {
                    self.power_manager.clear_wake_locks();
                    self.set_status(SystemStatus::Sleep)?;
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
            SystemCommand::ProximityUpdate {
                direction,
                distance_mm,
            } => {
                let current_status = self.power_manager.status();
                let (gesture, action) = self.feature_set.features().on_proximity_update(
                    direction,
                    distance_mm,
                    current_status,
                );

                match action {
                    ProximityAction::AcquireWakeLock => {
                        self.power_manager.acquire_wake_lock(None);
                    }
                    ProximityAction::ReleaseWakeLock => {
                        self.power_manager.release_wake_lock(None);
                    }
                    ProximityAction::WakeSystem => {
                        if !self.battery_critical() && !self.thermal_critical() {
                            self.set_status(SystemStatus::Active)?;
                        }
                    }
                    ProximityAction::None => {}
                }

                if let Some(g) = gesture {
                    self.handle_command(SystemCommand::Gesture(g))?;
                }
            }
            SystemCommand::BatteryAction(action) => match action {
                BatteryUpdateAction::GoToPowerDown => {
                    self.power_manager.clear_wake_locks();
                    self.set_status(SystemStatus::PowerDown)?;
                }
                BatteryUpdateAction::ClearBootTrap => {
                    self.power_manager.clear_boot_trap(BootTrapReason::Battery);
                    self.log_boot_trap_cleared(BootTrapReason::Battery);
                    if !self.power_manager.is_boot_trapped() {
                        self.set_status(SystemStatus::Active)?;
                    }
                }
                BatteryUpdateAction::ReportSoC => {
                    self.feature_set.features().on_battery_action(
                        action,
                        self.power_manager.status(),
                        self.battery_status,
                    );
                }
            },

            SystemCommand::Gesture(gesture) => {
                let current_status = self.power_manager.status();
                self.feature_set
                    .features()
                    .on_gesture(gesture, current_status);

                // Map gesture to action
                let action = self.feature_set.map_gesture(gesture, current_status);

                match action {
                    GestureAction::TogglePower => {
                        self.power_manager.log_gesture_telemetry(gesture);
                        if current_status == SystemStatus::PowerDown {
                            if !self.charger_connected() {
                                self.set_status(SystemStatus::Active)?;
                            }
                        } else {
                            self.power_manager.clear_wake_locks();
                            self.set_status(SystemStatus::PowerDown)?;
                        }
                    }
                    GestureAction::None => {}
                }
            }
            SystemCommand::StateChanged { from, to } => {
                if to == SystemStatus::Active {
                    self.power_manager.reset_on_wake();
                }
                let support = self.feature_set.get_device_support(to);
                let thermal_crit = self.thermal_critical();
                self.feature_set.features().on_state_changed(
                    from,
                    to,
                    support,
                    self.battery_status,
                    thermal_crit,
                );
            }
        }
        Ok(())
    }

    /// Handles updates from the thermal controller.
    #[tracing::instrument(level = "trace", skip(action))]
    pub fn handle_thermal_action(
        &mut self,
        action: ThermalUpdateAction,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        let current_status = self.power_manager.status();

        match action {
            ThermalUpdateAction::ClearBootTrap => {
                self.power_manager.clear_boot_trap(BootTrapReason::Thermal);
                self.log_boot_trap_cleared(BootTrapReason::Thermal);
            }
            ThermalUpdateAction::AlertTriggered => {
                self.feature_set
                    .features()
                    .on_alert_triggered(current_status);
            }
        }

        let is_boot_trapped = self.power_manager.is_boot_trapped();
        let transition = transition_thermal_update(current_status, action, is_boot_trapped);

        if transition.clear_wake_locks {
            self.power_manager.clear_wake_locks();
        }

        if let Some(next_status) = transition.next_status {
            self.set_status(next_status)?;
        }

        if action == ThermalUpdateAction::AlertTriggered {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            defmt::info!("SystemController: Alert triggered. LED indicator set to RED.");
        }

        Ok(())
    }

    fn log_active_boot_traps(&self) {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            let mask = self.power_manager.boot_trap_mask();
            let mut battery = "";
            let mut thermal = "";
            if mask.has(BootTrapReason::Battery) {
                battery = " Battery";
            }
            if mask.has(BootTrapReason::Thermal) {
                thermal = " Thermal";
            }
            defmt::info!(
                "SystemController: boot blocked by traps:{}{}",
                battery,
                thermal
            );
        }
    }

    /// Ticks the inactivity timer and active mode duration timer by a specified duration in milliseconds.
    /// Returns true if the 1-second system tick boundary was crossed.
    pub fn tick_ms(&mut self, ms: u32) -> bool {
        let crossed = self.power_manager.tick_ms(ms);
        let status = self.power_manager.status();
        let wake_locks = self.power_manager.wake_locks();

        let support = self.feature_set.get_device_support(status);

        self.feature_set
            .features()
            .on_tick(ms, crossed, status, support, wake_locks);

        if crossed {
            // Sleep after inactivity timeout
            if self.power_manager.inactive_ms()
                >= self.feature_set.inactivity_timeout_seconds() * 1000
            {
                let _ = self.set_status(SystemStatus::Sleep);
            }

            // Periodic boot trap logging
            if self.power_manager.is_boot_trapped() {
                if self.boot_trap_log_countdown == 0 {
                    self.boot_trap_log_countdown = 5; // Log every 5 seconds
                    self.log_active_boot_traps();
                } else {
                    self.boot_trap_log_countdown = self.boot_trap_log_countdown.saturating_sub(1);
                }
            } else {
                self.boot_trap_log_countdown = 0;
            }
        }

        crossed
    }

    /// Main execution loop.
    pub async fn run<const CMD_CAP: usize>(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, SystemCommand, CMD_CAP>,
        gesture_rx: embassy_sync::channel::Receiver<'static, MutexRaw, Gesture, 4>,
        thermal_rx: embassy_sync::channel::Receiver<
            'static,
            MutexRaw,
            crate::types::ThermalUpdateAction,
            4,
        >,
    ) -> ! {
        self.feature_set.features().on_init();
        self.power_manager
            .log_telemetry(TelemetryRecord::System(SystemStatus::PowerDown));

        let mut timer = PeriodicTimer::new(embassy_time::Duration::from_millis(1000));
        loop {
            if let Some(elapsed_ms) = timer.expired_and_reset() {
                let _ = self.tick_ms(elapsed_ms);
                continue;
            }

            let remaining_ms = timer.remaining_ms();

            let _ = select_branch_with_timeout!(
                embassy_time::Duration::from_millis(remaining_ms as u64),
                command_rx.receive() => |cmd| {
                    let _ = self.handle_command(cmd);
                    Some(())
                },
                gesture_rx.receive() => |gesture| {
                    let _ = self.handle_command(SystemCommand::Gesture(gesture));
                    Some(())
                },
                thermal_rx.receive() => |action| {
                    let _ = self.handle_thermal_action(action);
                    Some(())
                },
            );
        }
    }
}

/// Helper macro to implement the SystemFeatureSet trait for a feature set struct.
#[macro_export]
macro_rules! impl_system_feature_set {
    (
        impl <$mutex:ident, const $n:ident $(, const $extra:ident)?> for $struct_type:ty {
            motor: $motor_ty:ty => $motor_field:ident;
            battery: $battery_ty:ty => $battery_field:ident;
            proximity: $proximity_ty:ty => $proximity_field:ident;
            led: $led_ty:ty => $led_field:ident;
            thermal: $thermal_ty:ty => $thermal_field:ident;
        }
    ) => {
        impl<$mutex: embassy_sync::blocking_mutex::raw::RawMutex + 'static, const $n: usize $(, const $extra: usize)?>
            $crate::system_controller::SystemFeatureSet<$mutex, $n> for $struct_type
        {
            type Motor = $motor_ty;
            type Battery = $battery_ty;
            type Proximity = $proximity_ty;
            type Led = $led_ty;
            type Thermal = $thermal_ty;

            fn motor(&self) -> &Self::Motor {
                &self.$motor_field
            }
            fn battery(&self) -> &Self::Battery {
                &self.$battery_field
            }
            fn proximity(&self) -> &Self::Proximity {
                &self.$proximity_field
            }
            fn led(&self) -> &Self::Led {
                &self.$led_field
            }
            fn thermal(&self) -> &Self::Thermal {
                &self.$thermal_field
            }
        }
    };
}

subcommand_enum! {
    /// System subcommands for CLI processing.
    pub enum SystemSubcommand {
        /// Record activity to wake/extend system active time
        Activity,
        /// Trigger simulated crash/panic
        Crash,
    }
    "Invalid system subcommand. Expected: activity, crash"
}

/// Processes system-specific CLI subcommands.
pub fn handle_system_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<SystemSubcommand>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let cmd = subcommand.ok_or("Missing system subcommand")?;

    match cmd {
        SystemSubcommand::Activity => {
            let mut system_ctrl = resolver.resolve_system_ctrl(None);
            if let Ok(ref mut ctrl) = system_ctrl {
                ctrl.record_activity()
                    .map_err(|_| "Failed to record system activity")
            } else {
                let _ = core::writeln!(
                    writer,
                    "System controller not registered; activity ignored."
                );
                Ok(())
            }
        }
        SystemSubcommand::Crash => {
            panic!("Simulated crash dump flow");
        }
    }
}

impl<
        MutexRaw: RawMutex + 'static,
        F: SystemFeatureSet<MutexRaw, N>,
        const N: usize,
        const T_CAP: usize,
    > crate::BlockingSystemWriter for SystemController<MutexRaw, F, N, T_CAP>
{
    fn record_activity(&mut self) -> Result<(), PeripheralError> {
        let _ = self.handle_command(SystemCommand::ActivityDetected);
        Ok(())
    }

    fn clear_boot_trap(&mut self, reason: BootTrapReason) -> Result<(), PeripheralError> {
        if self.power_manager.has_boot_trap(reason) {
            self.power_manager.clear_boot_trap(reason);
            self.log_boot_trap_cleared(reason);
            if !self.power_manager.is_boot_trapped() {
                let _ = self.set_status(SystemStatus::Active);
            }
        }
        Ok(())
    }

    fn is_boot_trapped(&self) -> Result<bool, PeripheralError> {
        Ok(self.power_manager.is_boot_trapped())
    }
}
