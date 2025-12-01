
trait Transaction {
    fn deposit(&mut self, amount:f32) -> Result<f32, &str>;
    fn withdraw(&mut self, amount:f32) -> Result<f32, &str>;
    fn balance(&self) -> f32;
}


struct CheckingAccount {
    balance: f32,
    overdraft_limit: f32,  // Can go negative up to this limit
}

struct SavingsAccount {
    balance: f32,
    min_balance: f32,  // Cannot go below this
}

impl Transaction for CheckingAccount {
    fn deposit(&mut self, amount:f32) -> Result<f32, &str> {
        self.balance += amount;
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount:f32) -> Result<f32, &str> {
        if (self.balance - amount) >= -self.overdraft_limit {
            self.balance -= amount;
            Ok(self.balance)
        } else {
            Err("Error while withdrawing from account. Transaction exceeds allowed overdraft.")
        }
    }

    fn balance(&self) -> f32 {
        self.balance
    }
}

impl Transaction for SavingsAccount {
    fn deposit(&mut self, amount:f32) -> Result<f32, &str> {
        self.balance += amount;
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount:f32) -> Result<f32, &str> {
        if (self.balance - amount) >= self.min_balance {
            self.balance -= amount;
            Ok(self.balance)
        } else {
            Err("Error while withdrawing from account. Transaction exceeds allowed minimum balance.")
        }
    }
    
    fn balance(&self) -> f32 {
        self.balance
    }
}

fn process_transaction(account: &mut impl Transaction, amount: f32, is_deposit: bool) {
    if is_deposit {
        match account.deposit(amount) {
            Ok(balance) => println!("Deposited £{:.2}. New balance: £{:.2}", amount, balance),
            Err(e) => println!("Deposit failed: {}", e),
        }
    } else {
        match account.withdraw(amount) {
            Ok(balance) => println!("Withdrew £{:.2}. New balance: £{:.2}", amount, balance),
            Err(e) => println!("Withdrawal failed: {}", e),
        }
    }
}

fn main() {
    let mut checking = CheckingAccount {
        balance: 100.0,
        overdraft_limit: 50.0,
    };
    
    let mut savings = SavingsAccount {
        balance: 500.0,
        min_balance: 100.0,
    };
    
    // Test checking account
    process_transaction(&mut checking, 50.0, true);   // Deposit: 150.0
    process_transaction(&mut checking, 180.0, false); // Withdraw: -30.0 (OK, within overdraft)
    process_transaction(&mut checking, 30.0, false);  // Should fail (exceeds overdraft)
    
    // Test savings account
    process_transaction(&mut savings, 100.0, false);  // Withdraw: 400.0
    process_transaction(&mut savings, 350.0, false);  // Should fail (below min balance)
}
