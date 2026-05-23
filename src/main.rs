mod modules;
mod ai_stack;

use colored::*;

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
        let sensor1 = modules::sensor_debugger::SensorData::new(100, 1, 25.5);
        println!("Actual Display output{}",sensor1);
        println!("Actual Debug output: {:?}", sensor1);
    }
    
    Ok(())
}

