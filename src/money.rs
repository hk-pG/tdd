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

    pub fn dollar(amount: i32) -> Dollar {
        Dollar::new(amount)
    }

    pub fn franc(amount: i32) -> Franc {
        Franc::new(amount)
    }
}

///
/// 通貨構造体(Dollar)
///
#[derive(PartialEq, Debug)]
pub struct Dollar {
    money: Money,
}

impl Dollar {
    pub fn new(amount: i32) -> Dollar {
        Self {
            money: Money::new(amount, String::from("USD")),
        }
    }

    pub fn times(&self, multiplier: u32) -> Money {
        self.money.times(multiplier as i32)
    }
}

///
/// 通貨構造体(Franc)
///
#[derive(PartialEq, Debug)]
pub struct Franc {
    money: Money,
}

impl Franc {
    pub fn new(amount: i32) -> Franc {
        Self {
            money: Money::new(amount, String::from("CHF")),
        }
    }

    pub fn times(&self, multiplier: u32) -> Money {
        self.money.times(multiplier as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: $5 + 10 CHF = $10 (レートが2:1の場合)
    // TODO: Money::丸め処理をどうする？
    // TODO: 他のオブジェクトとの等価性比較
    // TODO: DollarとFrancの重複
    // TODO: timesの一般化
    // TODO: DollarとFrancの比較
    #[test]
    fn test_multiplication() {
        // arrange
        let five = Money::dollar(5);
        // act+assert
        assert_eq!(Money::dollar(10).money, five.times(2));
        assert_eq!(Money::dollar(15).money, five.times(3));
    }

    #[test]
    fn test_equals() {
        // act+assert
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(1));
        assert_eq!(Money::franc(5), Money::franc(5));
        assert_ne!(Money::franc(5), Money::franc(1));
    }

    // TODO: enumで比較可能にするまで放置
    // #[test]
    // fn test_equals_difference_currency() {
    //     assert_eq!(Money::dollar(10), Money::franc(10));
    // }
    #[test]
    fn test_franc_multiplication() {
        // arrange
        let five = Money::franc(5);
        // act+assert
        assert_eq!(Money::franc(10).money, five.times(2));
        assert_eq!(Money::franc(15).money, five.times(3));
    }

    #[test]
    pub fn test_currency() {
        assert_eq!("USD", Money::dollar(1).money.currency());
        assert_eq!("CHF", Money::franc(1).money.currency());
    }
}
