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
        key[0] as char
    }};
}


#[derive(Eq, PartialEq, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Red = 91,
    Green = 92,
    Orange = 93,
    Blue = 94,
    Magenta = 95,
    Cyan = 96,
    White = 97,
}

impl Color {
    pub const fn value(self) -> u8 {
        match self {
            Color::Red => 91,
            Color::Green => 92,
            Color::Orange => 93,
            Color::Blue => 94,
            Color::Magenta => 95,
            Color::Cyan => 96,
            Color::White => 97,
        }
    }
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

#[macro_export]
macro_rules! pr_color {
    ($fmt: literal ,$color: expr $(, $($arg: tt)+)?) =>{
        $crate::stdio::print(
            format_args!(concat!("\x1b[{}m", $fmt, "\x1b[0m"),
            $color
            $(, $($arg)+)?)
        )
    };
}

#[macro_export]
macro_rules! pr_info {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Green.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_notice {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Blue.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_warn {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Orange.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_err {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
       $crate::pr_color!($fmt,
            $crate::stdio::Color::Red.value()
            $(, $($arg)+)?
            )
    };
}


