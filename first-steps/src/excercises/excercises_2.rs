

fn parse_temp(input: &str) -> Option<(f32, char)> {
    // Split & collect str into a vector
    let parts: Vec<&str> = input.split(" ").collect();

    if parts.len()<2 {
        return None;
    }

    // Parse each part of the vector
    let val = parts[0].parse::<f32>().ok()?;
    let unit = parts[1].chars().next()?;

    // Return Some
    Some((val, unit))
}

// fn counter() {
//     let mut sum = 0;
//     let mut increments = 0;

//     loop {
//         if (sum % 100000000 == 0) & (sum != 0) {          // Every 1 Billion loops
//             increments += 1;
//             println!("100 Millions {}", increments)
//         }

//         // if sum >= (1000000000) {
//         //     break
//         // }
//         sum += 1;
//     }
// }

// "10+5"
fn calculate(input: &str) -> Option<i32> {
    // Hint: Split into Vec<&str>
    let parts: Vec<&str> = input.split(" ").collect();

    if parts.len() < 3 {
        return None;
    }

    // Hint: Parse numbers
    let (num_1, num_2) = (
        parts[0].parse::<i32>().ok()?,
        parts[2].parse::<i32>().ok()?
    );
    
    // Hint: Check for division by zero
    

    // Hint: Use match on operator
    match parts[1] {
        "+" => Some(num_1 + num_2),
        "-" => Some(num_1 - num_2),
        "*" => Some(num_1 * num_2),
        "/" => {
            if num_2 == 0 {
                return None;
            } else {
                Some(num_1 / num_2)
            }
        },
        _ => None,
    }
}


fn main() {
    let tests = vec![
        "10 + 5",
        "20 - 8",
        "6 * 7",
        "100 / 4",
        "50 / 0",    // Should return None
        "15 % 3",    // Unknown operator - None
        "invalid",
        "10 +",      // Missing number - None
    ];
    
    for test in tests {
        match calculate(test) {
            Some(result) => println!("{} = {}", test, result),
            None => println!("{} = ERROR", test),
        }
    }
}