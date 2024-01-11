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
            SyncExceptionType::TrappedSimdOrFloatingPoint => {
                "SIMD, or floating-point functionality trapped by AArch64."
            }
            SyncExceptionType::IllegalExecutionState => "Illegal Execution state.",
            SyncExceptionType::SVCAArch64 => "SVC instruction execution in AArch64 state.",
            SyncExceptionType::TrappedMSROrMRSAArch64 => {
                "Trapped MSR, MRS or System instruction execution in AArch64 state."
            }
            SyncExceptionType::ExceptionPointerAuthentication => {
                "Exception from a Pointer Authentication instruction authentication failure."
            }
            SyncExceptionType::InstructionAbortLowLevel => {
                "Instruction Abort from a lower Exception level."
            }
            SyncExceptionType::InstructionAbortCurrentLevel => {
                "Instruction Abort taken without a change in Exception level."
            }
            SyncExceptionType::PCAlignmentFault => "PC alignment fault exception.",
            SyncExceptionType::DataAbortLowLevel => "Data Abort from a lower Exception level.",
            SyncExceptionType::DataAbortCurrentLevel => {
                "Data Abort taken without a change in Exception level."
            }
            SyncExceptionType::SPAlignmentFault => "SP alignment fault exception.",
            SyncExceptionType::SErrorInterrupt => "SError interrupt.",
            SyncExceptionType::BreakpointLowerException => {
                "Breakpoint exception from a lower Exception level."
            }
            SyncExceptionType::BreakpointCurrentLevel => {
                "Breakpoint exception taken without a change in Exception level."
            }
            SyncExceptionType::SoftwareStepLowLevel => {
                "Software Step exception from a lower Exception level."
            }
            SyncExceptionType::SoftwareStepCurrentLevel => {
                "Software Step exception taken without a change in Exception level."
            }
            SyncExceptionType::WatchpointLowLevel => {
                "Watchpoint exception from a lower Exception level."
            }
            SyncExceptionType::WatchpointCurrentLevel => {
                "Watchpoint exception taken without a change in Exception level."
            }
            SyncExceptionType::BRKInstructionAArch64 => {
                "BRK instruction execution in AArch64 state."
            }
        }
    }
}

#[derive(Debug)]
pub struct SyncException {
    pub iss: u32,
    pub il: u8,
    pub ec: SyncExceptionType,
}
pub const FAULT_STATUS_MAP: [(u32, &str); 24] = [
    (
        0b000000,
        "Address size fault, level 0 of translation or translation table base register",
    ),
    (0b000001, "Address size fault, level 1"),
    (0b000010, "Address size fault, level 2"),
    (0b000011, "Address size fault, level 3"),
    (0b000100, "Translation fault, level 0"),
    (0b000101, "Translation fault, level 1"),
    (0b000110, "Translation fault, level 2"),
    (0b000111, "Translation fault, level 3"),
    (0b001001, "Access flag fault, level 1"),
    (0b001010, "Access flag fault, level 2"),
    (0b001011, "Access flag fault, level 3"),
    (0b001101, "Permission fault, level 1"),
    (0b001110, "Permission fault, level 2"),
    (0b001111, "Permission fault, level 3"),
    (
        0b010000,
        "Synchronous External abort, not on translation table walk",
    ),
    (0b010001, "Synchronous Tag Check fail"),
    (
        0b010100,
        "Synchronous External abort, on translation table walk, level 0",
    ),
    (
        0b010101,
        "Synchronous External abort, on translation table walk, level 1",
    ),
    (
        0b010110,
        "Synchronous External abort, on translation table walk, level 2",
    ),
    (
        0b010111,
        "Synchronous External abort, on translation table walk, level 3",
    ),
    (0b100001, "Alignment fault"),
    (0b110000, "TLB conflict abort"),
    (
        0b111101,
        "Section Domain Fault, used only for faults reported in the PAR_EL1",
    ),
    (
        0b111110,
        "Page Domain Fault, used only for faults reported in the PAR_EL1",
    ),
];
impl SyncException {
    pub fn new() -> Self {
        let value = reg_read_p!(ESR_EL1);
        Self {
            iss: get_bits!(value, 0, 26) as u32,
            il: get_bits!(value, 25, 1) as u8,
            ec: SyncExceptionType::from(get_bits!(value, 26, 7) as u16),
        }
    }
    pub fn fault_msg(&self) -> &str
    {
        for (id, msg) in FAULT_STATUS_MAP {
            if get_bits!(self.iss, 0, 6) == id
            {
                return msg
            }
        }
        ""
    }
}

impl Display for SyncException {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.ec.as_str())
    }
}
