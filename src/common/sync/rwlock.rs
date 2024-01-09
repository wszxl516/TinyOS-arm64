#![allow(dead_code)]

use core::{
    cell::UnsafeCell,
    fmt,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};
//https://github.com/mvdnes/spin-rs/blob/master/src/rwlock.rs
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct RwLock<T: ?Sized> {
    lock: AtomicUsize,
    data: UnsafeCell<T>,
}

const READER: usize = 1 << 2;
const UPGRADED: usize = 1 << 1;
const WRITER: usize = 1;

pub struct ReadGuard<'a, T: 'a + ?Sized> {
    lock: &'a AtomicUsize,
    data: *const T,
}

pub struct WriteGuard<'a, T: 'a + ?Sized> {
    inner: &'a RwLock<T>,
    data: *mut T,
}


// Same unsafe impls as `std::sync::RwLock`
unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

unsafe impl<T: ?Sized + Send + Sync> Send for WriteGuard<'_, T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for WriteGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Send for ReadGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Sync for ReadGuard<'_, T> {}

impl<T> RwLock<T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        RwLock {
            lock: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Consumes this `RwLock`, returning the underlying data.
    #[inline]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let RwLock { data, .. } = self;
        data.into_inner()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.get()
    }
}

impl<T: ?Sized> RwLock<T> {
    #[inline]
    pub fn read(&self) -> ReadGuard<T> {
        loop {
            match self.try_read() {
                Some(guard) => return guard,
                _ => {}
            }
        }
    }

    #[inline]
    pub fn write(&self) -> WriteGuard<T> {
        loop {
            match self.try_write_internal(false) {
                Some(guard) => return guard,
                _ => {}
            }
        }
    }
}

impl<T: ?Sized> RwLock<T> {
    fn acquire_reader(&self) -> usize {
        const MAX_READERS: usize = core::usize::MAX / READER / 2;

        let value = self.lock.fetch_add(READER, Ordering::Acquire);

        if value > MAX_READERS * READER {
            self.lock.fetch_sub(READER, Ordering::Relaxed);
            panic!("Too many lock readers, cannot safely proceed");
        } else {
            value
        }
    }

    #[inline]
    pub fn try_read(&self) -> Option<ReadGuard<T>> {
        let value = self.acquire_reader();

        // We check the UPGRADED bit here so that new readers are prevented when an UPGRADED lock is held.
        // This helps reduce writer starvation.
        if value & (WRITER | UPGRADED) != 0 {
            // Lock is taken, undo.
            self.lock.fetch_sub(READER, Ordering::Release);
            None
        } else {
            Some(ReadGuard {
                lock: &self.lock,
                data: unsafe { &*self.data.get() },
            })
        }
    }

    /// Return the number of readers that currently hold the lock (including upgradable readers).
    ///
    /// # Safety
    ///
    /// This function provides no synchronization guarantees and so its result should be considered 'out of date'
    /// the instant it is called. Do not use it for synchronization purposes. However, it may be useful as a heuristic.
    pub fn reader_count(&self) -> usize {
        let state = self.lock.load(Ordering::Relaxed);
        state / READER + (state & UPGRADED) / UPGRADED
    }

    /// Return the number of writers that currently hold the lock.
    ///
    /// Because [`RwLock`] guarantees exclusive mutable access, this function may only return either `0` or `1`.
    ///
    /// # Safety
    ///
    /// This function provides no synchronization guarantees and so its result should be considered 'out of date'
    /// the instant it is called. Do not use it for synchronization purposes. However, it may be useful as a heuristic.
    pub fn writer_count(&self) -> usize {
        (self.lock.load(Ordering::Relaxed) & WRITER) / WRITER
    }

    /// Force decrement the reader count.
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if there are outstanding `RwLockReadGuard`s
    /// live, or if called more times than `read` has been called, but can be
    /// useful in FFI contexts where the caller doesn't know how to deal with
    /// RAII. The underlying atomic operation uses `Ordering::Release`.
    #[inline]
    pub unsafe fn force_read_decrement(&self) {
        debug_assert!(self.lock.load(Ordering::Relaxed) & !WRITER > 0);
        self.lock.fetch_sub(READER, Ordering::Release);
    }

    /// Force unlock exclusive write access.
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if there are outstanding `RwLockWriteGuard`s
    /// live, or if called when there are current readers, but can be useful in
    /// FFI contexts where the caller doesn't know how to deal with RAII. The
    /// underlying atomic operation uses `Ordering::Release`.
    #[inline]
    pub unsafe fn force_write_unlock(&self) {
        debug_assert_eq!(self.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED), 0);
        self.lock.fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }

    #[inline(always)]
    fn try_write_internal(&self, strong: bool) -> Option<WriteGuard<T>> {
        if match strong {
            true => self
                .lock
                .compare_exchange(0, WRITER, Ordering::Acquire, Ordering::Relaxed),
            false => {
                self.lock
                    .compare_exchange_weak(0, WRITER, Ordering::Acquire, Ordering::Relaxed)
            }
        }
            .is_ok()
        {
            Some(WriteGuard {
                inner: self,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn try_write(&self) -> Option<WriteGuard<T>> {
        self.try_write_internal(true)
    }

    /// Attempt to lock this rwlock with exclusive write access.
    ///
    /// Unlike [`RwLock::try_write`], this function is allowed to spuriously fail even when acquiring exclusive write access
    /// would otherwise succeed, which can result in more efficient code on some platforms.
    #[inline]
    pub fn try_write_weak(&self) -> Option<WriteGuard<T>> {
        self.try_write_internal(false)
    }

    pub fn get_mut(&mut self) -> &mut T {
        // We know statically that there are no other references to `self`, so
        // there's no need to lock the inner lock.
        unsafe { &mut *self.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_read() {
            Some(guard) => write!(f, "RwLock {{ data: ")
                .and_then(|()| (&*guard).fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "RwLock {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for RwLock<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for RwLock<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<'rwlock, T: ?Sized> ReadGuard<'rwlock, T> {
    #[inline]
    pub fn leak(this: Self) -> &'rwlock T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &*this.data }
    }
}

impl<'rwlock, T: ?Sized + fmt::Debug> fmt::Debug for ReadGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Display> fmt::Display for ReadGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Debug> fmt::Debug for WriteGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Display> fmt::Display for WriteGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized> Deref for ReadGuard<'rwlock, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'rwlock, T: ?Sized> Deref for WriteGuard<'rwlock, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'rwlock, T: ?Sized> DerefMut for WriteGuard<'rwlock, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'rwlock, T: ?Sized> Drop for ReadGuard<'rwlock, T> {
    fn drop(&mut self) {
        debug_assert!(self.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED) > 0);
        self.lock.fetch_sub(READER, Ordering::Release);
    }
}

impl<'rwlock, T: ?Sized> Drop for WriteGuard<'rwlock, T> {
    fn drop(&mut self) {
        debug_assert_eq!(self.inner.lock.load(Ordering::Relaxed) & WRITER, WRITER);

        // Writer is responsible for clearing both WRITER and UPGRADED bits.
        // The UPGRADED bit may be set if an upgradeable lock attempts an upgrade while this lock is held.
        self.inner
            .lock
            .fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }
}

