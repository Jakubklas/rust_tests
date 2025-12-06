use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

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



#[tokio::main]
async fn main() {
    // Create a listener on local host
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("Server running on localhost:8000...");

    // Create a persistent loop
    loop {
        // Create a 1024 byte buffer vector
        let mut buff = vec![0u8; 1024];
        
        // async accept for requests
        let (mut socket, adr) = listener.accept().await.unwrap();
        println!("Got a connection request from {adr}\n");
        
        // read the socket to a bytes buffer
        let n = socket.read(&mut buff).await.unwrap();
        let n = String::from_utf8_lossy(&buff[..n]);
        
        // deserialize the JSON using serde into a class
        let response: SensorReading = serde_json::from_str(&n).unwrap();
        
        // respond the string back
        let json= serde_json::to_string( &ServerResponse {
            success: true,
            msg: format!("READING ==> Sensor ID: {} | Value: {} {}", response.id, response.value, response.unit)  // ✓
        }).unwrap();

        socket.write_all(json.as_bytes()).await.unwrap();

    }

}