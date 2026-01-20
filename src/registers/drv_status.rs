//! DRV_STATUS - Driver status register (0x6F)

use super::{Address, ReadableRegister, Register};

/// Driver status register.
///
/// Read-only register containing driver status flags and diagnostic information.
/// Use this to monitor driver health, temperature, and detect faults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DrvStatus(u32);

impl DrvStatus {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get OTPW - overtemperature pre-warning flag.
    ///
    /// True when die temperature exceeds ~120C.
    pub fn otpw(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Get OT - overtemperature shutdown flag.
    ///
    /// True when die temperature exceeds ~150C.
    /// Driver will shut down to protect the chip.
    pub fn ot(&self) -> bool {
        (self.0 >> 1) & 1 != 0
    }

    /// Get S2GA - short to GND indicator phase A.
    pub fn s2ga(&self) -> bool {
        (self.0 >> 2) & 1 != 0
    }

    /// Get S2GB - short to GND indicator phase B.
    pub fn s2gb(&self) -> bool {
        (self.0 >> 3) & 1 != 0
    }

    /// Get S2VSA - short to supply indicator phase A.
    pub fn s2vsa(&self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    /// Get S2VSB - short to supply indicator phase B.
    pub fn s2vsb(&self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    /// Get OLA - open load indicator phase A.
    ///
    /// True when no current flows through phase A despite driver being enabled.
    pub fn ola(&self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Get OLB - open load indicator phase B.
    ///
    /// True when no current flows through phase B despite driver being enabled.
    pub fn olb(&self) -> bool {
        (self.0 >> 7) & 1 != 0
    }

    /// Get T120 - temperature threshold flag.
    ///
    /// True when temperature exceeds 120C.
    pub fn t120(&self) -> bool {
        (self.0 >> 8) & 1 != 0
    }

    /// Get T143 - temperature threshold flag.
    ///
    /// True when temperature exceeds 143C.
    pub fn t143(&self) -> bool {
        (self.0 >> 9) & 1 != 0
    }

    /// Get T150 - temperature threshold flag.
    ///
    /// True when temperature exceeds 150C.
    pub fn t150(&self) -> bool {
        (self.0 >> 10) & 1 != 0
    }

    /// Get T157 - temperature threshold flag.
    ///
    /// True when temperature exceeds 157C.
    pub fn t157(&self) -> bool {
        (self.0 >> 11) & 1 != 0
    }

    /// Get CS_ACTUAL - actual current scale (0-31).
    ///
    /// The actual motor current scaling as used during operation.
    /// Reflects CoolStep adjustments if enabled.
    pub fn cs_actual(&self) -> u8 {
        ((self.0 >> 16) & 0x1F) as u8
    }

    /// Get STEALTH - StealthChop indicator.
    ///
    /// True when motor is in StealthChop mode.
    /// False when in SpreadCycle mode.
    pub fn stealth(&self) -> bool {
        (self.0 >> 30) & 1 != 0
    }

    /// Get STST - standstill indicator.
    ///
    /// True when motor is in standstill (no step pulses for >2^20 clocks).
    pub fn stst(&self) -> bool {
        (self.0 >> 31) & 1 != 0
    }

    /// Check if any short circuit condition is detected.
    pub fn short_detected(&self) -> bool {
        self.s2ga() || self.s2gb() || self.s2vsa() || self.s2vsb()
    }

    /// Check if any open load condition is detected.
    pub fn open_load_detected(&self) -> bool {
        self.ola() || self.olb()
    }

    /// Check if overtemperature warning or shutdown is active.
    pub fn overtemperature(&self) -> bool {
        self.otpw() || self.ot()
    }

    /// Check if any error condition is present.
    pub fn has_error(&self) -> bool {
        self.short_detected() || self.ot()
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

impl Default for DrvStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for DrvStatus {
    const ADDRESS: Address = Address::DrvStatus;
}

impl ReadableRegister for DrvStatus {}

impl From<u32> for DrvStatus {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<DrvStatus> for u32 {
    fn from(reg: DrvStatus) -> u32 {
        reg.0
    }
}
