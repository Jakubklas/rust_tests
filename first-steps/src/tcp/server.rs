use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use fastrand;

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

async fn gen_reading(id: u32) -> Result<Reading, String> {
    let loc = ["Boiler Room", "Plant Floor", "Cooling Room"];
    let idx = fastrand::usize(0..loc.len());

    let reading = Reading {
        id: id,
        location: loc[idx].to_string(),
        value: fastrand::u32(25..95) as f32,
        units: "F".to_string(),
    };

    Ok(reading)
}


async fn start_server(ip: &str) -> Result<(), String> {
    let listener = TcpListener::bind(ip).await
        .map_err(|e| e.to_string())?;
    println!("Server has started on {}", ip);

    loop {
        let mut buffer = vec![0u8; 1024];
        let (mut socket, addr) = listener.accept().await
            .map_err(|e| e.to_string())?;

        // let n = socket.read(&mut buffer).await
        //     .map_err(|e| e.to_string())?;

        let id = fastrand::u32(0..10);
        let reading = gen_reading(id).await
            .map_err(|e| e.to_string())?;

        println!("Reading sent => Sensor ID {}, Location: {}, Value: {} {}\n", reading.id, reading.location, reading.value, reading.units);
        let response = ServerResponse {
            id: id,
            reading: reading,
        };


        let json = serde_json::to_string(&response).unwrap();
        socket.write_all( json.as_bytes() ).await
            .map_err(|e| e.to_string())?;

        let n = socket.read(&mut buffer).await
            .map_err(|e| e.to_string())?;

            let client_response = String::from_utf8_lossy(&buffer[..n]);
            println!("Client said: {}\n", client_response);
    }

    Ok(())

}


#[tokio::main]
async fn main() {
    match start_server("localhost:8000").await {
        Ok(_) => println!("Server is running..."),
        Err(e) => println!("Server failed to start due to error: {}", e),
    }
}