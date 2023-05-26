use std::ops::Neg;

use bigdecimal::{BigDecimal, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    USD
}

#[derive(Debug, Clone)]
pub struct Money {
    pub nominal_value: Box<BigDecimal>,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: BigDecimal, currency: Currency) -> Self {
        return Money { nominal_value: Box::new(amount), currency };
    }

    pub fn zero(currency: Currency) -> Self {
        return Money { nominal_value: Box::new(BigDecimal::zero()), currency: currency };
    }

    pub fn negate(&self) -> Money {
        return Money { nominal_value: Box::new(self.nominal_value.clone().neg()), currency: self.currency };
    }

    pub fn add(&self, another_money: &Money) -> Money {
        assert_eq!(self.currency, another_money.currency);
        return Money { nominal_value: Box::new(&*self.nominal_value + &*another_money.nominal_value), currency: self.currency }; 
    }
    
    pub fn subtract(&self, another_money: &Money) -> Money {
        assert_eq!(self.currency, another_money.currency);
        return Money { nominal_value: Box::new(&*self.nominal_value - &*another_money.nominal_value), currency: self.currency };
    }
    
    pub fn biggerThan(&self, another_money: &Money) -> bool {
        return &*self.nominal_value > &*another_money.nominal_value;
    }

    pub fn is_negative(&self) -> bool {
        return *self.nominal_value < BigDecimal::zero();
    }
}