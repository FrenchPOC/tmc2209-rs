//! OTP_PROG - OTP programming register (0x04)

use super::{Address, Register, WritableRegister};

/// OTP programming register.
///
/// **Warning:** OTP = One Time Programmable. Bits can only be set to 1, never cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OtpProg(u32);

impl OtpProg {
    /// OTP bit selection (0-7).
    ///
    /// Selects which bit to program in the selected byte location.
    pub fn otpbit(&self) -> u8 {
        (self.0 & 0x07) as u8
    }

    /// Set OTP bit selection.
    pub fn set_otpbit(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x07) | ((value as u32) & 0x07);
        self
    }

    /// OTP byte selection (0-2).
    ///
    /// Selects the OTP byte location to program.
    pub fn otpbyte(&self) -> u8 {
        ((self.0 >> 4) & 0x03) as u8
    }

    /// Set OTP byte selection.
    pub fn set_otpbyte(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x30) | (((value as u32) & 0x03) << 4);
        self
    }

    /// OTP magic value.
    ///
    /// Set to 0xBD to enable programming.
    /// A programming time of minimum 10ms per bit is recommended.
    pub fn otpmagic(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Set OTP magic value.
    pub fn set_otpmagic(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0xFF00) | ((value as u32) << 8);
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

impl Register for OtpProg {
    const ADDRESS: Address = Address::OtpProg;
}

impl WritableRegister for OtpProg {}

impl From<u32> for OtpProg {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<OtpProg> for u32 {
    fn from(reg: OtpProg) -> u32 {
        reg.0
    }
}
