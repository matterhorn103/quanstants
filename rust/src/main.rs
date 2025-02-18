use quanstants::dimensions::Dimensions;
use quanstants::unit::BaseUnit;
use quanstants::quantity::Quantity;

fn main() {
    let dim = Dimensions::new(1, -1, 0, 0, 0, 0, 0);
    dbg!(dim);
    println!("{dim}");
    let mut m = BaseUnit::new()
    sets::add_set(&mut reg, sets::UnitSet::Base);
    let q1 = Quantity::new(2.0, reg.get_by_symbol("m").unwrap(), 0.0);
    dbg!(q1);
    let q2 = Quantity::new(3.0, reg.get_by_symbol("m").unwrap(), 0.0);
    dbg!(q2);
    let q3 = q1 + q2;
    dbg!(q3);
}
