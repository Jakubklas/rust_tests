

fn matcher(score: i32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        50..=69 => 'D',
        _ => 'F',
    }

}

fn num_matcher(x: i32) -> String {
    match x {
        1 => "One!",
        2..10 => "Sub-10!",
        10.. => "Over 9999!",
        _ => "Whatever..."
    }.to_string()

}


fn arrays() -> [i32; 5] {
    let mut arr = [0; 5];
    let mut add: i32 = 0;
    for i in 0..arr.len() {
        add +=1;
        arr[i] += add;
    }
    arr
}

fn sum_array(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}


fn loop_pattern() {
    let mut counter: i64 = 0;
    loop {
        counter += 1;
        if counter % 1000000 == 0 {
            println!("Millions: {counter}")
        }
    }
}

fn temp_converter(val: f32, unit: &str) {
    let unit_lower = unit.to_lowercase();

    if !["c", "f"].contains(&unit_lower.as_str())  {
        println!("Unit {unit} not available");
        return
    }
    
    match unit_lower.as_str() {
        "c" => println!("Farenheit equivalent of {} Celsuis is {} degrees", val, (val * 9.0 / 5.0) + 32.0),
        "f" => println!("Caelsius equivalent of {} Farenheir is {} degrees", val, (val - 32.0) * 5.0 / 9.0),
        _ => {},
    }
}


fn fib(n: i32) -> i32 {
    if [0, 1].contains(&n) {
        return n;
    }

    let mut a = 0;
    let mut b = 1;
    
    for int in 2..=n {
        let temp = a + b;
        a = b;
        b +=temp;
    } 
    
    b

}



fn main() {
    // println!("Sum of array {:?} is {:?}", arrays(), sum_array(arrays()));
    // temp_converter(5.0, "C");
    fib(5);
}
