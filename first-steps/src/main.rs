use bincode::config;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    #[serde(rename = "deviceId")]
    device_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SensorConfig {
    #[serde(rename = "sensorId")]                   //TODO: I understand that during serialization this saves the attribute's name with camel case - but why not just name it correctly in the struct itself? I don't find this useful at all. It feels like it will just produce missmatch asn confusion when reading/writing during I/O 
    sensor_id: u32,
    #[serde(rename = "sensorType")]
    sensor_type: SensorType,
    location: String,
    #[serde(default = "default_threshold")]
    threshold: f32,
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]                //TODO I actually find these derive statements very annoying to add to every single enum or struct or trait- is there nothing more practical than this? 
#[serde(rename_all = "lowercase")]
enum SensorType {
    #[serde(rename_all = "lowercase")]
    Temperature,
    Pressure,
    FlowRate,
}


fn default_threshold() -> f32 {
    50.0
}

fn load_config(path: &str) -> Result<SensorConfig, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let loaded_content: SensorConfig = serde_json::from_str(&content)
        .map_err(|e| e.to_string())?;
    
    Ok(loaded_content)
}

fn save_config(path: &str, config: SensorConfig) -> Result<(), String> {
    let json= serde_json::to_string_pretty(&config)
        .map_err(|e| e.to_string())?;

    let _ = fs::write(path, json)
        .map_err(|e| e.to_string())?;

    Ok(())

}


fn main() {
    let path = r"C:\Users\jklas\rust_tests\first-steps\stuff.json";
    let config = SensorConfig {
        sensor_id: 1,
        sensor_type: SensorType::Temperature,
        location: "Boiler Room".to_string(),
        threshold: 80.0,
        enabled: true,
        description: None,
    };


    match save_config(&path, config) {
        Ok(_) => println!("Successfully serialized into {}", &path.to_string()),
        Err(e) => println!("ERROR Serializing file: {}", e),
    }

    match load_config(&path) {
        Ok(content) => println!("{:?}", content),
        Err(e) => println!("ERROR Loading file: {}", e),
    }

}



