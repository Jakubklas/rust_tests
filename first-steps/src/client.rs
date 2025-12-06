use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpStream}};
use fastrand;
use tokio::time::{Duration, sleep};

#[derive(Debug, Deserialize, Serialize)]
struct SensorReading {
    id: u32,
    value: f32,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerResponse {
    success: bool,
    msg: String,
}

async fn send_reading(id: u32) {
    for _ in 0..10 {
        let mut socket = TcpStream::connect("localhost:8000").await.unwrap();
        let mut buff = vec![0u8; 1024];

        let reading = SensorReading{
            id: id,
            value: fastrand::f32() * 100.0,
            unit: "Mps".to_string(),
        };

        let json = serde_json::to_string(&reading).unwrap();
        socket.write_all( json.as_bytes() ).await.unwrap();

        let n = socket.read(&mut buff).await.unwrap();
        let response: ServerResponse = serde_json::from_str(
            &String::from_utf8_lossy(&buff[..n])
        ).unwrap();

        println!("READING ==> Sensor ID: {} | Msg: {}\n\n", id, response.msg);
        
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    
    println!("Connected to ther server...");

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for id in 0..10 {

        let h = tokio::spawn ( async move {
            send_reading(id).await;
        });

        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }


}