

fn parse_number(s: &str) -> Result<i32, String> {
    if let Ok(n) = s.parse::<i32>() {
        return Ok(n);
    }
    Err("Could not parse the numebr for string".to_string())
}

fn double(n: i32) -> Result<i32, String> {
    let dbl = n * 2;
    if dbl < 1_000 {
        return Ok(dbl);
    }
    Err("Overflow".to_string())
}

fn process(input: &str) -> Result<i32, String> {
    let n = parse_number(input)?;
    let dbl = double(n)?;

    Ok(dbl)
}

fn main() {
    for i in ["44", "12", "501", "abs", "331"] {
        match process(i) {
            Ok(n) => println!("{} --> {}", i, n),
            Err(e) => println!("{} --> {}", i, e),
        }
    }
}