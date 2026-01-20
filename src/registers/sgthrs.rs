//! SGTHRS - StallGuard threshold register (0x40)

use super::{Address, Register, WritableRegister};

/// StallGuard threshold register.
///
/// Detection threshold for stall detection. The StallGuard value SG_RESULT
/// becomes compared to this threshold. Higher values make stall detection
/// more sensitive.
///
/// When SG_RESULT falls below SGTHRS*2, the DIAG output is asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Sgthrs(u32);

impl Sgthrs {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the StallGuard threshold (0-255).
    ///
    /// When SG_RESULT < SGTHRS*2, stall is detected.
    /// - 0: StallGuard output disabled
    /// - Higher values = more sensitive stall detection
    pub fn threshold(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Set the StallGuard threshold (0-255).
    pub fn set_threshold(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0xFF) | (value as u32);
        self
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

impl Default for Sgthrs {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Sgthrs {
    const ADDRESS: Address = Address::Sgthrs;
}

impl WritableRegister for Sgthrs {}

impl From<u32> for Sgthrs {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Sgthrs> for u32 {
    fn from(reg: Sgthrs) -> u32 {
        reg.0
    }
}
