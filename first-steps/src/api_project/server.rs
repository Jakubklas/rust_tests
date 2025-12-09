use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post}};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{read_to_string, write};



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
#[derive(Deserialize)]
struct CreateSensorRequest {
    name: String,
    location: String,
    sensor_type: SensorType,
}

#[derive(Deserialize, Serialize, Clone)]
struct Reading {
    id: u32,
    sensor_id: u32,
    value: f32,
    timestamp: u64,
}
#[derive(Deserialize)]
struct CreateReadingRequest {
    sensor_id: u32,
    value: f32,
}

// ============ API Data Structures ============

#[derive(Deserialize, Serialize, Clone)]
struct AppData {
    sensors: Vec<Sensor>,
    readings: Vec<Reading>,
    next_sensor_id: u32,
    next_reading_id: u32,
}

struct AppState {
    data: Mutex<AppData>,
    file_path: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct FilterParams {
    min: Option<f32>,
    max: Option<f32>,
    id: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ServerResponse {
    success: bool,
    message: String,
}

// ============ HANDLERS ============

fn now() ->u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn save_to_file(state: &AppState) -> Result<(), String> {
    // Extracts data from state, serializes as JSON and saves to a file
    let data = state.data.lock().await;
    let json = serde_json::to_string_pretty(&*data)
        .map_err(|e| e.to_string())?;

    std::fs::write(&state.file_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn load_from_file() -> Result<AppState, String> {
    let file_path = "src/api_project/state.json";

    let content = std::fs::read_to_string(file_path)
            .map_err(|e| e.to_string())?;

        // Deserialize into AppData only
        let data: AppData = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;

        // Then construct AppState with a new Mutex wrapping the data
        Ok(AppState {
            data: Mutex::new(data),
            file_path: file_path.to_string(),
        })
}

// ============ METHODS ============

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


async fn create_sensor(Json(request): Json<CreateSensorRequest>) -> impl IntoResponse {
    // POST Creates a new sensor from a JSON payload
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    let sensor = Sensor {
            id: data.next_sensor_id,
            name: request.name,
            location: request.location,
            sensor_type: request.sensor_type,
            created_at: now(),
        };

    data.next_sensor_id += 1;
    data.sensors.push(sensor);
    drop(data); // Explicitly drop the lock

    save_to_file(&state).await.unwrap();

    let resp = ServerResponse {
        success: true,
        message: "New sensor saved successfully.".to_string(),
    };

    (StatusCode::OK, Json(resp)).into_response()
}


async fn delete_sensor(Path(id): Path<u32>) -> impl IntoResponse {
    // DELETE Deletes a sensor based on an id if exists
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    data.sensors.retain(|s: &Sensor| s.id != id);
    drop(data); // Explicitly drop the lock

    save_to_file(&state).await.unwrap();

    let resp = ServerResponse {
        success: true,
        message: format!("Sensor {id} was removed."),
    };

    (StatusCode::OK, Json(resp)).into_response()
}


async fn add_reading(Json(request): Json<CreateReadingRequest>) -> impl IntoResponse {
    let state = load_from_file().await.unwrap();
    let mut data = state.data.lock().await;

    let id_exists = data.sensors.iter().any(|s| s.id == request.sensor_id );

    if !id_exists {
        drop(data); // Drop lock before returning
        let resp = ServerResponse {
            success: false,
            message: format!("Sensor {} does not exist.", request.sensor_id),
        };
        return (StatusCode::NOT_FOUND, Json(resp)).into_response();
    }

    let reading = Reading {
        id: data.next_reading_id,
        sensor_id: request.sensor_id,
        value: request.value,
        timestamp: now(),
    };

    data.next_reading_id += 1;
    data.readings.push(reading);
    drop(data); // Explicitly drop the lock

    save_to_file(&state).await.unwrap();

    let resp = ServerResponse {
        success: true,
        message: format!("New reading added to sensor {}.", request.sensor_id),
    };

    (StatusCode::OK, Json(resp)).into_response()
}


async fn get_all_readings() -> impl IntoResponse {
    // GET Lists all readings
    let state = load_from_file().await.unwrap();
    let readings = &state.data.lock().await.readings;
    (StatusCode::OK, Json(readings.clone())).into_response()
}

async fn populate_state_file() -> Result<(), String> {
    let path = r"C:\Users\jklas\rust_tests\first-steps\src\api_project\state.json";
    
    let data = AppData {
            sensors: vec![],
            readings: vec![],
            next_reading_id: 0,
            next_sensor_id: 0,
    };

    let json_state = serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?;

    let _ = std::fs::write(path, json_state);
    

    Ok(())
}


async fn get_readings_by_sensor(Path(sensor_id): Path<u32>) -> impl IntoResponse {
    // GET Lists readings for a specific sensor
    let state = load_from_file().await.unwrap();
    let readings = &state.data.lock().await.readings;
    let filtered: Vec<Reading> = readings.iter()
        .filter(|r| r.sensor_id == sensor_id)
        .cloned()
        .collect();
    (StatusCode::OK, Json(filtered)).into_response()
}


async fn health_check() -> impl IntoResponse{
    let resp = ServerResponse {
        success: true,
        message: format!("Server is running.").to_string(),
    };

    (StatusCode::OK, Json(resp)).into_response()
}


// ============ MAIN ORCHESTRATION ============

#[tokio::main]
async fn main() {
    // Shared, locally saved state
    // let state = load_from_file().await.unwrap();

    // Shared in-memory state
    // let state = Arc::new( AppState {
    //     data: Mutex::new(data),
    //     file_path: r"/Users/jakubklas/rust_tests/first-steps/src/api_project/state.json".to_string(),
    // }
    // );

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sensors", get(list_sensors))
        .route("/sensors/:id", delete(delete_sensor))
        .route("/sensors", post(create_sensor))
        .route("/readings", get(get_all_readings))
        .route("/sensors/:id/readings", get(get_readings_by_sensor))
        .route("/sensors/readings", post(add_reading))
        // .with_state(&state)
        ;

    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("\nClearing latest state...");
    let _ = populate_state_file().await;

    println!("Server now listening on localhost:8000...");
    let _ = axum::serve(listener, app).await;
}