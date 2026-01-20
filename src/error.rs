//! Error types for TMC2209 driver.

use core::fmt;

/// Errors that can occur during TMC2209 communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// UART communication error (read or write failed).
    Uart(E),
    /// CRC checksum mismatch in received response.
    CrcMismatch,
    /// Invalid sync byte in response (expected 0x05).
    InvalidSync,
    /// Invalid master address in response (expected 0xFF).
    InvalidMasterAddress,
    /// Register address in response doesn't match request.
    AddressMismatch {
        /// The expected register address.
        expected: u8,
        /// The actual register address received.
        actual: u8,
    },
    /// Unknown register address received.
    UnknownAddress(u8),
    /// Invalid slave address (must be 0-3).
    InvalidSlaveAddress(u8),
    /// Response buffer too small.
    BufferTooSmall,
    /// No response received (timeout or no data).
    NoResponse,
}

impl<E> Error<E> {
    /// Map the UART error type to a different type.
    pub fn map_uart<F, E2>(self, f: F) -> Error<E2>
    where
        F: FnOnce(E) -> E2,
    {
        match self {
            Error::Uart(e) => Error::Uart(f(e)),
            Error::CrcMismatch => Error::CrcMismatch,
            Error::InvalidSync => Error::InvalidSync,
            Error::InvalidMasterAddress => Error::InvalidMasterAddress,
            Error::AddressMismatch { expected, actual } => {
                Error::AddressMismatch { expected, actual }
            }
            Error::UnknownAddress(addr) => Error::UnknownAddress(addr),
            Error::InvalidSlaveAddress(addr) => Error::InvalidSlaveAddress(addr),
            Error::BufferTooSmall => Error::BufferTooSmall,
            Error::NoResponse => Error::NoResponse,
        }
    }
}

impl<E: fmt::Debug> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Uart(e) => write!(f, "UART error: {:?}", e),
            Error::CrcMismatch => write!(f, "CRC checksum mismatch"),
            Error::InvalidSync => write!(f, "Invalid sync byte (expected 0x05)"),
            Error::InvalidMasterAddress => write!(f, "Invalid master address (expected 0xFF)"),
            Error::AddressMismatch { expected, actual } => {
                write!(
                    f,
                    "Register address mismatch: expected 0x{:02X}, got 0x{:02X}",
                    expected, actual
                )
            }
            Error::UnknownAddress(addr) => write!(f, "Unknown register address: 0x{:02X}", addr),
            Error::InvalidSlaveAddress(addr) => {
                write!(f, "Invalid slave address: {} (must be 0-3)", addr)
            }
            Error::BufferTooSmall => write!(f, "Response buffer too small"),
            Error::NoResponse => write!(f, "No response received"),
        }
    }
}
