use std::{collections::HashMap, iter::Map};

use crate::{expression::Expression, money::Money, pair::Pair};

pub struct Bank {
    rates: HashMap<Pair, i32>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    pub fn reduce(&self, source: &dyn Expression, to: String) -> Result<Money, String> {
        source.reduce(self, to)
    }

    pub fn addRate(&mut self, from: impl AsRef<str>, to: impl AsRef<str>, rate: i32) {
        self.rates
            .insert(Pair::new(from.as_ref(), to.as_ref()), rate);
    }

    pub fn rate(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> Option<i32> {
        let from = from.as_ref();
        let to = to.as_ref();

        if from.eq(to) {
            return Some(1);
        }

        self.rates.get(&Pair::new(from, to)).copied()
    }
}
