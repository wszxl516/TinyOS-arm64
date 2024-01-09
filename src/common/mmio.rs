#![allow(dead_code)]


#[derive(Debug)]
pub struct MMIO {
    base_addr: usize,
}

impl MMIO {
    pub const fn new(base: usize) -> MMIO {
        Self {
            base_addr: base,
        }
    }
    #[inline(always)]
    pub const fn offset(&self, offset: usize) -> MMIO {
        Self {
            base_addr: self.base_addr + offset,
        }
    }
    #[inline(always)]
    pub fn read<T: Sized>(&self) -> T {
        unsafe { (self.base_addr as *const T).read_volatile() }
    }
    #[inline(always)]
    pub fn write<T: Sized>(&self, value: T) {
        unsafe { (self.base_addr as *mut T).write_volatile(value) }
    }
}