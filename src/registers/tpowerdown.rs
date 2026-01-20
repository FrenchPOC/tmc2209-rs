//! TPOWERDOWN - Power down delay register (0x11)

use super::{Address, Register, WritableRegister};

/// Power down delay register.
///
/// Sets the delay from standstill detection to motor current power down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Tpowerdown(u32);

impl Tpowerdown {
    /// Default value (20).
    pub const DEFAULT: u32 = 0x00000014;

    /// Create with default value.
    pub fn new() -> Self {
        Self(Self::DEFAULT)
    }

    /// Get the power down delay value (0-255).
    ///
    /// Time range is about 0 to 5.6 seconds.
    /// Formula: delay = value × 2^18 × t_CLK
    ///
    /// **Note:** Minimum setting of 2 is required for automatic
    /// StealthChop PWM_OFFS_AUTO tuning.
    pub fn value(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Set the power down delay value (0-255).
    pub fn set_value(&mut self, value: u8) -> &mut Self {
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

impl Default for Tpowerdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Tpowerdown {
    const ADDRESS: Address = Address::Tpowerdown;
}

impl WritableRegister for Tpowerdown {}

impl From<u32> for Tpowerdown {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tpowerdown> for u32 {
    fn from(reg: Tpowerdown) -> u32 {
        reg.0
    }
}
