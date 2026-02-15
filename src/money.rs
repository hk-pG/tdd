pub struct Dollar {
    pub amount: i32,
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
    // TODO: amountをプライベートにする
    // TODO: Moneyの丸め処理をどうする？
    #[test]
    fn test_multiplication() {
        // arrange
        let five = Dollar::new(5);

        // act
        let product = five.times(2);

        // assert
        assert_eq!(10, product.amount);

        let product = five.times(3);

        assert_eq!(15, product.amount);
    }
}
