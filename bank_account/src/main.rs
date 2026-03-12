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
        let account_test = BankAccount::new(300.0);
        assert_eq!(account_test.balance(), 300.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account_test = BankAccount::new(600.0);
        account_test.deposit(70.0);
        assert_eq!(account_test.balance(), 670.0);
        
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account_test = BankAccount::new(450.0);
        account_test.withdraw(40.0);
        assert_eq!(account_test.balance(), 410.0);
    }

    #[test]
    fn check_on_balance() {
        // Write a test for depositing money
        let mut account_test = BankAccount::new(100.0);
        account_test.deposit(200.0);
        account_test.withdraw(150.0);
        assert_eq!(account_test.balance(), 150.0);
    }

    //Edge tests
    #[test]
    fn negative(){  //Test for negative numbers in both deposit and withdraw.
        let mut account_test = BankAccount::new(50.0);
        account_test.deposit(-20.0);
        account_test.withdraw(-40.0);
        assert_eq!(account_test.balance(), 50.0);
    }

    #[test]
    fn large_withdraw(){
        let mut account_test = BankAccount::new(200.0);
        account_test.withdraw(300.0);
        assert_eq!(account_test.balance(), 200.0);
    }
}

fn main() {
    
    let mut _account = BankAccount::new(500.0);  //Variable account is sent to new function in struct.\
    println!("Original balance is {}", _account.balance());

    _account.deposit(300.0);
    _account.withdraw(900.0);
    _account.withdraw(-500.0);
    _account.withdraw(400.0);
    println!("New balance is {}", _account.balance());

}
