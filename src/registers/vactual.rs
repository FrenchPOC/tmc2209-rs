//! VACTUAL - UART velocity control register (0x22)

use super::{Address, Register, WritableRegister};

/// UART velocity control register.
///
/// Allows moving the motor by UART control instead of step/dir pins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Vactual(u32);

impl Vactual {
    /// Create a new Vactual with velocity 0 (motor stopped, STEP input active).
    pub fn new() -> Self {
        Self(0)
    }

    /// Get the velocity value as a signed 24-bit integer.
    ///
    /// - 0: Normal operation, driver reacts to STEP input
    /// - Other: Motor moves with given velocity in microsteps/t
    ///
    /// With internal 12MHz clock: velocity_usteps_per_sec ≈ value × 0.715
    pub fn velocity(&self) -> i32 {
        // Sign-extend from 24 bits
        let val = self.0 & 0xFFFFFF;
        if val & 0x800000 != 0 {
            (val | 0xFF000000) as i32
        } else {
            val as i32
        }
    }

    /// Set the velocity value.
    ///
    /// Positive values = forward, negative = reverse.
    /// Range: -(2^23-1) to (2^23-1)
    pub fn set_velocity(&mut self, value: i32) -> &mut Self {
        self.0 = (value as u32) & 0xFFFFFF;
        self
    }

    /// Stop the motor (set velocity to 0).
    ///
    /// This re-enables STEP input control.
    pub fn stop(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    /// Check if UART velocity control is active.
    pub fn is_active(&self) -> bool {
        self.0 != 0
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

impl Register for Vactual {
    const ADDRESS: Address = Address::Vactual;
}

impl WritableRegister for Vactual {}

impl From<u32> for Vactual {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Vactual> for u32 {
    fn from(reg: Vactual) -> u32 {
        reg.0
    }
}
