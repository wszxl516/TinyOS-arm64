//example
//asm!("mrs {reg:x}, CurrentEL", reg = out(reg) current_el)};
#[macro_export]
macro_rules! read_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:ident, $asm_width:tt) => {
        // Reads the raw bits of the CPU register.
        match (){

        () => {
                let mut reg: $width;
                unsafe {
                    core::arch::asm!(concat!($asm_instr, " {reg:", $asm_width, "}, ", stringify!($asm_reg_name)),
                    reg = out(reg) reg, options(nomem, nostack));
                }
                reg
        }
        }
    }
}

//example
//asm!("msr SPSEL, {reg:x}", reg = in(reg) 1, options(readonly))
#[macro_export]
macro_rules! write_raw {
    ($width:ty, $asm_instr:tt, $asm_reg_name:ident, $asm_width:tt, $asm_value:expr) => {
        // Writes raw bits to the CPU register.
        unsafe {
            core::arch::asm!(concat!($asm_instr, " ", stringify!($asm_reg_name), ", {reg:", $asm_width, "}"),
            reg = in(reg) $asm_value, options(nomem, nostack))
        }

    }
}

//system coprocessor register
#[macro_export]
macro_rules! reg_write_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::write_raw!(usize, "msr", $asm_reg_name, "x", $asm_value)
    };
}

//system coprocessor register
#[macro_export]
macro_rules! reg_read_p {
    ($asm_reg_name:tt) => {
        $crate::read_raw!(usize, "mrs", $asm_reg_name, "x")
    };
}

//system coprocessor register
#[macro_export]
macro_rules! reg_update_p {
    ($asm_reg_name:tt, $asm_value:expr) => {
        let last_value = $crate::reg_read_p!($asm_reg_name);
        $crate::reg_write_p!($asm_reg_name, $asm_value | last_value)
    };
}

#[macro_export]
macro_rules! reg_write_g {
    ($asm_reg_name:tt, $asm_value:expr) => {
        $crate::write_raw!(usize, "mov", $asm_reg_name, "x", $asm_value)
    };
}

#[macro_export]
macro_rules! reg_read_g {
    ($asm_reg_name:tt) => {
        $crate::read_raw!(usize, "mov", $asm_reg_name, "x")
    };
}

#[macro_export]
macro_rules! reg_update_g {
    ($asm_reg_name:tt, $asm_value:expr) => {
        let last_value = $crate::read64($asm_reg_name);
        $crate::write_raw!(usize, "mov", $asm_reg_name, "x", $asm_value | last_value)
    };
}
//memory-mapped register read
#[macro_export]
macro_rules! reg_read_a {
    ($reg_addr:expr, $value_type:ty) => {
        unsafe { core::ptr::read_volatile(($reg_addr) as *const $value_type) }
    };
}

//memory-mapped register write
#[macro_export]
macro_rules! reg_write_a {
    ($reg_addr:expr, $value: expr, $value_type:ty) => {
        unsafe { core::ptr::write_volatile(($reg_addr) as *mut $value_type, $value) }
    };
}

#[macro_export]
macro_rules! get_bits {
    ($value: expr, $start:expr, $num_bits:expr) => {
        ($value >> $start) & ((1 << ($num_bits)) - 1)
    };
}

#[macro_export]
macro_rules! get_bit {
    ($value: expr, $num_bits:expr) => {
        ($value  & (1 << $num_bits)) >> $num_bits
    };
}
