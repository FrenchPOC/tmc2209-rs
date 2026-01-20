# tmc2209-rs

A `no_std` Rust driver for the **TMC2209** stepper motor driver, communicating via single-wire UART.

[![Crates.io](https://img.shields.io/crates/v/tmc2209.svg)](https://crates.io/crates/tmc2209)
[![Documentation](https://docs.rs/tmc2209/badge.svg)](https://docs.rs/tmc2209)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Full register support**: All 24 TMC2209 registers with type-safe bitfield accessors
- **Blocking and async APIs**: Choose based on your platform needs
- **Platform-agnostic**: Uses `embedded-io` traits for maximum portability
- **`no_std` compatible**: Perfect for embedded systems
- **Multi-driver support**: Daisy-chain up to 4 drivers (addresses 0-3)
- **Utility functions**: Current calculation, velocity conversion helpers
- **Optional `defmt` support**: For embedded debugging

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
tmc2209 = "0.1"
```

### Basic Usage (Blocking)

```rust
use tmc2209::{Tmc2209, MicrostepResolution};

// Create driver with UART peripheral and slave address 0
let mut driver = Tmc2209::new(uart, 0);

// Check connection
if driver.is_connected() {
    // Configure motor current (IRUN=16, IHOLD=8, delay=1)
    driver.set_current(16, 8, 1)?;

    // Set 16 microsteps with interpolation to 256
    driver.set_microsteps(MicrostepResolution::M16)?;

    // Enable StealthChop for silent operation
    driver.enable_stealthchop()?;

    // Move motor using UART velocity control
    driver.set_velocity(5000)?;

    // Check status
    let status = driver.drv_status()?;
    if status.ot() {
        // Overtemperature shutdown!
    }
}
```

### Async Usage

Enable the `async` feature:

```toml
[dependencies]
tmc2209 = { version = "0.1", default-features = false, features = ["async"] }
```

```rust
use tmc2209::Tmc2209;

let mut driver = Tmc2209::new(uart, 0);

// Async methods have _async suffix
driver.set_current_async(16, 8, 1).await?;
driver.set_velocity_async(5000).await?;
let status = driver.drv_status_async().await?;
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `blocking` | Yes | Enable blocking API using `embedded-io` |
| `async` | No | Enable async API using `embedded-io-async` |
| `defmt` | No | Enable `defmt::Format` for debugging |

## UART Configuration

The TMC2209 uses a single-wire UART interface:

- **Baud rate**: 115200 (default, configurable via OTP)
- **Format**: 8N1 (8 data bits, no parity, 1 stop bit)
- **Mode**: Half-duplex (single wire for TX and RX)

### Hardware Connection

```
MCU TX ──┬── TMC2209 PDN_UART
         │
MCU RX ──┘
```

Connect both TX and RX to PDN_UART through appropriate level shifting if needed.
A 1K resistor in series with TX is recommended.

## Register Access

### Type-Safe Register Access

```rust
use tmc2209::{Tmc2209, Chopconf, IholdIrun, DrvStatus};

// Read a register
let chopconf: Chopconf = driver.read_register()?;
println!("MRES: {}", chopconf.mres());
println!("TOFF: {}", chopconf.toff());

// Modify and write back
let mut chopconf = driver.read_register::<Chopconf>()?;
chopconf.set_mres(4)      // 16 microsteps
        .set_intpol(true) // Enable interpolation
        .set_toff(3);     // Enable driver
driver.write_register(&chopconf)?;

// Create a new register value
let mut irun = IholdIrun::new();
irun.set_irun(20)
    .set_ihold(10)
    .set_iholddelay(6);
driver.write_register(&irun)?;
```

### Raw Register Access

```rust
// Read by address
let value = driver.read_raw(0x6F)?; // DRV_STATUS

// Write by address
driver.write_raw(0x10, 0x00071010)?; // IHOLD_IRUN
```

## Available Registers

| Address | Name | Access | Description |
|---------|------|--------|-------------|
| 0x00 | GCONF | RW | Global configuration |
| 0x01 | GSTAT | R+WC | Global status flags |
| 0x02 | IFCNT | R | Interface transmission counter |
| 0x03 | SLAVECONF | W | UART slave configuration |
| 0x04 | OTP_PROG | W | OTP programming |
| 0x05 | OTP_READ | R | OTP memory read |
| 0x06 | IOIN | R | Input pin states |
| 0x07 | FACTORY_CONF | RW | Factory configuration |
| 0x10 | IHOLD_IRUN | W | Hold/run current settings |
| 0x11 | TPOWERDOWN | W | Power down delay |
| 0x12 | TSTEP | R | Measured step time |
| 0x13 | TPWMTHRS | W | StealthChop velocity threshold |
| 0x14 | TCOOLTHRS | W | CoolStep velocity threshold |
| 0x22 | VACTUAL | W | UART velocity control |
| 0x40 | SGTHRS | W | StallGuard threshold |
| 0x41 | SG_RESULT | R | StallGuard result |
| 0x42 | COOLCONF | W | CoolStep configuration |
| 0x6A | MSCNT | R | Microstep counter |
| 0x6B | MSCURACT | R | Microstep current |
| 0x6C | CHOPCONF | RW | Chopper configuration |
| 0x6F | DRV_STATUS | R | Driver status |
| 0x70 | PWMCONF | RW | StealthChop PWM configuration |
| 0x71 | PWM_SCALE | R | PWM scaling result |
| 0x72 | PWM_AUTO | R | Automatic PWM values |

## Convenience Methods

### Motor Control

```rust
// Enable/disable driver
driver.set_enabled(true)?;

// Set velocity (VACTUAL-based motion)
driver.set_velocity(1000)?;  // Forward
driver.set_velocity(-1000)?; // Reverse
driver.stop()?;              // Stop

// Current control
driver.set_current(run, hold, delay)?;

// Microstep resolution
driver.set_microsteps(MicrostepResolution::M16)?;
```

### Mode Selection

```rust
// Silent operation
driver.enable_stealthchop()?;

// High-torque operation
driver.enable_spreadcycle()?;

// CoolStep (adaptive current)
driver.enable_coolstep(4, 2)?; // semin=4, semax=2

// Sensorless homing
driver.configure_stall_detection(50)?;
```

### Status Monitoring

```rust
let status = driver.drv_status()?;

// Temperature
if status.otpw() { /* Warning >120C */ }
if status.ot() { /* Shutdown >150C */ }

// Faults
if status.short_detected() { /* Short circuit */ }
if status.open_load_detected() { /* Open coil */ }

// Motion
if status.stst() { /* Motor stopped */ }
if status.stealth() { /* StealthChop active */ }

// Current
let actual_current = status.cs_actual(); // 0-31
```

## Utility Functions

```rust
use tmc2209::{calculate_current_settings, velocity_to_vactual, DEFAULT_RSENSE};

// Calculate CS and VSENSE for target current
let (cs, vsense) = calculate_current_settings(800, DEFAULT_RSENSE)?; // 800mA

// Convert velocity to VACTUAL
let vactual = velocity_to_vactual(
    100.0,    // 100 steps/second
    256,      // 256 microsteps
    12_000_000 // 12MHz clock
);
```

## Multi-Driver Setup (Daisy Chain)

```rust
// Each driver needs a unique address (0-3)
let mut driver0 = Tmc2209::new(&mut uart, 0);
let mut driver1 = Tmc2209::new(&mut uart, 1);
let mut driver2 = Tmc2209::new(&mut uart, 2);
let mut driver3 = Tmc2209::new(&mut uart, 3);

// Configure addresses via MS1/MS2 pins on hardware:
// MS1=0, MS2=0 -> Address 0
// MS1=1, MS2=0 -> Address 1
// MS1=0, MS2=1 -> Address 2
// MS1=1, MS2=1 -> Address 3
```

## Examples

See the [`examples/`](examples/) directory for platform-specific examples:

- `basic.rs` - Basic motor control
- `stealthchop.rs` - Silent operation setup
- `sensorless_homing.rs` - Stall detection for homing

## Protocol Details

The TMC2209 uses a simple datagram-based UART protocol:

```
Read Request (4 bytes):
┌──────┬────────────┬──────────┬─────┐
│ 0x05 │ Slave Addr │ Reg Addr │ CRC │
└──────┴────────────┴──────────┴─────┘

Write Request (8 bytes):
┌──────┬────────────┬──────────────┬───────────────┬─────┐
│ 0x05 │ Slave Addr │ Reg Addr|0x80│ Data (32-bit) │ CRC │
└──────┴────────────┴──────────────┴───────────────┴─────┘

Read Response (8 bytes):
┌──────┬──────┬──────────┬───────────────┬─────┐
│ 0x05 │ 0xFF │ Reg Addr │ Data (32-bit) │ CRC │
└──────┴──────┴──────────┴───────────────┴─────┘
```

CRC-8 polynomial: 0x07 (SAE J1850)

## Related Resources

- [TMC2209 Datasheet](https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC2209_Datasheet_V103.pdf)
- [TMC-API Reference](https://github.com/trinamic/TMC-API)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
