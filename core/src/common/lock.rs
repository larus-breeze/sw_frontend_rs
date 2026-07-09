use core::{cell::UnsafeCell, sync::atomic::Ordering::{Acquire, Release}};
use portable_atomic::AtomicBool;

pub struct Lock<T> {
    value: UnsafeCell<Option<T>>,
    borrowed: AtomicBool,
}

/// An RAII implementation of a "scoped lock" of a Mutex.
/// When this structure is dropped (falls out of scope), the lock will be unlocked.
pub struct LockGuard<'a, T> {
    lock: &'a Lock<T>,
}

impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        // Release ordering ensures that all previous memory writes 
        // are visible before the lock is released.
        self.lock.borrowed.store(false, Release);
    }
}

impl<'a, T> LockGuard<'a, T> {
    /// Provides exclusive mutable access to the inner value.
    /// This is safe because the existence of the LockGuard guarantees exclusive access.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { (*self.lock.value.get()).as_mut() }
    }
}

impl<T> Default for Lock<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Lock<T> {
    /// Creates a new, empty lock instance.
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            borrowed: AtomicBool::new(false),
        }
    }

    /// Creates a new lock instance initialized with a value.
    pub const fn new_with_value(value: T) -> Self {
        Self {
            value: UnsafeCell::new(Some(value)),
            borrowed: AtomicBool::new(false),
        }
    }

    /// Tries to acquire the lock. 
    /// Returns a `LockGuard` if successful, or `Err(())` if already borrowed.
    pub fn try_lock(&self) -> Result<LockGuard<'_, T>, ()> {
        // Acquire ordering ensures that subsequent memory reads/writes 
        // cannot be reordered before this atomic check.
        if self.borrowed.compare_exchange(false, true, Acquire, Acquire).is_ok() {
            Ok(LockGuard { lock: self })
        } else {
            Err(())
        }
    }

    /// Sets or replaces the internal value. Panics if the lock is currently borrowed.
    pub fn set(&self, value: T) {
        if let Ok(_guard) = self.try_lock() {
            unsafe {
                *self.value.get() = Some(value);
            }
        } else {
            panic!("Lock is already borrowed!");
        }
    }

    /// Executes the closure `f` with an exclusive reference to the inner value.
    ///  - If the lock is available: calls f(Some(&mut val))
    ///  - If the lock is already borrowed: calls f(None) without breaking the existing lock.
    pub fn lock_during_use<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(Option<&mut T>) -> R,
    {
        match self.try_lock() {
            Ok(mut guard) => {
                // Lock acquired successfully, pass the mutable reference
                f(guard.as_mut())
            } // <- `guard` is automatically dropped here, unlocking the Mutex safely
            Err(_) => {
                // Lock is busy. Pass None, but DO NOT modify `self.borrowed`.
                f(None)
            }
        }
    }
}

// Thread-safety marker: Sync is safe now because `LockGuard` and the 
// Acquire/Release ordering guarantee strict exclusivity across execution contexts.
unsafe impl<T> Sync for Lock<T> {}