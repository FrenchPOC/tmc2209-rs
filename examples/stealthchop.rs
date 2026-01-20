//! StealthChop configuration example for TMC2209.
//!
//! This example shows how to configure StealthChop for quiet motor operation,
//! including automatic switching to SpreadCycle at high velocities.

#![allow(unused)]

use tmc2209::{MicrostepResolution, Tmc2209};

/// Configure StealthChop with velocity-based mode switching.
#[cfg(feature = "blocking")]
fn configure_stealthchop<U, E>(uart: U) -> Result<(), tmc2209::Error<E>>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{
    let mut driver = Tmc2209::new(uart, 0);

    // Basic setup
    driver.set_current(16, 8, 4)?;
    driver.set_microsteps(MicrostepResolution::M256)?;

    // =========================================================================
    // StealthChop Configuration
    // =========================================================================

    // Enable StealthChop mode (this is the default)
    driver.enable_stealthchop()?;

    // Configure StealthChop PWM parameters
    // - pwm_ofs: Base PWM amplitude (0-255)
    // - pwm_grad: PWM amplitude gradient (0-255)
    // - autoscale: Enable automatic current scaling
    // - autograd: Enable automatic gradient adaptation
    driver.configure_stealthchop(
        36,   // pwm_ofs (default)
        14,   // pwm_grad (default)
        true, // autoscale enabled (recommended)
        true, // autograd enabled (recommended)
    )?;

    // =========================================================================
    // Velocity-Based Mode Switching
    // =========================================================================

    // Set StealthChop/SpreadCycle velocity threshold (TPWMTHRS)
    // Below this TSTEP value, StealthChop is used
    // Above this velocity (lower TSTEP), SpreadCycle is used
    driver.set_stealthchop_threshold(500)?;

    // =========================================================================
    // Chopper Configuration
    // =========================================================================

    // Fine-tune chopper parameters for your motor
    driver.configure_chopper(
        3, // toff
        4, // hstrt
        1, // hend
        2, // tbl
    )?;

    // =========================================================================
    // Current Sense Configuration
    // =========================================================================

    // For high-current motors, use low sensitivity VSENSE
    driver.set_vsense(false)?;

    // Enable the driver
    driver.set_enabled(true)?;

    // =========================================================================
    // Monitor StealthChop Status
    // =========================================================================

    // Start moving
    driver.set_velocity(1000)?;

    // Check if StealthChop or SpreadCycle is active
    let _is_stealth = driver.is_stealthchop_active()?;

    // Check actual current being used
    let _actual_cs = driver.actual_current_scale()?;

    Ok(())
}

fn main() {
    println!("TMC2209 StealthChop Example");
    println!("This example shows how to configure silent operation.");
}
