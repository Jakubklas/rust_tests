use axum::{Router, routing::get, Json, extract::Path, routing::post};
use axum::{Router, routing::get, Json, extract::Path, routing::post};
use tokio::net::{TcpListener};
use tokio;
use serde::{Serialize, Deserialize};
use fastrand;

async fn hello() -> &'static str {
    "Hello World from Axum!"
}

#[derive(Serialize)]
struct Sensor {
    id: u32,
    value: f32,
}

#[derive(Serialize)]
struct ServerError {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct CreateSensor {
    location: String,
    value: f32,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    msg: String,
}


async fn get_sensor(Path(id): Path<u32>) -> Result<Json<Sensor>, Json<ServerError>> {
    if id > 5 {
        return Err(Json(ServerError { msg: "That sensor does not exist".to_string() }));
    }

    Ok(Json(Sensor { id, value: fastrand::f32() * 100.0 }))
}

async fn create_sensor(Json(payload): Json<CreateSensor>) -> Json<Response> {
    println!("Received {} with value {}", payload.location, payload.value);

    Json(Response {
        success: true,
        msg: format!("Created a sensor in {} with value {}", payload.location, payload.value),
    })
}   


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/sensor/:id", get(get_sensor))
        .route("/create_sensor", post(create_sensor));
    
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("Server running on 'localhost:8000'.");

    let _ = axum::serve(listener, app).await.unwrap();


}