// -----------------------------------------------------------------------------
// REQUIREMENT 1 — Starter: Project & types
// -----------------------------------------------------------------------------
// Set up a no_std-capable library crate (or a std binary that mirrors embedded
// patterns). Define Rust types for a single sensor reading: timestamp (u64 ms),
// sensor_id (u8), and value (f32). Implement Display and Debug. Write unit tests
// that construct readings and assert formatting.

#![no_std]

use core::fmt;
pub use crate::modules::sensor_types::SensorData;
// Constructor
impl SensorData{
    pub const fn new(timestamp: u32, id: u8, value: u32) -> Self
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

// Debug
impl fmt::Debug for SensorData{
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("SensorData")
            .field("timestamp", &self.timestamp)
            .field("id", &self.id)
            .field("value", &self.value)
            .finish()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    extern crate std;
    use std::format;

    #[test]
    fn test_sensor_constructor()
    {
        let data = SensorData::new(1_2211_2000,125,24);
        assert_eq!(data.timestamp, 1_2211_2000);
        assert_eq!(data.id, 125);
        assert_eq!(data.value, 24);
    }

    #[test]
    fn test_sensor_display()
    {
        let data = SensorData::new(500, 1, 10234);
        let expected = "[       500ms] Sensor(001): 1.23]";
        assert_eq!(format!("{}", data), expected);
    }

    #[test]
    fn test_sensor_debug()
    {
        let data = SensorData::new(500, 1, 111234);
        let expected = format!("{:?}", data);
        assert!(expected.contains("SensorData"));
    }
}