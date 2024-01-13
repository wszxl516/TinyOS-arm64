use alloc::boxed::Box;

use fdt::Fdt;
use lazy_static::lazy_static;

#[allow(unused_imports)]
pub use console::{gets, puts};

use crate::arch::BOOT_ARGS;
use crate::common::sync::Mutex;
use crate::devices::{virtio::blk::VirtIOBlk, virtio::VirtioBlkTrans};
use crate::devices::pci::bus::PCIBus;
use crate::pr_notice;

mod console;
pub mod pci;
mod uart;
mod virtio;
#[macro_use]
mod macros;

lazy_static! {
    static ref DTB: Fdt<'static> = {
        let dtb =
            unsafe { Fdt::from_ptr(BOOT_ARGS[0].into_vaddr().as_usize() as *const u8) }.unwrap();
        if let Some(platform) = dtb.find_node("/platform-bus") {
            pr_notice!(
                "Model: {}, Platform: {}\n",
                dtb.root().model(),
                platform.compatible().unwrap().first()
            )
        }
        dtb
    };
}
lazy_static! {
    static ref PCI_BUS: Mutex<PCIBus> =
        Mutex::new(PCIBus::from_fdt(&DTB.find_node("/pcie").unwrap()));
}
lazy_static! {
    static ref VIRT_BLK: Mutex<&'static mut VirtIOBlk> = {
        let trans = Box::leak(Box::new(
            VirtioBlkTrans::from_pci(PCI_BUS.lock().get_mut())
                .unwrap_or_else(|| panic!("virt blk pci not found!")),
        ));
        let virt_blk = Box::leak(Box::new(
            VirtIOBlk::new(trans).unwrap_or_else(|e| panic!("virt blk init failed: {:?}!", e)),
        ));
        Mutex::new(virt_blk)
    };
}

fn blk_info() {
    match VIRT_BLK.lock() {
        mut blk => {
            pr_notice!(
                "Disk size: {}MB ",
                blk.capacity * blk.blk_size / 1024 / 1024
            );
            let mut buffer = [0u8; 512];
            blk.get_mut().read_block(0, &mut buffer).unwrap();
            if buffer.ends_with(&[0x55, 0xAA]) {
                pr_notice!("DOS/MBR boot sector");
            }
            pr_notice!("\n");
        }
    }
}

pub fn init() {
    console::setup_console();
    blk_info();
}
