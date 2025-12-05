use serde::{Serialize, Deserialize};
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
    let path = r"C:\Users\jklas\rust_tests\first-steps\stuff.json";
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
    let _ = fs::write(&path, &json).unwrap();

    // Read the .json contents & desrialize
    let content = fs::read_to_string(&path).unwrap();
    let loaded: WeatherStation = serde_json::from_str(&content).unwrap();
    println!("Location: {}", loaded.location);
    println!("Station ID: {}", loaded.station_id);
    for r in loaded.readings {
        println!("      Timestamp: {}", r.timestamp);
        println!("      Temperature: {}", r.temp);
        println!("      Humidity: {}", r.humid);
        println!("      Pressure: {}\n\n", r.press);
    }






}



