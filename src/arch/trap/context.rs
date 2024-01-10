use core::arch::asm;
use core::fmt::{Display, Formatter};

use crate::{pr_err, reg_read_a};
use crate::common::symbol::find_symbol;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub reg: [usize; 29],
    pub fp: usize,
    pub lr: usize,
    pub usp: usize,
    pub elr: usize,
    pub spsr: usize,
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for i in (0..28).step_by(4) {
            write!(
                f,
                "x{:02}  {:#018x} x{:02}  {:#018x} x{:02}  {:#018x} x{:02}  {:#018x}\n",
                i,
                self.reg[i + 0],
                i + 1,
                self.reg[i + 1],
                i + 2,
                self.reg[i + 2],
                i + 3,
                self.reg[i + 3]
            )
                .unwrap();
        }
        write!(f, "x28  {:#018x} fp   {:#018x} lr   {:#018x} usp  {:#018x}\n", self.reg[28], self.fp, self.lr, self.usp).unwrap();
        write!(f, "elr  {:#018x} spsr {:#018x}\n", self.elr, self.spsr).unwrap();
        Ok(())
    }
}

impl Context {
    pub fn new_user(entry: usize, stack_top: usize) -> Self {
        Self {
            reg: [0;29],
            fp: 0,
            lr: 0,
            usp: stack_top,
            elr: entry,
            spsr: (1 << 9) | (1 << 8) | (0 << 7) | (1 << 6) | 0b0000,
        }
    }
    pub unsafe fn exec(&self, stack_top: usize) -> ! {
        asm!("
            mov     sp, x1
            ldp     x30, x9, [x0, 30 * 8]
            ldp     x10, x11, [x0, 32 * 8]
            msr     sp_el0, x9
            msr     elr_el1, x10
            msr     spsr_el1, x11

            ldp     x28, x29, [x0, 28 * 8]
            ldp     x26, x27, [x0, 26 * 8]
            ldp     x24, x25, [x0, 24 * 8]
            ldp     x22, x23, [x0, 22 * 8]
            ldp     x20, x21, [x0, 20 * 8]
            ldp     x18, x19, [x0, 18 * 8]
            ldp     x16, x17, [x0, 16 * 8]
            ldp     x14, x15, [x0, 14 * 8]
            ldp     x12, x13, [x0, 12 * 8]
            ldp     x10, x11, [x0, 10 * 8]
            ldp     x8, x9, [x0, 8 * 8]
            ldp     x6, x7, [x0, 6 * 8]
            ldp     x4, x5, [x0, 4 * 8]
            ldp     x2, x3, [x0, 2 * 8]
            ldp     x0, x1, [x0]

            eret",
        in("x0") self,
        in("x1") stack_top,
        options(noreturn),
        )
    }
    pub fn stacktrace(&self) {
        let (mut frame_pc, mut frame_fp);
        if self.fp == 0 {
            return;
        } else {
            frame_fp = self.fp;
            frame_pc = self.elr;
        }
        pr_err!("stack trace:\n");
        while frame_fp != 0 {
            pr_err!("\t#{:#018x}", frame_pc);
            match find_symbol(frame_pc) {
                None => {}
                Some(sym) => {
                    pr_err!(" {}+{:#x}\n", sym.name, frame_pc - sym.addr);
                }
            }
            /* Stack frame pointer should be 16 bytes aligned */
            if frame_fp & 0xF != 0 {
                break;
            }
            frame_pc = reg_read_a!(frame_fp + 8, usize);
            frame_fp = reg_read_a!(frame_fp, usize);
        }
    }
}
