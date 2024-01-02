use core::ffi::CStr;

use lazy_static::lazy_static;

use crate::lds_address;

const MAGIC: [u8; 8] = [b's', b'y', b'm', b'b', b'o', b'l', b's', b'\0'];

#[repr(C)]
#[derive(Debug)]
struct Header {
    magic: [u8; 8],
    version: u32,
    entry_num: u32,
    size: usize,
}

impl Header {
    const SIZE: usize = core::mem::size_of::<Self>();
    pub fn new() -> &'static Self {
        let addr = lds_address!(symbols_start);
        unsafe { &*(addr as *const Header) }
    }
    pub fn symbol_start(&self) -> *const u8 {
        (lds_address!(symbols_start) + Header::SIZE) as *const u8
    }
    pub const fn symbol_size(&self) -> usize {
        self.size - Header::SIZE
    }
    pub const fn symbol_num(&self) -> u32 {
        self.entry_num
    }
}
macro_rules! get_int {
    ($bytes_addr:expr, $int_type: ty) => {{
        let (num, other) = $bytes_addr
            .split_first_chunk::<{ core::mem::size_of::<$int_type>() }>()
            .unwrap();
        (<$int_type>::from_le_bytes(*num), other)
    }};
}
macro_rules! get_str {
    ($bytes_addr:expr) => {{
        let name = CStr::from_bytes_until_nul($bytes_addr)
            .unwrap()
            .to_str()
            .unwrap();
        let other = unsafe {
            core::slice::from_raw_parts(
                $bytes_addr.as_ptr().add(name.len() + 1),
                $bytes_addr.len() - name.len() - 1,
            )
        };
        (name, other)
    }};
}
#[derive(Debug)]
pub struct Symbol {
    pub name: &'static str,
    pub addr: usize,
    pub size: usize,
}
lazy_static! {
    static ref SYMBOL_HEADER: &'static Header = Header::new();
}
pub fn find_symbol(addr: usize) -> Option<Symbol> {
    let symbol_start = SYMBOL_HEADER.symbol_start();
    let symbol_size = SYMBOL_HEADER.symbol_size();
    let entry_num = SYMBOL_HEADER.symbol_num();
    let mut symbols = unsafe { core::slice::from_raw_parts(symbol_start, symbol_size) };
    let mut used_len = 0;
    let mut symbol = Symbol {
        name: "",
        addr: 0,
        size: 0,
    };
    if SYMBOL_HEADER.magic != MAGIC {
        panic!("invalid Symbol header.");
    }
    for _n in 0..entry_num {
        (symbol.addr, symbols) = get_int!(symbols, usize);
        (symbol.size, symbols) = get_int!(symbols, usize);
        (symbol.name, symbols) = get_str!(symbols);
        used_len += 8 + 8 + symbol.name.len() + 1;
        if addr >= symbol.addr && addr <= (symbol.addr + symbol.size) {
            return Some(symbol);
        }
        if used_len >= symbol_size || symbol.addr == 0 {
            break;
        }
    }
    return Some(symbol);
}
