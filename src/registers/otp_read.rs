//! OTP_READ - OTP memory read register (0x05)

use super::{Address, ReadableRegister, Register};

/// OTP memory read register.
///
/// Contains the power-up defaults stored in OTP memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OtpRead(u32);

impl OtpRead {
    /// FCLKTRIM value (0-31).
    ///
    /// Factory-programmed clock frequency trim.
    /// **Do not alter - differs between individual ICs.**
    pub fn otp_fclktrim(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    /// OTTRIM value.
    ///
    /// - 0: OT=143°C
    /// - 1: OT=150°C
    pub fn otp_ottrim(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Internal Rsense default.
    ///
    /// - `true`: Internal sense resistors
    /// - `false`: External sense resistors
    pub fn otp_internal_rsense(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// TBL default.
    ///
    /// - `false`: TBL=0b10
    /// - `true`: TBL=0b01
    pub fn otp_tbl(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// PWM_GRAD default (0-15).
    pub fn otp_pwm_grad(&self) -> u8 {
        ((self.0 >> 8) & 0x0F) as u8
    }

    /// PWM_AUTOGRAD default.
    pub fn otp_pwm_autograd(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// TPWM_THRS default (0-7).
    pub fn otp_tpwmthrs(&self) -> u8 {
        ((self.0 >> 13) & 0x07) as u8
    }

    /// PWM_OFS default.
    ///
    /// - `false`: PWM_OFS=36
    /// - `true`: PWM_OFS=0
    pub fn otp_pwm_ofs(&self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// PWM_REG default.
    ///
    /// - `false`: PWM_REG=0b1000
    /// - `true`: PWM_REG=0b0010
    pub fn otp_pwm_reg(&self) -> bool {
        self.0 & (1 << 17) != 0
    }

    /// PWM_FREQ default.
    ///
    /// - `false`: PWM_FREQ=0b01
    /// - `true`: PWM_FREQ=0b10
    pub fn otp_pwm_freq(&self) -> bool {
        self.0 & (1 << 18) != 0
    }

    /// IHOLDDELAY default (0-3).
    pub fn otp_iholddelay(&self) -> u8 {
        ((self.0 >> 19) & 0x03) as u8
    }

    /// IHOLD default (0-3).
    pub fn otp_ihold(&self) -> u8 {
        ((self.0 >> 21) & 0x03) as u8
    }

    /// SpreadCycle enabled by default.
    ///
    /// - `true`: SpreadCycle mode
    /// - `false`: StealthChop mode
    pub fn otp_en_spreadcycle(&self) -> bool {
        self.0 & (1 << 23) != 0
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

impl Register for OtpRead {
    const ADDRESS: Address = Address::OtpRead;
}

impl ReadableRegister for OtpRead {}

impl From<u32> for OtpRead {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<OtpRead> for u32 {
    fn from(reg: OtpRead) -> u32 {
        reg.0
    }
}
