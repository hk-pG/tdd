#[derive(PartialEq, Debug)]
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
}

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
    fn test_equals_difference_currency() {
        assert_eq!(
            Money::new(10, "USD".to_string()),
            Money::new(10, "USD".to_string())
        );
    }

    #[test]
    pub fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
