//! TCOOLTHRS - CoolStep threshold register (0x14)

use super::{Address, Register, WritableRegister};

/// CoolStep threshold register.
///
/// Lower threshold velocity for enabling CoolStep and StallGuard.
/// CoolStep is enabled when TCOOLTHRS >= TSTEP > TPWMTHRS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Tcoolthrs(u32);

impl Tcoolthrs {
    /// Create a new TCOOLTHRS with value 0 (CoolStep/StallGuard disabled).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the threshold value (0 to 2^20 - 1).
    pub fn value(&self) -> u32 {
        self.0 & 0xFFFFF
    }

    /// Set the threshold value.
    ///
    /// Set to disable CoolStep at low speeds where it cannot work reliably.
    /// StallGuard output is enabled when velocity exceeds this threshold.
    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.0 = value & 0xFFFFF;
        self
    }

    /// Alias for set_value - set the threshold.
    pub fn set_threshold(&mut self, value: u32) -> &mut Self {
        self.set_value(value)
    }

    /// Get the threshold value (alias for value()).
    pub fn threshold(&self) -> u32 {
        self.value()
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

impl Register for Tcoolthrs {
    const ADDRESS: Address = Address::Tcoolthrs;
}

impl WritableRegister for Tcoolthrs {}

impl From<u32> for Tcoolthrs {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tcoolthrs> for u32 {
    fn from(reg: Tcoolthrs) -> u32 {
        reg.0
    }
}
