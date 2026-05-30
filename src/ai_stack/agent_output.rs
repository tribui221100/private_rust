#![no_std]

//! # Requirement 2: Telemetry Frame Parser & Buffer Manager
//! 
//! This module provides zero-allocation parsing, validation, and ring-buffering
//! of incoming sensor packets. It uses AUTOSAR-style data types to align with
//! automotive architecture standards.

/// AUTOSAR equivalent type definitions for platform safety
pub type PduIdType = u16;       // Type for Protocol Data Unit Identifier (uniquely identifies sensor source)
pub type PduLengthType = u16;  // Type representing payload length in bytes

/// Custom error types for parser and buffer validation without string allocations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorError {
    InvalidHeader,
    PayloadLengthMismatch,
    ChecksumMismatch,
    BufferFull,
    BufferEmpty,
    IndexOutOfBounds,
}

/// Supported telemetry sensor types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorType {
    Temperature = 0x01,
    BarometricPressure = 0x02,
    Humidity = 0x03,
}

impl SensorType {
    /// Decodes a raw byte into a SensorType variant
    pub fn from_u8(value: u8) -> Result<Self, SensorError> {
        match value {
            0x01 => Ok(SensorType::Temperature),
            0x02 => Ok(SensorType::BarometricPressure),
            0x03 => Ok(SensorType::Humidity),
            _ => Err(SensorError::InvalidHeader), // Propagation of error variant avoids panic
        }
    }
}

/// Raw Frame Structure (Fixed size, no dynamic allocation)
/// [Header (1B)][PduID (2B)][SensorType (1B)][Payload (4B)][CRC-16 (2B)]
pub const FRAME_SIZE: usize = 10;
const HEADER_BYTE: u8 = 0xAA;

#[derive(Debug, Clone, Copy)]
pub struct SensorFrame {
    pub pdu_id: PduIdType,
    pub sensor_type: SensorType,
    pub payload: [u8; 4], // Stack-allocated fixed payload (ideal for embedded memory bounds)
}

impl SensorFrame {
    /// Parses a raw byte slice into a validated SensorFrame.
    /// This utilizes slice pattern matching for micro-architectural optimization.
    pub fn parse(raw: &[u8; FRAME_SIZE]) -> Result<Self, SensorError> {
        // Validate Header Sync Byte
        if raw[0] != HEADER_BYTE {
            return Err(SensorError::InvalidHeader);
        }

        // Extract PDU ID via big-endian byte reconstruction
        let pdu_id = ((raw[1] as u16) << 8) | (raw[2] as u16);

        // Decode the Sensor Type
        let sensor_type = SensorType::from_u8(raw[3])?;

        // Extract raw payload segment
        let mut payload = [0u8; 4];
        payload.copy_from_slice(&raw[4..8]); // Zero-allocation slice copy (compile-time boundary verified)

        // Validate Checksum (CRC-16 CCITT)
        let rx_checksum = ((raw[8] as u16) << 8) | (raw[9] as u16);
        let calculated_checksum = crc16::State::<crc16::XMODEM>::calculate(&raw[0..8]); // Validate entire header + payload

        if rx_checksum != calculated_checksum {
            return Err(SensorError::ChecksumMismatch);
        }

        Ok(SensorFrame {
            pdu_id,
            sensor_type,
            payload,
        })
    }
}

/// Static Ring Buffer for Sensor Telemetry
/// This manages packet processing queues without heap allocations (`std::vec::Vec`).
pub struct SensorRingBuffer<const N: usize> {
    buffer: [Option<SensorFrame>; N], // Array of optional structures (zero heap usage)
    write_ptr: usize,
    read_ptr: usize,
    count: usize,
}

impl<const N: usize> SensorRingBuffer<N> {
    /// Initializes a empty ring buffer using constant generics (allows compile-time size specification)
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            write_ptr: 0,
            read_ptr: 0,
            count: 0,
        }
    }

    /// Pushes a valid sensor frame into the buffer
    pub fn push(&mut self, frame: SensorFrame) -> Result<(), SensorError> {
        if self.count == N {
            return Err(SensorError::BufferFull);
        }

        self.buffer[self.write_ptr] = Some(frame);
        self.write_ptr = (self.write_ptr + 1) % N; // Wrap-around logic avoiding branch conditions
        self.count += 1;
        Ok(())
    }

    /// Pops the oldest sensor frame from the buffer
    pub fn pop(&mut self) -> Result<SensorFrame, SensorError> {
        if self.count == 0 {
            return Err(SensorError::BufferEmpty);
        }

        // Move value out of the optional buffer slot
        let frame = self.buffer[self.read_ptr].take().ok_or(SensorError::BufferEmpty)?;
        self.read_ptr = (self.read_ptr + 1) % N;
        self.count -= 1;
        Ok(frame)
    }

    /// Read-only inspection of buffer load level
    pub fn len(&self) -> usize {
        self.count
    }
}
