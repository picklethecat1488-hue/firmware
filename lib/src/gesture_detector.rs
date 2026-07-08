//! Gesture detection library for proximity-based system states.

#![deny(missing_docs)]

/// GestureDetector tracks Time-of-Flight (ToF) proximity sensor inputs.
/// A debounce state machine that tracks continuous proximity sensor holds using absolute system time in microseconds.
pub struct GestureDetector {
    press_start_time_us: Option<u64>,
    last_press_duration_us: u64,
    threshold_mm: u16,
    proximity_threshold_mm: u16,
    proximity_active: bool,
}

impl GestureDetector {
    /// Creates a new `GestureDetector` with custom thresholds in mm.
    pub const fn new(threshold_mm: u16, proximity_threshold_mm: u16) -> Self {
        Self {
            press_start_time_us: None,
            last_press_duration_us: 0,
            threshold_mm,
            proximity_threshold_mm,
            proximity_active: false,
        }
    }

    /// Updates the state machine with current sensor distances and the current absolute system time in microseconds.
    /// Returns:
    /// - Some(Gesture::DualLongPress) if held continuously for 5 seconds (5_000_000 us).
    /// - Some(Gesture::ProximityDetected) if proximity status transitions to active.
    /// - Some(Gesture::ProximityNotDetected) if proximity status transitions to inactive.
    /// - None otherwise.
    pub fn update(
        &mut self,
        gesture: model::types::Gesture,
        current_time_us: u64,
    ) -> Option<model::types::Gesture> {
        let (dist_north_mm, dist_east_mm, dist_west_mm) = match gesture {
            model::types::Gesture::Proximity(n, e, w) => (n, e, w),
            _ => return None,
        };
        let east_pressed = dist_east_mm < self.threshold_mm;
        let west_pressed = dist_west_mm < self.threshold_mm;

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
                return Some(model::types::Gesture::DualLongPress);
            }
        } else {
            self.press_start_time_us = None;
            self.last_press_duration_us = 0;
        }

        let in_range = dist_north_mm < self.proximity_threshold_mm
            || dist_east_mm < self.proximity_threshold_mm
            || dist_west_mm < self.proximity_threshold_mm;

        if in_range != self.proximity_active {
            self.proximity_active = in_range;
            if in_range {
                Some(model::types::Gesture::ProximityDetected)
            } else {
                Some(model::types::Gesture::ProximityNotDetected)
            }
        } else {
            None
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

    /// Resets the internal debounce timer and proximity state.
    pub fn reset(&mut self) {
        self.press_start_time_us = None;
        self.last_press_duration_us = 0;
        self.proximity_active = false;
    }
}
