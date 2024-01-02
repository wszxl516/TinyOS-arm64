#[allow(unused_imports)]
pub use pl011::Pl011Uart;

mod pl011;

pub trait Read {
    fn read_char(&self) -> Option<u8>;
    fn read_bytes(&self, buffer: &mut [u8]) -> usize;
}