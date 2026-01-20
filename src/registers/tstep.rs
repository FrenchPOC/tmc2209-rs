//! TSTEP - Measured step time register (0x12)

use super::{Address, ReadableRegister, Register};

/// Measured step time register.
///
/// Actual measured time between two 1/256 microsteps derived from step input frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Tstep(u32);

impl Tstep {
    /// Maximum value indicating overflow or standstill.
    pub const MAX_VALUE: u32 = 0xFFFFF;

    /// Get the measured step time value (0 to 2^20 - 1).
    ///
    /// In units of 1/f_CLK (typically 1/12MHz ≈ 83.3ns).
    /// Value of 0xFFFFF indicates overflow or standstill.
    pub fn value(&self) -> u32 {
        self.0 & 0xFFFFF
    }

    /// Alias for `value()` - get the TSTEP value.
    pub fn tstep(&self) -> u32 {
        self.value()
    }

    /// Check if the motor is in standstill or overflow condition.
    pub fn is_standstill(&self) -> bool {
        self.value() == Self::MAX_VALUE
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

impl Register for Tstep {
    const ADDRESS: Address = Address::Tstep;
}

impl ReadableRegister for Tstep {}

impl From<u32> for Tstep {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tstep> for u32 {
    fn from(reg: Tstep) -> u32 {
        reg.0
    }
}
