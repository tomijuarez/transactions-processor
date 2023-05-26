use crate::domain::{
    transaction::Transaction,
    balance::Balance,
    money::Currency
};

#[derive(Debug)]
pub struct Wallet {
    pub balance: Balance,
    pub movements: Vec<Transaction>
}

impl Wallet {
    
    pub fn new(movements: Vec<Transaction>) -> Wallet {
        return Wallet {
            balance: Balance::new(movements.clone(), Currency::USD), 
            movements: movements 
        }
    }

    pub fn add_transaction(&self, transaction: Transaction, precalculated_balance: Balance) -> Wallet {
        let transactions: Vec<Transaction> = self.movements
            .iter()
            .cloned()
            .chain(std::iter::once(transaction))
            .collect();

        return Wallet {balance: precalculated_balance, movements: transactions };
    }

    pub fn withdraw(&self, transaction: Transaction) -> Result<Wallet, String> {
        let resulting_balance = self.balance.withdraw(transaction.clone())?;
        return Ok(self.add_transaction(transaction, resulting_balance));
    }
    
    pub fn deposit(&self, transaction: Transaction) -> Result<Wallet, String> {
        let resulting_balance = self.balance.deposit(transaction.clone())?;
        return Ok(self.add_transaction(transaction, resulting_balance));
    }
}