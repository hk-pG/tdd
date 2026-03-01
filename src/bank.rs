use crate::{expression::Expression, money::Money, sum::Sum};

pub struct Bank {}

impl Bank {
    pub fn new() -> Bank {
        Bank {}
    }

    pub fn reduce(&self, source: &dyn Expression, to: String) -> Money {
        source.reduce(to)
    }
}
