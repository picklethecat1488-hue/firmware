//! Gesture detection library for proximity-based system states.

#![deny(missing_docs)]

use crate::types::{Direction, Gesture};

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
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ProximityEvent {
    /// Sensor update with direction and distance.
    SensorUpdate {
        /// Sensor direction (North, East, West).
        direction: Direction,
        /// Measured distance in mm.
        distance_mm: u16,
    },
}

/// Proximity state of a sensor tracked by the gesture detector.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub enum ProximityState {
    /// Distance is greater than press_threshold_mm
    OutOfRange,
    /// Distance is within press_threshold_mm
    InRange,
    /// Distance is less than 100mm
    Near,
    /// Distance is less than 20mm (down)
    Down,
}

/// Tracks distance and state for a single sensor.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
pub struct ProximityTracker {
    /// Sensor direction.
    pub direction: Direction,
    /// Latest measured distance in mm.
    pub distance_mm: u16,
    /// Latest proximity state.
    pub state: ProximityState,
}

impl ProximityTracker {
    /// Creates a new `ProximityTracker` for a given direction.
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            distance_mm: u16::MAX,
            state: ProximityState::OutOfRange,
        }
    }

    /// Resets the tracker state.
    pub fn reset(&mut self) {
        self.distance_mm = u16::MAX;
        self.state = ProximityState::OutOfRange;
    }
}

/// The duration in microseconds required to trigger a dual long press gesture (2 seconds).
pub const DUAL_LONG_PRESS_DURATION_US: u64 = 2_000_000;

/// The maximum number of proximity trackers (one for each sensor direction).
pub const MAX_PROXIMITY_TRACKERS: usize = 3;

/// ProximityGestureDetector tracks Time-of-Flight (ToF) proximity sensor inputs.
/// A debounce state machine that tracks continuous proximity sensor holds using absolute system time in microseconds.
pub struct ProximityGestureDetector {
    press_start_time_us: Option<u64>,
    last_press_duration_us: u64,
    press_threshold_mm: u16,
    near_threshold_mm: u16,
    wake_threshold_mm: u16,
    gesture_triggered: bool,
    /// Active proximity trackers for each sensor direction.
    pub trackers: [ProximityTracker; MAX_PROXIMITY_TRACKERS],
}

impl ProximityGestureDetector {
    /// Creates a new `ProximityGestureDetector` with custom thresholds in mm.
    pub const fn new(
        press_threshold_mm: u16,
        near_threshold_mm: u16,
        wake_threshold_mm: u16,
    ) -> Self {
        Self {
            press_start_time_us: None,
            last_press_duration_us: 0,
            press_threshold_mm,
            near_threshold_mm,
            wake_threshold_mm,
            gesture_triggered: false,
            trackers: [
                ProximityTracker::new(Direction::North),
                ProximityTracker::new(Direction::East),
                ProximityTracker::new(Direction::West),
            ],
        }
    }

    /// Registers a distance update for a given direction.
    pub fn register_distance(&mut self, direction: Direction, distance_mm: u16) {
        let new_state = if distance_mm <= self.press_threshold_mm {
            ProximityState::Down
        } else if distance_mm < self.near_threshold_mm {
            ProximityState::Near
        } else if distance_mm < self.wake_threshold_mm {
            ProximityState::InRange
        } else {
            ProximityState::OutOfRange
        };

        for tracker in &mut self.trackers {
            if tracker.direction == direction {
                tracker.distance_mm = distance_mm;
                if tracker.state != new_state {
                    tracker.state = new_state;
                }
                break;
            }
        }
    }

    /// Returns the current accumulated press duration in milliseconds.
    pub fn press_time_ms(&self) -> u32 {
        (self.last_press_duration_us / 1000) as u32
    }

    fn update_internal(&mut self, current_time_us: u64) -> Option<Gesture> {
        let mut east_pressed = false;
        let mut west_pressed = false;

        for tracker in &self.trackers {
            if tracker.direction == Direction::East {
                east_pressed = tracker.distance_mm < self.press_threshold_mm;
            } else if tracker.direction == Direction::West {
                west_pressed = tracker.distance_mm < self.press_threshold_mm;
            }
        }

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
            if duration >= DUAL_LONG_PRESS_DURATION_US && !self.gesture_triggered {
                self.gesture_triggered = true;
                return Some(Gesture::DualLongPress);
            }
        } else {
            self.press_start_time_us = None;
            self.last_press_duration_us = 0;
            self.gesture_triggered = false;
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
        self.gesture_triggered = false;
        for tracker in &mut self.trackers {
            tracker.reset();
        }
    }
}
