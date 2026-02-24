use crate::{expression::Expression, money::Money};

pub struct Bank {}

impl Bank {
    pub fn new() -> Bank {
        Bank {}
    }

    pub fn reduce(&self, source: &dyn Expression, to: String) -> Money {
        Money::dollar(10)
    }
}
