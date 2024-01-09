use core::cell::RefCell;
use core::hint::spin_loop;
use core::mem::size_of;

use bitflags::bitflags;

use super::{Error, Result};
use super::queue::VirtQueue;

pub struct VirtIOBlk {
    pub transport: &'static mut dyn Transport,
    pub queue: RefCell<VirtQueue>,
    pub capacity: u64,
    pub blk_size: u64,
}

///# Safety
unsafe trait AsBuf: Sized {
    fn as_buf(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, size_of::<Self>()) }
    }
    fn as_buf_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, size_of::<Self>()) }
    }
}
bitflags! {
    #[derive(Default, Debug)]
    pub struct DeviceStatus: u8 {
        const ACKNOWLEDGE = 1;
        const DRIVER = 2;
        const FAILED = 128;
        const FEATURES_OK = 8;
        const DRIVER_OK = 4;
        const DEVICE_NEEDS_RESET = 64;
    }
}
pub trait Transport {
    fn capacity(&self) -> u64;
    fn blk_size(&self) -> u64;
    fn status(&self) -> DeviceStatus;
    fn reset(&mut self);
    fn read_device_features(&mut self) -> u64;

    fn write_driver_features(&mut self, driver_features: u64);

    fn max_queue_size(&self) -> u32;

    fn notify(&mut self, queue: u32);

    fn set_status(&mut self, status: DeviceStatus);

    fn queue_set(
        &mut self,
        queue: u32,
        size: u32,
        desc: usize,
        driver: usize,
        device: usize,
    );

    fn queue_used(&mut self, queue: u32) -> bool;


    fn init(&mut self);
    fn finish_init(&mut self);
}


unsafe impl Sync for VirtIOBlk {}

unsafe impl Send for VirtIOBlk {}

impl VirtIOBlk {
    pub fn new<'a>(transport: &'static mut dyn Transport) -> Result<VirtIOBlk> {
        transport.init();
        let queue = RefCell::new(VirtQueue::new(transport, 0, 4)?);
        transport.finish_init();
        let capacity = transport.capacity();
        let blk_size = transport.blk_size();
        Ok(VirtIOBlk {
            transport,
            queue,
            capacity,
            blk_size,
        })
    }


    pub fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> Result {
        assert_eq!(buf.len(), self.blk_size as usize);
        let req = BlkReq {
            type_: ReqType::In,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut resp = BlkResp::default();
        self.queue.borrow_mut()
            .add(&[req.as_buf()], &[buf, resp.as_buf_mut()])?;
        self.transport.notify(0);
        while !self.queue.borrow_mut().can_pop() {
            spin_loop();
        }
        self.queue.borrow_mut().pop_used()?;
        match resp.status {
            RespStatus::Ok => Ok(()),
            RespStatus::IoErr => Err(Error::IoError),
            _ => Err(Error::InvalidParam),
        }
    }
    pub fn write_block(&mut self, block_id: usize, buf: &[u8]) -> Result {
        assert_eq!(buf.len(), self.blk_size as usize);
        let req = BlkReq {
            type_: ReqType::Out,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut resp = BlkResp::default();
        self.queue
            .borrow_mut()
            .add(&[req.as_buf(), buf], &[resp.as_buf_mut()])?;
        self.transport.notify(0);
        while !self.queue.borrow_mut().can_pop() {
            spin_loop();
        }
        self.queue.borrow_mut().pop_used()?;
        match resp.status {
            RespStatus::Ok => Ok(()),
            RespStatus::IoErr => Err(Error::IoError),
            _ => Err(Error::InvalidParam),
        }
    }
    pub fn clear_block(&mut self, block_id: usize, num_sectors: u32) -> Result {
        let req = BlkReq {
            type_: ReqType::WriteZeroes,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut resp = BlkResp::default();
        self.queue.borrow_mut().add(
            &[
                req.as_buf(),
                DiscardWriteZeroes {
                    sector: block_id,
                    num_sectors,
                    unmap: 0,
                }
                    .as_buf(),
            ],
            &[resp.as_buf_mut()],
        )?;
        self.transport.notify(0);
        while !self.queue.borrow_mut().can_pop() {
            spin_loop();
        }
        self.queue.borrow_mut().pop_used()?;
        match resp.status {
            RespStatus::Ok => Ok(()),
            RespStatus::IoErr => Err(Error::IoError),
            _ => Err(Error::InvalidParam),
        }
    }

    pub fn virt_queue_size(&self) -> u16 {
        self.queue.borrow().size()
    }

    pub fn status(&mut self) -> DeviceStatus {
        self.transport.status()
    }

    pub fn flush(&mut self, block_id: u64) -> Result {
        let req = BlkReq {
            type_: ReqType::Flush,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut resp = BlkResp::default();
        self.queue.borrow_mut().add(
            &[
                req.as_buf(),
                DiscardWriteZeroes {
                    sector: block_id as usize,
                    num_sectors: 1,
                    unmap: 0,
                }
                    .as_buf(),
            ],
            &[resp.as_buf_mut()],
        )?;
        self.transport.notify(0);
        while !self.queue.borrow_mut().can_pop() {
            spin_loop();
        }
        self.queue.borrow_mut().pop_used()?;
        match resp.status {
            RespStatus::Ok => Ok(()),
            RespStatus::IoErr => Err(Error::IoError),
            _ => Err(Error::InvalidParam),
        }
    }
    pub fn flush_all(&mut self) -> Result {
        for i in 0..self.capacity / self.blk_size {
            match self.flush(i) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DiscardWriteZeroes {
    sector: usize,
    num_sectors: u32,
    unmap: u32,
}

#[repr(C)]
#[derive(Debug)]
struct BlkReq {
    type_: ReqType,
    reserved: u32,
    sector: u64,
}

/// Response of a VirtIOBlk request.
unsafe impl AsBuf for BlkReq {}

unsafe impl AsBuf for BlkResp {}

unsafe impl AsBuf for DiscardWriteZeroes {}

#[repr(C)]
#[derive(Debug)]
pub struct BlkResp {
    status: RespStatus,
}

impl BlkResp {
    /// Return the status of a VirtIOBlk request.
    pub fn status(&self) -> RespStatus {
        self.status
    }
}

#[repr(u32)]
#[derive(Debug)]
enum ReqType {
    In = 0,
    Out = 1,
    Flush = 4,
    Discard = 11,
    WriteZeroes = 13,
}

/// Status of a VirtIOBlk request.
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RespStatus {
    /// Ok.
    Ok = 0,
    /// IoErr.
    IoErr = 1,
    /// Unsupported yet.
    Unsupported = 2,
    /// Not ready.
    _NotReady = 3,
}

impl Default for BlkResp {
    fn default() -> Self {
        BlkResp {
            status: RespStatus::_NotReady,
        }
    }
}
