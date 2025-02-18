use quanstants::dimensions::Dimensions;
use quanstants::unit::{BaseUnit, CompoundUnit, LinearFactor};
use quanstants::quantity::LinearQuantity;

fn main() {
    let dim = Dimensions::new(1, -1, 0, 0, 0, 0, 0);
    dbg!(dim);
    println!("{dim}");
    let m = BaseUnit::new(String::from("m"), String::from("metre"), Dimensions { L: 1, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 0 });
    let s = BaseUnit::new(String::from("s"), String::from("second"), Dimensions { L: 0, M: 0, T: 1, I: 0, Θ: 0, N: 0, J: 0 });
    let q1 = LinearQuantity::new(2.0, CompoundUnit::new(&[LinearFactor::Base(m.clone(), 1)]), 0.0);
    //dbg!(q1);
    let q2 = LinearQuantity::new(3.0, CompoundUnit::new(&[LinearFactor::Base(m, 1)]), 0.0);
    //dbg!(q2);
    let q3 = q1.clone() + q2.clone();
    dbg!(q3);
    let q4 = LinearQuantity::new(4.0, CompoundUnit::new(&[LinearFactor::Base(s, 1)]), 0.0);
    let q5 = q1 * q4;
    println!("{}", q5);
}
