trait Account {
    fn deposit(&mut self, amount: f32);
    fn withdraw(&mut self, amount: f32) -> Result<(), &'static str>;
    fn balance(&self) -> f32;
}

struct BankAccount {
    balance: f32,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f32) -> Result<(), &'static str> {
        if amount > self.balance {
            return Err("Insufficient funds");
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f32 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount { balance: 0.0 };
    account.deposit(100.0);
    println!("Deposit: 100.0, Balance: {}", account.balance());
    account.withdraw(50.0).unwrap();
    println!("Withdraw: 50.0, Balance: {}", account.balance());
}
