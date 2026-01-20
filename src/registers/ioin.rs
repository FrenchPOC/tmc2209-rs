//! IOIN - Input pin states register (0x06)

use super::{Address, ReadableRegister, Register};

/// Input pin states register.
///
/// Reads the state of all input pins and the IC version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ioin(u32);

impl Ioin {
    /// ENN pin state.
    pub fn enn(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// MS1 pin state.
    pub fn ms1(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// MS2 pin state.
    pub fn ms2(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// DIAG pin state.
    pub fn diag(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// PDN_UART pin state.
    pub fn pdn_uart(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// STEP pin state.
    pub fn step(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// SPREAD (SEL_A) pin state.
    pub fn spread_en(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// DIR pin state.
    pub fn dir(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// IC version number.
    ///
    /// 0x21 is the first version of the TMC2209.
    /// Identical numbers mean full digital compatibility.
    pub fn version(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Get the raw register value.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Create from raw value.
    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }
}

impl Register for Ioin {
    const ADDRESS: Address = Address::Ioin;
}

impl ReadableRegister for Ioin {}

impl From<u32> for Ioin {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Ioin> for u32 {
    fn from(reg: Ioin) -> u32 {
        reg.0
    }
}
