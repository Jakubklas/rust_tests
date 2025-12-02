use std::fs;

struct LogSummary {
    info_count: i32,
    warn_count: i32,
    error_count: i32,
    misc: i32,
    errors: Vec<String>,
}

impl LogSummary {
    fn new() -> LogSummary {
        LogSummary { info_count: 0, warn_count: 0, error_count: 0, misc: 0, errors: Vec::new() }
    }

    fn analyze_log(&mut self, path: &str) -> Result<(), String> {
        let file = fs::read_to_string(path)
            .map_err(|_| "The specified file path could not be found or read".to_string())?;

        for line in file.lines() {
            let parts: Vec<&str> = line.splitn(2, ":").collect();
            if parts.len() < 2 { continue; }
            let log_type: &str = &parts[0].trim().to_lowercase();
            let msg = parts[1]; 
            
            match log_type {
                "info" => self.info_count += 1,
                "warn" => self.warn_count += 1,
                "error" => {
                    self.error_count += 1;
                    self.errors.push(msg.to_string());
                },
                _ => self.misc += 1,
            }
        }
        Ok(())
    }

    fn print_summary(&self) {
        print!("Info Count: {},\nWarn Count: {}\nError Count: {}\nMisc Count: {}\n", self.info_count, self.warn_count, self.error_count, self.misc);
        print!("\nErrors collected:\n");
        for e in &self.errors {
            println!("==> {e}");
        }
    }
}

fn main() {
    let path = String::from("C:/Users/jklas/rust_tests/first-steps/app.log");
    let mut logsum = LogSummary::new();

    match logsum.analyze_log(&path) {
        Ok(_) => logsum.print_summary(),
        Err(e) => println!("{}", e)
    }
}