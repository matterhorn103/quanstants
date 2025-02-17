use quanstants::dimensions::Dimensions;
use quanstants::unit::BaseUnit;
use quanstants::quantity::Quantity;

fn main() {
    let dim = Dimensions::new(1, -1, 0, 0, 0, 0, 0);
    dbg!(dim);
    println!("{dim}");
    let m = BaseUnit::new(String::from("m"), String::from("metre"), Dimensions { L: 1, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 0 });
    let q1 = Quantity::new(2.0, m.clone(), 0.0);
    //dbg!(q1);
    let q2 = Quantity::new(3.0, m.clone(), 0.0);
    //dbg!(q2);
    let q3 = q1 + q2;
    dbg!(q3);
}
