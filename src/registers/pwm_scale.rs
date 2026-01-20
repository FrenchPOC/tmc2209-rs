//! PWM_SCALE - PWM scaling result register (0x71)

use super::{Address, ReadableRegister, Register};

/// PWM scaling result register.
///
/// Read-only register containing the automatic PWM scaling results.
/// Used to monitor StealthChop auto-tuning status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PwmScale(u32);

impl PwmScale {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get PWM_SCALE_SUM (0-255).
    ///
    /// Actual PWM duty cycle (sum of amplitude and offset).
    /// Results from automatic amplitude regulation.
    pub fn pwm_scale_sum(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get PWM_SCALE_AUTO (signed, -255 to +255).
    ///
    /// Result of automatic amplitude regulation based on current measurement.
    /// Signed 9-bit value.
    pub fn pwm_scale_auto(&self) -> i16 {
        let raw = ((self.0 >> 16) & 0x1FF) as u16;
        // Sign-extend from 9-bit to 16-bit
        if raw & 0x100 != 0 {
            (raw | 0xFE00) as i16
        } else {
            raw as i16
        }
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

impl Default for PwmScale {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for PwmScale {
    const ADDRESS: Address = Address::PwmScale;
}

impl ReadableRegister for PwmScale {}

impl From<u32> for PwmScale {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<PwmScale> for u32 {
    fn from(reg: PwmScale) -> u32 {
        reg.0
    }
}
