use core::{cell::UnsafeCell, sync::atomic::Ordering::Relaxed};
use portable_atomic::AtomicBool;

pub struct Lock<T> {
    value: UnsafeCell<Option<T>>,
    borrowed: AtomicBool,
}

impl<T> Default for Lock<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Lock<T> {
    /// New with empty value
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            borrowed: AtomicBool::new(false),
        }
    }

    /// New with given value
    pub const fn new_with_value(value: T) -> Self {
        Self {
            value: UnsafeCell::new(Some(value)),
            borrowed: AtomicBool::new(false),
        }
    }

    /// Sets or replaces the internal value, panics if it is borrowed
    pub fn set(&self, value: T) {
        if self.borrowed.load(Relaxed) {
            panic!();
        }
        let v = self.value.get();
        unsafe {
            *v = Some(value);
        }
        self.borrowed.store(false, Relaxed);
    }

    /// lock() calls f with an value as argument
    ///  - when value is set and value is not borowed: f(Some(&mut val))
    ///  - when value is not set or is it borrowd: f(None)
    ///
    /// This function automatically ensures that the internal value is released again
    /// after use.
    pub fn lock_during_use<F, R>(&'a self, mut f: F) -> R
    where
        F: FnMut(Option<&mut T>) -> R,
    {
        let result = self.get_mut();
        let opt_val = result.unwrap_or_default();

        let r: R = f(opt_val);
        self.borrowed.store(false, Relaxed);
        r
    }

    /// Enable value after a call to get_mut()
    ///
    /// Note: The user is responsible for doing this at the right time!
    pub fn unlock(&self) {
        self.borrowed.store(false, Relaxed);
    }

    /// Get a reference to the value, if available
    ///
    /// If the value has just been used, this function returns an
    /// error. In all other cases, an option with or without content
    /// is returned
    #[allow(clippy::result_unit_err)]
    pub fn get_mut(&'a self) -> Result<Option<&'a mut T>, ()> {
        let r = self
            .borrowed
            .compare_exchange(false, true, Relaxed, Relaxed);

        match r {
            // we know definitly, the *self.value.get() results in a valid ptr, so unsafe is ok.
            Ok(_) => Ok(unsafe { (*self.value.get()).as_mut() }),
            Err(_) => Err(()),
        }
    }
}

// AtomicBool ensures that there is no problem on a single processor system with sync
unsafe impl<T> Sync for Lock<T> {}
