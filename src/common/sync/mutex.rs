#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::default::Default;
use core::hint::spin_loop;
use core::marker::Sync;
use core::ops::{Deref, DerefMut, Drop};
use core::option::Option::{self, None, Some};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::arch::reg::DAIF;

pub struct Mutex<T: ?Sized> {
    no_irq: bool,
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    irq_enabled_before: bool,
    lock: &'a AtomicBool,
    data: &'a mut T,
    no_irq: bool,
}

impl<'a, T: ?Sized> MutexGuard<'a, T> {
    pub fn get_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
    pub fn get(&self) -> &T {
        &*self.deref()
    }
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            no_irq: false,
        }
    }
    pub const fn new_no_irq(data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            no_irq: true,
        }
    }

    #[allow(unused)]
    pub fn into_inner(self) -> T {
        let Mutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    pub fn lock(&self) -> MutexGuard<T> {
        let mut irq_state = false;
        if self.no_irq {
            irq_state = !DAIF::Irq.is_disabled();
            DAIF::Irq.disable();
        }
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                spin_loop();
            }
        }
        MutexGuard {
            irq_enabled_before: irq_state,
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
            no_irq: self.no_irq,
        }
    }

    #[allow(unused)]
    pub unsafe fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let mut irq_state = false;
        if self.no_irq {
            irq_state = !DAIF::Irq.is_disabled();
            DAIF::Irq.disable();
        }
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard {
                irq_enabled_before: irq_state,
                lock: &self.lock,
                data: unsafe { &mut *self.data.get() },
                no_irq: self.no_irq,
            })
        } else {
            if irq_state {
                DAIF::Irq.enable();
            }
            None
        }
    }
}


impl<T: ?Sized + Default> Default for Mutex<T> {
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        if self.irq_enabled_before && self.no_irq {
            DAIF::Irq.enable();
        }
    }
}
