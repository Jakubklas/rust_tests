use axum::{Json, Router, extract::{Path, Query, State}, handler::HandlerWithoutStateExt, http::StatusCode, response::IntoResponse, routing::{delete, get, post}};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::{fmt::format, fs::{self, read}};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};



// ============ Readings & Data Structures ============

#[derive(Deserialize, Serialize, Clone)]
enum SensorType {
    Thermometer,
    FlowMeter,
    PressureSensor,
}
#[derive(Deserialize, Serialize, Clone)]
struct Sensor {
    id: u32,
    name: String,
    location: String,
    sensor_type: SensorType,
    created_at: u64,
}
#[derive(Deserialize, Serialize)]
struct Reading {
    id: u32,
    sensor_id: u32,
    value: f32,
    timestamp: u64,
}

// ============ API Related ============
#[derive(Deserialize, Serialize)]
struct AppData {
    sensors: Vec<Sensor>,
    readings: Vec<Reading>,
    next_sensor_id: u32,
    next_reading_id: u32,
}

// #[derive(Deserialize, Serialize, Clone)]
struct AppState {
    data: Mutex<AppData>,
    file_path: String,
}

#[derive(Deserialize)]
struct FilterParams {
    min: Option<f32>,
    max: Option<f32>,
    id: Option<u32>,
}

#[derive(Serialize)]
struct ServerResponse {
    success: bool,
    message: String,
}

// ============ METHODS ============

fn now() ->u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn save_to_file(state: &AppState) -> Result<(), String> {
    // Extracts data from state, serializes as JSON and saves to a file
    let data = state.data.lock().await;
    let json = serde_json::to_string_pretty(*&data)
        .map_err(|e| e.to_string())?;

    fs::write(&state.file_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn load_from_file() -> Result<AppState, String> {
    let content = fs::read_to_string("/Users/jakubklas/rust_tests/first-steps/src/api_project/state.json")
        .map_err(|e| e.to_string())?;

    let new_data: AppState = serde_json::from_str(&content)
        .map_err(|e| e.to_string())?;
    
    Ok(new_data)
}

async fn list_sensors(Query(params): Query<FilterParams>) -> impl IntoResponse {
    // GET Lists sensors matching filters if provided
    let state = load_from_file().await.unwrap();
    let sensors = &state.data.lock().await.sensors;
    
    let filtered: Vec<Sensor> = sensors.iter()
        .filter(|s| {
            let cond =params.id.map_or(true, |p| s.id == p);
            cond
        })
        .cloned()
        .collect();

    (StatusCode::OK, Json(filtered)).into_response()
}

async fn create_sensor(name: String, location: String, sensor_type: SensorType) -> impl IntoResponse {
    // POST Creates a new sensor
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    let sensor = Sensor {
            id: data.next_sensor_id,
            name: name,
            location: location,
            sensor_type: sensor_type,
            created_at: now(),
        };
    
    data.next_reading_id += 1;
    data.sensors.push(sensor);
    save_to_file(&state).await.unwrap();
    
    let resp = ServerResponse {
        success: true,
        message: "New sensor saved successfully.".to_string(),
    };

    (StatusCode::OK, Json(resp)).into_response()
}


async fn delete_sensor(id: u32) -> impl IntoResponse {
    // DELETE Deletes a sensor based on an id if exists
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    data.sensors.retain(|s: &Sensor| s.id != id);
    save_to_file(&state).await.unwrap();

    let resp = ServerResponse {
        success: true,
        message: format!("Sensor {id} was removed.").to_string(),
    };

    (StatusCode::OK, Json(resp)).into_response()
}

async fn add_reading(sensor_id: u32) -> impl IntoResponse {
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    let reading = Reading {
        id: data.next_reading_id,
        sensor_id: sensor_id,
        value: fastrand::f32()*100.0,
        timestamp: now(),
    };
    
    data.next_reading_id += 1;
    data.readings.push(reading);
    save_to_file(&state).await.unwrap();
    
    let resp = ServerResponse {
        success: true,
        message: format!("New reading added to sensor {sensor_id}.").to_string(),
    };

    (StatusCode::OK, Json(resp)).into_response()
}



async fn get_readings(id: Option<Path<u32>>) -> impl IntoResponse {
    // GET Lists readings by optional Path(id)
    let state = load_from_file().await.unwrap();
    let readings = &state.data.lock().await.readings;
    
    match id {
        Some(Path(sensor_id)) => {
            let filtered: Vec<&Reading> = readings.iter().filter(|r| r.sensor_id == sensor_id).collect();
            (StatusCode::OK, Json(filtered)).into_response()
        },
        None => {
            (StatusCode::OK, Json(readings.clone())).into_response()
        },
    }
}

async fn health_check() -> impl IntoResponse{
    (StatusCode::OK, Json(_)).into_response()
}


#[tokio::main]
async fn main() {
    todo!()
}