// -----------------------------------------------------------------------------
// REQUIREMENT 1 — Starter: Project & types
// -----------------------------------------------------------------------------
// Set up a no_std-capable library crate (or a std binary that mirrors embedded
// patterns). Define Rust types for a single sensor reading: timestamp (u64 ms),
// sensor_id (u8), and value (f32). Implement Display and Debug. Write unit tests
// that construct readings and assert formatting.

#![no_std]

use core::fmt;
#[derive(Debug, PartialEq)]

pub struct SensorData{
    pub timestamp: u32,
    pub id: u8,
    pub value: f32
}

// Constructor
impl SensorData{
    pub const fn new(timestamp: u32, id: u8, value: f32) -> Self
    {
        Self{
            timestamp,
            id,
            value
        }
    }
}

// Display
impl fmt::Display for SensorData{
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(
            f,
            "[{:>10}ms] Sensor({:03}): {:.2}]",
            &self.timestamp, &self.id, &self.value
        )
    }
}