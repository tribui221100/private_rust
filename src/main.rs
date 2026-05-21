mod modules;
mod ai_cli;
mod agent;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}","--- INTIALIZE AI AGENT (RULE-BASED) ---".yellow().bold());

    let _ = agent::run_agent().await;
    Ok(())
}

