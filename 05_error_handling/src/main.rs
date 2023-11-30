struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

trait Account {
    /// Deposits the specified amount into the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to be deposited.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the deposit is successful, otherwise returns an `Err` with an error message.
    fn deposit(&mut self, amount: f64) -> Result<(), String>;

    /// Withdraws the specified amount from the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to be withdrawn.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the withdrawal is successful, otherwise returns an `Err` with an error message.
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;

    /// Returns the current balance of the account.
    fn balance(&self) -> f64;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount.is_sign_negative() {
            return Err("Cannot deposit negative amount".to_string());
        }

        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount.is_sign_negative() {
            return Err("Cannot withdraw negative amount".to_string());
        }

        if self.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut sam_account = BankAccount {
        account_number: String::from("acc_1"),
        holder_name: String::from("Sam"),
        balance: 100.0,
    };
    let mut john_account = BankAccount {
        account_number: String::from("acc_2"),
        holder_name: String::from("Jonh"),
        balance: 50.0,
    };

    match sam_account.deposit(50.0) {
        Ok(()) => diplay_account_summary(&sam_account),
        Err(err) => println!("Error: {}", err),
    }
    println!("\n");
    match john_account.withdraw(10.0) {
        Ok(()) => diplay_account_summary(&john_account),
        Err(err) => println!("Error: {}", err),
    }
}

/// Displays the account summary.
///
/// # Arguments
///
/// * `account` - The bank account for which to display the summary.
fn diplay_account_summary(account: &BankAccount) {
    println!("Account Summary");
    println!("===============");
    println!("Account number: {}", account.account_number);
    println!("Holder name: {}", account.holder_name);
    println!("Balance: {:.2}", account.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_deposit() {
        let mut account = BankAccount {
            account_number: String::from("acc_1"),
            holder_name: String::from("Sam"),
            balance: 100.0,
        };

        let result = account.deposit(50.0);

        assert!(result.is_ok());
        assert_eq!(account.balance, 150.0);
    }

    #[test]
    fn test_account_deposit_with_negative_amount() {
        let mut account = BankAccount {
            account_number: String::from("acc_1"),
            holder_name: String::from("Sam"),
            balance: 100.0,
        };

        let result = account.deposit(-50.0);

        assert_eq!(
            result.unwrap_err(),
            "Cannot deposit negative amount".to_string()
        );
    }

    #[test]
    fn test_account_withdraw() {
        let mut account = BankAccount {
            account_number: String::from("acc_1"),
            holder_name: String::from("Sam"),
            balance: 100.0,
        };

        let result = account.withdraw(50.0);

        assert!(result.is_ok());
        assert_eq!(account.balance, 50.0);
    }

    #[test]
    fn test_account_withdraw_with_negative_amount() {
        let mut account = BankAccount {
            account_number: String::from("acc_1"),
            holder_name: String::from("Sam"),
            balance: 100.0,
        };

        let result = account.withdraw(-50.0);

        assert_eq!(
            result.unwrap_err(),
            "Cannot withdraw negative amount".to_string()
        );
    }

    #[test]
    fn test_account_withdraw_with_insufficient_funds() {
        let mut account = BankAccount {
            account_number: String::from("acc_1"),
            holder_name: String::from("Sam"),
            balance: 100.0,
        };

        let result = account.withdraw(150.0);

        assert_eq!(result.unwrap_err(), "Insufficient funds".to_string());
        assert_eq!(account.balance, 100.0);
    }
}
