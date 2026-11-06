use crate::{bank::Bank, money::Money};

pub trait Expression {
    fn reduce(&self, bank: &Bank, to: String) -> Money;
}
