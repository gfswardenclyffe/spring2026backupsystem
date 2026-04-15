#[derive(Debug)]
pub struct BankAccount {
    balance: f64, // the attribute is private by default
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount { // bank account constructor
        BankAccount {balance: initial_balance,}
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 { // if the amount is positibe
            self.balance += amount;
        }
        // ignore if amount is negative
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance { // if the amount is positive and less than the balance
            self.balance -= amount;
        }
        // ignore if the amount is negative or greater than the balance
    }

    pub fn balance(&self) -> f64 { // getter function
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 { // check input rate is positive
            self.balance += self.balance * rate;z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() { // test creating a new account
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() { // test depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() { // test withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);
    }

    #[test]
    fn test_over_withdraw() { // edge case withdrawing more than balance
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_deposit() { // edge case depositing negative amount
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_withdraw() { // edge case withdrawing negative amount
        let mut account = BankAccount::new(100.0);
        account.withdraw(-10.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_interest() { // test increasing balance by interest rate
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.10);
        assert_eq!(account.balance(), 110.0);
    }
}