use crate::domain::money::Money;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransactionType {
    DEPOSIT, WITHDRAW
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub amount: Money,
    pub operation: TransactionType,
    pub at: String
}

impl Transaction {
    pub fn new(amount: Money, operation: TransactionType, at: String) -> Result<Transaction, String> {
        if amount.is_negative() {
            return Err(String::from("The amount cannot be negative"));
        }

        return Ok(Transaction { amount, operation, at });
    }

    pub fn signed_amount(&self) -> Money {
        return match self.operation {
            TransactionType::DEPOSIT => self.amount.clone(),
            TransactionType::WITHDRAW => self.amount.negate()
        };
    }
}