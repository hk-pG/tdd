use crate::money::Money;

mod bank;
mod expression;
mod money;

fn main() {
    println!("Hello, world!");
    let test = Money::dollar(10);
    println!("{:?}", test.times(1).currency());
    let test = Money::franc(10);
    println!("{:?}", test.times(1).currency());
}
