//! IHOLD_IRUN - Driver current control register (0x10)

use super::{Address, Register, WritableRegister};

/// Driver current control register.
///
/// Sets the motor hold and run currents, and the power-down delay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IholdIrun(u32);

impl IholdIrun {
    /// Default value: IRUN=31, IHOLD=16, IHOLDDELAY=1
    pub const DEFAULT: u32 = 0x00071703;

    /// Create with default values.
    pub fn new() -> Self {
        Self(Self::DEFAULT)
    }

    /// Standstill current (0-31).
    ///
    /// 0 = 1/32 of max current, 31 = 32/32 of max current.
    /// Setting 0 allows freewheeling or coil short circuit for standstill.
    pub fn ihold(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    /// Set standstill current (0-31).
    pub fn set_ihold(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x1F) | ((value as u32) & 0x1F);
        self
    }

    /// Motor run current (0-31).
    ///
    /// 0 = 1/32 of max current, 31 = 32/32 of max current.
    /// Recommended: 16-31 for best microstep performance.
    pub fn irun(&self) -> u8 {
        ((self.0 >> 8) & 0x1F) as u8
    }

    /// Set motor run current (0-31).
    pub fn set_irun(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x1F00) | (((value as u32) & 0x1F) << 8);
        self
    }

    /// IHOLDDELAY (0-15).
    ///
    /// Delay per current reduction step after standstill detection.
    /// - 0: Instant power down
    /// - 1-15: Delay in multiples of 2^18 clocks
    pub fn iholddelay(&self) -> u8 {
        ((self.0 >> 16) & 0x0F) as u8
    }

    /// Set IHOLDDELAY (0-15).
    pub fn set_iholddelay(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x0F0000) | (((value as u32) & 0x0F) << 16);
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

impl Default for IholdIrun {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for IholdIrun {
    const ADDRESS: Address = Address::IholdIrun;
}

impl WritableRegister for IholdIrun {}

impl From<u32> for IholdIrun {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<IholdIrun> for u32 {
    fn from(reg: IholdIrun) -> u32 {
        reg.0
    }
}
