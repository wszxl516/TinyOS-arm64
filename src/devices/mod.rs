mod console;
mod uart;
#[allow(unused_imports)]
pub use console::{gets, puts};

pub fn init(){
    console::setup_console();
}

