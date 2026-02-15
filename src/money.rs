#[derive(PartialEq, Debug)]
pub struct Money {
    pub amount: i32,
}

impl Money {
    pub fn new(amount: i32) -> Money {
        Money { amount }
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
            money: Money::new(amount),
        }
    }

    pub fn times(&self, multiplier: u32) -> Dollar {
        Self {
            money: Money::new(self.money.amount * multiplier as i32),
        }
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
            money: Money::new(amount),
        }
    }

    pub fn times(&self, multiplier: u32) -> Franc {
        Self {
            money: Money::new(self.money.amount * multiplier as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: $5 + 10 CHF = $10 (レートが2:1の場合)
    // TODO: Moneyの丸め処理をどうする？
    // TODO: 他のオブジェクトとの等価性比較
    // TODO: DollarとFrancの重複
    // TODO: timesの一般化
    // TODO: DollarとFrancの比較
    #[test]
    fn test_multiplication() {
        // arrange
        let five = Dollar::new(5);
        // act+assert
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equals() {
        // act+assert
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert_ne!(Dollar::new(5), Dollar::new(1));
        assert_eq!(Franc::new(5), Franc::new(5));
        assert_ne!(Franc::new(5), Franc::new(1));
    }

    #[test]
    fn test_franc_multiplication() {
        // arrange
        let five = Franc::new(5);
        // act+assert
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
