mod modules;
mod ai_stack;

use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}","--- INTIALIZE AI AGENT (RULE-BASED) ---".yellow().bold());

    let _ = ai_stack::agent::run_agent().await;
    Ok(())
}

