//! Sensorless homing example for TMC2209.
//!
//! This example demonstrates how to use StallGuard for sensorless homing,
//! allowing the motor to detect when it hits a mechanical endstop without
//! requiring physical limit switches.

#![allow(unused)]

use tmc2209::{MicrostepResolution, Tmc2209};

/// Perform sensorless homing using StallGuard.
///
/// # Important Notes
///
/// 1. StallGuard works best in SpreadCycle mode
/// 2. The motor must be moving above the TCOOLTHRS velocity
/// 3. Use moderate speeds for reliable detection
/// 4. Tune SGTHRS for your specific motor and mechanics
#[cfg(feature = "blocking")]
fn sensorless_homing<U, E>(uart: U) -> Result<(), tmc2209::Error<E>>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{
    let mut driver = Tmc2209::new(uart, 0);

    // =========================================================================
    // Step 1: Basic Configuration
    // =========================================================================

    // Set appropriate current for homing (usually lower than running current)
    driver.set_current(12, 6, 4)?;

    // Use lower microstep resolution for more reliable stall detection
    driver.set_microsteps(MicrostepResolution::M16)?;

    // =========================================================================
    // Step 2: Configure StallGuard
    // =========================================================================

    // CRITICAL: Use SpreadCycle mode for StallGuard
    driver.enable_spreadcycle()?;

    // Configure stall detection
    driver.configure_stall_detection(50)?;

    // Set TCOOLTHRS - StallGuard is only active when TSTEP < TCOOLTHRS
    driver.set_coolstep_threshold(0xFFFFF)?;

    driver.set_enabled(true)?;

    // =========================================================================
    // Step 3: Perform Homing
    // =========================================================================

    // Move towards the endstop
    let homing_velocity = -2000i32;
    driver.set_velocity(homing_velocity)?;

    // Poll for stall detection
    let max_iterations = 10000u32;

    for _ in 0..max_iterations {
        if driver.is_stalled()? {
            break;
        }
        // Add platform-specific delay here
    }

    // Stop the motor immediately
    driver.stop()?;

    // =========================================================================
    // Step 4: Post-Homing Configuration
    // =========================================================================

    // Switch back to StealthChop for quiet operation
    driver.enable_stealthchop()?;

    // Restore normal running current
    driver.set_current(20, 10, 6)?;

    Ok(())
}

/// Alternative: Home in both directions to find center.
#[cfg(feature = "blocking")]
fn home_to_center<U, E>(uart: U) -> Result<(), tmc2209::Error<E>>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{
    let mut driver = Tmc2209::new(uart, 0);

    // Configure for homing
    driver.set_current(12, 6, 4)?;
    driver.enable_spreadcycle()?;
    driver.configure_stall_detection(50)?;
    driver.set_coolstep_threshold(0xFFFFF)?;
    driver.set_enabled(true)?;

    // Home to minimum position
    driver.set_velocity(-2000)?;
    while !driver.is_stalled()? {
        // Wait or add timeout
    }
    driver.stop()?;
    let _min_position = 0i32;

    // Home to maximum position
    driver.set_velocity(2000)?;
    let mut step_count = 0i32;
    while !driver.is_stalled()? {
        step_count += 1;
    }
    driver.stop()?;
    let max_position = step_count;

    // Move to center
    let _center = max_position / 2;
    driver.set_velocity(-2000)?;
    // Move for half the counted steps
    driver.stop()?;

    Ok(())
}

fn main() {
    println!("TMC2209 Sensorless Homing Example");
    println!("This example shows how to use StallGuard for sensorless homing.");
}
