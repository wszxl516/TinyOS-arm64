use core::fmt::{self, Write};

use super::write;

const STDOUT: usize = 1;

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
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    () => {$crate::stdio::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::stdio::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! pr_color {
    () => {$crate::stdio::print(format_args!("\n"))};
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
    () => {$crate::stdio::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Green.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_notice {
    () => {$crate::stdio::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Blue.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_warn {
    () => {$crate::stdio::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::stdio::Color::Orange.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_err {
    () => {$crate::stdio::print(format_args!("\n"))};
    ($fmt: literal $(, $($arg: tt)+)?) =>{
       $crate::pr_color!($fmt,
            $crate::stdio::Color::Red.value()
            $(, $($arg)+)?
            )
    };
}


