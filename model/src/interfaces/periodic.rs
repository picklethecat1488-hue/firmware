//! Generic interface for periodic interval configuration.

#![deny(missing_docs)]

use crate::types::PeriodicInterval;

/// Trait implemented by controllers that can be run periodically at a configurable interval.
pub trait Periodic {
    /// Configures the periodic execution interval.
    fn set_interval(&self, interval: PeriodicInterval);
}
