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

    pub fn from(augend: i32, addend: i32, currency: String) -> Sum {
        Sum {
            augend: Money::new(augend, currency.clone()),
            addend: Money::new(addend, currency.clone()),
        }
    }
}

impl Expression for Sum {
    fn reduce(&self, to: String) -> Money {
        let amount = self.augend.amount + self.addend.amount;
        Money::new(amount, to)
    }
}
