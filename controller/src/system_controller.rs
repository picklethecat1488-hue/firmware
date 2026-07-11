//! System controller for managing global modes (Active/Sleep), inactivity timeouts, and coordinating other loops.

#![deny(missing_docs)]

pub use firmware_lib::gesture_detector::ProximityEvent;

use crate::Sender;
use embassy_sync::blocking_mutex::raw::RawMutex;
use firmware_lib::{BatteryUpdateAction, PeriodicTimer, PowerManager};

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

crate::define_controller_channels!(SystemChannel, SystemSender, SystemReceiver, SystemCommand);

impl crate::battery_controller::FromBatteryUpdate for SystemCommand {
    fn from_battery_update(state_of_charge: u8, charger_state: ChargeState) -> Self {
        SystemCommand::BatteryUpdate {
            state_of_charge,
            charger_state,
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
        let power_manager = PowerManager::new(telemetry_tx, boot_reason);

        Self {
            power_manager,
            feature_set,
            battery_status: None,
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
    pub fn set_status(
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
    pub fn update_battery_status(
        &mut self,
        state_of_charge: u8,
        charger_state: ChargeState,
    ) -> Result<(), firmware_lib::system::TransitionError> {
        let res = self.feature_set.features().on_battery_update(
            state_of_charge,
            charger_state,
            self.power_manager.status(),
            self.power_manager.boot_power_down(),
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
                    self.power_manager.set_boot_power_down(false);
                    self.set_status(SystemStatus::Active)?;
                    #[cfg(all(target_arch = "arm", target_os = "none"))]
                    defmt::info!(
                        "SystemController: exiting PowerDown state. Waking up to Active mode."
                    );
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
        }

        crossed
    }

    /// Main execution loop.
    pub async fn run<const CMD_CAP: usize>(
        mut self,
        command_rx: embassy_sync::channel::Receiver<'static, MutexRaw, SystemCommand, CMD_CAP>,
        gesture_rx: embassy_sync::channel::Receiver<'static, MutexRaw, Gesture, 4>,
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
        $controller_type:ty,
        $system_rx:expr,
        $gesture_rx:expr
    ) => {
        mod $task_module {
            use super::*;

            #[embassy_executor::task]
            pub async fn task(
                mut controller: $controller_type,
                system_rx: $crate::StaticReceiver<$crate::system_controller::SystemCommand, 4>,
                gesture_rx: $crate::StaticReceiver<model::types::Gesture, 4>,
            ) {
                controller.run(system_rx, gesture_rx).await;
            }
        }

        $spawner
            .spawn($task_module::task($controller, $system_rx, $gesture_rx))
            .unwrap();
    };
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

/// Processes system-specific CLI subcommands.
pub fn handle_system_cli<
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
    C: crate::ShellConfig,
>(
    resolver: &impl crate::ShellDeviceResolver<C>,
    subcommand: Option<&str>,
    writer: &mut embedded_cli::writer::Writer<'_, W, E>,
) -> Result<(), &'static str> {
    let system_ctrl = resolver.resolve_system_ctrl(None)?;
    match subcommand {
        Some("activity") => process_system_command(system_ctrl, writer, SystemCliCommand::Activity),
        Some("crash") => process_system_command(system_ctrl, writer, SystemCliCommand::Crash),
        _ => Err("Invalid system subcommand. Expected: activity, crash"),
    }
}

/// Actions that can be mapped from gestures.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GestureAction {
    /// No action.
    None,
    /// Toggle system power state (Active <-> PowerDown).
    TogglePower,
}

/// Action returned by the proximity feature update.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProximityAction {
    /// No action.
    None,
    /// Acquire system wake lock.
    AcquireWakeLock,
    /// Release system wake lock.
    ReleaseWakeLock,
    /// Wake system if asleep.
    WakeSystem,
}

/// Battery status summary passed to features and stored on the system controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatteryStatus {
    /// True if the battery level is critically low.
    pub battery_critical: bool,
    /// True if the charger is connected and charging.
    pub charger_connected: bool,
    /// The mapped LED state for the current state of charge.
    pub soc_led_state: model::types::SystemLedState,
}

/// Devices that can be power-managed by the system.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Device {
    /// The motor.
    Motor,
    /// Proximity/gesture sensors.
    Sensors,
    /// Status indicator LED.
    Led,
    /// Battery / Fuel gauge.
    Battery,
    /// Thermal monitoring.
    Thermal,
}

/// Device activity support status in the current system state.
#[derive(Debug, Clone, Copy)]
pub struct DeviceSupport {
    /// True if motor is supported.
    pub motor: bool,
    /// True if battery monitoring is supported.
    pub battery: bool,
    /// True if proximity sensors are supported.
    pub proximity: bool,
    /// True if LED is supported.
    pub led: bool,
    /// True if thermal monitoring is supported.
    pub thermal: bool,
}

/// A single system feature that can react to system events and ticks.
pub trait SystemFeature<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Hook called when the system starts running.
    fn on_init(&self) {}

    /// Returns true if the feature has a critical thermal alert.
    fn thermal_critical(&self) -> bool {
        false
    }

    /// Hook called to process raw battery updates.
    /// Only the battery feature should implement this to update the battery manager.
    fn on_battery_update(
        &self,
        _state_of_charge: u8,
        _charger_state: model::types::ChargeState,
        _status: model::types::SystemStatus,
        _boot_power_down: bool,
    ) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
        None
    }

    /// Hook called when a proximity update is received.
    fn on_proximity_update(
        &self,
        _direction: model::types::Direction,
        _distance_mm: u16,
        _status: model::types::SystemStatus,
    ) -> (Option<model::types::Gesture>, ProximityAction) {
        (None, ProximityAction::None)
    }

    /// Hook called when the system state changes.
    fn on_state_changed(
        &self,
        _from: model::types::SystemStatus,
        _to: model::types::SystemStatus,
        _support: DeviceSupport,
        _battery_status: Option<BatteryStatus>,
        _thermal_critical: bool,
    ) {
    }

    /// Hook called when a battery action is triggered.
    fn on_battery_action(
        &self,
        _action: BatteryUpdateAction,
        _status: model::types::SystemStatus,
        _battery_status: Option<BatteryStatus>,
    ) {
    }

    /// Hook called when a gesture is detected.
    fn on_gesture(&self, _gesture: model::types::Gesture, _status: model::types::SystemStatus) {}

    /// Map an incoming gesture to a system action.
    fn map_gesture(
        &self,
        _gesture: model::types::Gesture,
        _status: model::types::SystemStatus,
    ) -> GestureAction {
        GestureAction::None
    }

    /// Hook called periodically.
    fn on_tick(
        &self,
        _elapsed_ms: u32,
        _crossed_tick: bool,
        _status: model::types::SystemStatus,
        _support: DeviceSupport,
        _wake_locks: u32,
    ) {
    }

    /// Hook called when a thermal or motor safety alert is triggered.
    fn on_alert_triggered(&self, _status: model::types::SystemStatus) {}
}

/// Trait implemented by collections (like tuples) of system features to dispatch hooks.
pub trait FeatureList<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Dispatch on_init hook to all features.
    fn on_init(&self);
    /// Combine thermal critical status from all features.
    fn thermal_critical(&self) -> bool;
    /// Dispatch on_battery_update hook to features.
    fn on_battery_update(
        &self,
        state_of_charge: u8,
        charger_state: model::types::ChargeState,
        status: model::types::SystemStatus,
        boot_power_down: bool,
    ) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)>;
    /// Dispatch on_proximity_update hook to features.
    fn on_proximity_update(
        &self,
        direction: model::types::Direction,
        distance_mm: u16,
        status: model::types::SystemStatus,
    ) -> (Option<model::types::Gesture>, ProximityAction);
    /// Dispatch on_state_changed hook to all features.
    fn on_state_changed(
        &self,
        from: model::types::SystemStatus,
        to: model::types::SystemStatus,
        support: DeviceSupport,
        battery_status: Option<BatteryStatus>,
        thermal_critical: bool,
    );
    /// Dispatch on_battery_action hook to all features.
    fn on_battery_action(
        &self,
        action: BatteryUpdateAction,
        status: model::types::SystemStatus,
        battery_status: Option<BatteryStatus>,
    );
    /// Dispatch on_gesture hook to all features.
    fn on_gesture(&self, gesture: model::types::Gesture, status: model::types::SystemStatus);
    /// Dispatch map_gesture hook to features.
    fn map_gesture(
        &self,
        gesture: model::types::Gesture,
        status: model::types::SystemStatus,
    ) -> GestureAction;
    /// Dispatch on_tick hook to all features.
    fn on_tick(
        &self,
        elapsed_ms: u32,
        crossed_tick: bool,
        status: model::types::SystemStatus,
        support: DeviceSupport,
        wake_locks: u32,
    );
    /// Dispatch on_alert_triggered hook to all features.
    fn on_alert_triggered(&self, status: model::types::SystemStatus);
}

macro_rules! impl_feature_list_for_tuple {
    ($($T:ident),*) => {
        impl<MutexRaw: RawMutex + 'static, const N: usize, $($T: SystemFeature<MutexRaw, N>),*> FeatureList<MutexRaw, N> for ($($T,)*) {
            #[inline(always)]
            fn on_init(&self) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_init();)*
            }

            #[inline(always)]
            fn thermal_critical(&self) -> bool {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.thermal_critical() ||)* false
            }

            #[inline(always)]
            fn on_battery_update(&self, _state_of_charge: u8, _charger_state: model::types::ChargeState, _status: model::types::SystemStatus, _boot_power_down: bool) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    if let Some(res) = $T.on_battery_update(_state_of_charge, _charger_state, _status, _boot_power_down) {
                        return Some(res);
                    }
                )*
                None
            }

            #[inline(always)]
            fn on_proximity_update(&self, _direction: model::types::Direction, _distance_mm: u16, _status: model::types::SystemStatus) -> (Option<model::types::Gesture>, ProximityAction) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                #[allow(unused_mut)]
                let mut merged_gesture = None;
                #[allow(unused_mut)]
                let mut merged_action = ProximityAction::None;
                $(
                    let (g, a) = $T.on_proximity_update(_direction, _distance_mm, _status);
                    if g.is_some() {
                        merged_gesture = g;
                    }
                    if a != ProximityAction::None {
                        merged_action = a;
                    }
                )*
                (merged_gesture, merged_action)
            }

            #[inline(always)]
            fn on_state_changed(&self, _from: model::types::SystemStatus, _to: model::types::SystemStatus, _support: DeviceSupport, _battery_status: Option<BatteryStatus>, _thermal_critical: bool) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_state_changed(_from, _to, _support, _battery_status, _thermal_critical);)*
            }

            #[inline(always)]
            fn on_battery_action(&self, _action: BatteryUpdateAction, _status: model::types::SystemStatus, _battery_status: Option<BatteryStatus>) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_battery_action(_action, _status, _battery_status);)*
            }

            #[inline(always)]
            fn on_gesture(&self, _gesture: model::types::Gesture, _status: model::types::SystemStatus) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_gesture(_gesture, _status);)*
            }

            #[inline(always)]
            fn map_gesture(&self, _gesture: model::types::Gesture, _status: model::types::SystemStatus) -> GestureAction {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    let action = $T.map_gesture(_gesture, _status);
                    if action != GestureAction::None {
                        return action;
                    }
                )*
                GestureAction::None
            }

            #[inline(always)]
            fn on_tick(&self, _elapsed_ms: u32, _crossed_tick: bool, _status: model::types::SystemStatus, _support: DeviceSupport, _wake_locks: u32) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_tick(_elapsed_ms, _crossed_tick, _status, _support, _wake_locks);)*
            }

            #[inline(always)]
            fn on_alert_triggered(&self, _status: model::types::SystemStatus) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_alert_triggered(_status);)*
            }
        }
    }
}

impl_feature_list_for_tuple!();
impl_feature_list_for_tuple!(A);
impl_feature_list_for_tuple!(A, B);
impl_feature_list_for_tuple!(A, B, C);
impl_feature_list_for_tuple!(A, B, C, D);
impl_feature_list_for_tuple!(A, B, C, D, E);
impl_feature_list_for_tuple!(A, B, C, D, E, F);
impl_feature_list_for_tuple!(A, B, C, D, E, F, G);
impl_feature_list_for_tuple!(A, B, C, D, E, F, G, H);
impl_feature_list_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_feature_list_for_tuple!(A, B, C, D, E, F, G, H, I, J);
