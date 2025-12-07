mod libs;
use libs::{SensorReading, Response};
use tokio::{ net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt} };
use tokio::sync::mpsc;                 //TODO Is this a synchronout version of mpsc? I thought it was suppposed to be async?
use tokio::fs::OpenOptions;

async fn logger_task(mut rx: mpsc::Receiver<(SensorReading, String)>) -> Result<(), String> {
    // Serializes & saves logs into a log file readings.log
    while let Some((reading, path)) = rx.recv().await {
        
        let json = serde_json::to_string(&reading)
            .map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await
        .map_err(|e| e.to_string())?;
    
    file.write_all( json.as_bytes() ).await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}


async fn alert_task(mut rx: mpsc::Receiver<SensorReading>) {
    while let Some(reading) = rx.recv().await {
        let thres = 80.0;
        if reading.value > thres {
            println!("\nALERT => The reading for Sensor {} ({}) is above the threshold of {}.\n", 
                reading.sensor_id, reading.value, thres);
        }
    }
}

async fn handle_connection(mut socket: TcpStream, mut buf: Vec<u8>, tx_alert: mpsc::Sender<SensorReading>, tx_log: mpsc::Sender<(SensorReading, String)>, path: &str) {
    // Reading & deserializing the socket
    let n = socket.read(&mut buf).await.unwrap();
    let response: SensorReading = serde_json::from_str(
        &String::from_utf8_lossy(&buf[..n])
    ).unwrap();

    // Alert & Log tasks
    tx_alert.send(response.clone()).await.unwrap();
    tx_log.send((response.clone(), path.to_string())).await.unwrap();

    // Responding back to client
    socket.write(b"OK!").await.unwrap();
}


#[tokio::main]
async fn main() {
    let path = "/Users/jakubklas/rust_tests/first-steps/src/sensor_monitoring/readings.log";
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    println!("Server has started on localhost:8000...");
    
    // Channels setup
    let (tx_alert, rx_alert) = mpsc::channel(32);
    let (tx_log, rx_log) = mpsc::channel(32);
    tokio::spawn( logger_task(rx_log) );
    tokio::spawn( alert_task(rx_alert) );


    // Server loop
    loop {
        let (socket, adr) = listener.accept().await.unwrap();
        let buf = vec![0u8; 1024];
        let tx_alert_clone = tx_alert.clone();
        let tx_log_clone = tx_log.clone();

        tokio::spawn(async move {
            handle_connection(socket, buf, tx_alert_clone, tx_log_clone, path).await
        });        

    }

    
}