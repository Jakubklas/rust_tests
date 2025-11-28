
struct TemperatureLogger {
    readings: Vec<f32>,
    location: String
}

impl TemperatureLogger {
    fn new(location: String) -> TemperatureLogger {
        TemperatureLogger {
            readings: Vec::new(),
            location: location,
        }
    }

    fn add_reading(&mut self, temp: f32) {
        self.readings.push(temp);
    }

    fn average(&self) -> Option<f32> {        
        if self.readings.is_empty() {
            println!("Cannot average an empty vector.");
            return None;
        } else {
            Some(self.readings.iter().sum::<f32>() / self.readings.len() as f32)
        }
    }

    fn max(&self) -> Option<f32> {
        if self.readings.is_empty() {
            println!("Cannot take max from an empty vector.");
            return None;
        } else {
            let mut max = self.readings[0];
            for &r in &self.readings[1..] {
                if r > max {
                    max = r;
                }
            }
            Some(max)
        }
    }

    fn print_summary(&self) {
        println!("Location: {}", self.location);
        println!("Location: {}", self.readings.len());

        match self.average() {
            Some(avg) => println!("Average temperature {}.", avg),
            None => println!("No average avalable"),
        }

        match self.max() {
            Some(max) => println!("Max temperature {}.", max),
            None => println!("No average avalable"),
        }
    }
}


fn main() {
    let mut london = TemperatureLogger::new("London".to_string());
    
    london.add_reading(15.5);
    london.add_reading(18.2);
    london.add_reading(12.8);
    london.add_reading(20.1);
    
    london.print_summary();
}
