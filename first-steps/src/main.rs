use serde::{Serialize, Deserialize};
use tokio::task::LocalSet;
use std::fs;
use fastrand;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherStation {
    station_id: u32,
    location: String,
    readings: Vec<Reading>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reading {
    timestamp: String,
    temp: f32,
    humid: f32,
    press: f32,
}

fn main() {
    let json_path = r"C:\Users\jklas\rust_tests\first-steps\stuff.json";
    let bin_path = r"C:\Users\jklas\rust_tests\first-steps\stuff.bin";
    
    let mut readings = Vec::new();
    
    for i in 0..5 {
        readings.push( Reading {
            timestamp: (1092 + i).to_string(),
            temp: fastrand::f32() * 100.0,
            humid: fastrand::f32() * 100.0,
            press: fastrand::f32() * 100.0,
        })
    } 

    let station = WeatherStation {
        station_id: 1,
        location: "Belfast".to_string(),
        readings: readings,
    };

    // Serialize & save to .json file
    let json = serde_json::to_string_pretty(&station).unwrap();
    let _ = fs::write(&json_path, &json).unwrap();

    let binary = bincode::serialize(&station).unwrap();
    let _ = fs::write(&bin_path, &binary).unwrap();

    // Read the .json contents & desrialize
    let content = fs::read_to_string(&json_path).unwrap();
    let loaded_json: WeatherStation = serde_json::from_str(&content).unwrap();
    println!("Location: {}", loaded_json.location);
    println!("Station ID: {}", loaded_json.station_id);
    for r in loaded_json.readings {
        println!("      Timestamp: {}", r.timestamp);
        println!("      Temperature: {}", r.temp);
        println!("      Humidity: {}", r.humid);
        println!("      Pressure: {}\n\n", r.press);
    }

    // let loaded_binary: WeatherStation = bincode::deserialize(&content).unwrap();
    let json_size = fs::metadata(&json_path).unwrap().len();
    let bin_size = fs::metadata(&bin_path).unwrap().len();
    println!("\n==== SIZE COMPARISON ====");
    println!("JSON Bytes: {}", json_size);
    println!("Binary Bytes: {}", bin_size);

}



