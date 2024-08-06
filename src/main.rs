trait Account{
    fn deposit(&mut self, amount: i64) -> Result<(), String>;
    fn withdraw(&mut self, amount: i64) -> Result<(), String>;
    fn balance(&mut self) -> i64;
}

struct BankAccount{
    account_number: i64,
    holder_name: String,
    balance: i64
}

impl Account for BankAccount{
    fn deposit(&mut self, amount: i64) -> Result<(), String> {
        if amount < 2 {
            return Err("Amount is not sufficient to be added to your balance".to_string());
        }
        else{
            return Ok(self.balance += amount);
        }
    }

    fn withdraw(&mut self, amount: i64) -> Result<(), String> {
        if amount > self.balance {
            return Err("You can't receive more than you have, if you can do please call me!".to_string());
        }
        else{
            return Ok(self.balance -= amount);
        }
    }

    fn balance(&mut self) -> i64
    {
        return self.balance;
    }

}


fn main() {
    let mut first_bank_account_instance = BankAccount{
        account_number: 24820,
        holder_name: "WrackedBoss".to_string(),
        balance: 0
    };

    let mut second_bank_account_instance = BankAccount{
        account_number: 24821,
        holder_name: "LastBossBender".to_string(),
        balance: 15000
    };


    let deposit_transaction = first_bank_account_instance.deposit(1);

    match deposit_transaction{
        Ok(v) => (),
        Err(e) => println!("{}", e)
    }

    let withdraw_transaction = second_bank_account_instance.withdraw(170000);

    match withdraw_transaction{
        Ok(e) => (),
        Err(e) => println!("{}", e)
    }

    println!("Balance of first account is {}, and balance of second account is {}", first_bank_account_instance.balance(), second_bank_account_instance.balance());
}
