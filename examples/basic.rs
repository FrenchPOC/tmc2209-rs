//! Basic motor control example for TMC2209.
//!
//! This example demonstrates basic motor control operations.
//! It is meant to be adapted for your specific hardware platform.
//!
//! Note: This is a documentation example showing API usage.
//! For a complete working example, you need to integrate with
//! your platform's UART implementation.

// This example requires std for compilation but the library is no_std
#![allow(unused)]

use tmc2209::{MicrostepResolution, Tmc2209};

/// Example function showing basic motor control.
///
/// Replace `YourUartType` with your platform's UART type that implements
/// `embedded_io::Read` and `embedded_io::Write`.
#[cfg(feature = "blocking")]
fn basic_motor_control<U, E>(uart: U) -> Result<(), tmc2209::Error<E>>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{
    // Create driver with slave address 0
    let mut driver = Tmc2209::new(uart, 0);

    // Check if the driver is responding
    if !driver.is_connected() {
        // Handle connection error
        return Err(tmc2209::Error::NoResponse);
    }

    // Clear any previous errors
    driver.clear_gstat()?;

    // Configure motor current
    // IRUN=20 (run current), IHOLD=10 (hold current), IHOLDDELAY=6
    driver.set_current(20, 10, 6)?;

    // Set microstep resolution to 16 with interpolation to 256
    driver.set_microsteps(MicrostepResolution::M16)?;
    driver.set_interpolation(true)?;

    // Enable StealthChop for quiet operation
    driver.enable_stealthchop()?;

    // Enable the driver
    driver.set_enabled(true)?;

    // Start moving the motor forward
    driver.set_velocity(5000)?;

    // Check status
    let status = driver.drv_status()?;

    if status.ot() {
        // Overtemperature shutdown - stop immediately!
        driver.stop()?;
        driver.set_enabled(false)?;
    }

    if status.otpw() {
        // Overtemperature warning - reduce current or add cooling
        println!("Warning: Motor temperature is high");
    }

    // Reverse direction
    driver.set_velocity(-5000)?;

    // Stop the motor
    driver.stop()?;

    Ok(())
}

fn main() {
    println!("TMC2209 Basic Example");
    println!("This example shows API usage patterns.");
    println!("Integrate with your platform's UART for actual use.");
}
