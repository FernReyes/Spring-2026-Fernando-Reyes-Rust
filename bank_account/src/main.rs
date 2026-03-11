#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {   //Making a new balance for new account.
        // Implement this method
        BankAccount{
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0{
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount < self.balance && amount > 0.0{
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
    }

    // Add more tests here
}

fn main() {
    
    let _account = BankAccount::new(500.0);  //Variable account is sent to new function in struct.

}
