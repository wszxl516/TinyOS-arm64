use core::arch::asm;

//https://armv8-doc.readthedocs.io/en/latest/12.html#
// Instruction Synchronization Barrier
#[allow(dead_code)]
#[inline(always)]
pub fn isb_all() {
    unsafe { asm!("ISB SY", options(nomem, nostack)) }
}

// The Translation Lookaside Buffer
#[allow(dead_code)]
#[inline(always)]
pub fn tlb_all() {
    unsafe { asm!("tlbi vmalle1is", options(nomem, nostack)) }
}

#[allow(dead_code)]
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb", options(nomem, nostack)) }
}

#[allow(dead_code)]
#[inline(always)]
pub fn tlb_one(entry: usize) {
    unsafe { asm!("tlbi VAE1, {}", in(reg) entry, options(nomem, nostack)) }
}

// Data Memory Barrier
#[allow(dead_code)]
#[inline(always)]
pub fn dsb_all() {
    unsafe { asm!("DSB SY", options(nomem, nostack)) }
}


// ensure write has completed
#[allow(dead_code)]
#[inline(always)]
pub fn dsb_write() {
    unsafe { asm!("DSB ISHST", options(nomem, nostack)) }
}


// ensure completion of TLB invalidation
#[allow(dead_code)]
#[inline(always)]
pub fn tlb_invalid() {
    unsafe { asm!("DSB ISH", options(nomem, nostack)) }
}