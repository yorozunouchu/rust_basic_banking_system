trait Account {
    fn deposit(&mut self, amount: u64) -> Result<(), String>;
    fn withdraw(&mut self, amount: u64) -> Result<(), String>;
    fn balance(&self) -> u64;
}

struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) -> Result<(), String> {
        match self.balance.checked_add(amount) {
            Some(new_balance) => {
                self.balance = new_balance;
                Ok(())
            },
            None => Err(String::from("Overflow Error, deposit amount too large"))
        }
    }

    fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        match self.balance.checked_sub(amount) {
            Some(new_balance) => {
                self.balance = new_balance;
                Ok(())
            },
            None => Err(String::from("Insufficient funds for withdrawal"))
        }
    }

    fn balance(&self) -> u64 {
        self.balance
    }
}

enum TransactionType {
    Deposit,
    Withdraw,
}

fn process_transaction(account: &mut BankAccount, transaction_type: TransactionType, amount: u64) {
    let result = match transaction_type {
        TransactionType::Deposit => account.deposit(amount),
        TransactionType::Withdraw => account.withdraw(amount),
    };

    match result {
        Ok(_) => match transaction_type {
            TransactionType::Deposit => println!("Deposit of {} successful", amount),
            TransactionType::Withdraw => println!("Withdrawal of {} successful", amount),
        },
        Err(e) => println!("Transaction error: {}", e),
    }
}

fn display_balance(account: &BankAccount) {
    println!("Account #{}: {} has a balance of {}", account.account_number, account.holder_name, account.balance());
}

fn main() {
    let mut alice = BankAccount {
        account_number: 1000001,
        holder_name: String::from("Alice"),
        balance: 10000,
    };
    let mut bob = BankAccount {
        account_number: 1000002,
        holder_name: String::from("Bob"),
        balance: 10000,
    };

    process_transaction(&mut alice, TransactionType::Deposit, 500);
    process_transaction(&mut bob, TransactionType::Withdraw, 300);

    display_balance(&alice);
    display_balance(&bob);
}
