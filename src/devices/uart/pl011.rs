#![allow(dead_code)]

use core::fmt::{self, Write};

use tock_registers::{register_bitfields, register_structs};
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

use crate::devices::uart::Read;

register_structs! {
    #[repr(C)]
    PL011Regs {
        //data Register.
        (0x000 => pub data: ReadWrite<u8>),
        (0x001 =>  __reserved_0: [u8; 3]),
        //Receive Status Register / Error Clear Register
        (0x004 => pub receive_status: ReadWrite<u32, RECV_STATUS::Register>),
        (0x008 => __reserved_1: [u32; 4]),
        //Flag Register
        (0x018 => pub flags: ReadOnly<u32,Flags::Register>),
        (0x01c => __reserved_2: u32),
        //IrDA Low-Power Counter Register
        (0x020 => pub lp_counter: ReadWrite<u32>),
        //Integer Baud Rate Register
        (0x024 => pub integer_baud_rate : WriteOnly<u32, INTEGER_BAUD_RATE::Register>),
        //Fractional Baud Rate Register
        (0x028 => pub fractional_baud_rate : WriteOnly<u32, FRACTIONAL_BAUD_RATE::Register>),
        //Line Control Register
        (0x02c => pub line_control : WriteOnly<u32, LINE_CONTROL::Register>),
        //Control Register
        (0x030 => pub control : ReadWrite<u32, CONTROL::Register>),
        //Interrupt FIFO Level Select Register
        (0x034 => pub interrupt_fifo_level_select : ReadWrite<u32>),
        //Interrupt Mask Set/Clear Register
        (0x038 => pub interrupt_mask_set : ReadWrite<u32, INTERRUPT_MASK::Register>),
        //Raw Interrupt Status Register
        (0x03c => pub raw_interrupt_status : ReadOnly<u32>),
        //Masked Interrupt Status Register
        (0x040 => pub masked_interrupt_status : ReadOnly<u32, INTERRUPT_MASK::Register>),
        //Interrupt Clear Register
        (0x044 => pub interrupt_clear : WriteOnly<u32, INTERRUPT_CLEAR::Register>),
        //DMA Control Register
        (0x048 => pub dma_control : ReadWrite<u32>),
        (0x4c => @END),
        //0x04C-0x07C Reserved
        //0x080-0x08C Reserved for test purposes
        //0x090-0xFCC Reserved
        //0xFD0-0xFDC Reserved for future ID expansion
        //Peripheral Identification Registers
        // (0xFE0 => pub peripheral_identification: [ReadOnly<u32>; 4]),
        //PrimeCell Identification Registers
        // (0xFF0 => pub prime_cell_identification: [ReadOnly<u32>; 4]),
    }
}
register_bitfields![u32,
    INTERRUPT_CLEAR [
        ALL OFFSET(0) NUMBITS(11) [
            Clear = 0x7ff
        ]
    ]
];
register_bitfields![u32,
    INTEGER_BAUD_RATE [
        VALUE OFFSET(0) NUMBITS(16) []
    ],
    FRACTIONAL_BAUD_RATE [
        FBRD OFFSET(0) NUMBITS(6) []
    ],
];

register_bitfields![u32,
    RECV_STATUS [
        Framing_error OFFSET(0) NUMBITS(1) [
            Valid = 0,
            Error = 1
        ],
        Parity_error OFFSET(1) NUMBITS(1) [
            Valid = 0,
            Error = 1
        ],
        Break_error OFFSET(2) NUMBITS(1) [
            Valid = 0,
            Error = 1
        ],
        Overrun_error OFFSET(3) NUMBITS(1) [
            Valid = 0,
            Error = 1
        ],
        Clear_error OFFSET(7) NUMBITS(1) [],
    ],
];

register_bitfields![u32,
    INTERRUPT_MASK [
        OEIM OFFSET(10) NUMBITS(1) [],
        BEIM OFFSET(9) NUMBITS(1) [],
        PEIM OFFSET(8) NUMBITS(1) [],
        FEIM OFFSET(7) NUMBITS(1) [],
        RTIM OFFSET(6) NUMBITS(1) [],
        TXIM OFFSET(5) NUMBITS(1) [],
        RXIM OFFSET(4) NUMBITS(1) [],
        DSRMIM OFFSET(3) NUMBITS(1) [],
        DCDMIM OFFSET(2) NUMBITS(1) [],
        CTSMIM OFFSET(1) NUMBITS(1) [],
        RIMIM OFFSET(0) NUMBITS(1) [],
    ],
];
register_bitfields![u32,
    CONTROL [
        UARTEN OFFSET(0) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1
        ],
        SIREN OFFSET(1) NUMBITS(1) [
            DISABLE = 0,
            ENABLE = 1
        ],
        SIRLP OFFSET(2) NUMBITS(1) [
            HIGH = 0,
            LOW = 1
        ],
        LBE OFFSET(7) NUMBITS(1) [],
        TXE OFFSET(8) NUMBITS(1) [],
        RXE OFFSET(9) NUMBITS(1) [],
        DTR OFFSET(10) NUMBITS(1) [],
        RTS OFFSET(11) NUMBITS(1) [],
        Out1 OFFSET(12) NUMBITS(1) [],
        Out2 OFFSET(13) NUMBITS(1) [],
        RTSEn OFFSET(14) NUMBITS(1) [],
        CTSEn OFFSET(15) NUMBITS(1) [],
    ],
];

register_bitfields![u32,
    LINE_CONTROL [
        LCRH OFFSET(5) NUMBITS(2) [
            WLEN_8 = (3 << 5),
            WLEN_7 = (2 << 5),
            WLEN_6 = (1 << 5),
            WLEN_5 = (0 << 5),
        ],
        LCRH_SPS OFFSET(7) NUMBITS(1) [],
        LCRH_FEN OFFSET(4) NUMBITS(1) [],
        LCRH_STP2 OFFSET(3) NUMBITS(1) [],
        LCRH_EPS OFFSET(2) NUMBITS(1) [],
        LCRH_PEN OFFSET(1) NUMBITS(1) [],
        LCRH_BRK OFFSET(0) NUMBITS(1) [],
    ],
];
register_bitfields![u32,

    Flags [
        ready_to_send OFFSET(0) NUMBITS(1)[
            Ready = 1,
            NotReady = 0
        ],
        ready_to_recv OFFSET(1) NUMBITS(1)[
            Ready = 1,
            NotReady = 0
        ],
        Data_carrier OFFSET(2) NUMBITS(1)[
            LOW = 1,
            High = 0
        ],
        BUSY OFFSET(3) NUMBITS(1)[
            Busy = 1,
            Idle = 0
        ],
        Receive_fifo_empty OFFSET(4) NUMBITS(1)[
            Empty = 1,
        ],
        Transmit_fifo_full OFFSET(5) NUMBITS(1)[
            Full = 1,
        ],
        Receive_fifo_full  OFFSET(6) NUMBITS(1)[
            Full = 1
        ],
        Transmit_fifo_empty  OFFSET(7) NUMBITS(1)[
            Empty = 1
        ],
    ],

];
//https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers
pub struct Pl011Uart {
    reg: *mut PL011Regs,
    base_addr: usize,
}

impl Pl011Uart {
    pub const fn new(addr: usize) -> Self {
        Self {
            reg: addr as *mut PL011Regs,
            base_addr: addr,
        }
    }
    pub fn init(&self, baud_rate: u32, uart_clk: u32) {
        self.reg().receive_status.set(0);
        self.reg()
            .control
            .write(CONTROL::TXE::CLEAR + CONTROL::RXE::CLEAR + CONTROL::UARTEN::DISABLE);
        self.reg()
            .interrupt_clear
            .write(INTERRUPT_CLEAR::ALL::Clear);
        self.reg()
            .interrupt_mask_set
            .write(INTERRUPT_MASK::RXIM::SET);
        self.reg().interrupt_fifo_level_select.set(0);
        if baud_rate != 0 {
            let divisor = (uart_clk * 4) / baud_rate;
            self.reg().integer_baud_rate.set(divisor >> 6);
            self.reg().fractional_baud_rate.set(divisor & 0x3f);
        }

        self.reg().line_control.write(
            LINE_CONTROL::LCRH::WLEN_8
                + LINE_CONTROL::LCRH_PEN::CLEAR
                + LINE_CONTROL::LCRH_STP2::CLEAR
                + LINE_CONTROL::LCRH_FEN::SET,
        );
        self.reg()
            .control
            .write(CONTROL::UARTEN::ENABLE + CONTROL::RXE::SET + CONTROL::TXE::SET);
    }
    pub fn is_rx_interrupt(&self) -> bool {
        self.reg()
            .masked_interrupt_status
            .matches_all(INTERRUPT_MASK::RXIM::SET)
    }
    pub fn is_tx_interrupt(&self) -> bool {
        self.reg()
            .masked_interrupt_status
            .matches_all(INTERRUPT_MASK::TXIM::SET)
    }
    pub fn flush(&self) {
        while self.reg().control.matches_all(CONTROL::UARTEN::SET)
            && self
            .reg()
            .flags
            .matches_all(Flags::Transmit_fifo_empty::CLEAR)
        {}
    }
    const fn reg(&self) -> &'static mut PL011Regs {
        unsafe { &mut *self.reg }
    }
    #[inline]
    pub fn rx_is_empty(&self) -> bool {
        self.reg()
            .flags
            .any_matching_bits_set(Flags::Receive_fifo_empty::Empty)
    }
    #[inline]
    fn tx_is_full(&self) -> bool {
        self.reg()
            .flags
            .matches_all(Flags::Transmit_fifo_full::Full)
    }
    #[inline]
    fn is_busy(&self) -> bool {
        self.reg().flags.matches_all(Flags::BUSY::Busy)
    }
    //
    #[inline]
    fn rx_ready(&self) -> bool {
        !self.is_busy() && !self.rx_is_empty()
    }
    //
    #[inline]
    fn tx_ready(&self) -> bool {
        !self.is_busy() && !self.tx_is_full()
    }
    #[inline]
    fn recv(&self) -> Option<u8> {
        match self.rx_ready() {
            false => None,
            true => Some(self.reg().data.get()),
        }
    }
    #[inline]
    fn send(&self, value: u8) {
        while !self.tx_ready() {}
        self.reg().data.set(value);
    }
}

impl Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.send(c as u8);
        }
        Ok(())
    }
}

impl Read for Pl011Uart {
    fn read_char(&self) -> Option<u8> {
        match self.recv() {
            None => None,
            Some(c) => Some(c),
        }
    }

    fn read_bytes(&self, buffer: &mut [u8]) -> usize {
        let mut read_len = 0;
        for i in 0..buffer.len() {
            match self.read_char() {
                None => {}
                Some(c) => {
                    buffer[i] = c;
                    read_len += 1;
                }
            }
        }
        read_len
    }
}
