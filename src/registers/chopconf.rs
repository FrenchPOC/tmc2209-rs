//! CHOPCONF - Chopper configuration register (0x6C)

use super::{Address, MicrostepResolution, ReadableRegister, Register, WritableRegister};

/// Chopper configuration register.
///
/// Controls the chopper (current regulation) and microstep settings.
/// This is one of the most important registers for motor tuning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Chopconf(u32);

impl Chopconf {
    /// Default value recommended by Trinamic.
    /// TOFF=3, HSTRT=4, HEND=1, TBL=2, MRES=0 (256 microsteps)
    pub const DEFAULT: u32 = 0x10000053;

    /// Create with default values.
    pub fn new() -> Self {
        Self(Self::DEFAULT)
    }

    /// Get TOFF (0-15).
    ///
    /// Off-time setting controls chopper frequency.
    /// - 0: Driver disabled (all bridges off)
    /// - 1: Driver enabled with minimum off-time
    /// - 2-15: Off-time = 12 + 32*TOFF clocks
    ///
    /// Recommended: 3-5 for most applications.
    pub fn toff(&self) -> u8 {
        (self.0 & 0x0F) as u8
    }

    /// Set TOFF (0-15).
    pub fn set_toff(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !0x0F) | ((value as u32) & 0x0F);
        self
    }

    /// Get HSTRT (0-7).
    ///
    /// Hysteresis start value (adds 1-8 to HEND).
    /// HSTRT + HEND must be <= 16.
    pub fn hstrt(&self) -> u8 {
        ((self.0 >> 4) & 0x07) as u8
    }

    /// Set HSTRT (0-7).
    pub fn set_hstrt(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self
    }

    /// Get HEND (0-15).
    ///
    /// Hysteresis end (low) value.
    /// Sets the hysteresis low point for current chopping.
    /// Effective value: -3 to 12 (register value - 3).
    pub fn hend(&self) -> u8 {
        ((self.0 >> 7) & 0x0F) as u8
    }

    /// Set HEND (0-15).
    pub fn set_hend(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x0F << 7)) | (((value as u32) & 0x0F) << 7);
        self
    }

    /// Get TBL (0-3).
    ///
    /// Comparator blank time select:
    /// - 0: 16 clocks
    /// - 1: 24 clocks
    /// - 2: 36 clocks (recommended)
    /// - 3: 54 clocks
    pub fn tbl(&self) -> u8 {
        ((self.0 >> 15) & 0x03) as u8
    }

    /// Set TBL (0-3).
    pub fn set_tbl(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self
    }

    /// Get VSENSE.
    ///
    /// Sense resistor voltage-based current scaling:
    /// - false: Low sensitivity (high current range)
    /// - true: High sensitivity (low current range, 0.18x)
    pub fn vsense(&self) -> bool {
        (self.0 >> 17) & 1 != 0
    }

    /// Set VSENSE.
    pub fn set_vsense(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 17;
        } else {
            self.0 &= !(1 << 17);
        }
        self
    }

    /// Get MRES - microstep resolution (0-8).
    ///
    /// - 0: 256 microsteps (native)
    /// - 1: 128 microsteps
    /// - 2: 64 microsteps
    /// - 3: 32 microsteps
    /// - 4: 16 microsteps
    /// - 5: 8 microsteps
    /// - 6: 4 microsteps
    /// - 7: 2 microsteps (half step)
    /// - 8: Full step
    pub fn mres(&self) -> u8 {
        ((self.0 >> 24) & 0x0F) as u8
    }

    /// Set MRES - microstep resolution (0-8).
    pub fn set_mres(&mut self, value: u8) -> &mut Self {
        self.0 = (self.0 & !(0x0F << 24)) | (((value as u32) & 0x0F) << 24);
        self
    }

    /// Get the microstep resolution as enum.
    pub fn microstep_resolution(&self) -> MicrostepResolution {
        MicrostepResolution::from_mres(self.mres())
    }

    /// Set the microstep resolution from enum.
    pub fn set_microstep_resolution(&mut self, res: MicrostepResolution) -> &mut Self {
        self.set_mres(res.to_mres())
    }

    /// Get INTPOL.
    ///
    /// Interpolation to 256 microsteps:
    /// - true: Enable interpolation (recommended for MRES < 256)
    /// - false: No interpolation
    pub fn intpol(&self) -> bool {
        (self.0 >> 28) & 1 != 0
    }

    /// Set INTPOL.
    pub fn set_intpol(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 28;
        } else {
            self.0 &= !(1 << 28);
        }
        self
    }

    /// Get DEDGE.
    ///
    /// Enable double edge step pulses:
    /// - true: Step on both rising and falling STEP edges
    /// - false: Step on rising edge only
    pub fn dedge(&self) -> bool {
        (self.0 >> 29) & 1 != 0
    }

    /// Set DEDGE.
    pub fn set_dedge(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 29;
        } else {
            self.0 &= !(1 << 29);
        }
        self
    }

    /// Get DISS2G.
    ///
    /// Disable short to GND protection:
    /// - true: Disable short to GND detection
    /// - false: Enable short to GND detection (default)
    pub fn diss2g(&self) -> bool {
        (self.0 >> 30) & 1 != 0
    }

    /// Set DISS2G.
    pub fn set_diss2g(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 30;
        } else {
            self.0 &= !(1 << 30);
        }
        self
    }

    /// Get DISS2VS.
    ///
    /// Disable short to supply protection:
    /// - true: Disable short to VS detection
    /// - false: Enable short to VS detection (default)
    pub fn diss2vs(&self) -> bool {
        (self.0 >> 31) & 1 != 0
    }

    /// Set DISS2VS.
    pub fn set_diss2vs(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 31;
        } else {
            self.0 &= !(1 << 31);
        }
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

impl Default for Chopconf {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Chopconf {
    const ADDRESS: Address = Address::Chopconf;
}

impl ReadableRegister for Chopconf {}
impl WritableRegister for Chopconf {}

impl From<u32> for Chopconf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Chopconf> for u32 {
    fn from(reg: Chopconf) -> u32 {
        reg.0
    }
}
