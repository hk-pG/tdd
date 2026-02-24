use crate::bank::Bank;
use crate::expression::Expression;

#[derive(PartialEq, Debug)]
pub struct Money {
    amount: i32,
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

    pub fn plus(&self, addend: &Money) -> Box<dyn Expression> {
        Box::new(Money::new(self.amount + addend.amount, self.currency()))
    }
}

impl Expression for Money {}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO: $5 + 10 CHF = $10 (レートが2:1の場合)
    // TODO: Money::丸め処理をどうする？
    // TODO: 他のオブジェクトとの等価性比較
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
        let sum: Box<dyn Expression> = five.plus(&five);
        let bank = Bank::new();
        let reduced: Money = bank.reduce(sum.as_ref(), "USD".to_string());
        assert_eq!(Money::dollar(10), reduced);
    }
}
