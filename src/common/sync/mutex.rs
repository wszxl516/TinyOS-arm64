#![allow(dead_code)]
use core::cell::UnsafeCell;
use core::default::Default;
use core::hint::spin_loop as cpu_relax;
use core::marker::Sync;
use core::ops::{Deref, DerefMut, Drop};
use core::option::Option::{self, None, Some};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
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
    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }

    #[allow(unused)]
    pub fn into_inner(self) -> T {
        let Mutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    fn obtain_lock(&self) {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .unwrap_or(false)
            != false
        {
            while self.lock.load(Ordering::Relaxed) {
                cpu_relax();
            }
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.obtain_lock();
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    #[allow(unused)]
    pub unsafe fn force_unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        match self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(l) => match l {
                true => None,
                false => Some(MutexGuard {
                    lock: &self.lock,
                    data: unsafe { &mut *self.data.get() },
                }),
            },
            Err(_) => None,
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
    }
}
