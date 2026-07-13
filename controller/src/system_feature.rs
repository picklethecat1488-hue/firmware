//! System features definitions and tuples dispatcher.

#![deny(missing_docs)]

use crate::types::{BatteryStatus, DeviceSupport, GestureAction, ProximityAction};
use embassy_sync::blocking_mutex::raw::RawMutex;
use firmware_lib::BatteryUpdateAction;

/// A single system feature that can react to system events and ticks.
pub trait SystemFeature<MutexRaw: RawMutex + 'static, const N: usize> {
    /// True if this feature defines/provides thermal thresholds.
    const HAS_THERMAL_THRESHOLDS: bool = false;

    /// Hook called when the system starts running.
    fn on_init(&self) {}

    /// Returns the default boot trap mask for this feature.
    fn default_boot_trap_mask(&self) -> u32 {
        0
    }

    /// Returns the thermal overheating threshold in milli-Celsius.
    fn thermal_overheating_temp_threshold(&self) -> i32 {
        45000
    }

    /// Returns the thermal critical threshold in milli-Celsius.
    fn thermal_critical_temp_threshold(&self) -> i32 {
        60000
    }

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
        _is_boot_trapped: bool,
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
    /// Number of features that define thermal thresholds.
    const THERMAL_FEATURE_COUNT: usize;
    /// Compile-time check that at most one feature defines thermal thresholds.
    const CHECK_THERMAL_FEATURE_COUNT: ();

    /// Dispatch on_init hook to all features.
    fn on_init(&self);
    /// Combine default boot trap masks from all features.
    fn default_boot_trap_mask(&self) -> u32;
    /// Combine thermal critical status from all features.
    fn thermal_critical(&self) -> bool;
    /// Dispatch on_battery_update hook to features.
    fn on_battery_update(
        &self,
        state_of_charge: u8,
        charger_state: model::types::ChargeState,
        status: model::types::SystemStatus,
        is_boot_trapped: bool,
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
    /// Combine thermal overheating thresholds from all features.
    fn thermal_overheating_temp_threshold(&self) -> i32;
    /// Combine thermal critical thresholds from all features.
    fn thermal_critical_temp_threshold(&self) -> i32;
}

macro_rules! impl_feature_list_for_tuple {
    ($($T:ident),*) => {
        impl<MutexRaw: RawMutex + 'static, const N: usize, $($T: SystemFeature<MutexRaw, N>),*> FeatureList<MutexRaw, N> for ($($T,)*) {
            const THERMAL_FEATURE_COUNT: usize = 0 $(+ if $T::HAS_THERMAL_THRESHOLDS { 1 } else { 0 })*;
            const CHECK_THERMAL_FEATURE_COUNT: () = {
                if <Self as FeatureList<MutexRaw, N>>::THERMAL_FEATURE_COUNT > 1 {
                    panic!("Multiple features cannot define thermal thresholds!");
                }
            };

            #[inline(always)]
            fn on_init(&self) {
                let _ = <Self as FeatureList<MutexRaw, N>>::CHECK_THERMAL_FEATURE_COUNT;
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $( $T.on_init(); )*
            }

            #[inline(always)]
            fn default_boot_trap_mask(&self) -> u32 {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $( $T.default_boot_trap_mask() | )* 0
            }

            #[inline(always)]
            fn thermal_critical(&self) -> bool {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $($T.thermal_critical() ||)* false
            }

            #[inline(always)]
            fn on_battery_update(&self, _state_of_charge: u8, _charger_state: model::types::ChargeState, _status: model::types::SystemStatus, _is_boot_trapped: bool) -> Option<(Option<BatteryUpdateAction>, BatteryStatus)> {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    if let Some(res) = $T.on_battery_update(_state_of_charge, _charger_state, _status, _is_boot_trapped) {
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

            #[inline(always)]
            fn thermal_overheating_temp_threshold(&self) -> i32 {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    if $T::HAS_THERMAL_THRESHOLDS {
                        return $T.thermal_overheating_temp_threshold();
                    }
                )*
                45000
            }

            #[inline(always)]
            fn thermal_critical_temp_threshold(&self) -> i32 {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    if $T::HAS_THERMAL_THRESHOLDS {
                        return $T.thermal_critical_temp_threshold();
                    }
                )*
                60000
            }
        }
    }
}

macro_rules! impl_feature_list_for_tuples {
    () => {
        impl_feature_list_for_tuple!();
    };
    ($($T:ident),+ $(,)?) => {
        impl_feature_list_for_tuple!($($T),+);
        impl_feature_list_for_tuples_helper!($($T),+);
    };
}

macro_rules! impl_feature_list_for_tuples_helper {
    ($head:ident, $($tail:ident),+) => {
        impl_feature_list_for_tuples!($($tail),+);
    };
    ($last:ident) => {
        impl_feature_list_for_tuples!();
    };
}

impl_feature_list_for_tuples!(A, B, C, D, E, F, G, H, I, J);
