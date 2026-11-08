use crate::bank::Bank;
use crate::expression::Expression;
use crate::sum::Sum;

#[derive(PartialEq, Debug, Clone)]
pub struct Money {
    pub amount: i32,
    currency: String,
}

impl Money {
    pub fn new(amount: i32, currency: String) -> Money {
        Money { amount, currency }
    }

    pub fn times(&self, multiplier: i32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency(),
        }
    }

    pub fn currency(&self) -> String {
        self.currency.to_string()
    }

    pub fn dollar(amount: i32) -> Money {
        Money::new(amount, "USD".to_string())
    }

    pub fn franc(amount: i32) -> Money {
        Money::new(amount, "CHF".to_string())
    }

    pub fn plus(&self, addend: &Money) -> Box<Sum> {
        Box::new(Sum::from(self.amount, addend.amount, self.currency()))
    }
}

impl Expression for Money {
    fn reduce(&self, bank: &Bank, to: String) -> Result<Money, String> {
        let rate = bank
            .rate(&self.currency, &to)
            .ok_or_else(|| format!("rate you requested {} is NOT defined.", to))?;

        Ok(Money::new(self.amount / rate, to))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bank::Bank;

    // P108
    // TODO: $5 + 10 CHF = $10 (レートが2:1の場合)
    // TODO: $5 + $5 が Moneyを返す
    #[test]
    fn test_multiplication() {
        // arrange
        let five = Money::dollar(5);
        // act+assert
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equals() {
        // act+assert
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(1));
        assert_ne!(Money::franc(5), Money::dollar(5));
    }

    #[test]
    pub fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    pub fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five.plus(&five);
        let bank = Bank::new();
        let reduced: Money = bank.reduce(sum.as_ref(), "USD".to_string()).unwrap();
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    pub fn test_plus_returns_sum() {
        let five = Money::dollar(5);
        let result = five.plus(&five);
        let sum = result.as_ref();
        assert_eq!(five, sum.augend);
        assert_eq!(five, sum.addend);
    }

    #[test]
    pub fn test_reduce_sum() {
        let sum = Sum::new(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let result = bank.reduce(&sum, "USD".to_string()).unwrap();
        assert_eq!(Money::dollar(7), result);
    }

    #[test]
    pub fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(&Money::dollar(1), "USD".to_string()).unwrap();
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    pub fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.addRate("CHF", "USD", 2);
        let result = bank.reduce(&Money::franc(2), "USD".to_string()).unwrap();
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    pub fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate("USD", "USD").unwrap());
    }
}
