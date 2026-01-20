# AGENTS.md - Development Guide for tmc2209-rs

This guide is for AI coding agents working on the TMC2209 Rust driver crate.

## Repository Info

- **Name**: tmc2209-rs
- **License**: MIT
- **Repository**: https://github.com/FrenchPOC/tmc2209-rs
- **Type**: `no_std` embedded Rust library
- **Rust Version**: 1.75+

## Build Commands

```bash
# Check compilation with all features
cargo check --all-features

# Build library
cargo build --all-features

# Build examples (won't run without hardware)
cargo build --examples --all-features

# Run all tests
cargo test --all-features

# Run a single test
cargo test test_name --all-features

# Run tests in a specific module
cargo test crc::tests --all-features

# Build documentation
cargo doc --all-features --no-deps --open

# Check formatting
cargo fmt -- --check

# Run clippy linter
cargo clippy --all-features -- -D warnings

# Verify package before publishing
cargo publish --dry-run
```

## Code Style Guidelines

### Module Structure

- **`no_std` compatible**: Never use `std`, use `core` only
- **Module organization**: Public modules at crate root (crc, datagram, driver, error, registers, util)
- **Re-exports**: Common types re-exported in `lib.rs` for convenience
- **Feature gates**: Use `#[cfg(feature = "...")]` for blocking/async/defmt

### Imports

**Order**:
1. Standard/core library (never `std`, only `core`)
2. External crates
3. Crate modules (use `crate::`)
4. Super/self imports

**Example**:
```rust
use crate::datagram::{ReadRequest, ReadResponse};
use crate::error::Error;
use crate::registers::{Chopconf, DrvStatus, ReadableRegister};
```

**Rules**:
- Group imports logically
- Use explicit imports (avoid `use crate::registers::*`)
- Sort alphabetically within groups

### Types and Naming

**Conventions**:
- **Structs**: PascalCase, descriptive (`Tmc2209`, `ReadRequest`, `Chopconf`)
- **Functions**: snake_case, verb-first for actions (`read_register`, `set_current`)
- **Constants**: SCREAMING_SNAKE_CASE (`DEFAULT`, `SYNC`, `MAX_VALUE`)
- **Type parameters**: Single uppercase letter (`U`, `E`, `R`)
- **Lifetimes**: Lowercase letter with apostrophe (`'a`, `'b`)

**Register Structs Pattern**:
```rust
/// Register documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RegName(u32);

impl RegName {
    pub fn new() -> Self { Self(default_value) }
    pub fn field_name(&self) -> Type { /* extract bits */ }
    pub fn set_field_name(&mut self, value: Type) -> &mut Self { /* set bits */ self }
    pub fn raw(&self) -> u32 { self.0 }
    pub fn from_raw(value: u32) -> Self { Self(value) }
}

impl Register for RegName {
    const ADDRESS: Address = Address::RegName;
}
```

**Driver Method Pattern**:
- Blocking methods: `method_name()`
- Async methods: `method_name_async()`
- Both follow same signature pattern

### Documentation

**Required**:
- `//!` module-level docs at top of every file
- `///` doc comments on all public items
- `# Arguments`, `# Returns`, `# Example` sections where appropriate
- `# Errors` for fallible functions
- `# Panics` if function can panic
- `# Safety` for unsafe code

**Example**:
```rust
/// Set the motor currents.
///
/// # Arguments
///
/// * `run_current` - Run current (0-31)
/// * `hold_current` - Hold current (0-31)
/// * `hold_delay` - Delay before reducing to hold current (0-15)
pub fn set_current(&mut self, run_current: u8, hold_current: u8, hold_delay: u8) -> Result<(), Error<E>>
```

### Error Handling

**Never panic in library code** (except in constructors for invalid invariants):
- Use `Result<T, Error<E>>` for fallible operations
- The `Error<E>` type is generic over UART error type
- Map UART errors: `.map_err(Error::Uart)?`
- Create specific error variants, not catch-all

**Pattern**:
```rust
pub fn operation(&mut self) -> Result<Value, Error<E>> {
    let data = self.uart.read(...).map_err(Error::Uart)?;
    // validate
    if !valid {
        return Err(Error::InvalidSync);
    }
    Ok(result)
}
```

### Register Bitfield Access

**Bit manipulation pattern**:
```rust
// Get field (bits [high:low])
pub fn field(&self) -> u8 {
    ((self.0 >> low) & mask) as u8
}

// Set field (builder pattern, returns &mut Self)
pub fn set_field(&mut self, value: u8) -> &mut Self {
    self.0 = (self.0 & !(mask << low)) | (((value as u32) & mask) << low);
    self
}

// Boolean flags
pub fn flag(&self) -> bool {
    (self.0 >> bit) & 1 != 0
}

pub fn set_flag(&mut self, value: bool) -> &mut Self {
    if value {
        self.0 |= 1 << bit;
    } else {
        self.0 &= !(1 << bit);
    }
    self
}
```

### Testing

- Test modules use `#[cfg(test)] mod tests`
- Each test file tests its own module
- Use descriptive test names: `test_verb_subject`
- Examples: `test_crc_read_request`, `test_current_to_cs`

### No-Std Compatibility

**Never use**:
- `std::` (use `core::` instead)
- `f32::round()` (implement custom `round_f32()` helper)
- Heap allocation (`Box`, `Vec`, `String`)
- Threads, file I/O, networking

**Custom implementations needed**:
- Floating point rounding (see `util.rs::round_f32`)
- Fixed-size buffers only

### Feature Gates

**Pattern**:
```rust
#[cfg(feature = "blocking")]
impl<U, E> Tmc2209<U>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{ /* blocking methods */ }

#[cfg(feature = "async")]
impl<U, E> Tmc2209<U>
where
    U: embedded_io_async::Read<Error = E> + embedded_io_async::Write<Error = E>,
{ /* async methods */ }
```

## Common Patterns

**Builder pattern for register configuration**:
```rust
let mut reg = Chopconf::new();
reg.set_toff(3)
   .set_hstrt(4)
   .set_hend(1);
driver.write_register(&reg)?;
```

**Convenience method pattern** (add to driver.rs):
```rust
pub fn high_level_operation(&mut self, params) -> Result<(), Error<E>> {
    let mut reg = self.read_register::<RegType>()?;
    reg.set_field(value);
    self.write_register(&reg)
}
```

## Adding New Features

1. **New register**: Create `src/registers/regname.rs`, add to `mod.rs`
2. **New convenience method**: Add to both blocking and async impl blocks in `driver.rs`
3. **New utility function**: Add to `util.rs` with tests
4. **Update re-exports**: Add to `lib.rs` if commonly used

## Commit Guidelines

- Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`
- Reference issues: `fix: correct CRC calculation (#123)`
- Keep commits focused and atomic

## Notes

- **Never commit** without running tests: `cargo test --all-features`
- **Hardware**: This library requires TMC2209 hardware for integration testing
- **Examples**: Compile but won't run without platform UART setup
- **Documentation examples**: Use `ignore` for hardware-dependent code
