
// The init statement with data
struct Rectangle {                                          //TODO: Should we initialize some of these vars with values here?
    height: f32,
    width: f32,
}

// Methods equivalent
impl Rectangle {
    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn edge_lenght(&self, mult: f32) -> f32 {
        let mult = mult as f32;
        mult * 2.0 * (self.height + self.width)             //TODO: can I simply cast the mult var into a float32 values?
    }

    fn new(size: f32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}




fn main() {
    println!("Hey");
    let rec = Rectangle::new(30.0);    //TODO: Why do we need the new() impl? What is the ::create_new() notation for i.e. why not hae the same as with any other impl (e.g; rec.area())?
    println!("Area: {}, Outline: {}", rec.area(), rec.edge_lenght(3.0));
}