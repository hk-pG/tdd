use crate::bank::Bank;
use crate::expression::Expression;
use crate::money::Money;

pub struct Sum {
    pub augend: Money,
    pub addend: Money,
}

impl Sum {
    pub fn new(augend: Money, addend: Money) -> Sum {
        Sum { augend, addend }
    }
}

impl Expression for Sum {
    fn reduce(&self, bank: &Bank, to: &str) -> Result<Money, String> {
        let amount = self.augend.reduce(bank, &to)?.amount + self.addend.reduce(bank, to)?.amount;
        Ok(Money::new(amount, to))
    }
}
