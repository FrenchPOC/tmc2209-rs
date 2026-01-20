//! # TMC2209 Driver
//!
//! A `no_std` Rust driver for the TMC2209 stepper motor driver.
//!
//! This crate provides:
//! - Full register definitions for all 24 TMC2209 registers
//! - Type-safe register access with bitfield getters/setters
//! - High-level `Tmc2209` driver struct for UART communication
//! - Blocking API (feature `blocking`)
//! - Async API (feature `async`)
//! - Utility functions for current/velocity calculations
//!
//! ## Features
//!
//! - `blocking` (default): Enable blocking UART API using `embedded_io` traits
//! - `async`: Enable async UART API using `embedded_io_async` traits
//! - `defmt`: Enable `defmt::Format` derives for debugging
//!
//! ## Example
//!
//! ```ignore
//! use tmc2209::{Tmc2209, registers::{IholdIrun, Chopconf, MicrostepResolution}};
//!
//! // Create driver with UART and slave address 0
//! let mut driver = Tmc2209::new(uart, 0);
//!
//! // Check connection
//! if driver.is_connected() {
//!     // Set motor current (run=16, hold=8)
//!     driver.set_current(16, 8, 1)?;
//!
//!     // Set microstep resolution
//!     driver.set_microsteps(MicrostepResolution::M16)?;
//!
//!     // Enable StealthChop for quiet operation
//!     driver.enable_stealthchop()?;
//!
//!     // Start moving at velocity 1000
//!     driver.set_velocity(1000)?;
//! }
//! ```
//!
//! ## Protocol Overview
//!
//! The TMC2209 uses a simple UART protocol at 115200 baud (configurable):
//!
//! - Read request: 4 bytes `[0x05, slave_addr, reg_addr, CRC]`
//! - Write request: 8 bytes `[0x05, slave_addr, reg_addr|0x80, data[3:0], CRC]`
//! - Read response: 8 bytes `[0x05, 0xFF, reg_addr, data[3:0], CRC]`
//!
//! The driver handles echo bytes automatically (TMC2209 echoes all sent data
//! on its single-wire UART interface).

#![no_std]
#![warn(missing_docs)]

pub mod crc;
pub mod datagram;
pub mod driver;
pub mod error;
pub mod registers;
pub mod util;

// Re-export main types at crate root
pub use driver::Tmc2209;
pub use error::Error;

// Re-export commonly used register types
pub use registers::{
    Address, Chopconf, Coolconf, DrvStatus, FactoryConf, Gconf, Gstat, Ifcnt, IholdIrun, Ioin,
    MicrostepResolution, Mscnt, Mscuract, OtpProg, OtpRead, Pwmconf, PwmAuto, PwmScale,
    ReadableRegister, Register, SgResult, Sgthrs, Slaveconf, StandstillMode, Tcoolthrs, Tpowerdown,
    Tpwmthrs, Tstep, Vactual, WritableRegister,
};

// Re-export utility functions
pub use util::{
    calculate_current_settings, cs_to_current, current_to_cs, optimal_vsense, tstep_to_velocity,
    velocity_to_tpwmthrs, velocity_to_vactual, DEFAULT_FCLK, DEFAULT_RSENSE,
};

// Re-export datagram types for advanced usage
pub use datagram::{ReadRequest, ReadResponse, ResponseReader, WriteRequest, MASTER_ADDR, SYNC};
