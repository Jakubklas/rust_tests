use std::result;

// use chrono::Duration as ChronoDuration;
use tokio::{io::DuplexStream, time::{Duration, sleep}};


async fn sensor_a() {
    let wait = Duration::from_secs(2);
    sleep(wait).await;
    println!("Sensor A -> 125 psi");
}

async fn sensor_b() {
    let time = Duration::from_secs(2);
    sleep(time).await;
    println!("Sensor B -> 65 psi")
}

async fn concurrent() {
    tokio::join!(sensor_a(), sensor_b());
}

async fn spawning() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        42
    });

    println!("Doing other work");

    let result = handle.await.unwrap_err();

    println!("Task returned: {}", result);
}

async fn wait_fn(sec:u64) {
    println!("Function started & will wait for {sec} seconds");
    sleep(Duration::from_secs(sec)).await;
    println!("BOOM!");
}

async fn sensor(sec:u64, val:i32) -> i32 {
    println!("Function started & will wait for {sec} seconds");
    sleep(Duration::from_secs(sec)).await;
    val
}


async fn read_sensor(id: u32) -> (u32, f32) {
    sleep(Duration::from_millis(100*id as u64)).await;
    (id, id as f32 * 10.5)
} 




#[tokio::main]          // What does this decorator ACTUALLY do?
async fn main() {
    // spawning().await
    // tokio::join!(wait_fn(1), wait_fn(2), wait_fn(3));
    // let (pressure, temperature, flow_rate) = tokio::join!(sensor(2, 45), sensor(3, 10), sensor(4, 99));
    // println!("Pressure: {}, Temp: {}, Flow: {}", pressure, temperature, flow_rate);

    let sensor_ids = vec![1, 2, 3, 4, 5];
    let mut handles = Vec::new();

    for n in sensor_ids {
        let handle = tokio::spawn(async move {read_sensor(n).await});         // Technically we're running tasks a.) asynchronously and b.) on 5 separate threads combining async + parallelism benefits?
        handles.push(handle);
    }

    for h in handles { 
        let (id, val) = h.await.unwrap().await;                      // Why unwrap() and not '?' ?  // Why do we need 2x await here?
        println!("Got the result {} --> {}", id, val )
    }

}