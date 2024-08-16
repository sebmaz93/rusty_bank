#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(name: String) -> Self {
        Bank {
            name,
            accounts: vec![],
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account)
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

fn main() {
    let mut bank = Bank::new("sss".to_string());
    let mut account = Account::new(123, String::from("Seb"));
    let mut account2 = Account::new(1234, String::from("Maz"));

    account.deposit(900);
    account2.deposit(2200);
    account.withdraw(500);
    account2.withdraw(300);
    println!("{:#?}", account.summary());

    bank.add_account(account);
    bank.add_account(account2);
    println!("{:#?}", bank.total_balance());
    println!("{:#?}", bank.summary());
}

// types of values that break the ownership rules (numbers, bool, char, array (if every inside is copyable),
// tuples (if every inside copyable), references
