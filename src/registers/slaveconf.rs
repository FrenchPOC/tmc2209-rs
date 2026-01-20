//! SLAVECONF - Slave configuration register (0x03)

use super::{Address, Register, WritableRegister};

/// Slave configuration register.
///
/// Configures the UART reply delay for read accesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Slaveconf(u32);

impl Slaveconf {
    /// Get SENDDELAY value.
    ///
    /// Time until reply is sent after read access:
    /// - 0, 1: 8 bit times (don't use in multi-slave)
    /// - 2, 3: 3×8 bit times
    /// - 4, 5: 5×8 bit times
    /// - 6, 7: 7×8 bit times
    /// - 8, 9: 9×8 bit times
    /// - 10, 11: 11×8 bit times
    /// - 12, 13: 13×8 bit times
    /// - 14, 15: 15×8 bit times
    pub fn senddelay(&self) -> u8 {
        ((self.0 >> 8) & 0x0F) as u8
    }

    /// Set SENDDELAY value (0-15).
    pub fn set_senddelay(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x0F00) | (((value as u32) & 0x0F) << 8);
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

impl Register for Slaveconf {
    const ADDRESS: Address = Address::Slaveconf;
}

impl WritableRegister for Slaveconf {}

impl From<u32> for Slaveconf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Slaveconf> for u32 {
    fn from(reg: Slaveconf) -> u32 {
        reg.0
    }
}
