pub use crate::fdt_get;
pub use crate::pci_addr;
#[macro_export]
macro_rules! fdt_get {
    ($slice: ident,$value_type: ty) => {{
        let b: (&[u8; core::mem::size_of::<$value_type>()], &[u8]) =
            $slice.split_first_chunk().unwrap();
        #[allow(unused_assignments)]
        $slice = b.1;
        <$value_type>::from_be_bytes(*b.0)
    }};
}

#[macro_export]
macro_rules! pci_addr {
    ($base: expr, $bus: expr, $device: expr, $func:expr) => {{
        crate::mm::VirtAddr::from_phy(
            $base + (((($bus as  usize) << 20) | (($device as usize) << 15) | (($func as usize) << 12)) as usize),
        )
        .as_usize()
    }};
}
