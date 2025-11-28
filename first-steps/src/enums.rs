
enum Scale {
    Celsius(f32),
    Farenheit(f32),
    Kelvin(f32),
}

impl Scale {

    fn to_celsius(&self) -> f32 {
        match self {
            Scale::Celsius(c) => *c,
            Scale::Farenheit(f) => (*f - 32.0) * 5.0 / 9.0,
            Scale::Kelvin(k) => *k - 273.15,
        }
    }

}

fn main() {
    let tmp1 = Scale::Celsius(25.0);
    let tmp2 = Scale::Farenheit(50.0);
    let tmp3 = Scale::Kelvin(75.0);

    let var = 67;
    let var_ref = &var;
    println!("{}", var_ref);

    println!("{}", tmp1.to_celsius());
    println!("{}", tmp2.to_celsius());
    println!("{}", tmp3.to_celsius());
}