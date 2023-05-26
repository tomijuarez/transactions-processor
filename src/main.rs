mod domain;

use bigdecimal::BigDecimal;

use crate::{
    domain::money::Money, domain::money::Currency, domain::transaction::Transaction, domain::transaction::TransactionType, domain::wallet::Wallet
};

fn instantiate_wallet() -> Result<bool, String> {
    let transactions = Vec::from([
        Transaction::new(Money::new(BigDecimal::from(50000), Currency::USD), TransactionType::DEPOSIT, String::from("yesterday"))?,
        Transaction::new(Money::new(BigDecimal::from(20000), Currency::USD), TransactionType::WITHDRAW, String::from("yesterday"))?,
        Transaction::new(Money::new(BigDecimal::from(10000), Currency::USD), TransactionType::DEPOSIT, String::from("today"))?,
        Transaction::new(Money::new(BigDecimal::from(70000), Currency::USD), TransactionType::DEPOSIT, String::from("today"))?
    ]);
    
    //Estaría bueno que los elementos del vector sean referencias y no copiar.
    let wallet = Wallet::new(transactions);
    
    println!("Current balance: {}", wallet.balance.amount.nominal_value);
    
    let new_wallet =
        wallet.withdraw(
            Transaction::new(Money::new(BigDecimal::from(20000), Currency::USD), TransactionType::WITHDRAW, String::from("today"))?).unwrap();
    
    println!("Current balance: {}", wallet.balance.amount.nominal_value);
    println!("Current balance: {}", new_wallet.balance.amount.nominal_value);

    return Ok(true);
}

fn main() {
    match instantiate_wallet() {
        Ok(_) => println!("Wallet instantiated successfully"),
        Err(error) => println!("Error instantiating wallet: {}", error)
    }
}