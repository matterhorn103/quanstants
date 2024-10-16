mod unit;

// use crate::unit;

fn main() {
    let new_unit = unit::Unit {symbol: String::from("J"), name: String::from("joule"), def: &unit::BaseUnit::Metre};
    println!("{}", new_unit.symbol);
    println!("Hello, world!");
}
