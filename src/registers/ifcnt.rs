//! IFCNT - Interface transmission counter (0x02)

use super::{Address, ReadableRegister, Register};

/// Interface transmission counter.
///
/// This register is incremented with each successful UART write access.
/// Read to check serial transmission for lost data.
/// Wraps around from 255 to 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ifcnt(u32);

impl Ifcnt {
    /// Get the interface counter value (0-255).
    pub fn count(&self) -> u8 {
        (self.0 & 0xFF) as u8
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

impl Register for Ifcnt {
    const ADDRESS: Address = Address::Ifcnt;
}

impl ReadableRegister for Ifcnt {}

impl From<u32> for Ifcnt {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Ifcnt> for u32 {
    fn from(reg: Ifcnt) -> u32 {
        reg.0
    }
}
