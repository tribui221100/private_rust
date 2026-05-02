// bandwith calculator.rs
use std::io;

/*
Requirement: Bandwidth calculator for Radar (Basic)
Assume you are configuring a Radar at Bosch. You need to write a small program to calculate the total data transferred back through the Ethernet port.
Requirement:
1. Allow the user to enter the number of sensors (Sensor Count).
2. Allow the user to enter the data rate of each sensor (unit: MB/s).
3. Calculate the total bandwidth.
4. If the total bandwidth exceeds 100 MB/s, print a warning: "Ethernet overload risk!". If not, print "Safe bandwidth".
*/
const MAX_ETHERNET_BW: f64 = 100.0;

fn get_sensorcount() -> u32
{
        // 1. Enter number of sensor
        println!("Enter the number of sensors: "); 
        let mut sensor_count = String::new();
        io::stdin().read_line(&mut sensor_count).expect("Please type a number!");
        let sensor_count: u32 = sensor_count.trim().parse().expect("Please type a number!");
        return sensor_count;
}

fn get_datarate() -> f64{
        // 2. Enter data of each sensor
        println!("Enter the data rate of each sensor (unit: MB/s): ");
        let mut data_rate = String::new();
        io::stdin().read_line(&mut data_rate).expect("Please type a number!");
        let data_rate: f64 = data_rate.trim().parse().expect("Please type a number!");
        return data_rate;
}

fn bandwith_calculator(count: u32, rate: f64) -> f64 {
    count as f64 * rate
}

fn is_overload(total_bw: f64) -> bool{
    total_bw > MAX_ETHERNET_BW
}

pub fn run()
{
    let l_sensor_count = get_sensorcount();
    let l_data_rate = get_datarate();
    
    let total_bandwidth = bandwith_calculator(l_sensor_count,l_data_rate);   

    if is_overload(total_bandwidth){
        println!("Ethernet overload risk!");
    } else {
        println!("Safe bandwidth");
    }
}