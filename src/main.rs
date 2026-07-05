mod modules;
mod ai_stack;
mod runner;

use crate::modules::sensor_types::SensorData;
use crate::modules::sensor_cacher::SensorCache;
use crate::ai_stack::agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    runner::run().await;
    Ok(())
}

