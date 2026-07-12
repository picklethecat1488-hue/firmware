//! Gesture detection library for proximity-based system states.

#![deny(missing_docs)]

use model::types::{Direction, Gesture};

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
    press_threshold_mm: u16,
    distance_east: u16,
    distance_west: u16,
}

impl ProximityGestureDetector {
    /// Creates a new `ProximityGestureDetector` with custom thresholds in mm.
    pub const fn new(press_threshold_mm: u16) -> Self {
        Self {
            press_start_time_us: None,
            last_press_duration_us: 0,
            press_threshold_mm,
            distance_east: 1000,
            distance_west: 1000,
        }
    }

    /// Registers a distance update for a given direction.
    pub fn register_distance(&mut self, direction: Direction, distance_mm: u16) {
        match direction {
            Direction::East => self.distance_east = distance_mm,
            Direction::West => self.distance_west = distance_mm,
            _ => {}
        }
    }

    /// Returns the current accumulated press duration in milliseconds.
    pub fn press_time_ms(&self) -> u32 {
        (self.last_press_duration_us / 1000) as u32
    }

    fn update_internal(&mut self, current_time_us: u64) -> Option<Gesture> {
        let east_pressed = self.distance_east < self.press_threshold_mm;
        let west_pressed = self.distance_west < self.press_threshold_mm;

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
        None
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
        self.distance_east = 1000;
        self.distance_west = 1000;
    }
}

/// Channel type for gesture communication.
pub type GestureChannel<MutexRaw, const N: usize> =
    embassy_sync::channel::Channel<MutexRaw, Gesture, N>;
/// Sender type for gesture communication.
pub type GestureSender<MutexRaw, const N: usize> =
    embassy_sync::channel::Sender<'static, MutexRaw, Gesture, N>;
/// Receiver type for gesture communication.
pub type GestureReceiver<MutexRaw, const N: usize> =
    embassy_sync::channel::Receiver<'static, MutexRaw, Gesture, N>;
