#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SgiData {
    All,
    List {
        affinity3: u8,
        affinity2: u8,
        affinity1: u8,
        target_list: u16,
    },
}

pub enum Trigger {
    Edge = 0,
    Level = 1,
    None,
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct IntId(pub(crate) u32);

impl IntId {
    /// The ID of the first Software Generated Interrupt.
    const SGI_START: u32 = 0;

    /// The ID of the first Private Peripheral Interrupt.
    const PPI_START: u32 = 16;

    /// The ID of the first Shared Peripheral Interrupt.
    const SPI_START: u32 = 32;

    /// The first special interrupt ID.
    const SPECIAL_START: u32 = 1020;

    /// Returns the interrupt ID for the given Software Generated Interrupt.
    pub const fn new_empty() -> Self {
        Self { 0: 0 }
    }
    pub const fn sgi(sgi: u32) -> Self {
        assert!(sgi < Self::PPI_START);
        Self(Self::SGI_START + sgi)
    }

    /// Returns the interrupt ID for the given Private Peripheral Interrupt.
    pub const fn ppi(ppi: u32) -> Self {
        assert!(ppi < Self::SPI_START - Self::PPI_START);
        Self(Self::PPI_START + ppi)
    }

    /// Returns the interrupt ID for the given Shared Peripheral Interrupt.
    pub const fn spi(spi: u32) -> Self {
        assert!(spi < Self::SPECIAL_START);
        Self(Self::SPI_START + spi)
    }

    /// Returns whether this interrupt ID is for a Software Generated Interrupt.
    pub(crate) fn is_sgi(self) -> bool {
        self.0 < Self::PPI_START
    }

    /// Returns whether this interrupt ID is private to a core, i.e. it is an SGI or PPI.
    fn is_private(self) -> bool {
        self.0 < Self::SPI_START
    }
}