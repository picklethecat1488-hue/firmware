//! Periodic timer utilities.

use embassy_time::{Duration, Instant};

/// A periodic timer for managing timeout conditions and periodic ticks.
pub struct PeriodicTimer {
    interval: Duration,
    last_tick: Instant,
}

impl PeriodicTimer {
    /// Creates a new periodic timer with the specified interval.
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_tick: Instant::now(),
        }
    }

    /// Resets the last tick time to the current instant.
    pub fn reset(&mut self) {
        self.last_tick = Instant::now();
    }

    /// Returns true if the timer interval has elapsed.
    pub fn expired(&self) -> bool {
        let elapsed = Instant::now().duration_since(self.last_tick);
        elapsed >= self.interval
    }

    /// Returns the remaining duration until the next tick.
    pub fn remaining(&self) -> Duration {
        let elapsed = Instant::now().duration_since(self.last_tick);
        if elapsed >= self.interval {
            Duration::from_ticks(0)
        } else {
            self.interval - elapsed
        }
    }

    /// Returns the remaining time in milliseconds until the next tick.
    pub fn remaining_ms(&self) -> u32 {
        self.remaining().as_millis() as u32
    }

    /// Returns the elapsed milliseconds since the last tick/reset, and resets the timer to now.
    pub fn elapsed_ms_and_reset(&mut self) -> u32 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;
        elapsed.as_millis() as u32
    }

    /// If the interval has elapsed, resets the timer to now and returns the elapsed milliseconds.
    /// Otherwise, returns None.
    pub fn expired_and_reset(&mut self) -> Option<u32> {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        if elapsed >= self.interval {
            self.last_tick = now;
            Some(elapsed.as_millis() as u32)
        } else {
            None
        }
    }
}
