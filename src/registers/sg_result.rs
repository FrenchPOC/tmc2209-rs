//! SG_RESULT - StallGuard result register (0x41)

use super::{Address, ReadableRegister, Register};

/// StallGuard result register.
///
/// Read-only register containing the StallGuard measurement result.
/// The value indicates the mechanical load on the motor.
///
/// - High values indicate low load (motor running freely)
/// - Low values indicate high load (motor stalling)
/// - When SG_RESULT < SGTHRS*2, stall is detected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SgResult(u32);

impl SgResult {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the StallGuard result value (0-510).
    ///
    /// This is a 10-bit value representing mechanical load:
    /// - 0-510: Load indicator (higher = less load)
    /// - Values near 0 indicate stall condition
    pub fn result(&self) -> u16 {
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

impl Default for SgResult {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for SgResult {
    const ADDRESS: Address = Address::SgResult;
}

impl ReadableRegister for SgResult {}

impl From<u32> for SgResult {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SgResult> for u32 {
    fn from(reg: SgResult) -> u32 {
        reg.0
    }
}
