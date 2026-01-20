//! MSCNT - Microstep counter register (0x6A)

use super::{Address, ReadableRegister, Register};

/// Microstep counter register.
///
/// Read-only register containing the actual microstep counter position
/// within one electrical period (0-1023).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Mscnt(u32);

impl Mscnt {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the microstep counter value (0-1023).
    ///
    /// This is the actual microstep position within one electrical period
    /// of 1024 microsteps (256 * 4 phases).
    pub fn count(&self) -> u16 {
        (self.0 & 0x3FF) as u16
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

impl Default for Mscnt {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Mscnt {
    const ADDRESS: Address = Address::Mscnt;
}

impl ReadableRegister for Mscnt {}

impl From<u32> for Mscnt {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Mscnt> for u32 {
    fn from(reg: Mscnt) -> u32 {
        reg.0
    }
}
