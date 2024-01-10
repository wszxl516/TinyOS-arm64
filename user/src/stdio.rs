use core::fmt::{self, Write};
const STDOUT: usize = 1;

use super::{write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! read_key {
    () => {{
        let mut key: [u8; 1] = [0u8; 1];
        crate::std::read(0, &mut key);
        key[0]
    }};
}

#[macro_export]
macro_rules! print {
    () => {$crate::console::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    () => {$crate::console::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

