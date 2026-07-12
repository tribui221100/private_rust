use colored::*;
use std::time::{Duration, Instant};
use std::thread;


use crate::SensorData;
use crate::SensorCache;
use crate::agent;
use crate::handson::leetcode;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ai")]
    {
        run_ai().await;
        println!("ai mode running");
    }

    #[cfg(feature = "req1")]
    {
        run_req1();
        println!("req1 running");
    }

    #[cfg(feature = "req2")]
    {
        run_req2();
        println!("req2 running");
    }   

    #[cfg(feature = "req3")]
    {
        run_req3();
        println!("req3 running");
    }

    #[cfg(feature = "handson")]
    {
        println!("handson running");
        run_handson(13);
    }
    

    Ok(())
}

    #[cfg(feature = "ai")]
    pub async fn run_ai()
    {
        let _ = agent::run_agent().await;
    }
    
    // Requirement 1
    #[cfg(feature = "req1")]
    pub fn run_req1()
    {
        let sensor1 = SensorData::new(100, 1, 25);
        println!("Actual Display output{}",sensor1);
        println!("Actual Debug output: {:?}", sensor1);
    }
    
    #[cfg(feature = "req2")]
    pub fn run_req2()
    {
        let rx_frame = [0xAB,0x01,0x20,0xFF,0x54,0x12,0x00,0xAB,0x99,0x10,0x1F];
        let rx_data = SensorData::parse(&rx_frame);
        
        match rx_data {
            Ok(data) => {
                let interpreted_data = SensorData::new(
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
    pub fn run_req3()
    {
        let adc_reader = || {
            thread::sleep(Duration::from_millis(50));
            println!("Read from ADC");
            95
        };

        let mut engine_temp = SensorCache::new(adc_reader,Duration::from_secs(1));

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


    #[cfg(feature = "handson")]
    pub fn run_handson(num: i8)
    {
        match num {
            3 => {
                let result = leetcode::Solution::Leet3_LongestSubstring("abcabcbb".to_string());
                println!("Result for Leet3: {}", result);
            }
            13 => {
                let result = leetcode::Solution::roman_to_int("MCDLXXVI".to_string());
                println!("Result for Leet13: {}", result);
            }
            _ => println!("Invalid input. Please enter a number between 1 and 3."),
        }

    }