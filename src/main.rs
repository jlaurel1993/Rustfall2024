mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawal: {}", account.balance());

    account.apply_interest(0.05);
    println!("After applying interest: {}", account.balance());
}
