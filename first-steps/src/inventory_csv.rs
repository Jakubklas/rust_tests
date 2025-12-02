use core::error;


struct Product {
    name: String,
    quantity: u32,      //What is u32 and how does it differ from f32 and i32?
    price: f32,
}

struct ParseResult {
    products: Vec<Product>,
    errors: Vec<String>,
}

impl ParseResult {
    fn new() -> ParseResult {
        ParseResult { products: Vec::new(), errors: Vec::new() }
    }

    fn parse_data(&mut self, data: &str) -> Result<(), String> {

        for ln in data.lines() {
            let cols: Vec<&str> = ln.split(",").collect();
            
            if cols.len() != 3 {
                self.errors.push("Invalid row, less/more than 3 columns".to_string());
                continue;
            }

            let name = cols[0].trim();
            let name = if name.is_empty() { "unknown" } else { name }.to_string();
            
            let quantity = match cols[1].parse::<u32>() {
                Ok(v) => v,
                Err(m) => {
                    self.errors.push(m.to_string());
                    continue;
                },
            };

            let price = match cols[2].parse::<f32>() {
                Ok(v) => v,
                Err(m) => {
                    self.errors.push(m.to_string());
                    continue;
                },
            };

            self.products.push(Product { 
                name: name, 
                quantity: quantity, 
                price: price, 
            })

        }
        Ok(())
    }

    fn report(&self) {
        print!("PRODUCT | PRICE | QUANTITY\n");
        
        for i in &self.products {
            print!("{} | {} | {}\n", i.name, i.price, i.quantity);
        }
    }


}



fn main() {
    let data = "
    Apple,50,1.25
    Bread,twenty,2.00
    Milk,30,1.75
    ,10,5.00
    Cheese,25
    Eggs,12,3.50
    Butter,-5,4.00
    ";

    let mut parser = ParseResult::new();
    let _ = parser.parse_data(data);
    parser.report();
    
}