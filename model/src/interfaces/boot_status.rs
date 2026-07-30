//! Generic boot status trait.

#![deny(missing_docs)]

/// Trait for recording boot status and early errors.
pub trait BootStatus {
    /// Record a boot-time peripheral error.
    fn record_error(&mut self, error: crate::types::PeripheralError);
}

impl<const N: usize> BootStatus for heapless::Vec<crate::types::PeripheralError, N> {
    fn record_error(&mut self, error: crate::types::PeripheralError) {
        let _ = self.push(error);
    }
}
