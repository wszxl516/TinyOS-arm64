#![allow(dead_code)]

pub use backend::VirtioBlkTrans;

pub const DISK_BLK_SIZE: usize = 512;

// pub mod blk;
pub mod backend;
pub mod blk;
pub mod queue;
#[macro_export]
macro_rules! align {
    ($addr:expr, $page_size:ident) => {
        ($addr + $page_size - 1) & !($page_size - 1)
    };
}
pub const PAGE_SIZE: usize = 0x1000;

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}