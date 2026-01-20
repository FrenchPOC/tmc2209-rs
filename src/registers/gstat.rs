//! GSTAT - Global status register (0x01)

use super::{Address, ReadableRegister, Register, WritableRegister};

/// Global status register.
///
/// Contains status flags that indicate reset, driver errors, and undervoltage.
/// Flags can be cleared by writing `1` to the corresponding bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Gstat(u32);

impl Gstat {
    /// Reset flag.
    ///
    /// Indicates that the IC has been reset since the last read.
    /// All registers have been cleared to reset values.
    pub fn reset(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Clear the reset flag by setting it to 1.
    pub fn clear_reset(&mut self) -> &mut Self {
        self.0 |= 1 << 0;
        self
    }

    /// Driver error flag.
    ///
    /// Indicates the driver has been shut down due to overtemperature
    /// or short circuit detection. Read DRV_STATUS for details.
    pub fn drv_err(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Clear the driver error flag by setting it to 1.
    pub fn clear_drv_err(&mut self) -> &mut Self {
        self.0 |= 1 << 1;
        self
    }

    /// Undervoltage on charge pump.
    ///
    /// Indicates an undervoltage condition. The driver is disabled.
    /// This flag is not latched and clears automatically.
    pub fn uv_cp(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Check if any error flags are set.
    pub fn has_errors(&self) -> bool {
        self.drv_err() || self.uv_cp()
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

impl Register for Gstat {
    const ADDRESS: Address = Address::Gstat;
}

impl ReadableRegister for Gstat {}
impl WritableRegister for Gstat {} // Write-clear register

impl From<u32> for Gstat {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Gstat> for u32 {
    fn from(reg: Gstat) -> u32 {
        reg.0
    }
}
