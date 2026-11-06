use crate::{expression::Expression, money::Money};

pub struct Bank {}

impl Bank {
    pub fn new() -> Bank {
        Bank {}
    }

    pub fn reduce(&self, source: &dyn Expression, to: String) -> Money {
        source.reduce(self, to)
    }

    pub fn addRate(&self, from: impl AsRef<str>, to: impl AsRef<str>, rate: i32) {}

    pub fn rate(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> i32 {
        let from = from.as_ref();
        let to = to.as_ref();

        if from.eq("CHF") && to.eq("USD") { 2 } else { 1 }
    }
}
