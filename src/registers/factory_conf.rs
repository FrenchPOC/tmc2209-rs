//! FACTORY_CONF - Factory configuration register (0x07)

use super::{Address, ReadableRegister, Register, WritableRegister};

/// Factory configuration register.
///
/// Contains clock frequency trim and overtemperature settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FactoryConf(u32);

impl FactoryConf {
    /// FCLKTRIM value (0-31).
    ///
    /// Clock frequency trim. 0 = lowest, 31 = highest frequency.
    /// **Pre-programmed by factory to 12MHz - do not alter.**
    pub fn fclktrim(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    /// Set FCLKTRIM value.
    pub fn set_fclktrim(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x1F) | ((value as u32) & 0x1F);
        self
    }

    /// OTTRIM value (0-3).
    ///
    /// Overtemperature threshold selection:
    /// - 0b00: OT=143°C, OTPW=120°C
    /// - 0b01: OT=150°C, OTPW=120°C
    /// - 0b10: OT=150°C, OTPW=143°C
    /// - 0b11: OT=157°C, OTPW=143°C
    pub fn ottrim(&self) -> u8 {
        ((self.0 >> 8) & 0x03) as u8
    }

    /// Set OTTRIM value.
    pub fn set_ottrim(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x300) | (((value as u32) & 0x03) << 8);
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

impl Register for FactoryConf {
    const ADDRESS: Address = Address::FactoryConf;
}

impl ReadableRegister for FactoryConf {}
impl WritableRegister for FactoryConf {}

impl From<u32> for FactoryConf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<FactoryConf> for u32 {
    fn from(reg: FactoryConf) -> u32 {
        reg.0
    }
}
