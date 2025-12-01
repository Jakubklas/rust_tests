use std::collections::HashMap;

#[derive(Debug)]
struct Expense {
    id: u32,
    amount: f32,
    category: String,
    description: String,
}

struct ExpenseTracker {
    expenses: Vec<Expense>,
    budget_limits: HashMap<String, f32>,
    next_id: u32,
}

impl ExpenseTracker {
    fn new() -> ExpenseTracker {
        ExpenseTracker {
            expenses: Vec::new(),
            budget_limits: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_expense(&mut self, amount:f32, category:&str, description:&str) -> Result<(), String>{
        if let Some(limit) = self.budget_limits.get(category) {                     //WHY no .to_strung() or dereferenceing?
            // let spending = self.spending_by_category();
            // let spent = spending.get(category).unwrap_or(&0.0);

            let spent: f32 = self.expenses
                .iter()
                .filter(|e| e.category == category)
                .map(|e| e.amount)
                .sum();

            if (spent + amount) > *limit {
                return Err("Transaction cancelled as it would exceed limit.".to_string());      //WHY need return?
            }
        }
        
        self.expenses.push(Expense{
            id: self.next_id,
            amount: amount,
            category: category.to_string(),
            description: description.to_string(),
        });
        self.next_id += 1;
        println!("Added an expense of {}£ for {} ", amount, category);
        Ok(())
    }

    fn remove_expense(&mut self, id: u32) -> Result<(), String> {
        let position = self.expenses.iter().position(|e| e.id == id);

        match position {
            Some(idx) => {
                self.expenses.remove(idx);
                return Ok(());
            },
            None => Err(format!("Failed to remove expense. No expense with ID {}", id)),
        }
    }

    fn set_budget(&mut self, category: &str, limit: f32) {
        self.budget_limits.insert(category.to_string(), limit);
        println!("Budget limit of {} was set for category '{}'", limit, category.to_string());
    }

    fn list_expenses(&self) {
        println!("EXPENSES BREAKDOWN:\n");
        for e in &self.expenses {
            println!("ID: {} | Category: {} | Amount: £{:.2} | Decription: {}", e.id, e.category, e.amount, e.description);
        }
    }

    fn total_spending(&self) -> f32 {
        let mut sum = 0.0;
        for e in &self.expenses {
            sum += e.amount;
        }
        sum
    }

    fn spending_by_category(&self) -> HashMap<String, f32> {
        let mut categories = HashMap::new();
        for e in &self.expenses {
            let current = categories.entry(e.category.clone()).or_insert(0.0);
            *current += e.amount;       // Have to dereference since current is just a stack ref to the values
        }
        categories
    }

    fn print_summary(&self) {
        for (key, val) in self.spending_by_category() {
            println!("Category: {} | Amount Spent: {}", key, val);
        }
    }
}



fn main() {
    let mut tracker = ExpenseTracker::new();
    
    // Set budge limits
    tracker.set_budget("Food", 70.0);

    // Add some expenses
    tracker.add_expense(50.0, "Food", "Groceries");
    tracker.add_expense(30.0, "Transport", "Bus ticket");
    tracker.add_expense(100.0, "Food", "Restaurant");
    tracker.add_expense(25.0, "Entertainment", "Cinema");
    
    // List all
    tracker.list_expenses();
    
    // Show summary
    tracker.print_summary();

    // Removing expenses
    match tracker.remove_expense(2) {
        Ok(_) => println!("Removed expense #2"),
        Err(e) => println!("Error: {}", e),
    };


}