//! TPWMTHRS - StealthChop threshold register (0x13)

use super::{Address, Register, WritableRegister};

/// StealthChop threshold register.
///
/// Sets the upper velocity threshold for StealthChop mode.
/// When TSTEP >= TPWMTHRS, StealthChop is active.
/// When velocity exceeds threshold, switches to SpreadCycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Tpwmthrs(u32);

impl Tpwmthrs {
    /// Create a new TPWMTHRS with value 0 (StealthChop always active).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the threshold value (0 to 2^20 - 1).
    ///
    /// - 0: Disabled, StealthChop used for all velocities
    /// - Other: Threshold in TSTEP units
    pub fn value(&self) -> u32 {
        self.0 & 0xFFFFF
    }

    /// Set the threshold value.
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

    /// Disable the threshold (use StealthChop for all velocities).
    pub fn disable(&mut self) -> &mut Self {
        self.0 = 0;
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

impl Register for Tpwmthrs {
    const ADDRESS: Address = Address::Tpwmthrs;
}

impl WritableRegister for Tpwmthrs {}

impl From<u32> for Tpwmthrs {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tpwmthrs> for u32 {
    fn from(reg: Tpwmthrs) -> u32 {
        reg.0
    }
}
