mod modules;
mod ai_stack;
mod handson;

use crate::modules::sensor_types::SensorError;
use colored::*;
use std::time::{Duration, Instant};
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}","--- INTIALIZE AI AGENT (RULE-BASED) ---".yellow().bold());
    
    #[cfg(feature="ai")]
    {
        let _ = ai_stack::agent::run_agent().await;
    }
    
    // Requirement 1
    #[cfg(feature = "req1")]
    {
        let sensor1 = modules::sensor_debugger::SensorData::new(100, 1, 25);
        println!("Actual Display output{}",sensor1);
        println!("Actual Debug output: {:?}", sensor1);
    }
    
    #[cfg(feature = "req2")]
    {
        let rx_frame = [0xAB,0x01,0x20,0xFF,0x54,0x12,0x00,0xAB,0x99,0x10,0x1F];
        let rx_data = crate::modules::sensor_types::SensorData::parse
                                                        (&rx_frame);
        
        match rx_data {
            Ok(data) => {
                let interpreted_data = modules::sensor_debugger::SensorData::new(
                                                    data.timestamp,
                                                    data.id,
                                                    data.value);
                println!("Interpretted data {}",interpreted_data);
            }
            Err(e) => {
                println!("Dropped packet due to error: {:?}", e);
            }
        }
    }

    // External Requirement - Cache
    #[cfg(feature = "req3")]
    {
        let adc_reader = || {
            thread::sleep(Duration::from_millis(50));
            println!("Read from ADC");
            95
        };

        let mut engine_temp = modules::sensor_cacher::SensorCache::new(adc_reader,Duration::from_secs(1));

        println!("--- 1: System initialize, No Cache ---");
        let t1 = engine_temp.get_value();
        assert_eq!(t1, 95);

        println!("\n--- 2: Immediately invoked (after 100ms) -> get from Cache ---");
        thread::sleep(Duration::from_millis(100));
        let t2 = engine_temp.get_value();
        assert_eq!(t2, 95);

        println!("\n---3: Longer time (after 1.2s) -> Cache timeout and read hw again ---");
        thread::sleep(Duration::from_millis(1200));
        let t3 = engine_temp.get_value();
        assert_eq!(t3, 95);
        
        println!("\nCongrates.");
    }
    Ok(())
}

