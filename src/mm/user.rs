use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::addr2slice;
use crate::mm::{USER_END, USER_START};

macro_rules! user_ptr_ok {
    ($addr: expr, $size: expr) => {
        assert!($addr != 0 && USER_START <= $addr && $addr + $size <= USER_END)
    };
}
pub trait UserBuffer<T>
where
    Self: Sized + Clone,
{
    fn copy_from_user(user_src: UserPtr<T>) -> Option<Self>;

    fn copy_to_user(&self, user_dst: &mut UserPtr<T>);
}

impl<T: Clone + From<u8>> UserBuffer<T> for Vec<T> {
    fn copy_from_user(user_src: UserPtr<T>) -> Option<Vec<T>> {
        let mut buffer = vec![T::from(0u8); user_src.len()];
        user_src.copy_to(buffer.as_mut_slice(), user_src.len());
        Some(buffer)
    }

    fn copy_to_user(&self, user_dst: &mut UserPtr<T>) {
        user_dst.copy_from(self.as_slice(), user_dst.len())
    }
}

impl UserBuffer<u8> for String {
    fn copy_from_user(user_src: UserPtr<u8>) -> Option<Self> {
        match Vec::<u8>::copy_from_user(user_src) {
            None => None,
            Some(buffer) => match String::from_utf8(buffer) {
                Ok(s) => Some(s),
                Err(_) => None,
            },
        }
    }

    fn copy_to_user(&self, user_dst: &mut UserPtr<u8>) {
        self.as_bytes().to_vec().copy_to_user(user_dst)
    }
}

#[repr(transparent)]
pub struct UserPtr<T: 'static> {
    ptr: &'static mut [T],
}

impl<T> UserPtr<T> {
    pub fn new(addr: usize, size: usize) -> Self {
        Self {
            ptr: addr2slice!(addr, size, T),
        }
    }
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_mut_ptr()
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.ptr.len()
    }

    pub fn copy_to(&self, dst: &mut [T], len: usize) {
        user_ptr_ok!(self.as_ptr().addr(), self.len());
        unsafe { self.as_ptr().copy_to_nonoverlapping(dst.as_mut_ptr(), len) }
    }
    pub fn copy_from(&mut self, buf: &[T], len: usize) {
        user_ptr_ok!(self.as_ptr().addr(), self.len());
        unsafe {
            self.as_mut_ptr()
                .copy_from_nonoverlapping(buf.as_ptr(), len)
        }
    }
}
