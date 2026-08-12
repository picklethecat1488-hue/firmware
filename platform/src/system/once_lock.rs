//! Target-agnostic OnceLock primitive for inter-core pointer and state synchronization.

use core::cell::{RefCell, UnsafeCell};
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{RawWaker, RawWakerVTable, Waker};
use critical_section::Mutex;

const NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |data| RawWaker::new(data, &NOOP_VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

const DUMMY_WAKER: Waker =
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &NOOP_VTABLE)) };

/// A synchronization primitive which can be written to once and can be shared between cores.
pub struct OnceLock<T> {
    cell: UnsafeCell<Option<T>>,
    initialized: AtomicBool,
    waker: Mutex<RefCell<Waker>>,
}

unsafe impl<T> Sync for OnceLock<T> {}
unsafe impl<T> Send for OnceLock<T> {}

impl<T> OnceLock<T> {
    /// Creates a new empty OnceLock.
    pub const fn new() -> Self {
        Self {
            cell: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
            waker: Mutex::new(RefCell::new(DUMMY_WAKER)),
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

        // Wake the pending waker and replace it with DUMMY_WAKER to drop it.
        critical_section::with(|cs| {
            let waker = self.waker.borrow(cs).replace(DUMMY_WAKER);
            waker.wake();
        });

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        cortex_m::asm::sev();
        Ok(())
    }

    /// Gets the reference to the underlying value asynchronously, yielding if not initialized.
    pub async fn wait(&self) -> &T {
        // Fast path: if already initialized, return immediately without locking
        if self.initialized.load(Ordering::Acquire) {
            return unsafe { (*self.cell.get()).as_ref().unwrap() };
        }

        struct WaitFuture<'a, T> {
            once_lock: &'a OnceLock<T>,
        }

        impl<'a, T> core::future::Future for WaitFuture<'a, T> {
            type Output = &'a T;

            fn poll(
                self: core::pin::Pin<&mut Self>,
                cx: &mut core::task::Context<'_>,
            ) -> core::task::Poll<Self::Output> {
                if self.once_lock.initialized.load(Ordering::Acquire) {
                    return core::task::Poll::Ready(unsafe {
                        (*self.once_lock.cell.get()).as_ref().unwrap()
                    });
                }

                critical_section::with(|cs| {
                    if self.once_lock.initialized.load(Ordering::Acquire) {
                        core::task::Poll::Ready(unsafe {
                            (*self.once_lock.cell.get()).as_ref().unwrap()
                        })
                    } else {
                        let mut waker_slot = self.once_lock.waker.borrow(cs).borrow_mut();
                        let current_waker = cx.waker();
                        if !waker_slot.will_wake(current_waker) {
                            *waker_slot = current_waker.clone();
                        }
                        core::task::Poll::Pending
                    }
                })
            }
        }

        WaitFuture { once_lock: self }.await
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

/// A convenience wrapper for CLI completion signals sent across task boundaries.
///
/// Implements Send and Sync by wrapping a raw pointer to a OnceLock.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CliSignal<T>(pub *const OnceLock<T>);

unsafe impl<T> Send for CliSignal<T> {}
unsafe impl<T> Sync for CliSignal<T> {}

impl<T> CliSignal<T> {
    /// Creates a new CliSignal from a OnceLock reference.
    pub fn new(lock: &OnceLock<T>) -> Self {
        Self(lock as *const _)
    }

    /// Sets the value of the OnceLock.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the underlying OnceLock has not been dropped.
    pub unsafe fn set(&self, value: T) -> Result<(), T> {
        (&*self.0).set(value)
    }

    /// Waits for the OnceLock to be initialized asynchronously.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the underlying OnceLock has not been dropped.
    pub async unsafe fn wait(&self) -> &T {
        (&*self.0).wait().await
    }
}

impl<T> core::fmt::Debug for CliSignal<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CliSignal({:p})", self.0)
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
        assert_eq!(*futures::executor::block_on(lock.wait()), ptr);
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

        assert_eq!(*futures::executor::block_on(lock.wait()), ptr);
        handle.join().unwrap();
    }

    #[test]
    fn test_once_lock_async_wait() {
        use std::sync::Arc;
        let lock = Arc::new(OnceLock::<SendPtr>::new());
        let lock_clone = Arc::clone(&lock);

        let mut val = 456u32;
        let ptr = SendPtr(&mut val as *mut u32 as *mut ());

        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(15));
            assert!(lock_clone.set(ptr).is_ok());
        });

        let result = futures::executor::block_on(lock.wait());
        assert_eq!(*result, ptr);
        handle.join().unwrap();
    }
}
