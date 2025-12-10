use reqwest::Client;
use serde::{Serialize, Deserialize};

const BASE_URL: &str = "http://localhost:8000";

#[derive(Deserialize, Serialize, Debug)]
struct Reading {
    id: u32,
    sensor_id: u32,
    value: f32,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddReadingRequest {
    sensor_id: u32,
    value: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // let response = client
    //     .get(format!("{}/sensors/1/readings", BASE_URL))
    //     .send()
    //     .await
    //     .unwrap();

    for _ in 0..10_000 {
        let request = AddReadingRequest{
            sensor_id: fastrand::u32(1..9),
            value: fastrand::f32(),
        };

        let response = client
            .post(format!("{}/sensors/readings", BASE_URL))
            .json(&request)
            .send()
            .await
            .unwrap();

        println!("Full response:\n {:?}\n\n", &response);
        println!("URL: {}", response.url());
        println!("Status: {}", response.status());
        println!("Headers: {:?}", response.headers());
        
        let body:ServerResponse = response.json().await.unwrap();
        println!("Body: {:?}", body.message);   
    }

    // for r in body {
    //     println!("✓ READING => {:?}", r);
    // }
}

