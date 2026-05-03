mod bandwith_calculator;
mod sensor_debugger;
fn main() {
    //bandwith_calculator::run();
    
    // Requirement 1
    #[cfg(feature = "req1")]
    {
        let sensor1 = sensor_debugger::SensorData::new(100, 1, 25.5);
        println!("Actual Display output{}",sensor1);
        println!("Actual Debug output: {:?}", sensor1);
    }

    // Requirement 2
    #[cfg(feature = "req2")]
    {
        println!("T.B.D");
    }
}
