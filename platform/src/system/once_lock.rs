//! Target-agnostic OnceLock primitive for inter-core pointer and state synchronization.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// A synchronization primitive which can be written to once and can be shared between cores.
pub struct OnceLock<T> {
    cell: UnsafeCell<Option<T>>,
    initialized: AtomicBool,
}

unsafe impl<T> Sync for OnceLock<T> {}
unsafe impl<T> Send for OnceLock<T> {}

impl<T> OnceLock<T> {
    /// Creates a new empty OnceLock.
    pub const fn new() -> Self {
        Self {
            cell: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
        }
    }
}

impl<T> Default for OnceLock<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OnceLock<T> {
    /// Sets the value of the OnceLock.
    ///
    /// Returns Err if the OnceLock was already initialized.
    pub fn set(&self, value: T) -> Result<(), T> {
        if self.initialized.load(Ordering::Acquire) {
            return Err(value);
        }
        // SAFETY: We checked initialized, and OnceLock can only be written once.
        unsafe {
            let slot = &mut *self.cell.get();
            *slot = Some(value);
        }
        self.initialized.store(true, Ordering::Release);
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        cortex_m::asm::sev();
        Ok(())
    }

    /// Gets the reference to the underlying value, blocking/spinning if it is not initialized yet.
    pub fn wait(&self) -> &T {
        while !self.initialized.load(Ordering::Acquire) {
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            cortex_m::asm::wfe();
            #[cfg(not(all(target_arch = "arm", target_os = "none")))]
            std::thread::yield_now();
        }
        // SAFETY: The value is initialized and will never be modified again.
        unsafe { (*self.cell.get()).as_ref().unwrap() }
    }

    /// Tries to get the reference to the underlying value if it is initialized.
    pub fn get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            // SAFETY: The value is initialized and will never be modified again.
            unsafe { (*self.cell.get()).as_ref() }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(not(all(target_arch = "arm", target_os = "none")), derive(Debug))]
    struct SendPtr(pub *mut ());
    unsafe impl Send for SendPtr {}
    unsafe impl Sync for SendPtr {}

    #[test]
    fn test_once_lock_pointer_flow() {
        let lock: OnceLock<SendPtr> = OnceLock::new();
        assert!(lock.get().is_none());

        let mut val = 42u32;
        let ptr = SendPtr(&mut val as *mut u32 as *mut ());
        assert!(lock.set(ptr).is_ok());
        assert_eq!(*lock.wait(), ptr);
        assert_eq!(lock.get(), Some(&ptr));

        let mut val2 = 100u32;
        assert!(lock.set(SendPtr(&mut val2 as *mut u32 as *mut ())).is_err());
    }

    #[test]
    fn test_once_lock_pointer_blocking_wait() {
        use std::sync::Arc;
        let lock = Arc::new(OnceLock::<SendPtr>::new());
        let lock_clone = Arc::clone(&lock);

        let mut val = 123u32;
        let ptr = SendPtr(&mut val as *mut u32 as *mut ());

        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(15));
            assert!(lock_clone.set(ptr).is_ok());
        });

        assert_eq!(*lock.wait(), ptr);
        handle.join().unwrap();
    }
}
