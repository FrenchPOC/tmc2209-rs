//! PWMCONF - StealthChop PWM configuration register (0x70)

use super::{Address, ReadableRegister, Register, StandstillMode, WritableRegister};

/// StealthChop PWM configuration register.
///
/// Controls StealthChop PWM operation for silent motor operation.
/// StealthChop uses voltage PWM instead of current chopping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pwmconf(u32);

impl Pwmconf {
    /// Default value with StealthChop enabled and auto-tuning.
    /// PWM_OFS=36, PWM_GRAD=14, pwm_freq=1, pwm_autoscale=1, pwm_autograd=1
    pub const DEFAULT: u32 = 0xC10D0024;

    /// Create with default values.
    pub fn new() -> Self {
        Self(Self::DEFAULT)
    }

    /// Get PWM_OFS (0-255).
    ///
    /// User-defined PWM amplitude offset (0-255).
    /// When pwm_autoscale=1: Used as base for automatic scaling.
    /// When pwm_autoscale=0: Used directly as PWM amplitude.
    pub fn pwm_ofs(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Set PWM_OFS (0-255).
    pub fn set_pwm_ofs(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0xFF) | (value as u32);
        self
    }

    /// Get PWM_GRAD (0-255).
    ///
    /// User-defined PWM amplitude gradient.
    /// Velocity-dependent gradient for PWM amplitude.
    /// When pwm_autoscale=1: Scaled automatically.
    pub fn pwm_grad(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Set PWM_GRAD (0-255).
    pub fn set_pwm_grad(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0xFF << 8)) | ((value as u32) << 8);
        self
    }

    /// Get PWM_FREQ (0-3).
    ///
    /// PWM frequency selection:
    /// - 0: fPWM = 2/1024 fCLK
    /// - 1: fPWM = 2/683 fCLK (recommended)
    /// - 2: fPWM = 2/512 fCLK
    /// - 3: fPWM = 2/410 fCLK
    pub fn pwm_freq(&self) -> u8 {
        ((self.0 >> 16) & 0x03) as u8
    }

    /// Set PWM_FREQ (0-3).
    pub fn set_pwm_freq(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self
    }

    /// Get PWM_AUTOSCALE.
    ///
    /// Enable automatic current scaling:
    /// - true: Automatic tuning (recommended)
    /// - false: Use PWM_GRAD and PWM_OFS directly
    pub fn pwm_autoscale(&self) -> bool {
        (self.0 >> 18) & 1 != 0
    }

    /// Set PWM_AUTOSCALE.
    pub fn set_pwm_autoscale(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 18;
        } else {
            self.0 &= !(1 << 18);
        }
        self
    }

    /// Get PWM_AUTOGRAD.
    ///
    /// Enable automatic gradient tuning:
    /// - true: Automatic gradient adaptation (recommended)
    /// - false: Use PWM_GRAD directly
    pub fn pwm_autograd(&self) -> bool {
        (self.0 >> 19) & 1 != 0
    }

    /// Set PWM_AUTOGRAD.
    pub fn set_pwm_autograd(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 19;
        } else {
            self.0 &= !(1 << 19);
        }
        self
    }

    /// Get FREEWHEEL (0-3).
    ///
    /// Standstill mode when motor current is zero:
    /// - 0: Normal operation
    /// - 1: Freewheeling
    /// - 2: Coil shorted using LS drivers (strong braking)
    /// - 3: Coil shorted using HS drivers (braking)
    pub fn freewheel(&self) -> u8 {
        ((self.0 >> 20) & 0x03) as u8
    }

    /// Set FREEWHEEL (0-3).
    pub fn set_freewheel(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self
    }

    /// Get the freewheel mode as enum.
    pub fn standstill_mode(&self) -> StandstillMode {
        StandstillMode::from_bits(self.freewheel())
    }

    /// Set the freewheel mode from enum.
    pub fn set_standstill_mode(&mut self, mode: StandstillMode) -> &mut Self {
        self.set_freewheel(mode.to_bits())
    }

    /// Get PWM_REG (0-15).
    ///
    /// Maximum PWM amplitude change per half wave:
    /// - 1-15: Limit PWM amplitude change
    /// - 0: No limit (not recommended)
    pub fn pwm_reg(&self) -> u8 {
        ((self.0 >> 24) & 0x0F) as u8
    }

    /// Set PWM_REG (0-15).
    pub fn set_pwm_reg(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x0F << 24)) | (((value as u32) & 0x0F) << 24);
        self
    }

    /// Get PWM_LIM (0-15).
    ///
    /// PWM automatic scale amplitude limit when switching on.
    /// Limits starting current during StealthChop operation.
    pub fn pwm_lim(&self) -> u8 {
        ((self.0 >> 28) & 0x0F) as u8
    }

    /// Set PWM_LIM (0-15).
    pub fn set_pwm_lim(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x0F << 28)) | (((value as u32) & 0x0F) << 28);
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

impl Default for Pwmconf {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Pwmconf {
    const ADDRESS: Address = Address::Pwmconf;
}

impl ReadableRegister for Pwmconf {}
impl WritableRegister for Pwmconf {}

impl From<u32> for Pwmconf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Pwmconf> for u32 {
    fn from(reg: Pwmconf) -> u32 {
        reg.0
    }
}
