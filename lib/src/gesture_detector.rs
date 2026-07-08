//! Gesture detection library for proximity-based system states.

#![deny(missing_docs)]

use controller::telemetry_controller::ProximityTelemetryClient;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use model::telemetry::TelemetryClient;
use model::types::{Direction, Gesture, TelemetryRecord};

/// Trait for extensible gesture detection.
pub trait GestureDetector<Input> {
    /// The type of gesture produced by this detector.
    type Output;

    /// Processes a new input sample and returns a gesture event if detected.
    fn update(&mut self, input: Input, current_time_us: u64) -> Option<Self::Output>;

    /// Resets the internal state of the detector.
    fn reset(&mut self);
}

/// Proximity event from individual ToF sensors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProximityEvent {
    /// Sensor update with direction and distance.
    SensorUpdate {
        /// Sensor direction (North, East, West).
        direction: Direction,
        /// Measured distance in mm.
        distance_mm: u16,
    },
}

/// ProximityGestureDetector tracks Time-of-Flight (ToF) proximity sensor inputs.
/// A debounce state machine that tracks continuous proximity sensor holds using absolute system time in microseconds.
pub struct ProximityGestureDetector {
    press_start_time_us: Option<u64>,
    last_press_duration_us: u64,
    threshold_mm: u16,
    proximity_threshold_mm: u16,
    proximity_active: bool,
    distance_north: u16,
    distance_east: u16,
    distance_west: u16,
}

impl ProximityGestureDetector {
    /// Creates a new `ProximityGestureDetector` with custom thresholds in mm.
    pub const fn new(threshold_mm: u16, proximity_threshold_mm: u16) -> Self {
        Self {
            press_start_time_us: None,
            last_press_duration_us: 0,
            threshold_mm,
            proximity_threshold_mm,
            proximity_active: false,
            distance_north: 1000,
            distance_east: 1000,
            distance_west: 1000,
        }
    }

    /// Returns the proximity detection threshold in millimeters.
    pub fn proximity_threshold_mm(&self) -> u16 {
        self.proximity_threshold_mm
    }

    /// Registers a distance update for a given direction.
    pub fn register_distance(&mut self, direction: Direction, distance_mm: u16) {
        match direction {
            Direction::North => self.distance_north = distance_mm,
            Direction::East => self.distance_east = distance_mm,
            Direction::West => self.distance_west = distance_mm,
        }
    }

    /// Returns the current accumulated press duration in milliseconds.
    pub fn press_time_ms(&self) -> u32 {
        (self.last_press_duration_us / 1000) as u32
    }

    /// Returns the current proximity active state.
    pub fn proximity_active(&self) -> bool {
        self.proximity_active
    }

    /// Manually sets the proximity active state.
    pub fn set_proximity_active(&mut self, active: bool) {
        self.proximity_active = active;
    }

    fn update_internal(&mut self, current_time_us: u64) -> Option<Gesture> {
        let east_pressed = self.distance_east < self.threshold_mm;
        let west_pressed = self.distance_west < self.threshold_mm;

        if east_pressed && west_pressed {
            let start = match self.press_start_time_us {
                Some(s) => s,
                None => {
                    self.press_start_time_us = Some(current_time_us);
                    current_time_us
                }
            };
            let duration = current_time_us.saturating_sub(start);
            self.last_press_duration_us = duration;
            if duration >= 5_000_000 {
                return Some(Gesture::DualLongPress);
            }
        } else {
            self.press_start_time_us = None;
            self.last_press_duration_us = 0;
        }

        let in_range = self.distance_north < self.proximity_threshold_mm
            || self.distance_east < self.proximity_threshold_mm
            || self.distance_west < self.proximity_threshold_mm;

        if in_range != self.proximity_active {
            self.proximity_active = in_range;
            if in_range {
                Some(Gesture::ProximityDetected)
            } else {
                Some(Gesture::ProximityNotDetected)
            }
        } else {
            None
        }
    }
}

impl GestureDetector<(Direction, u16)> for ProximityGestureDetector {
    type Output = Gesture;

    fn update(&mut self, input: (Direction, u16), current_time_us: u64) -> Option<Self::Output> {
        self.register_distance(input.0, input.1);
        self.update_internal(current_time_us)
    }

    fn reset(&mut self) {
        self.press_start_time_us = None;
        self.last_press_duration_us = 0;
        self.proximity_active = false;
        self.distance_north = 1000;
        self.distance_east = 1000;
        self.distance_west = 1000;
    }
}

/// Task running gesture detection locally for the proximity subsystem.
#[embassy_executor::task]
pub async fn proximity_gesture_task(
    rx: Receiver<'static, CriticalSectionRawMutex, ProximityEvent, 4>,
    gesture_tx: Sender<'static, CriticalSectionRawMutex, Gesture, 4>,
    telemetry_tx: Sender<
        'static,
        CriticalSectionRawMutex,
        TelemetryRecord,
        { controller::telemetry_controller::CHANNEL_CAPACITY },
    >,
    proximity_threshold_mm: u16,
) -> ! {
    let mut gesture_detector = ProximityGestureDetector::new(20, proximity_threshold_mm);
    let mut proximity_telemetry_client =
        ProximityTelemetryClient::new(Some(telemetry_tx), proximity_threshold_mm);
    loop {
        let ProximityEvent::SensorUpdate {
            direction,
            distance_mm,
        } = rx.receive().await;
        proximity_telemetry_client.report((direction, distance_mm));

        let now_us = embassy_time::Instant::now().as_micros();
        if let Some(gesture) = gesture_detector.update((direction, distance_mm), now_us) {
            gesture_tx.send(gesture).await;
        }
    }
}

/// Macro helper to run the proximity gesture task.
#[macro_export]
macro_rules! run_proximity_gesture_task {
    (
        $spawner:expr,
        $rx:expr,
        $gesture_tx:expr,
        $telemetry_tx:expr,
        $proximity_threshold_mm:expr
    ) => {
        $spawner
            .spawn($crate::gesture_detector::proximity_gesture_task(
                $rx,
                $gesture_tx,
                $telemetry_tx,
                $proximity_threshold_mm,
            ))
            .unwrap();
    };
}
