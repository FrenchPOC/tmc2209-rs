//! TMC2209 register definitions and traits.
//!
//! This module contains:
//! - Register traits for type-safe access
//! - The `Address` enum for all register addresses
//! - Individual register structs with bitfield accessors

mod gconf;
mod gstat;
mod ifcnt;
mod slaveconf;
mod otp_prog;
mod otp_read;
mod ioin;
mod factory_conf;
mod ihold_irun;
mod tpowerdown;
mod tstep;
mod tpwmthrs;
mod tcoolthrs;
mod vactual;
mod sgthrs;
mod sg_result;
mod coolconf;
mod mscnt;
mod mscuract;
mod chopconf;
mod drv_status;
mod pwmconf;
mod pwm_scale;
mod pwm_auto;

pub use gconf::Gconf;
pub use gstat::Gstat;
pub use ifcnt::Ifcnt;
pub use slaveconf::Slaveconf;
pub use otp_prog::OtpProg;
pub use otp_read::OtpRead;
pub use ioin::Ioin;
pub use factory_conf::FactoryConf;
pub use ihold_irun::IholdIrun;
pub use tpowerdown::Tpowerdown;
pub use tstep::Tstep;
pub use tpwmthrs::Tpwmthrs;
pub use tcoolthrs::Tcoolthrs;
pub use vactual::Vactual;
pub use sgthrs::Sgthrs;
pub use sg_result::SgResult;
pub use coolconf::Coolconf;
pub use mscnt::Mscnt;
pub use mscuract::Mscuract;
pub use chopconf::Chopconf;
pub use drv_status::DrvStatus;
pub use pwmconf::Pwmconf;
pub use pwm_scale::PwmScale;
pub use pwm_auto::PwmAuto;

/// Trait for all TMC2209 registers.
pub trait Register: Sized + Copy + Clone + Default + Into<u32> + From<u32> {
    /// The register address.
    const ADDRESS: Address;

    /// Get the register address.
    fn address() -> Address {
        Self::ADDRESS
    }
}

/// Trait for registers that can be read.
pub trait ReadableRegister: Register {}

/// Trait for registers that can be written.
pub trait WritableRegister: Register {}

/// TMC2209 register addresses.
///
/// All registers in the TMC2209 and their 7-bit addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Address {
    /// Global configuration (0x00) - RW
    Gconf = 0x00,
    /// Global status flags (0x01) - R+WC
    Gstat = 0x01,
    /// Interface transmission counter (0x02) - R
    Ifcnt = 0x02,
    /// UART slave configuration (0x03) - W
    Slaveconf = 0x03,
    /// OTP programming (0x04) - W
    OtpProg = 0x04,
    /// OTP memory read (0x05) - R
    OtpRead = 0x05,
    /// Input pin states (0x06) - R
    Ioin = 0x06,
    /// Factory configuration (0x07) - RW
    FactoryConf = 0x07,
    /// Hold/run current (0x10) - W
    IholdIrun = 0x10,
    /// Power down delay (0x11) - W
    Tpowerdown = 0x11,
    /// Measured step time (0x12) - R
    Tstep = 0x12,
    /// StealthChop threshold (0x13) - W
    Tpwmthrs = 0x13,
    /// CoolStep threshold (0x14) - W
    Tcoolthrs = 0x14,
    /// UART velocity control (0x22) - W
    Vactual = 0x22,
    /// StallGuard threshold (0x40) - W
    Sgthrs = 0x40,
    /// StallGuard result (0x41) - R
    SgResult = 0x41,
    /// CoolStep configuration (0x42) - W
    Coolconf = 0x42,
    /// Microstep counter (0x6A) - R
    Mscnt = 0x6A,
    /// Microstep current (0x6B) - R
    Mscuract = 0x6B,
    /// Chopper configuration (0x6C) - RW
    Chopconf = 0x6C,
    /// Driver status (0x6F) - R
    DrvStatus = 0x6F,
    /// StealthChop PWM configuration (0x70) - RW
    Pwmconf = 0x70,
    /// PWM scaling result (0x71) - R
    PwmScale = 0x71,
    /// Automatic PWM values (0x72) - R
    PwmAuto = 0x72,
}

impl Address {
    /// Convert a u8 to an Address if it's a known register.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Address::Gconf),
            0x01 => Some(Address::Gstat),
            0x02 => Some(Address::Ifcnt),
            0x03 => Some(Address::Slaveconf),
            0x04 => Some(Address::OtpProg),
            0x05 => Some(Address::OtpRead),
            0x06 => Some(Address::Ioin),
            0x07 => Some(Address::FactoryConf),
            0x10 => Some(Address::IholdIrun),
            0x11 => Some(Address::Tpowerdown),
            0x12 => Some(Address::Tstep),
            0x13 => Some(Address::Tpwmthrs),
            0x14 => Some(Address::Tcoolthrs),
            0x22 => Some(Address::Vactual),
            0x40 => Some(Address::Sgthrs),
            0x41 => Some(Address::SgResult),
            0x42 => Some(Address::Coolconf),
            0x6A => Some(Address::Mscnt),
            0x6B => Some(Address::Mscuract),
            0x6C => Some(Address::Chopconf),
            0x6F => Some(Address::DrvStatus),
            0x70 => Some(Address::Pwmconf),
            0x71 => Some(Address::PwmScale),
            0x72 => Some(Address::PwmAuto),
            _ => None,
        }
    }

    /// Check if this register is readable.
    pub fn is_readable(self) -> bool {
        matches!(
            self,
            Address::Gconf
                | Address::Gstat
                | Address::Ifcnt
                | Address::OtpRead
                | Address::Ioin
                | Address::FactoryConf
                | Address::Tstep
                | Address::SgResult
                | Address::Mscnt
                | Address::Mscuract
                | Address::Chopconf
                | Address::DrvStatus
                | Address::Pwmconf
                | Address::PwmScale
                | Address::PwmAuto
        )
    }

    /// Check if this register is writable.
    pub fn is_writable(self) -> bool {
        matches!(
            self,
            Address::Gconf
                | Address::Gstat
                | Address::Slaveconf
                | Address::OtpProg
                | Address::FactoryConf
                | Address::IholdIrun
                | Address::Tpowerdown
                | Address::Tpwmthrs
                | Address::Tcoolthrs
                | Address::Vactual
                | Address::Sgthrs
                | Address::Coolconf
                | Address::Chopconf
                | Address::Pwmconf
        )
    }

    /// Get the raw address value.
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

impl From<Address> for u8 {
    fn from(addr: Address) -> u8 {
        addr as u8
    }
}

/// Microstep resolution setting.
///
/// Number of microsteps per full step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MicrostepResolution {
    /// 256 microsteps per full step (native resolution)
    #[default]
    M256 = 0,
    /// 128 microsteps per full step
    M128 = 1,
    /// 64 microsteps per full step
    M64 = 2,
    /// 32 microsteps per full step
    M32 = 3,
    /// 16 microsteps per full step
    M16 = 4,
    /// 8 microsteps per full step
    M8 = 5,
    /// 4 microsteps per full step
    M4 = 6,
    /// 2 microsteps per full step (half step)
    M2 = 7,
    /// Full step
    M1 = 8,
}

impl MicrostepResolution {
    /// Convert from MRES register value.
    pub fn from_mres(mres: u8) -> Self {
        match mres {
            0 => Self::M256,
            1 => Self::M128,
            2 => Self::M64,
            3 => Self::M32,
            4 => Self::M16,
            5 => Self::M8,
            6 => Self::M4,
            7 => Self::M2,
            8 => Self::M1,
            _ => Self::M256,
        }
    }

    /// Convert to MRES register value.
    pub fn to_mres(self) -> u8 {
        self as u8
    }

    /// Get the number of microsteps.
    pub fn microsteps(self) -> u16 {
        match self {
            Self::M256 => 256,
            Self::M128 => 128,
            Self::M64 => 64,
            Self::M32 => 32,
            Self::M16 => 16,
            Self::M8 => 8,
            Self::M4 => 4,
            Self::M2 => 2,
            Self::M1 => 1,
        }
    }
}

/// Standstill mode when motor current is zero (IHOLD=0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum StandstillMode {
    /// Normal operation
    #[default]
    Normal = 0,
    /// Freewheeling
    Freewheeling = 1,
    /// Coil shorted using LS drivers (strong braking)
    StrongBraking = 2,
    /// Coil shorted using HS drivers (braking)
    Braking = 3,
}

impl StandstillMode {
    /// Convert from register value.
    pub fn from_bits(value: u8) -> Self {
        match value & 0x03 {
            0 => Self::Normal,
            1 => Self::Freewheeling,
            2 => Self::StrongBraking,
            3 => Self::Braking,
            _ => Self::Normal,
        }
    }

    /// Convert to register value.
    pub fn to_bits(self) -> u8 {
        self as u8
    }
}
