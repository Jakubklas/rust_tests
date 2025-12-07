use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorReading {
    pub sensor_id: u32,
    pub location: String,
    pub value: f32,
    pub timestamp: u64,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    pub status: String,
    pub alert: bool,
    pub msg: String,
    pub reading: SensorReading,
}
