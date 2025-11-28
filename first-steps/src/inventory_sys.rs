
struct Product {
    name: String,
    price: f32,
    quantity: u32,
}

struct Inventory {
    products: Vec<Product>
}


impl Product {
    fn new(name:String, price:f32, quantity:u32) -> Product {
        Product{name, price, quantity}
    }

    fn total_value(&self) -> f32 {
        self.price * (self.quantity as f32)
    }

}

impl Inventory {
    fn new() -> Inventory {
        Inventory{
            products:Vec::new()    // Starting a new empty inventory
        }
    }

    fn add_product(&mut self, product: Product) {
        for p in &mut self.products {
            if p.name == product.name {
                p.price = product.price;
                p.quantity += product.quantity;
                return;
            }
        }
        
        self.products.push(product);
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        for p in &self.products {
            if p.name == name {
                println!("Found it: Product Name: {} | Quantity: {}, | Price: {}", p.name, p.quantity, p.price);
                return Some(p);
            }
        }
        println!("Product {} not found", name);
        None
    }

    fn total_value(&self) -> f32 {
        let mut total = 0.0;
        for p in &self.products {
            total += p.total_value();
        }
        total
    }
    
}


fn main() {
    let mut inventory = Inventory::new();
    
    inventory.add_product(Product::new("Laptop".to_string(), 999.99, 5));
    inventory.add_product(Product::new("Mouse".to_string(), 29.99, 50));
    inventory.add_product(Product::new("Keyboard".to_string(), 79.99, 20));
    inventory.add_product(Product::new("Laptop".to_string(), 999.99, 10));
    inventory.add_product(Product::new("Laptop".to_string(), 949.99, 30));
    inventory.add_product(Product::new("Laptop".to_string(), 8999.99, 1));
    
    println!("Total inventory value: £{:.2}", inventory.total_value());
    
    match inventory.find_product("Mouse") {
        Some(product) => println!("Found: {} - £{} (qty: {})", 
                                  product.name, product.price, product.quantity),
        None => println!("Product not found"),
    }
}