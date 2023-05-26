use crate::domain::{
    transaction::Transaction,
    money::Money,
    money::Currency
};

#[derive(Debug, Clone)]
pub struct Balance {
    pub amount: Money,
    pub currency: Currency,
}

impl Balance {

    pub fn new(history: Vec<Transaction>, currency: Currency) -> Self {
        let amount: Money = history.iter()
            .map(|transaction| transaction.signed_amount())
            .reduce(|balance, amount| balance.add(&amount))
            .unwrap_or(Money::zero(currency));
        
        return Balance { amount, currency };
    }
    
    pub fn withdraw(&self, transaction: Transaction) -> Result<Balance, String> {
        if transaction.amount.biggerThan(&self.amount) {
            return Err(String::from("The balance is not enough"));
        }
        return Ok(Balance { amount: self.amount.subtract(&transaction.amount), currency: self.currency });
    }
    
    pub fn deposit(&self, transaction: Transaction) -> Result<Balance, String> {
        return Ok(Balance { amount: self.amount.add(&transaction.amount), currency: self.currency });
    }
}