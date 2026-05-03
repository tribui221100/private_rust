mod bandwith_calculator;
mod sensor_debugger;
fn main() {
    //bandwith_calculator::run();
    
    // Requirement 1
    let sensor1 = sensor_debugger::SensorData::new(100, 1, 25.5);
    println!("{}",sensor1);
}
