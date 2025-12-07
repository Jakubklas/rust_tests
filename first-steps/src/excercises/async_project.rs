use tokio::time::{Duration, sleep, timeout};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::fs;



#[derive(Debug, Clone)]
enum SensorType {
    Temperature,
    Pressure,
    FlowRate,
}
enum Message {
    Reading { id: u32, sensor_type: SensorType, value: f32, location: String },
    Alert { id: u32, sensor_type: SensorType, value: f32, threshold: f32, location: String },
    Error { id: u32, sensor_type: SensorType, location: String, error: String },
}
struct Sensor {
    id: u32,
    sensor_type: SensorType,
    location: String,
    on: bool,
}
struct Stats {
    readings: u32,
    alerts: u32,
    errors: u32,
}


impl Sensor {

    async fn run(&self, tx: mpsc::Sender<Message>) {

        let thres = match &self.sensor_type {
            SensorType::Temperature => 80.0,
            SensorType::Pressure => 150.0,
            SensorType::FlowRate => 500.0,
        };

        for _ in 0..2000 {
            // Simulate sensor taking time to generate a reading (10% chance of slow response)
            let wait_time = if fastrand::i32(0..1_00) > 90 { 2_000 } else { 1 };

            // Try to generate sensor reading with timeout
            let reading_result = timeout(Duration::from_millis(500), sleep(Duration::from_millis(wait_time))).await;

            match reading_result {
                Ok(_) => {
                    // Sensor responded in time, generate and send reading
                    let rand = fastrand::f32() * 100.0;

                    let value = match self.sensor_type {
                        SensorType::Temperature => rand.min(100.0),
                        SensorType::Pressure => rand.min(200.0),
                        SensorType::FlowRate => rand.min(550.0),
                    };

                    // Determining the message
                    let msg = if value > thres {
                        Message::Alert { id: self.id, sensor_type: self.sensor_type.clone(), value, threshold: thres, location: self.location.clone() }
                    } else if (90.0..105.0).contains(&value) { // Randomized sensor failure
                        Message::Error { id: self.id, sensor_type: self.sensor_type.clone(), location: self.location.clone() , error: "Randomized sensor failure...".to_string() }
                    } else {
                        Message::Reading { id: self.id, sensor_type: self.sensor_type.clone(), value, location: self.location.clone() }
                    };

                    let _ = tx.send(msg).await;
                }
                Err(_) => {
                    // Timeout occurred - sensor took too long to respond
                    let timeout_error = Message::Error {
                        id: self.id,
                        sensor_type: self.sensor_type.clone(),
                        location: self.location.clone(),
                        error: "Sensor timeout - response time exceeded limit".to_string(),
                    };
                    let _ = tx.send(timeout_error).await;
                }
            }

            // Small delay between readings
            sleep(Duration::from_millis(1)).await;
        }
    }
}



impl Stats {
    fn new() -> Stats {
        Stats { readings: 0, alerts: 0, errors: 0 }
    }
}


#[tokio::main]
async fn main() {
    // Get the mpsp sender and receiver
    let (tx, mut rx) = mpsc::channel::<Message>(32);       // How to decide the size of the buffer and wha are the trade-offs
    
    // Create the sensors
    let sensors = vec![
        Sensor { id: 1, sensor_type: SensorType::Temperature, location: "Boiler Room".to_string(), on: true },     // I can never decide whether a struct needs a ::new() method or no
        Sensor { id: 2, sensor_type: SensorType::Pressure, location: "Main Tank".to_string(), on: true },
        Sensor { id: 3, sensor_type: SensorType::FlowRate, location: "Inlet Valve".to_string(), on: true },
        Sensor { id: 4, sensor_type: SensorType::Temperature, location: "Cooling Unit".to_string(), on: true },
    ];

    // Storing sensor locations for the report later
    let mut sensor_locations: HashMap<u32, String> = HashMap::new();
    for s in &sensors {
        sensor_locations.insert(s.id, s.location.clone());
    }

    // Spawn a run furute per sensor with a loop
    for s in sensors {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            s.run(tx_clone).await;
        });
    }

    // Drop the original sender so the receiver knows when all senders are done
    drop(tx);

    // While Some(val), keep receiving the messages
    let mut stats: HashMap<u32, Stats> = HashMap::new();

    while let Some(msg) = rx.recv().await {
        // Print & log in a HashMap<(Sensor, Stats)>
        
        match msg {
            Message::Reading { id, sensor_type, value, location } => {
                println!("[{}] | READING | Sensor {} ({:?}): {}", location, id, sensor_type, value);
                stats.entry(id).or_insert(Stats::new()).readings += 1;
            },
            Message::Alert { id, sensor_type, value, threshold, location } => {
                println!("[{}] | ALERT | Sensor {} ({:?}): {} (threshold: {})", location, id, sensor_type, value, threshold);
                stats.entry(id).or_insert(Stats::new()).alerts += 1;
            },
            Message::Error { id, sensor_type, location, error } => {
                println!("[{}] | ERROR | Sensor {} ({:?}) | Message: {}", location, id, sensor_type, error);
                stats.entry(id).or_insert(Stats::new()).errors += 1;
            },        
        };
    }

    let mut report = String::from("Factory Sensor Report\n=====================\n");
    let path = String::from(format!(r"C:\Users\jklas\rust_tests\first-steps\sensors_report.txt"));  

    for (id, stat) in stats {
        let location = sensor_locations.get(&id);
        let line = format!(
            "Sensor {} ({}): {} readings, {} alerts, {} errors\n",
            id, location.unwrap_or(&String::from("Unknown")), stat.readings, stat.alerts, stat.errors
        );
        report.push_str(&line);
    }

    match fs::write(&path, report) {
        Ok(_) => println!("\nReport saved successfully! to {}", &path),
        Err(e) => println!("\nFailed saving report: {}", e),
    }

}
