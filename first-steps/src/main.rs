fn main() {
    let inputs = vec!["25.5", "invalid", "30.0", "abc", "-5.5"];
    
    for input in inputs {
        // TODO: Call parse_temp and handle Result with match
        // If Ok(temp), convert and print
        // If Err(msg), print the error
        let result = parse_temp(input);

        match result {
            Ok(temp) => println!("{} -> {:?}", input, temp),
            Err(msg) => println!("{} -> {:?}", input, msg),
        }
    }
}

// TODO: Write this function
// Returns Result<f32, String>
// Ok(parsed_value) if valid, Err("error message") if invalid
fn parse_temp(input: &str) -> Result<f32, String> {
    // Hint: input.parse::<f32>() returns Result<f32, ParseFloatError>
    // You can use match on that result
    // Return Ok(value) or Err("Invalid temperature format".to_string())
    let result = input.parse::<f32>();
    
    match result {
        Ok(num) => Ok(converter(num)),
        Err(_) => Err("Invalid temperature format".to_string())
    }
}

fn converter(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}


// **Expected output:**

// 25.5°C = 77.9°F
// Error: Invalid temperature format
// 30.0°C = 86.0°F
// Error: Invalid temperature format
// -5.5°C = 22.1°F