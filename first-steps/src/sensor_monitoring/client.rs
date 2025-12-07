mod libs;
use libs::{SensorReading, Response};

// use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpStream}};
use fastrand;
use tokio::time::{Duration, sleep};


async fn send_reading(id: u32) {
    for _ in 0..10 {
        let mut socket = TcpStream::connect("localhost:8000").await.unwrap();
        let mut buff = vec![0u8; 1024];

        let reading = SensorReading{
            sensor_id: id,
            value: fastrand::usize(5..100) as f32,
            location: "Factory_Location".to_string(),
            timestamp: 27857207170,
        };

        let json = serde_json::to_string(&reading).unwrap();
        socket.write_all( json.as_bytes() ).await.unwrap();

        let n = socket.read(&mut buff).await.unwrap();
        
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    
    println!("Connected to ther server...");

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for id in 0..20 {

        let h = tokio::spawn ( async move {
            send_reading(id).await;
        });

        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }


}