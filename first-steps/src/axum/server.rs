use axum::{Json, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post, delete}};
use tokio::{net::TcpListener, sync::Mutex };        //What is Mutex? Is it some sort of a struct allowing for mutable variable in an async mode?
use std::sync::Arc;           //What is arc?

struct AppState {
    readings: Mutex<Vec<Reading>>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Reading {
    id: u32,
    value: f32
}

#[derive(Debug, serde::Serialize)]
struct ServerMessage {
    msg: String,
}


async fn add_reading(State(state): State<Arc<AppState>>, Json(payload): Json<Reading>) -> impl IntoResponse {
    let reading = Reading {
        id: payload.id,
        value: payload.value,
    };
    
    let mut readings = state.readings.lock().await;
    readings.push(reading);
    
    let msg = ServerMessage { msg: "Reading added to the state".to_string() };
    (StatusCode::OK, Json(msg)).into_response()
}

async fn get_reading(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> impl IntoResponse {
    let readings = state.readings.lock().await;
    let mut body = Vec::new();

    for r in readings.iter() {
        if r.id == id {
            body.push(r.clone());
        }
    }

    (StatusCode::OK, Json(body)).into_response()
}

async fn delete_reading(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> impl IntoResponse {
    let mut readings = state.readings.lock().await;
    let original_len = readings.len();
    
    readings.retain(|r| r.id != id);
    
    let deleted = original_len - readings.len();
    let msg = ServerMessage {msg: format!("Deleted {deleted} readings with id {id}...")};

    (StatusCode::OK, Json(msg)).into_response()
}

async fn list_readings(State(state): State<Arc<AppState>>) -> Json<Vec<Reading>> {
    let readings = state.readings.lock().await;      //What is .lock() doing?
    Json(readings.clone())        //Why do we have to clone? To prevent async race conditions?
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        readings: Mutex::new(vec![]),
    });

    let app = Router::new()
        .route("/", get("Sensors API v2".to_string()))
        .route("/list", get(list_readings))
        .route("/add", post(add_reading))
        .route("/get_reading/:id", get(get_reading))
        .route("/delete/:id", delete(delete_reading))
        .with_state(state)
        ;

    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("\nServer now listening...\n");

    axum::serve(listener, app).await.unwrap();
}