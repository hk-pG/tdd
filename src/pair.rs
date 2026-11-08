#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pair {
    from: String,
    to: String,
}

impl Pair {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        let from = from.into();
        let to = to.into();
        Self { from, to }
    }
}

#[cfg(test)]
mod test {}
