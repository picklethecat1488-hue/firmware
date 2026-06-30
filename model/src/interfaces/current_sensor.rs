//! Generic current sensor interface.

#![deny(missing_docs)]

/// Trait representing a current sensor capable of reading current draw.
pub trait CurrentSensor {
    /// Error type for current sensor transactions.
    type Error;

    /// Reads the current draw in milliamperes (mA).
    fn read_current_ma(&mut self) -> Result<i32, Self::Error>;
}
