#[derive(PartialEq, Debug)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Dollar {
        Self { amount }
    }

    pub fn times(&self, multiplier: u32) -> Dollar {
        Self {
            amount: self.amount * multiplier as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: $5 + 10 CHF = $10 (レートが2:1の場合)
    // TODO: Moneyの丸め処理をどうする？
    // TODO: 他のオブジェクトとの等価性比較
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
        // arrange
        let five = Dollar::new(5);

        // act+assert
        assert_eq!(five, Dollar::new(5));
        assert_ne!(five, Dollar::new(1));
        assert_eq!(Dollar::new(0), Dollar::new(0));
        assert_ne!(Dollar::new(1), Dollar::new(10));
    }
}
