use std::arch::x86_64::_mm_permute_ps;
use std::f32::consts::E;
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


async fn read_sensor_channel(id: u32, tx: mpsc::Sender<(u32, Result<f32, String>)>) {
    let reading = async {
        if id == 3 {
            return Err("Sensor designed to fail".to_string());
        }
        let time = if id == 2 { 10 } else { 1 };
        sleep(Duration::from_secs(time)).await;
        Ok(id as f32 * 785.12)
    };

    let result = match timeout(Duration::from_secs(5), reading).await {
        Ok(r) => r,
        Err(_) => Err("Timeout".to_string()),
    };

    let _ = tx.send((id, result)).await;
}


#[tokio::main]
async fn main() {  
    // Create the channel
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn tasks that will each send data to the channel's receiver
    let sensor_ids = vec![1, 2, 3, 4, 5];
    
    for id in sensor_ids {
        let tx_clone = tx.clone();
        tokio::spawn( async move { 
            read_sensor_channel(id, tx_clone).await 
        });
    }

    drop(tx);

    while let Some((id, val)) = rx.recv().await {
        
        match val {
            Ok(val) => println!("Sensor {} | Value {}", id, val) ,
            Err(e) => println!("Sensor {} | Value {}", id, e) , 
        }

    }
}