//! Gesture detection library for proximity-based system states.

#![no_std]
#![deny(missing_docs)]

/// GestureDetector tracks Time-of-Flight (ToF) proximity sensor inputs
/// A debounce state machine that tracks continuous proximity sensor holds using absolute system time in microseconds.
pub struct GestureDetector {
    press_start_time_us: Option<u64>,
    last_press_duration_us: u64,
    threshold_mm: u16,
}

impl GestureDetector {
    /// Creates a new `GestureDetector` with a custom proximity threshold in mm.
    pub const fn new(threshold_mm: u16) -> Self {
        Self {
            press_start_time_us: None,
            last_press_duration_us: 0,
            threshold_mm,
        }
    }

    /// Updates the state machine with current sensor distances and the current absolute system time in microseconds.
    /// Returns:
    /// - Some(Gesture::DualLongPress) if held continuously for 5 seconds.
    /// - Some(Gesture::ProximityDetected) if any sensor is < 300 mm.
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

        if dist_north_mm < 300 || dist_east_mm < 300 || dist_west_mm < 300 {
            Some(model::types::Gesture::ProximityDetected)
        } else {
            Some(model::types::Gesture::ProximityNotDetected)
        }
    }

    /// Returns the current accumulated press duration in milliseconds.
    pub fn press_time_ms(&self) -> u32 {
        (self.last_press_duration_us / 1000) as u32
    }

    /// Resets the internal debounce timer state.
    pub fn reset(&mut self) {
        self.press_start_time_us = None;
        self.last_press_duration_us = 0;
    }
}
