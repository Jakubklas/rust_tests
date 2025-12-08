use axum::{Json, Router, extract::{Path, State, Query}, http::StatusCode, response::IntoResponse, routing::{get, post, delete}};
use serde_json::de::Read;
use tokio::{net::TcpListener, sync::Mutex };
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use fastrand;

#[derive(Deserialize)]
struct FilterParams {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Deserialize, Clone, Serialize)]
struct Reading {
    id: u32,
    value: f32,
}

// #[derive(Debug)]
struct ApiState {
    readings: Mutex<Vec<Reading>>,
}

async fn list_readings(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<FilterParams>,
) -> impl IntoResponse {
    let readings = state.readings.lock().await;

    let filtered: Vec<Reading> = readings
        .iter()
        .filter(|r| {
            let min_cond = params.min.map_or(true, |m| r.value >= m);
            let max_cond = params.max.map_or(true, |m| r.value <= m);
            min_cond && max_cond
        })
        .cloned()
        .collect()
        ;

    (StatusCode::OK, Json(filtered)).into_response()
}

#[tokio::main]
async fn main() {
    // Pre-populate readings
    let initialization: Vec<Reading> = (1..10).map(|id| Reading{ id:id, value:fastrand::f32()*100.0 }).collect();

    // Initialize the ApiState
    let state = Arc::new(
        ApiState {
            readings: Mutex::new(initialization),
    });

    let service = Router::new()
        .route("/list", get(list_readings))
        .with_state(state)
        ;

    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("\nServer now listening...\n");
    axum::serve(listener, service).await.unwrap()

}
