use std::result;

use tokio::time::{Duration, sleep, timeout};


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


#[tokio::main]
async fn main() {  
    let sensor_ids = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];             

    for i in sensor_ids {
        let result = timeout(Duration::from_secs(2), slow_sensor(i));

        let h = tokio::spawn(
            async move { result.await }
        );                      
        handles.push(h)
    }

    for h in handles {
        match h.await.unwrap() {
            Ok(r) => println!("{}", r),
            Err(e) => println!("{}", e),
        }
    }
}