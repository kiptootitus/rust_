// Borrowing: References

// You can pass a value without taking ownership by borrowing it.
fn main() {
    let mut account = BankAccount{
        owner: "Titus".to_string(),
        balance: 100.95,
    };
    account.check_balance();
    account.withdraw(50.2);
}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing this amount {} Ksh., owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn  check_balance(&self){
        println!("Acount owned by {}, has a balance of {} Ksh.", self.owner, self.balance);
    }
}