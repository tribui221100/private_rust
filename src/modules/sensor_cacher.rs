// -----------------------------------------------------------------------------
// REQUIREMENT EXT.1 — Cacher
// -----------------------------------------------------------------------------
// Context:
// You are writing firmware for an ECU (Engine Control Unit) in a car. This ECU needs to continuously monitor the engine temperature from a hardware temperature sensor via an ADC (Analog-to-Digital Converter).
// Reading the value from the ADC takes about 50 milliseconds (very slow for a microcontroller).
// However, other tasks in the system (such as displaying the screen, safety checks) require retrieving this temperature data continuously (hundreds of times per second).
// Problem Requirement:
// Write a SensorCache structure in Rust to encapsulate this ADC reading logic.
// Instead of using a fixed Option, this Cache must manage data using a Time-To-Live (TTL) / Timeout mechanism. That is, the cached value is only valid for 1000 milliseconds (1 second) from the time it is read from the hardware.
// When the function to retrieve the value is called:
// If the cache is still valid (Current time - Read time < 1 second): Immediately return the old value in the cache (Do not reread the hardware).
// If the cache has expired or there is no data (None): Call the Closure (ACC hardware read action), update the new value and timestamp in the cache.

use std::time::{Duration, Instant};
use std::thread;

pub struct SensorCache<T> 
    where T: Fn() -> u32
    {
        read_hw: T,
        ccached_value: Option<u32>,
        last_read_time: Option<Instant>,
        ttl: Duration
    }

impl <T> SensorCache <T> 
where 
    T: Fn() -> u32
{
    pub fn new(hw_reader: T, duration: Duration) -> Self {
        SensorCache{
            read_hw: hw_reader,
            ccached_value: None,
            last_read_time: None,
            ttl: duration
        }
    }

    pub fn get_value(&mut self) -> u32 { 
        let now = Instant::now();

        match (self.ccached_value,self.last_read_time) {
            (Some(val), Some(time)) => {
                if now - self.ttl > time{
                    println!("-> [Cache Hit] Return from Cache");
                    val
                } else {
                    (self.read_hw)()
                }
            }
            _ => (self.read_hw)()
        }
        
    }
}