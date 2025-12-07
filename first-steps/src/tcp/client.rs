use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use tokio::time::{Duration, sleep};

#[derive(Debug, Serialize, Deserialize)]
struct Reading {
    id: u32,
    location: String,
    value: f32,
    units: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerResponse {
    id: u32,
    reading: Reading,
}

async fn send_request(ip: &str) {
    for _ in 0..50 {
        let mut socket = TcpStream::connect(ip).await.unwrap();
        let mut buffer = vec![0u8; 1024];

        let n = socket.read(&mut buffer).await.unwrap();

        let text = String::from_utf8_lossy(&buffer[..n]);
        let content: ServerResponse = serde_json::from_str(&text).unwrap();

        let id: u32 = content.id;
        let reading: Reading = content.reading;
        println!("Response ID: {} | READING => Location: {}, Value: {} {}", id, reading.location, reading.value, reading.units);
        
        socket.write_all(b"Thanks dude!").await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
    
}



#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    
    for _ in 0..3 {
        let h = tokio::spawn( async move {
            send_request("localhost:8000").await;
        });

        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }

}