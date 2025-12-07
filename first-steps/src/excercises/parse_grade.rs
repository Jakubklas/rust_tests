

fn parse_grade(input: &str) -> Option<(String, i32)> {
    // Parse string into a vector of strings
    let parts: Vec<&str> = input.split(" ").collect();
    // parse the subje
    ct & grade
    if parts.len()<2 {
        return None;
    }

    let subject = String::from(parts[0]);
    let grade = parts[1].parse::<i32>().ok()?;       //The '?' should imediately return None if the string is not parseable to i32
    // return an Some(String, grade?)
    Some((subject, grade))

}



fn main() {
    let test = vec![
        "Science 8",
        "Invalid",
        "Math 54",
        "Art 1",
    ];

    for i in test {
        println!("{:?}", parse_grade(i));
    }
}