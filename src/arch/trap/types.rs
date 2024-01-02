use core::fmt::{Display, Formatter};

use crate::{get_bits, reg_read_p};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u16)]
#[allow(dead_code)]
pub enum SyncExceptionType {
    UnknownReason = 0b000000,
    TrappedWFIorWFE = 0b000001,
    TrappedSimdOrFloatingPoint = 0b000111,
    IllegalExecutionState = 0b001110,
    SVCAArch64 = 0b010101,
    TrappedMSROrMRSAArch64 = 0b011000,
    ExceptionPointerAuthentication = 0b011100,
    InstructionAbortLowLevel = 0b100000,
    InstructionAbortCurrentLevel = 0b100001,
    PCAlignmentFault = 0b100010,
    DataAbortLowLevel = 0b100100,
    DataAbortCurrentLevel = 0b100101,
    SPAlignmentFault = 0b100110,
    SErrorInterrupt = 0b101111,
    BreakpointLowerException = 0b110000,
    BreakpointCurrentLevel = 0b110001,
    SoftwareStepLowLevel = 0b110010,
    SoftwareStepCurrentLevel = 0b110011,
    WatchpointLowLevel = 0b110100,
    WatchpointCurrentLevel = 0b110101,
    BRKInstructionAArch64 = 0b111100,
}

impl From<u16> for SyncExceptionType {
    fn from(value: u16) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Display for SyncExceptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl SyncExceptionType {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            SyncExceptionType::UnknownReason => "Unknown reason.",
            SyncExceptionType::TrappedWFIorWFE => "Trapped WFI or WFE instruction execution.",
            SyncExceptionType::TrappedSimdOrFloatingPoint => "SIMD, or floating-point functionality trapped by AArch64.",
            SyncExceptionType::IllegalExecutionState => "Illegal Execution state.",
            SyncExceptionType::SVCAArch64 => "SVC instruction execution in AArch64 state.",
            SyncExceptionType::TrappedMSROrMRSAArch64 => "Trapped MSR, MRS or System instruction execution in AArch64 state.",
            SyncExceptionType::ExceptionPointerAuthentication => "Exception from a Pointer Authentication instruction authentication failure.",
            SyncExceptionType::InstructionAbortLowLevel => "Instruction Abort from a lower Exception level.",
            SyncExceptionType::InstructionAbortCurrentLevel => "Instruction Abort taken without a change in Exception level.",
            SyncExceptionType::PCAlignmentFault => "PC alignment fault exception.",
            SyncExceptionType::DataAbortLowLevel => "Data Abort from a lower Exception level.",
            SyncExceptionType::DataAbortCurrentLevel => "Data Abort taken without a change in Exception level.",
            SyncExceptionType::SPAlignmentFault => "SP alignment fault exception.",
            SyncExceptionType::SErrorInterrupt => "SError interrupt.",
            SyncExceptionType::BreakpointLowerException => "Breakpoint exception from a lower Exception level.",
            SyncExceptionType::BreakpointCurrentLevel => "Breakpoint exception taken without a change in Exception level.",
            SyncExceptionType::SoftwareStepLowLevel => "Software Step exception from a lower Exception level.",
            SyncExceptionType::SoftwareStepCurrentLevel => "Software Step exception taken without a change in Exception level.",
            SyncExceptionType::WatchpointLowLevel => "Watchpoint exception from a lower Exception level.",
            SyncExceptionType::WatchpointCurrentLevel => "Watchpoint exception taken without a change in Exception level.",
            SyncExceptionType::BRKInstructionAArch64 => "BRK instruction execution in AArch64 state."
        }
    }
}


pub struct SyncException {
    pub iss: u32,
    pub il: u8,
    pub ec: SyncExceptionType,
}

impl SyncException {
    pub fn new() -> Self {
        let value = reg_read_p!(ESR_EL1);
        Self {
            iss: get_bits!(value, 0, 25) as u32,
            il: get_bits!(value, 25, 1) as u8,
            ec: SyncExceptionType::from(get_bits!(value, 26, 6) as u16),
        }
    }
}

impl Display for SyncException {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.ec.as_str()
        )
    }
}