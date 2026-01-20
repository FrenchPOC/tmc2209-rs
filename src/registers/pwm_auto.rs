//! PWM_AUTO - Automatic PWM configuration register (0x72)

use super::{Address, ReadableRegister, Register};

/// Automatic PWM configuration register.
///
/// Read-only register containing the automatic PWM tuning results.
/// These values are computed by the auto-tuning algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PwmAuto(u32);

impl PwmAuto {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get PWM_OFS_AUTO (0-255).
    ///
    /// Automatically determined offset value.
    /// Result of automatic amplitude calibration.
    pub fn pwm_ofs_auto(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get PWM_GRAD_AUTO (0-255).
    ///
    /// Automatically determined gradient value.
    /// Result of automatic gradient calibration for velocity-dependent current.
    pub fn pwm_grad_auto(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
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

impl Default for PwmAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for PwmAuto {
    const ADDRESS: Address = Address::PwmAuto;
}

impl ReadableRegister for PwmAuto {}

impl From<u32> for PwmAuto {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<PwmAuto> for u32 {
    fn from(reg: PwmAuto) -> u32 {
        reg.0
    }
}
