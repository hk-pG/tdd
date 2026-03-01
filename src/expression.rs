use crate::money::Money;

pub trait Expression {
    fn reduce(&self, to: String) -> Money;
}
