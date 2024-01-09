use core::mem::size_of;
use core::slice;
use core::sync::atomic::{fence, Ordering};

use bitflags::*;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::ReadWrite;

use crate::align;
use crate::devices::virtio::blk::Transport;
use crate::mm::{PAGE_SIZE, VirtAddr};
use crate::mm::heap::page_alloc;

use super::{Error, Result};

#[derive(Debug, Clone)]
pub struct VirtQueue {
    /// Descriptor table
    desc: usize,
    /// Available ring
    avail: usize,
    /// Used ring
    used: usize,

    /// The index of queue
    queue_idx: u32,
    /// The size of the queue.
    ///
    /// This is both the number of descriptors, and the number of slots in the available and used
    /// rings.
    queue_size: u16,
    /// The number of used queues.
    num_used: u16,
    /// The head desc index of the free list.
    free_head: u16,
    avail_idx: u16,
    last_used_idx: u16,
}

impl VirtQueue {
    pub fn new(transport: &mut dyn Transport, idx: usize, size: u16) -> Result<Self> {
        if transport.queue_used(idx as u32) {
            return Err(Error::AlreadyUsed);
        }
        if !size.is_power_of_two() || transport.max_queue_size() < size as u32 {
            return Err(Error::InvalidParam);
        }
        let layout = VirtQueueLayout::new(size);
        // Allocate contiguous pages.
        let dma = page_alloc(layout.size / PAGE_SIZE).as_usize();
        transport.queue_set(
            idx as u32,
            size as u32,
            VirtAddr::new(dma).as_phy().as_usize(),
            VirtAddr::new(dma + layout.avail_offset).as_phy().as_usize(),
            VirtAddr::new(dma + layout.used_offset).as_phy().as_usize(),
        );

        let desc =
            unsafe { slice::from_raw_parts_mut(dma as *mut Descriptor, size as usize) };

        // Link descriptors together.
        for i in 0..(size - 1) {
            desc[i as usize].next.set(i + 1);
        }

        Ok(VirtQueue {
            desc: dma,
            avail: dma + layout.avail_offset,
            used: dma + layout.used_offset,
            queue_size: size,
            queue_idx: idx as u32,
            num_used: 0,
            free_head: 0,
            avail_idx: 0,
            last_used_idx: 0,
        })
    }
    #[inline]
    fn get_desc(&self) -> &mut [Descriptor] {
        unsafe {
            slice::from_raw_parts_mut(
                self.desc as *mut Descriptor,
                self.queue_size as usize,
            )
        }
    }
    #[inline]
    fn get_avail(&self) -> &mut AvailRing {
        unsafe { &mut *(self.avail as *mut AvailRing) }
    }
    #[inline]
    fn get_used(&self) -> &mut UsedRing {
        unsafe { &mut *(self.used as *mut UsedRing) }
    }

    pub fn add(&mut self, inputs: &[&[u8]], outputs: &[&mut [u8]]) -> Result<u16> {
        if inputs.is_empty() && outputs.is_empty() {
            return Err(Error::InvalidParam);
        }
        if inputs.len() + outputs.len() + self.num_used as usize > self.queue_size as usize {
            return Err(Error::BufferTooSmall);
        }

        // allocate descriptors from free list
        let head = self.free_head;
        let mut last = self.free_head;
        for input in inputs.iter() {
            let desc = &mut self.get_desc()[self.free_head as usize];
            desc.set_buf(input);
            desc.flags.set(DescFlags::NEXT.bits());
            last = self.free_head;
            self.free_head = desc.next.get();
        }
        for output in outputs.iter() {
            let desc = &mut self.get_desc()[self.free_head as usize];
            desc.set_buf(output);
            desc.flags.set((DescFlags::NEXT | DescFlags::WRITE).bits());
            last = self.free_head;
            self.free_head = desc.next.get();
        }
        // set last_elem.next = NULL
        {
            let desc = &mut self.get_desc()[last as usize];
            let mut flags = DescFlags::from_bits_truncate(desc.flags.get());
            flags.remove(DescFlags::NEXT);
            desc.flags.set(flags.bits());
        }
        self.num_used += (inputs.len() + outputs.len()) as u16;

        let avail_slot = self.avail_idx & (self.queue_size - 1);
        self.get_avail().ring[avail_slot as usize].set(head);

        // write barrier
        fence(Ordering::SeqCst);

        // increase head of avail ring
        self.avail_idx = self.avail_idx.wrapping_add(1);
        self.get_avail().idx.set(self.avail_idx);
        Ok(head)
    }

    /// Whether there is a used element that can pop.
    pub fn can_pop(&self) -> bool {
        self.last_used_idx != self.get_used().idx.get()
    }

    /// The number of free descriptors.
    pub fn available_desc(&self) -> usize {
        (self.queue_size - self.num_used) as usize
    }

    /// Recycle descriptors in the list specified by head.
    ///
    /// This will push all linked descriptors at the front of the free list.
    fn recycle_descriptors(&mut self, mut head: u16) {
        let origin_free_head = self.free_head;
        let mut tmp = 0u16;
        self.free_head = head;
        loop {
            let desc = &mut self.get_desc()[head as usize];
            let flags = desc.flags.get();
            tmp += 1;
            if DescFlags::from_bits_truncate(flags).contains(DescFlags::NEXT) {
                head = desc.next.get();
            } else {
                desc.next.set(origin_free_head);
                break;
            }
        }
        self.num_used -= tmp
    }

    /// Get a token from device used buffers, return (token, len).
    ///
    /// Ref: linux virtio_ring.c virtqueue_get_buf_ctx
    pub fn pop_used(&mut self) -> Result<(u16, u32)> {
        if !self.can_pop() {
            return Err(Error::NotReady);
        }
        // read barrier
        fence(Ordering::SeqCst);

        let last_used_slot = self.last_used_idx & (self.queue_size - 1);
        let index = self.get_used().ring[last_used_slot as usize].id.get() as u16;
        let len = self.get_used().ring[last_used_slot as usize].len.get();

        self.recycle_descriptors(index);
        self.last_used_idx = self.last_used_idx.wrapping_add(1);

        Ok((index, len))
    }

    /// Return size of the queue.
    pub fn size(&self) -> u16 {
        self.queue_size
    }
}

/// The inner layout of a VirtQueue.
///
/// Ref: 2.6.2 Legacy Interfaces: A Note on Virtqueue Layout
struct VirtQueueLayout {
    avail_offset: usize,
    used_offset: usize,
    size: usize,
}

impl VirtQueueLayout {
    fn new(queue_size: u16) -> Self {
        assert!(
            queue_size.is_power_of_two(),
            "queue size should be a power of 2"
        );
        let queue_size = queue_size as usize;
        let desc = size_of::<Descriptor>() * queue_size;
        let avail = size_of::<u16>() * (3 + queue_size);
        let used = size_of::<u16>() * 3 + size_of::<UsedElem>() * queue_size;
        VirtQueueLayout {
            avail_offset: desc,
            used_offset: align!(desc + avail, PAGE_SIZE),
            size: align!(desc + avail, PAGE_SIZE) + align!(used, PAGE_SIZE),
        }
    }
}

#[repr(C, align(16))]
pub(crate) struct Descriptor {
    addr: ReadWrite<u64>,
    len: ReadWrite<u32>,
    flags: ReadWrite<u16>,
    next: ReadWrite<u16>,
}

impl Descriptor {
    fn set_buf(&mut self, buf: &[u8]) {
        self.addr
            .set(VirtAddr::new(buf.as_ptr() as usize).as_phy().as_usize() as u64);
        self.len.set(buf.len() as u32);
    }
}

bitflags! {
    /// Descriptor flags
    struct DescFlags: u16 {
        const NEXT = 1;
        const WRITE = 2;
        const INDIRECT = 4;
    }
}

/// The driver uses the available ring to offer buffers to the device:
/// each ring entry refers to the head of a descriptor chain.
/// It is only written by the driver and read by the device.
#[repr(C)]
struct AvailRing {
    flags: ReadWrite<u16>,
    /// A driver MUST NOT decrement the idx.
    idx: ReadWrite<u16>,
    ring: [ReadWrite<u16>; 32],
    // actual size: queue_size
    used_event: ReadWrite<u16>, // unused
}

/// The used ring is where the device returns buffers once it is done with them:
/// it is only written to by the device, and read by the driver.
#[repr(C)]
struct UsedRing {
    flags: ReadWrite<u16>,
    idx: ReadWrite<u16>,
    ring: [UsedElem; 32],
    // actual size: queue_size
    avail_event: ReadWrite<u16>, // unused
}

#[repr(C)]
struct UsedElem {
    id: ReadWrite<u32>,
    len: ReadWrite<u32>,
}
