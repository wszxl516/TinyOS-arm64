#![allow(dead_code)]
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::arch::reg::DAIF;

pub struct SpinNoIrqLock<T: ?Sized> {
    pub(crate) lock: AtomicBool,
    pub(crate) data: UnsafeCell<T>,
}

pub struct SpinNoIrqLockGuard<'a, T: ?Sized + 'a> {
    irq_enabled_before: bool,
    lock: &'a AtomicBool,
    data: &'a mut T,
}

unsafe impl<T: ?Sized + Send> Sync for SpinNoIrqLock<T> {}

unsafe impl<T: ?Sized + Send> Send for SpinNoIrqLock<T> {}

impl<T> SpinNoIrqLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    pub unsafe fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }

    pub fn lock(&self) -> SpinNoIrqLockGuard<T> {
        let irq_enabled_before = !DAIF::Irq.is_disabled();
        DAIF::Irq.disable();
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                core::hint::spin_loop();
            }
        }
        SpinNoIrqLockGuard {
            irq_enabled_before,
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    pub fn try_lock(&self) -> Option<SpinNoIrqLockGuard<T>> {
        let irq_enabled_before = !DAIF::Irq.is_disabled();
        DAIF::Irq.disable();
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinNoIrqLockGuard {
                irq_enabled_before,
                lock: &self.lock,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            if irq_enabled_before {
                DAIF::Irq.enable();
            }
            None
        }
    }
}


impl<T: Default> Default for SpinNoIrqLock<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for SpinNoIrqLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for SpinNoIrqLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for SpinNoIrqLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        if self.irq_enabled_before {
            DAIF::Irq.enable();
        }
    }
}
