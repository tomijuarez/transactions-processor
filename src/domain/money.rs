use std::ops::Neg;

use bigdecimal::{BigDecimal, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    USD,
    ARS
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub nominal_value: Box<BigDecimal>,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: BigDecimal, currency: Currency) -> Self {
        let amount = amount.with_scale(4);
        return Money { nominal_value: Box::new(amount), currency };
    }

    pub fn zero(currency: Currency) -> Self {
        return Money::new(BigDecimal::zero(), currency);
    }

    pub fn negate(&self) -> Money {
        return Money::new(self.nominal_value.clone().neg(), self.currency);
    }

    pub fn add(&self, another_money: &Money) -> Result<Money, String> {
        if self.currency != another_money.currency {
            return Err(String::from("Cannot add money with different currencies"));
        }

        return Ok(Money::new(&*self.nominal_value + &*another_money.nominal_value, self.currency));
    }
    
    pub fn subtract(&self, another_money: &Money) -> Result<Money, String> {
        if self.currency != another_money.currency {
            return Err(String::from("Cannot add money with different currencies"));
        }
        
        return Ok(Money::new(&*self.nominal_value - &*another_money.nominal_value, self.currency));
    }
    
    pub fn bigger_than(&self, another_money: &Money) -> bool {
        return &*self.nominal_value > &*another_money.nominal_value;
    }

    pub fn is_negative(&self) -> bool {
        return *self.nominal_value < BigDecimal::zero();
    }
}

#[cfg(test)]
mod test {
    use bigdecimal::FromPrimitive;

    use super::*;

    #[test]
    fn test_money_new() {
        let actual_money = Money::new(BigDecimal::from(100), Currency::USD);
        let expected_money = Money{nominal_value: Box::new(BigDecimal::from(100)), currency: Currency::USD};
        assert_eq!(actual_money, expected_money);
    }

    #[test]
    fn test_money_zero() {
        let actual_money = Money::zero(Currency::USD);
        let expected_money = Money{nominal_value: Box::new(BigDecimal::zero()), currency: Currency::USD};
        assert_eq!(actual_money, expected_money);
    }

    #[test]
    fn test_money_negate_from_positive_number() {
        let money = Money::new(BigDecimal::from(100), Currency::USD);
        let actual_money = money.negate();
        let expected_money = Money::new(BigDecimal::from(-100), Currency::USD);
        assert_eq!(actual_money, expected_money);
    }

    #[test]
    fn test_money_negate_from_negative_number() {
        let money = Money::new(BigDecimal::from(-100), Currency::USD);
        let actual_money = money.negate();
        let expected_money = Money::new(BigDecimal::from(100), Currency::USD);
        assert_eq!(actual_money, expected_money);
    }

    #[test]
    fn test_money_add_with_positive_numbers() {
        let money = Money::new(BigDecimal::from_f32(0.2).unwrap(), Currency::USD);
        let another_money = Money::new(BigDecimal::from_f64(0.1).unwrap(), Currency::USD);
        let expected_money = Money::new(BigDecimal::from_f32(0.3).unwrap(), Currency::USD);
        let expected_result = Ok(expected_money);
        let actual_result = money.add(&another_money);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_subtract_with_positive_numbers() {
        let money = Money::new(BigDecimal::from_f32(1.98).unwrap(), Currency::USD);
        let another_money = Money::new(BigDecimal::from_f32(0.99).unwrap(), Currency::USD);
        let expected_money = Money::new(BigDecimal::from_f32(0.99).unwrap(), Currency::USD);
        let expected_result = Ok(expected_money);
        let actual_result = money.subtract(&another_money);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_subtract_with_negative_numbers() {
        let money = Money::new(BigDecimal::from(20000), Currency::USD);
        let another_money = Money::new(BigDecimal::from_f32(2.25).unwrap(), Currency::USD);
        let expected_money = Money::new(BigDecimal::from_f32(19997.75).unwrap(), Currency::USD);
        let expected_result = Ok(expected_money);
        let actual_result = money.subtract(&another_money);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_bigger_than_is_true() {
        let money = Money::new(BigDecimal::from_f32(100.0001).unwrap(), Currency::USD);
        let another_money = Money::new(BigDecimal::from(100), Currency::USD);
        assert!(money.bigger_than(&another_money));
    }

    #[test]
    fn test_bigger_than_is_false() {
        let money = Money::new(BigDecimal::from_f32(0.9981).unwrap(), Currency::USD);
        let another_money = Money::new(BigDecimal::from_f32(0.99811).unwrap(), Currency::USD);
        assert!(!money.bigger_than(&another_money));
    }

    #[test]
    fn test_is_negative_is_true() {
        let money = Money::new(BigDecimal::from_f32(-1587.0001).unwrap(), Currency::USD);
        assert!(money.is_negative());
    }

    #[test]
    fn test_is_negative_is_false() {
        let money = Money::new(BigDecimal::from_f32(1587.0001).unwrap(), Currency::USD);
        assert!(!money.is_negative());
    }
}