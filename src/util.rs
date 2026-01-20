//! Utility functions for TMC2209 calculations.
//!
//! This module provides helper functions for common calculations like
//! RMS current, velocity conversions, etc.

/// Default sense resistor value in ohms (common value).
pub const DEFAULT_RSENSE: f32 = 0.11;

/// Internal voltage reference for current sensing (in volts).
pub const VREF: f32 = 0.325;

/// Round a f32 value to the nearest integer (no_std compatible).
#[inline]
fn round_f32(x: f32) -> f32 {
    // Simple rounding: add 0.5 and truncate for positive, subtract 0.5 for negative
    if x >= 0.0 {
        (x + 0.5) as i32 as f32
    } else {
        (x - 0.5) as i32 as f32
    }
}

/// Calculate the CS (current scale) value for a given RMS current.
///
/// # Arguments
///
/// * `rms_current_ma` - Desired RMS motor current in milliamps
/// * `rsense` - Sense resistor value in ohms
/// * `vsense` - VSENSE bit setting (true = high sensitivity, false = low)
///
/// # Returns
///
/// The CS value (0-31) to use in IRUN or IHOLD, or None if current is too high.
///
/// # Formula
///
/// For VSENSE=0 (low sensitivity):
///   I_RMS = (CS + 1) / 32 * V_FS / (sqrt(2) * R_SENSE)
///   where V_FS = 0.325V
///
/// For VSENSE=1 (high sensitivity):
///   V_FS = 0.180V
///
/// Solving for CS:
///   CS = (I_RMS * sqrt(2) * R_SENSE * 32 / V_FS) - 1
pub fn current_to_cs(rms_current_ma: u16, rsense: f32, vsense: bool) -> Option<u8> {
    let rms_current = rms_current_ma as f32 / 1000.0;
    let vfs = if vsense { 0.180 } else { 0.325 };

    // sqrt(2) ≈ 1.41421356
    let sqrt2 = 1.41421356f32;
    let cs_float = (rms_current * sqrt2 * rsense * 32.0 / vfs) - 1.0;

    if cs_float < 0.0 {
        Some(0)
    } else if cs_float > 31.0 {
        None // Current too high for this setting
    } else {
        Some(round_f32(cs_float) as u8)
    }
}

/// Calculate the RMS current for a given CS value.
///
/// # Arguments
///
/// * `cs` - Current scale value (0-31)
/// * `rsense` - Sense resistor value in ohms
/// * `vsense` - VSENSE bit setting (true = high sensitivity, false = low)
///
/// # Returns
///
/// The RMS current in milliamps.
pub fn cs_to_current(cs: u8, rsense: f32, vsense: bool) -> u16 {
    let vfs = if vsense { 0.180 } else { 0.325 };
    let sqrt2 = 1.41421356f32;

    let cs = (cs.min(31) + 1) as f32;
    let rms_current = cs / 32.0 * vfs / (sqrt2 * rsense);

    round_f32(rms_current * 1000.0) as u16
}

/// Determine optimal VSENSE setting for a given RMS current.
///
/// Returns true if high sensitivity (VSENSE=1) should be used.
/// High sensitivity is preferred for lower currents for better precision.
///
/// # Arguments
///
/// * `rms_current_ma` - Desired RMS current in milliamps
/// * `rsense` - Sense resistor value in ohms
pub fn optimal_vsense(rms_current_ma: u16, rsense: f32) -> bool {
    // Calculate max current for VSENSE=1 (high sensitivity)
    let max_current_vsense1 = cs_to_current(31, rsense, true);

    // Use high sensitivity if desired current is within range
    rms_current_ma <= max_current_vsense1
}

/// Calculate CS and VSENSE for a target RMS current.
///
/// This function automatically selects the optimal VSENSE setting
/// and returns the corresponding CS value.
///
/// # Arguments
///
/// * `rms_current_ma` - Desired RMS current in milliamps
/// * `rsense` - Sense resistor value in ohms
///
/// # Returns
///
/// A tuple of (CS, VSENSE), or None if current is too high.
pub fn calculate_current_settings(rms_current_ma: u16, rsense: f32) -> Option<(u8, bool)> {
    // Try high sensitivity first (better for lower currents)
    let vsense = optimal_vsense(rms_current_ma, rsense);
    let cs = current_to_cs(rms_current_ma, rsense, vsense)?;

    Some((cs, vsense))
}

/// Convert velocity in steps/second to VACTUAL register value.
///
/// # Arguments
///
/// * `steps_per_sec` - Velocity in full steps per second
/// * `microsteps` - Microstep resolution (1, 2, 4, 8, 16, 32, 64, 128, or 256)
/// * `fclk` - Internal clock frequency in Hz (typically 12 MHz)
///
/// # Returns
///
/// The VACTUAL register value.
///
/// # Formula
///
/// VACTUAL = velocity * 2^23 / fCLK
/// where velocity is in microsteps/second
pub fn velocity_to_vactual(steps_per_sec: f32, microsteps: u16, fclk: u32) -> i32 {
    let microsteps_per_sec = steps_per_sec * microsteps as f32;
    let vactual = microsteps_per_sec * 8388608.0 / fclk as f32; // 2^23 = 8388608
    round_f32(vactual) as i32
}

/// Convert TSTEP register value to velocity in steps/second.
///
/// # Arguments
///
/// * `tstep` - TSTEP register value
/// * `microsteps` - Microstep resolution
/// * `fclk` - Internal clock frequency in Hz (typically 12 MHz)
///
/// # Returns
///
/// Velocity in full steps per second, or None if tstep is 0.
pub fn tstep_to_velocity(tstep: u32, microsteps: u16, fclk: u32) -> Option<f32> {
    if tstep == 0 {
        return None;
    }

    // TSTEP is the time between microsteps in clock cycles
    let microsteps_per_sec = fclk as f32 / tstep as f32;
    let steps_per_sec = microsteps_per_sec / microsteps as f32;

    Some(steps_per_sec)
}

/// Calculate TPWMTHRS for a given velocity threshold.
///
/// TPWMTHRS sets the upper velocity limit for StealthChop.
/// Above this velocity, the driver switches to SpreadCycle.
///
/// # Arguments
///
/// * `steps_per_sec` - Velocity threshold in full steps per second
/// * `microsteps` - Microstep resolution
/// * `fclk` - Internal clock frequency in Hz
///
/// # Returns
///
/// The TPWMTHRS register value.
pub fn velocity_to_tpwmthrs(steps_per_sec: f32, microsteps: u16, fclk: u32) -> u32 {
    if steps_per_sec <= 0.0 {
        return 0xFFFFF; // Maximum value (always StealthChop)
    }

    let microsteps_per_sec = steps_per_sec * microsteps as f32;
    let tstep = fclk as f32 / microsteps_per_sec;

    (tstep as u32).min(0xFFFFF)
}

/// Default TMC2209 internal clock frequency (12 MHz).
pub const DEFAULT_FCLK: u32 = 12_000_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_to_cs() {
        // With 0.11 ohm sense resistor, VSENSE=0
        // Max current ≈ 2.1A RMS
        let cs = current_to_cs(1000, 0.11, false);
        assert!(cs.is_some());

        // Very high current should return None
        let cs = current_to_cs(5000, 0.11, false);
        assert!(cs.is_none());
    }

    #[test]
    fn test_cs_to_current() {
        // CS=31 with 0.11 ohm, VSENSE=0 should give max current
        let current = cs_to_current(31, 0.11, false);
        assert!(current > 2000); // Should be around 2.1A
    }

    #[test]
    fn test_velocity_conversion() {
        // 100 steps/sec with 256 microsteps at 12MHz
        let vactual = velocity_to_vactual(100.0, 256, 12_000_000);
        assert!(vactual > 0);

        // Convert back from TSTEP
        let tstep = 12_000_000 / (100 * 256); // Approximate
        let velocity = tstep_to_velocity(tstep as u32, 256, 12_000_000);
        assert!(velocity.is_some());
    }
}
