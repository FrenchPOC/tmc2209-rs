//! MSCURACT - Microstep current register (0x6B)

use super::{Address, ReadableRegister, Register};

/// Microstep current register.
///
/// Read-only register containing the actual microstep currents for both
/// coils A and B. Values are signed 9-bit values in range -255 to +255.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Mscuract(u32);

impl Mscuract {
    /// Create with default value (0).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get CUR_A - actual current for coil A.
    ///
    /// Returns a signed 9-bit value (-255 to +255).
    /// Represents the actual motor current for coil A.
    pub fn cur_a(&self) -> i16 {
        let raw = (self.0 & 0x1FF) as u16;
        // Sign-extend from 9-bit to 16-bit
        if raw & 0x100 != 0 {
            (raw | 0xFE00) as i16
        } else {
            raw as i16
        }
    }

    /// Get CUR_B - actual current for coil B.
    ///
    /// Returns a signed 9-bit value (-255 to +255).
    /// Represents the actual motor current for coil B.
    pub fn cur_b(&self) -> i16 {
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

impl Default for Mscuract {
    fn default() -> Self {
        Self::new()
    }
}

impl Register for Mscuract {
    const ADDRESS: Address = Address::Mscuract;
}

impl ReadableRegister for Mscuract {}

impl From<u32> for Mscuract {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Mscuract> for u32 {
    fn from(reg: Mscuract) -> u32 {
        reg.0
    }
}
