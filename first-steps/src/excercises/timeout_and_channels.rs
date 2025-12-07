use std::arch::x86_64::_mm_permute_ps;
use std::fs::read;
use std::io::Read;
use std::result;

use tokio::time::{Duration, sleep, timeout};
use tokio::sync::mpsc;


async fn read_sensor(id: u32) -> Result<f32, String> {
    sleep(Duration::from_secs(1)).await;
    if id == 3 {
        return Err("Failed to read sensor".to_string());
    }
    
    Ok(id as f32 * 10.16)
}


async fn slow_sensor(id: u32) -> f32 {
    
    let time = if id == 3 { 5 } else {1};
    sleep(Duration::from_secs(time)).await;
    
    id as f32 * 91.76 
}


async fn send_hello() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    let msg = rx.recv().await.unwrap();
    println!("{}", msg);
}


async fn read_sensor_channel(id: u32, tx: mpsc::Sender<(u32, f32)>) {
    sleep(Duration::from_secs(1 * id as u64)).await;
    let val = id as f32 * 785.12;

    tx.send((id, val)).await.unwrap();
}


#[tokio::main]
async fn main() {  
    // Create the channel
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn tasks that will each send data to the channel's receiver
    let sensor_ids = vec![1, 2, 3, 4, 5];
    let mut handles = Vec::new();
    
    for id in sensor_ids {
        let tx_clone = tx.clone();
        let handle = tokio::spawn(async move {
            read_sensor_channel(id, tx_clone).await;
        });
        handles.push(handle);
    }

    for h in handles {
        match h.await {
            Ok(_) => {
                if let Some(received) = rx.recv().await {
                    println!("{:?}", received);
                } else {
                    println!("No message received");
                }
            },
            Err(e) => println!("{}", e), 
        }
    }
}