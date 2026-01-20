//! UART datagram types for TMC2209 communication.
//!
//! The TMC2209 uses a simple UART protocol with three datagram types:
//! - Read request (4 bytes): Request to read a register
//! - Write request (8 bytes): Request to write a register
//! - Read response (8 bytes): Response containing register data

use crate::crc;
use crate::error::Error;
use crate::registers::Address;

/// Sync byte used in all TMC2209 datagrams.
pub const SYNC: u8 = 0x05;

/// Master address used in read responses (always 0xFF).
pub const MASTER_ADDR: u8 = 0xFF;

/// Write bit OR'd with register address for write requests.
pub const WRITE_BIT: u8 = 0x80;

/// Address mask (7 bits).
pub const ADDRESS_MASK: u8 = 0x7F;

/// Read request datagram (4 bytes).
///
/// Format: `[SYNC, slave_addr, reg_addr, CRC]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadRequest {
    bytes: [u8; Self::LEN],
}

impl ReadRequest {
    /// Length of a read request in bytes.
    pub const LEN: usize = 4;

    /// Create a new read request for the given slave and register address.
    ///
    /// # Arguments
    ///
    /// * `slave_addr` - Slave address (0-3)
    /// * `reg_addr` - Register address to read
    pub fn new(slave_addr: u8, reg_addr: Address) -> Self {
        let mut bytes = [SYNC, slave_addr, reg_addr as u8, 0];
        bytes[3] = crc::compute(&bytes[..3]);
        Self { bytes }
    }

    /// Create a read request from a raw register address.
    ///
    /// Use this when you need to read a register by its raw address value.
    pub fn from_raw_addr(slave_addr: u8, reg_addr: u8) -> Self {
        let mut bytes = [SYNC, slave_addr, reg_addr & ADDRESS_MASK, 0];
        bytes[3] = crc::compute(&bytes[..3]);
        Self { bytes }
    }

    /// Get the request as a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the slave address.
    #[inline]
    pub fn slave_addr(&self) -> u8 {
        self.bytes[1]
    }

    /// Get the register address.
    #[inline]
    pub fn reg_addr(&self) -> u8 {
        self.bytes[2]
    }
}

impl AsRef<[u8]> for ReadRequest {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

/// Write request datagram (8 bytes).
///
/// Format: `[SYNC, slave_addr, reg_addr | 0x80, data[3], data[2], data[1], data[0], CRC]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteRequest {
    bytes: [u8; Self::LEN],
}

impl WriteRequest {
    /// Length of a write request in bytes.
    pub const LEN: usize = 8;

    /// Create a new write request for the given slave, register address, and data.
    ///
    /// # Arguments
    ///
    /// * `slave_addr` - Slave address (0-3)
    /// * `reg_addr` - Register address to write
    /// * `data` - 32-bit data value to write
    pub fn new(slave_addr: u8, reg_addr: Address, data: u32) -> Self {
        Self::from_raw(slave_addr, reg_addr as u8, data)
    }

    /// Create a write request from raw address and data.
    pub fn from_raw(slave_addr: u8, reg_addr: u8, data: u32) -> Self {
        let data_bytes = data.to_be_bytes();
        let mut bytes = [
            SYNC,
            slave_addr,
            (reg_addr & ADDRESS_MASK) | WRITE_BIT,
            data_bytes[0],
            data_bytes[1],
            data_bytes[2],
            data_bytes[3],
            0,
        ];
        bytes[7] = crc::compute(&bytes[..7]);
        Self { bytes }
    }

    /// Get the request as a byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the slave address.
    #[inline]
    pub fn slave_addr(&self) -> u8 {
        self.bytes[1]
    }

    /// Get the register address (without write bit).
    #[inline]
    pub fn reg_addr(&self) -> u8 {
        self.bytes[2] & ADDRESS_MASK
    }

    /// Get the data value.
    #[inline]
    pub fn data(&self) -> u32 {
        u32::from_be_bytes([self.bytes[3], self.bytes[4], self.bytes[5], self.bytes[6]])
    }
}

impl AsRef<[u8]> for WriteRequest {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

/// Read response datagram (8 bytes).
///
/// Format: `[SYNC, master_addr (0xFF), reg_addr, data[3], data[2], data[1], data[0], CRC]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadResponse {
    bytes: [u8; Self::LEN],
}

impl ReadResponse {
    /// Length of a read response in bytes.
    pub const LEN: usize = 8;

    /// Index of the sync byte.
    pub const SYNC_IDX: usize = 0;
    /// Index of the master address byte.
    pub const MASTER_ADDR_IDX: usize = 1;
    /// Index of the register address byte.
    pub const REG_ADDR_IDX: usize = 2;
    /// Start index of data bytes.
    pub const DATA_START_IDX: usize = 3;
    /// End index of data bytes (exclusive).
    pub const DATA_END_IDX: usize = 7;
    /// Index of the CRC byte.
    pub const CRC_IDX: usize = 7;

    /// Parse a read response from a byte buffer.
    ///
    /// # Arguments
    ///
    /// * `bytes` - 8-byte buffer containing the response
    ///
    /// # Returns
    ///
    /// The parsed response, or an error if validation fails.
    pub fn from_bytes<E>(bytes: [u8; Self::LEN]) -> Result<Self, Error<E>> {
        let response = Self { bytes };
        response.validate()?;
        Ok(response)
    }

    /// Parse a response from a slice, copying the data.
    ///
    /// # Errors
    ///
    /// Returns `Error::BufferTooSmall` if the slice is less than 8 bytes.
    pub fn from_slice<E>(slice: &[u8]) -> Result<Self, Error<E>> {
        if slice.len() < Self::LEN {
            return Err(Error::BufferTooSmall);
        }
        let mut bytes = [0u8; Self::LEN];
        bytes.copy_from_slice(&slice[..Self::LEN]);
        Self::from_bytes(bytes)
    }

    /// Validate the response structure.
    fn validate<E>(&self) -> Result<(), Error<E>> {
        // Check sync byte
        if self.bytes[Self::SYNC_IDX] != SYNC {
            return Err(Error::InvalidSync);
        }

        // Check master address
        if self.bytes[Self::MASTER_ADDR_IDX] != MASTER_ADDR {
            return Err(Error::InvalidMasterAddress);
        }

        // Check CRC
        if !crc::verify(&self.bytes) {
            return Err(Error::CrcMismatch);
        }

        Ok(())
    }

    /// Get the register address from the response.
    #[inline]
    pub fn reg_addr(&self) -> u8 {
        self.bytes[Self::REG_ADDR_IDX]
    }

    /// Get the typed register address if known.
    pub fn address(&self) -> Option<Address> {
        Address::from_u8(self.reg_addr())
    }

    /// Get the 32-bit data value.
    #[inline]
    pub fn data(&self) -> u32 {
        u32::from_be_bytes([self.bytes[3], self.bytes[4], self.bytes[5], self.bytes[6]])
    }

    /// Get the raw bytes of the response.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.bytes
    }

    /// Check if the CRC is valid.
    #[inline]
    pub fn crc_valid(&self) -> bool {
        crc::verify(&self.bytes)
    }
}

impl AsRef<[u8]> for ReadResponse {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

/// Response reader for non-blocking/streaming response parsing.
///
/// This reader maintains state between read calls, allowing you to
/// parse responses byte-by-byte or in chunks.
#[derive(Debug, Default)]
pub struct ResponseReader {
    /// Current index into the response buffer.
    index: usize,
    /// Buffer for accumulating response bytes.
    buffer: [u8; ReadResponse::LEN],
}

impl ResponseReader {
    /// Create a new response reader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the reader state.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Feed bytes to the reader and attempt to parse a response.
    ///
    /// # Returns
    ///
    /// A tuple of (bytes_consumed, optional_response).
    /// The response is `Some` when a complete valid response is parsed.
    pub fn feed<E>(&mut self, bytes: &[u8]) -> (usize, Option<Result<ReadResponse, Error<E>>>) {
        let mut consumed = 0;
        let mut remaining = bytes;

        loop {
            // Looking for sync byte
            while self.index == 0 {
                match remaining.first() {
                    Some(&SYNC) => {
                        self.buffer[0] = SYNC;
                        self.index = 1;
                        remaining = &remaining[1..];
                        consumed += 1;
                    }
                    Some(_) => {
                        // Skip non-sync bytes
                        remaining = &remaining[1..];
                        consumed += 1;
                    }
                    None => {
                        return (consumed, None);
                    }
                }
            }

            // Looking for master address
            if self.index == 1 {
                match remaining.first() {
                    Some(&MASTER_ADDR) => {
                        self.buffer[1] = MASTER_ADDR;
                        self.index = 2;
                        remaining = &remaining[1..];
                        consumed += 1;
                    }
                    Some(_) => {
                        // Not a valid response, reset
                        self.index = 0;
                        continue;
                    }
                    None => {
                        return (consumed, None);
                    }
                }
            }

            // Read remaining bytes
            let needed = ReadResponse::LEN - self.index;
            let available = remaining.len().min(needed);

            self.buffer[self.index..self.index + available]
                .copy_from_slice(&remaining[..available]);
            self.index += available;
            consumed += available;

            // Check if we have a complete response
            if self.index == ReadResponse::LEN {
                self.index = 0;
                let result = ReadResponse::from_bytes(self.buffer);
                return (consumed, Some(result));
            }

            return (consumed, None);
        }
    }

    /// Get the current number of bytes buffered.
    pub fn buffered(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_request() {
        let req = ReadRequest::new(0, Address::Gconf);
        assert_eq!(req.slave_addr(), 0);
        assert_eq!(req.reg_addr(), 0x00);
        assert_eq!(req.as_bytes().len(), 4);
    }

    #[test]
    fn test_write_request() {
        let req = WriteRequest::new(0, Address::Gconf, 0x00000040);
        assert_eq!(req.slave_addr(), 0);
        assert_eq!(req.reg_addr(), 0x00);
        assert_eq!(req.data(), 0x00000040);
        assert_eq!(req.as_bytes().len(), 8);
    }

    #[test]
    fn test_response_reader() {
        // Create a mock response
        let mut response_bytes = [SYNC, MASTER_ADDR, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00];
        response_bytes[7] = crc::compute(&response_bytes[..7]);

        let mut reader = ResponseReader::new();
        let (consumed, result) = reader.feed::<()>(&response_bytes);

        assert_eq!(consumed, 8);
        assert!(result.is_some());
        let response = result.unwrap().unwrap();
        assert_eq!(response.data(), 0x00000040);
    }
}
