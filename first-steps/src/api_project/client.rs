use serde::{Deserialize, Serialize};
use reqwest::Client;
use fastrand;

// Mirror the server's data structures
#[derive(Deserialize, Serialize, Debug)]
enum SensorType {
    Thermometer,
    FlowMeter,
    PressureSensor,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sensor {
    id: u32,
    name: String,
    location: String,
    sensor_type: SensorType,
    created_at: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Reading {
    id: u32,
    sensor_id: u32,
    value: f32,
    timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct ServerResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct CreateSensorRequest {
    name: String,
    location: String,
    sensor_type: SensorType,
}

#[derive(Serialize)]
struct AddReadingRequest {
    sensor_id: u32,
    value: f32,
}

const BASE_URL: &str = "http://localhost:8000";

async fn health_check(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .get(&format!("{}/health", BASE_URL))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("✓ Health Check: {} - {}", status, body);
    Ok(body)
}

async fn create_sensor(
    client: &Client,
    name: String,
    location: String,
    sensor_type: SensorType,
) -> Result<ServerResponse, Box<dyn std::error::Error>> {
    let request = CreateSensorRequest {
        name,
        location,
        sensor_type,
    };

    let response = client
        .post(&format!("{}/sensors", BASE_URL))
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let result: ServerResponse = response.json().await?;

    println!("✓ Create Sensor: {} - {}", status, result.message);
    Ok(result)
}

async fn list_sensors(client: &Client, id: Option<u32>) -> Result<Vec<Sensor>, Box<dyn std::error::Error>> {
    let mut url = format!("{}/sensors", BASE_URL);

    if let Some(sensor_id) = id {
        url = format!("{}?id={}", url, sensor_id);
    }

    let response = client
        .get(&url)
        .send()
        .await?;

    let status = response.status();
    let sensors: Vec<Sensor> = response.json().await?;

    println!("✓ List Sensors: {} - Found {} sensor(s)", status, sensors.len());
    for sensor in &sensors {
        println!("  - ID: {}, Name: {}, Location: {}", sensor.id, sensor.name, sensor.location);
    }

    Ok(sensors)
}

async fn delete_sensor(client: &Client, id: u32) -> Result<ServerResponse, Box<dyn std::error::Error>> {
    let response = client
        .delete(&format!("{}/sensors/{}", BASE_URL, id))
        .send()
        .await?;

    let status = response.status();
    let result: ServerResponse = response.json().await?;

    println!("✓ Delete Sensor: {} - {}", status, result.message);
    Ok(result)
}

async fn add_reading(client: &Client, sensor_id: u32) -> Result<ServerResponse, Box<dyn std::error::Error>> {
    let request = AddReadingRequest {
        sensor_id,
        value: fastrand::f32() * 100.0,
    };

    let response = client
        .post(&format!("{}/sensors/readings", BASE_URL))
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let result: ServerResponse = response.json().await?;

    println!("✓ Add Reading: {} - {}", status, result.message);
    Ok(result)
}

async fn get_all_readings(client: &Client) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let response = client
        .get(&format!("{}/readings", BASE_URL))
        .send()
        .await?;

    let status = response.status();
    let readings: Vec<Reading> = response.json().await?;

    println!("✓ Get All Readings: {} - Found {} reading(s)", status, readings.len());
    Ok(readings)
}

async fn get_readings_by_sensor(client: &Client, sensor_id: u32) -> Result<Vec<Reading>, Box<dyn std::error::Error>> {
    let response = client
        .get(&format!("{}/sensors/{}/readings", BASE_URL, sensor_id))
        .send()
        .await?;

    let status = response.status();
    let readings: Vec<Reading> = response.json().await?;

    println!("✓ Get Readings for Sensor {}: {} - Found {} reading(s)", sensor_id, status, readings.len());
    for reading in &readings {
        println!("  - ID: {}, Value: {:.2}, Timestamp: {}", reading.id, reading.value, reading.timestamp);
    }

    Ok(readings)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("=== Testing API Endpoints ===\n");

    // 1. Health check
    health_check(&client).await?;
    println!();

    // 2. Create sensors
    println!("Creating sensors...");
    create_sensor(&client, "Temp-01".to_string(), "Room A".to_string(), SensorType::Thermometer).await?;
    create_sensor(&client, "Flow-01".to_string(), "Pipe B".to_string(), SensorType::FlowMeter).await?;
    create_sensor(&client, "Press-01".to_string(), "Tank C".to_string(), SensorType::PressureSensor).await?;
    println!();

    // 3. List all sensors
    println!("Listing all sensors...");
    let sensors = list_sensors(&client, None).await?;
    println!();

    // 4. List specific sensor
    if !sensors.is_empty() {
        println!("Listing specific sensor (ID: 0)...");
        list_sensors(&client, Some(0)).await?;
        println!();
    }

    // 5. Add readings
    println!("Adding readings...");
    add_reading(&client, 0).await?;
    add_reading(&client, 0).await?;
    add_reading(&client, 1).await?;
    println!();

    // 6. Get all readings
    println!("Getting all readings...");
    get_all_readings(&client).await?;
    println!();

    // 7. Get readings for specific sensor
    println!("Getting readings for sensor 0...");
    get_readings_by_sensor(&client, 0).await?;
    println!();

    // 8. Delete a sensor
    println!("Deleting sensor 2...");
    delete_sensor(&client, 2).await?;
    println!();

    // 9. List sensors again to verify deletion
    println!("Listing sensors after deletion...");
    list_sensors(&client, None).await?;

    println!("\n=== All tests completed! ===");

    Ok(())
}
