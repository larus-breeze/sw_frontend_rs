use core::{cell::UnsafeCell, sync::atomic::Ordering::Relaxed};
use portable_atomic::AtomicBool;

pub struct Lock<T> {
    value: UnsafeCell<Option<T>>,
    borrowed: AtomicBool,
}

impl<'a, T> Lock<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            borrowed: AtomicBool::new(false),
        }
    }

    pub fn set(&self, value: T) {
        let v = self.value.get();
        unsafe {
            *v = Some(value);
        }
        self.borrowed.store(false, Relaxed);
    }

    pub fn lock<F, R>(&'a self, mut f: F) -> R
    where
        F: FnMut(Option<&mut T>) -> R,
    {
        let opt_val = self.get_mut();
        let r: R = f(opt_val);
        self.borrowed.store(false, Relaxed);
        r
    }

    fn get_mut(&'a self) -> Option<&'a mut T> {
        let r = self
            .borrowed
            .compare_exchange(false, true, Relaxed, Relaxed);
        // we know definitly, the *self.value.get() results in a valid ptr, so unsafe is ok.
        let opt_v = unsafe { (&mut *self.value.get()).as_mut() };
        // returns Some(&mut T) when a) value is set and b) value is not used by others
        match opt_v {
            Some(v) => match r {
                Ok(_) => Some(v),
                Err(_) => None,
            },
            None => None,
        }
    }
}

// AtomicBool ensures that there is no problem on a single processor system with sync
unsafe impl<T> Sync for Lock<T> {}
