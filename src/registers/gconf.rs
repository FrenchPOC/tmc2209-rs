//! GCONF - Global configuration register (0x00)

use super::{Address, ReadableRegister, Register, WritableRegister};

/// Global configuration register.
///
/// Controls general driver settings including analog current scaling,
/// internal/external sense resistors, chopper mode selection, and more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Gconf(u32);

impl Gconf {
    /// Create a new GCONF with default value.
    ///
    /// Default: 0x00000040 (pdn_disable=1 for UART operation)
    pub fn new() -> Self {
        Self(0x00000040)
    }

    /// I_scale_analog: Use voltage supplied to VREF as current reference.
    ///
    /// - `true`: Use external VREF voltage
    /// - `false`: Use internal reference derived from 5VOUT
    pub fn i_scale_analog(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set I_scale_analog.
    pub fn set_i_scale_analog(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 0;
        } else {
            self.0 &= !(1 << 0);
        }
        self
    }

    /// Internal Rsense: Use internal sense resistors.
    ///
    /// - `true`: Use internal sense resistors
    /// - `false`: Use external sense resistors on BRA/BRB pins
    pub fn internal_rsense(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Set internal_rsense.
    pub fn set_internal_rsense(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 1;
        } else {
            self.0 &= !(1 << 1);
        }
        self
    }

    /// Enable SpreadCycle mode.
    ///
    /// - `true`: SpreadCycle mode enabled
    /// - `false`: StealthChop PWM mode enabled (if configured)
    pub fn en_spreadcycle(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Set en_spreadcycle.
    pub fn set_en_spreadcycle(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 2;
        } else {
            self.0 &= !(1 << 2);
        }
        self
    }

    /// Shaft: Inverse motor direction.
    ///
    /// - `true`: Inverse direction
    /// - `false`: Normal direction
    pub fn shaft(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Set shaft (motor direction).
    pub fn set_shaft(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 3;
        } else {
            self.0 &= !(1 << 3);
        }
        self
    }

    /// Index output shows OTPW (overtemperature prewarning).
    ///
    /// - `true`: INDEX outputs OTPW
    /// - `false`: INDEX outputs first microstep position
    pub fn index_otpw(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Set index_otpw.
    pub fn set_index_otpw(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 4;
        } else {
            self.0 &= !(1 << 4);
        }
        self
    }

    /// Index output shows step pulses.
    ///
    /// - `true`: INDEX shows step pulses (toggles on each step)
    /// - `false`: INDEX as selected by index_otpw
    pub fn index_step(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Set index_step.
    pub fn set_index_step(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 5;
        } else {
            self.0 &= !(1 << 5);
        }
        self
    }

    /// PDN_UART disable.
    ///
    /// **Must be set to `true` when using UART interface.**
    ///
    /// - `true`: PDN_UART input disabled, UART interface enabled
    /// - `false`: PDN_UART controls standstill current reduction
    pub fn pdn_disable(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Set pdn_disable.
    pub fn set_pdn_disable(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 6;
        } else {
            self.0 &= !(1 << 6);
        }
        self
    }

    /// Microstep resolution selected by MRES register.
    ///
    /// - `true`: MRES bits in CHOPCONF select resolution
    /// - `false`: MS1/MS2 pins select resolution
    pub fn mstep_reg_select(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Set mstep_reg_select.
    pub fn set_mstep_reg_select(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 7;
        } else {
            self.0 &= !(1 << 7);
        }
        self
    }

    /// Multistep filter enable.
    ///
    /// - `true`: Step pulse optimization enabled for >750Hz
    /// - `false`: Disabled
    pub fn multistep_filt(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Set multistep_filt.
    pub fn set_multistep_filt(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 8;
        } else {
            self.0 &= !(1 << 8);
        }
        self
    }

    /// Test mode (do not use).
    ///
    /// **Must be set to `false` for normal operation.**
    pub fn test_mode(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Set test_mode.
    pub fn set_test_mode(&mut self, value: bool) -> &mut Self {
        if value {
            self.0 |= 1 << 9;
        } else {
            self.0 &= !(1 << 9);
        }
        self
    }

    /// DIAG0 output shows stall (StallGuard).
    ///
    /// When enabled, DIAG0 (active low) indicates a stall condition.
    /// Used for sensorless homing.
    ///
    /// Note: This is an alias that combines the DIAG functionality.
    /// The TMC2209 uses the DIAG pin for multiple purposes.
    pub fn diag0_stall(&self) -> bool {
        // In TMC2209, stall is indicated via DIAG when configured
        // This is controlled by enabling the proper mode
        self.0 & (1 << 4) == 0 && self.0 & (1 << 5) == 0
    }

    /// Enable DIAG0 output for stall indication.
    ///
    /// This configures the DIAG pin to output stall detection
    /// for sensorless homing applications.
    pub fn set_diag0_stall(&mut self, value: bool) -> &mut Self {
        if value {
            // Clear index_otpw and index_step to enable stall output
            self.0 &= !(1 << 4);
            self.0 &= !(1 << 5);
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

impl Register for Gconf {
    const ADDRESS: Address = Address::Gconf;
}

impl ReadableRegister for Gconf {}
impl WritableRegister for Gconf {}

impl From<u32> for Gconf {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Gconf> for u32 {
    fn from(reg: Gconf) -> u32 {
        reg.0
    }
}
