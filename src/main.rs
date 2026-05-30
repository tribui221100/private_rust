mod modules;
mod ai_stack;

use crate::modules::sensor_types::SensorError;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}","--- INTIALIZE AI AGENT (RULE-BASED) ---".yellow().bold());
    
    #[cfg(feature="ai")]
    {
        let _ = ai_stack::agent::run_agent().await;
    }
    
    // Requirement 1
    //#[cfg(feature = "req1")]
    {
        let sensor1 = modules::sensor_debugger::SensorData::new(100, 1, 25);
        println!("Actual Display output{}",sensor1);
        println!("Actual Debug output: {:?}", sensor1);
    }
    
    //#[cfg(feature = "req2")]
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

    Ok(())
}

