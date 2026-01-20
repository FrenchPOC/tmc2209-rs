//! COOLCONF - CoolStep and StallGuard2 configuration register (0x42)

use super::{Address, Register, WritableRegister};

/// CoolStep and StallGuard2 configuration register.
///
/// Controls CoolStep adaptive current control and StallGuard filtering.
/// CoolStep automatically adjusts motor current based on load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Coolconf(u32);

impl Coolconf {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get SEMIN (0-15).
    ///
    /// Minimum StallGuard value for CoolStep current increase.
    /// - 0: CoolStep disabled
    /// - 1-15: If SG_RESULT < SEMIN*32, motor current is increased
    pub fn semin(&self) -> u8 {
        (self.0 & 0x0F) as u8
    }

    /// Set SEMIN (0-15).
    pub fn set_semin(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x0F) | ((value as u32) & 0x0F);
        self
    }

    /// Get SEUP (0-3).
    ///
    /// Current increment step width:
    /// - 0: +1
    /// - 1: +2
    /// - 2: +4
    /// - 3: +8
    pub fn seup(&self) -> u8 {
        ((self.0 >> 5) & 0x03) as u8
    }

    /// Set SEUP (0-3).
    pub fn set_seup(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self
    }

    /// Get SEMAX (0-15).
    ///
    /// StallGuard hysteresis value for CoolStep current decrease.
    /// If SG_RESULT > (SEMIN + SEMAX + 1)*32, motor current is decreased.
    pub fn semax(&self) -> u8 {
        ((self.0 >> 8) & 0x0F) as u8
    }

    /// Set SEMAX (0-15).
    pub fn set_semax(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x0F << 8)) | (((value as u32) & 0x0F) << 8);
        self
    }

    /// Get SEDN (0-3).
    ///
    /// Current decrement step width:
    /// - 0: Decrement by 32
    /// - 1: Decrement by 8
    /// - 2: Decrement by 2
    /// - 3: Decrement by 1
    pub fn sedn(&self) -> u8 {
        ((self.0 >> 13) & 0x03) as u8
    }

    /// Set SEDN (0-3).
    pub fn set_sedn(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self
    }

    /// Get SEIMIN.
    ///
    /// Minimum current for CoolStep:
    /// - false: 1/2 of IRUN
    /// - true: 1/4 of IRUN
    pub fn seimin(&self) -> bool {
        (self.0 >> 15) & 1 != 0
    }

    /// Set SEIMIN.
    pub fn set_seimin(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 15;
        } else {
            self.0 &= !(1 << 15);
        }
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

impl Default for Coolconf {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Coolconf {
    const ADDRESS: Address = Address::Coolconf;
}

impl WritableRegister for Coolconf {}

impl From<u32> for Coolconf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Coolconf> for u32 {
    fn from(reg: Coolconf) -> u32 {
        reg.0
    }
}
