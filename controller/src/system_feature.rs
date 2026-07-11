//! System feature definitions and collections for the system controller.

use crate::system_controller::TelemetrySender;
use crate::telemetry_controller::ProximityTelemetryClient;
use crate::{
    BatteryCommand, BatterySender, LedSender, MotorCommand, MotorSender, SensorCommand,
    SensorSender, ThermalCommand, ThermalSender,
};
use embassy_sync::blocking_mutex::raw::RawMutex;
use firmware_lib::{
    gesture_detector::ProximityGestureDetector, BatteryManager, BatteryUpdateAction, ThermalManager,
};
use model::types::{ChargeState, Direction, Gesture, MotorSpeed, SystemLedState, SystemStatus};

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
    pub soc_led_state: SystemLedState,
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
        _charger_state: ChargeState,
        _status: SystemStatus,
        _boot_power_down: bool,
    ) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
        None
    }

    /// Hook called when a proximity update is received.
    fn on_proximity_update(
        &self,
        _direction: Direction,
        _distance_mm: u16,
        _status: SystemStatus,
    ) -> (Option<Gesture>, ProximityAction) {
        (None, ProximityAction::None)
    }

    /// Hook called when the system state changes.
    fn on_state_changed(
        &self,
        _from: SystemStatus,
        _to: SystemStatus,
        _support: DeviceSupport,
        _battery_status: Option<BatteryStatus>,
        _thermal_critical: bool,
    ) {
    }

    /// Hook called when a battery action is triggered.
    fn on_battery_action(
        &self,
        _action: BatteryUpdateAction,
        _status: SystemStatus,
        _battery_status: Option<BatteryStatus>,
    ) {
    }

    /// Hook called when a gesture is detected.
    fn on_gesture(&self, _gesture: Gesture, _status: SystemStatus) {}

    /// Map an incoming gesture to a system action.
    fn map_gesture(&self, _gesture: Gesture, _status: SystemStatus) -> GestureAction {
        GestureAction::None
    }

    /// Hook called periodically.
    fn on_tick(
        &self,
        _elapsed_ms: u32,
        _crossed_tick: bool,
        _status: SystemStatus,
        _support: DeviceSupport,
        _wake_locks: u32,
    ) {
    }

    /// Hook called when a thermal or motor safety alert is triggered.
    fn on_alert_triggered(&self, _status: SystemStatus) {}
}

/// Standard config implementation for MotorFeature.
pub struct MotorFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Motor channel sender
    pub motor_tx: Option<MotorSender<MutexRaw, N>>,
    /// Maximum motor speed
    pub max_speed: MotorSpeed,
}
impl<MutexRaw: RawMutex + 'static, const N: usize> MotorFeatureConfig<MutexRaw, N> {
    /// Creates a new `MotorFeatureConfig`.
    pub fn new(motor_tx: Option<MotorSender<MutexRaw, N>>, max_speed: MotorSpeed) -> Self {
        Self {
            motor_tx,
            max_speed,
        }
    }
}
impl<MutexRaw: RawMutex + 'static, const N: usize> SystemFeature<MutexRaw, N>
    for MotorFeatureConfig<MutexRaw, N>
{
    fn on_state_changed(
        &self,
        _from: SystemStatus,
        _to: SystemStatus,
        support: DeviceSupport,
        _battery_status: Option<BatteryStatus>,
        _thermal_critical: bool,
    ) {
        if let Some(ref motor_tx) = self.motor_tx {
            if support.motor {
                let _ = motor_tx.try_send(MotorCommand::SetSpeed(self.max_speed));
            } else {
                let _ = motor_tx.try_send(MotorCommand::Stop);
            }
        }
    }
}

/// Standard config implementation for BatteryFeature.
pub struct BatteryFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Battery channel sender
    pub battery_tx: Option<BatterySender<MutexRaw, N>>,
    /// Battery manager for battery thresholds and status
    pub battery_manager: core::cell::RefCell<BatteryManager>,
}
impl<MutexRaw: RawMutex + 'static, const N: usize> BatteryFeatureConfig<MutexRaw, N> {
    /// Creates a new `BatteryFeatureConfig`.
    pub fn new(
        battery_tx: Option<BatterySender<MutexRaw, N>>,
        battery_manager: BatteryManager,
    ) -> Self {
        Self {
            battery_tx,
            battery_manager: core::cell::RefCell::new(battery_manager),
        }
    }
}
impl<MutexRaw: RawMutex + 'static, const N: usize> SystemFeature<MutexRaw, N>
    for BatteryFeatureConfig<MutexRaw, N>
{
    fn on_init(&self) {
        let mut bm = self.battery_manager.borrow_mut();
        let low_threshold = bm.low_soc_threshold();
        if bm.critical_soc_threshold() >= low_threshold {
            bm.set_critical_soc_threshold(low_threshold - 1);
        }
    }

    fn on_battery_update(
        &self,
        state_of_charge: u8,
        charger_state: ChargeState,
        status: SystemStatus,
        boot_power_down: bool,
    ) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
        let mut bm = self.battery_manager.borrow_mut();
        let action =
            bm.update_battery_status(state_of_charge, charger_state, status, boot_power_down);
        let battery_critical = bm.battery_critical();
        let charger_connected = bm.charger_connected();
        let soc_led_state = bm.get_soc_led_state();
        Some((
            action,
            BatteryStatus {
                battery_critical,
                charger_connected,
                soc_led_state,
            },
        ))
    }

    fn on_state_changed(
        &self,
        _from: SystemStatus,
        _to: SystemStatus,
        _support: DeviceSupport,
        _battery_status: Option<BatteryStatus>,
        _thermal_critical: bool,
    ) {
        if let Some(ref battery_tx) = self.battery_tx {
            let _ = battery_tx.try_send(BatteryCommand::UpdateWakeLocks(0));
        }
    }

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        crossed_tick: bool,
        _status: SystemStatus,
        support: DeviceSupport,
        wake_locks: u32,
    ) {
        if crossed_tick && support.battery {
            if let Some(ref battery_tx) = self.battery_tx {
                let _ = battery_tx.try_send(BatteryCommand::UpdateWakeLocks(wake_locks));
                let _ = battery_tx.try_send(BatteryCommand::CheckStatus);
            }
        }
    }
}

/// Standard config implementation for ProximityFeature.
pub struct ProximityFeatureConfig<
    MutexRaw: RawMutex + 'static,
    const N: usize,
    const S_CAP: usize = 3,
    const T_CAP: usize = { crate::telemetry_controller::CHANNEL_CAPACITY },
> {
    /// Sensor channel senders
    pub sensor_txs: heapless::Vec<SensorSender<MutexRaw, N>, S_CAP>,
    /// Proximity gesture detector state
    pub gesture_detector: core::cell::RefCell<ProximityGestureDetector>,
    /// Proximity telemetry client
    pub telemetry_client: core::cell::RefCell<ProximityTelemetryClient<'static, MutexRaw, T_CAP>>,
    /// Active proximity detection state
    pub proximity_active: core::cell::Cell<bool>,
    /// Proximity detection threshold
    pub wake_threshold_mm: u16,
    /// Last seen distances indexed by Direction (0 = North, 1 = East, 2 = West)
    pub distances: [core::cell::Cell<u16>; 3],
    /// Mapped action for DualLongPress gesture
    pub dual_long_press_action: GestureAction,
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const S_CAP: usize, const T_CAP: usize>
    ProximityFeatureConfig<MutexRaw, N, S_CAP, T_CAP>
{
    /// Creates a new `ProximityFeatureConfig` with the given list of sensor senders (up to S_CAP).
    pub fn new(
        sensor_senders: &[SensorSender<MutexRaw, N>],
        press_threshold_mm: u16,
        wake_threshold_mm: u16,
        dual_long_press_action: GestureAction,
        telemetry_tx: Option<TelemetrySender<'static, MutexRaw, T_CAP>>,
    ) -> Self {
        let mut sensor_txs = heapless::Vec::new();
        for sender in sensor_senders {
            let _ = sensor_txs.push(*sender);
        }
        Self {
            sensor_txs,
            gesture_detector: core::cell::RefCell::new(ProximityGestureDetector::new(
                press_threshold_mm,
            )),
            telemetry_client: core::cell::RefCell::new(ProximityTelemetryClient::new(
                telemetry_tx,
                wake_threshold_mm,
            )),
            proximity_active: core::cell::Cell::new(false),
            wake_threshold_mm,
            distances: [
                core::cell::Cell::new(1000),
                core::cell::Cell::new(1000),
                core::cell::Cell::new(1000),
            ],
            dual_long_press_action,
        }
    }
}

impl<MutexRaw: RawMutex + 'static, const N: usize, const S_CAP: usize, const T_CAP: usize>
    SystemFeature<MutexRaw, N> for ProximityFeatureConfig<MutexRaw, N, S_CAP, T_CAP>
{
    fn on_proximity_update(
        &self,
        direction: Direction,
        distance_mm: u16,
        status: SystemStatus,
    ) -> (Option<Gesture>, ProximityAction) {
        use firmware_lib::gesture_detector::GestureDetector as _;
        use model::telemetry::TelemetryClient as _;
        self.telemetry_client
            .borrow_mut()
            .report((direction, distance_mm));

        let now_us = embassy_time::Instant::now().as_micros();
        let gesture = self
            .gesture_detector
            .borrow_mut()
            .update((direction, distance_mm), now_us);

        // Register distance locally in the feature using direction map index
        let idx = match direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::West => 2,
        };
        self.distances[idx].set(distance_mm);

        let in_range = self
            .distances
            .iter()
            .any(|d| d.get() < self.wake_threshold_mm);

        let mut action = ProximityAction::None;
        if in_range != self.proximity_active.get() {
            self.proximity_active.set(in_range);
            if in_range {
                if status == SystemStatus::Active {
                    action = ProximityAction::AcquireWakeLock;
                } else if status == SystemStatus::Sleep {
                    action = ProximityAction::WakeSystem;
                }
            } else if status == SystemStatus::Active {
                action = ProximityAction::ReleaseWakeLock;
            }
        }

        (gesture, action)
    }

    fn map_gesture(&self, gesture: Gesture, _status: SystemStatus) -> GestureAction {
        #[allow(unreachable_patterns)]
        match gesture {
            Gesture::DualLongPress => self.dual_long_press_action,
            _ => GestureAction::None,
        }
    }

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        _crossed_tick: bool,
        _status: SystemStatus,
        support: DeviceSupport,
        _wake_locks: u32,
    ) {
        if support.proximity {
            for sensor_tx in &self.sensor_txs {
                let _ = sensor_tx.try_send(SensorCommand::ReadSensors);
            }
        }
    }
}

/// Standard config implementation for LedFeature.
pub struct LedFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// LED channel sender
    pub led_tx: Option<LedSender<MutexRaw, N>>,
}
impl<MutexRaw: RawMutex + 'static, const N: usize> LedFeatureConfig<MutexRaw, N> {
    /// Creates a new `LedFeatureConfig`.
    pub fn new(led_tx: Option<LedSender<MutexRaw, N>>) -> Self {
        Self { led_tx }
    }
}
impl<MutexRaw: RawMutex + 'static, const N: usize> SystemFeature<MutexRaw, N>
    for LedFeatureConfig<MutexRaw, N>
{
    fn on_init(&self) {
        if let Some(ref led_tx) = self.led_tx {
            let _ = led_tx.try_send(SystemLedState::Off);
        }
    }

    fn on_state_changed(
        &self,
        _from: SystemStatus,
        to: SystemStatus,
        support: DeviceSupport,
        battery_status: Option<BatteryStatus>,
        thermal_critical: bool,
    ) {
        if let Some(ref led_tx) = self.led_tx {
            let led = if support.led {
                if to == SystemStatus::Active {
                    battery_status
                        .map(|s| s.soc_led_state)
                        .unwrap_or(SystemLedState::Off)
                } else if thermal_critical {
                    SystemLedState::BlinksRedFourTimes
                } else {
                    SystemLedState::SolidBlue
                }
            } else if battery_status.map(|s| s.battery_critical).unwrap_or(false) {
                SystemLedState::BlinksRedOncePerThirtySeconds
            } else if battery_status.map(|s| s.charger_connected).unwrap_or(false) {
                battery_status
                    .map(|s| s.soc_led_state)
                    .unwrap_or(SystemLedState::Off)
            } else {
                SystemLedState::Off
            };
            let _ = led_tx.try_send(led);
        }
    }

    fn on_battery_action(
        &self,
        action: BatteryUpdateAction,
        status: SystemStatus,
        battery_status: Option<BatteryStatus>,
    ) {
        if action == BatteryUpdateAction::ReportSoC {
            if let Some(ref led_tx) = self.led_tx {
                if battery_status.map(|s| s.battery_critical).unwrap_or(false) {
                    let _ = led_tx.try_send(SystemLedState::BlinksRedOncePerThirtySeconds);
                } else if status == SystemStatus::PowerDown {
                    let led = if battery_status.map(|s| s.charger_connected).unwrap_or(false) {
                        battery_status
                            .map(|s| s.soc_led_state)
                            .unwrap_or(SystemLedState::Off)
                    } else {
                        SystemLedState::Off
                    };
                    let _ = led_tx.try_send(led);
                } else if status == SystemStatus::Active {
                    if let Some(s) = battery_status {
                        let _ = led_tx.try_send(s.soc_led_state);
                    }
                }
            }
        }
    }

    fn on_alert_triggered(&self, status: SystemStatus) {
        if status == SystemStatus::Sleep {
            if let Some(ref led_tx) = self.led_tx {
                let _ = led_tx.try_send(SystemLedState::BlinksRedFourTimes);
            }
        }
    }
}

/// Standard config implementation for ThermalFeature.
pub struct ThermalFeatureConfig<MutexRaw: RawMutex + 'static, const N: usize> {
    /// Thermal channel sender
    pub thermal_tx: Option<ThermalSender<MutexRaw, N>>,
    /// Thermal manager for checking alerts
    pub thermal_manager: core::cell::RefCell<ThermalManager>,
}
impl<MutexRaw: RawMutex + 'static, const N: usize> ThermalFeatureConfig<MutexRaw, N> {
    /// Creates a new `ThermalFeatureConfig`.
    pub fn new(thermal_tx: Option<ThermalSender<MutexRaw, N>>) -> Self {
        Self {
            thermal_tx,
            thermal_manager: core::cell::RefCell::new(ThermalManager::new()),
        }
    }
}
impl<MutexRaw: RawMutex + 'static, const N: usize> SystemFeature<MutexRaw, N>
    for ThermalFeatureConfig<MutexRaw, N>
{
    fn thermal_critical(&self) -> bool {
        self.thermal_manager.borrow().thermal_critical()
    }

    fn on_alert_triggered(&self, _status: SystemStatus) {
        self.thermal_manager.borrow_mut().set_thermal_critical(true);
    }

    fn on_tick(
        &self,
        _elapsed_ms: u32,
        crossed_tick: bool,
        _status: SystemStatus,
        support: DeviceSupport,
        _wake_locks: u32,
    ) {
        if crossed_tick && support.thermal {
            if let Some(ref thermal_tx) = self.thermal_tx {
                let _ = thermal_tx.try_send(ThermalCommand::CheckTemp);
            }
        }
    }
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
        charger_state: ChargeState,
        status: SystemStatus,
        boot_power_down: bool,
    ) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)>;
    /// Dispatch on_proximity_update hook to features.
    fn on_proximity_update(
        &self,
        direction: Direction,
        distance_mm: u16,
        status: SystemStatus,
    ) -> (Option<Gesture>, ProximityAction);
    /// Dispatch on_state_changed hook to all features.
    fn on_state_changed(
        &self,
        from: SystemStatus,
        to: SystemStatus,
        support: DeviceSupport,
        battery_status: Option<BatteryStatus>,
        thermal_critical: bool,
    );
    /// Dispatch on_battery_action hook to all features.
    fn on_battery_action(
        &self,
        action: BatteryUpdateAction,
        status: SystemStatus,
        battery_status: Option<BatteryStatus>,
    );
    /// Dispatch on_gesture hook to all features.
    fn on_gesture(&self, gesture: Gesture, status: SystemStatus);
    /// Dispatch map_gesture hook to features.
    fn map_gesture(&self, gesture: Gesture, status: SystemStatus) -> GestureAction;
    /// Dispatch on_tick hook to all features.
    fn on_tick(
        &self,
        elapsed_ms: u32,
        crossed_tick: bool,
        status: SystemStatus,
        support: DeviceSupport,
        wake_locks: u32,
    );
    /// Dispatch on_alert_triggered hook to all features.
    fn on_alert_triggered(&self, status: SystemStatus);
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
            fn on_battery_update(&self, _state_of_charge: u8, _charger_state: ChargeState, _status: SystemStatus, _boot_power_down: bool) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
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
            fn on_proximity_update(&self, _direction: Direction, _distance_mm: u16, _status: SystemStatus) -> (Option<Gesture>, ProximityAction) {
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
            fn on_state_changed(&self, _from: SystemStatus, _to: SystemStatus, _support: DeviceSupport, _battery_status: Option<BatteryStatus>, _thermal_critical: bool) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_state_changed(_from, _to, _support, _battery_status, _thermal_critical);)*
            }

            #[inline(always)]
            fn on_battery_action(&self, _action: BatteryUpdateAction, _status: SystemStatus, _battery_status: Option<BatteryStatus>) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_battery_action(_action, _status, _battery_status);)*
            }

            #[inline(always)]
            fn on_gesture(&self, _gesture: Gesture, _status: SystemStatus) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_gesture(_gesture, _status);)*
            }

            #[inline(always)]
            fn map_gesture(&self, _gesture: Gesture, _status: SystemStatus) -> GestureAction {
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
            fn on_tick(&self, _elapsed_ms: u32, _crossed_tick: bool, _status: SystemStatus, _support: DeviceSupport, _wake_locks: u32) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_tick(_elapsed_ms, _crossed_tick, _status, _support, _wake_locks);)*
            }

            #[inline(always)]
            fn on_alert_triggered(&self, _status: SystemStatus) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.on_alert_triggered(_status);)*
            }
        }
    };
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
