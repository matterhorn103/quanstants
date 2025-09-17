use std::{fmt, ops::{Add, Div, Mul, Sub}};

use crate::{dimensions::Dimensions, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity {
    pub number: f64,
    pub unit: Unit,
    pub uncertainty: f64,
}

impl Quantity {
    pub fn new(number: f64, unit: Unit, uncertainty: f64) -> Self {
        Self {
            number,
            unit,
            uncertainty,
        }
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol())
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(
                self.number + rhs.number,
                self.unit,
                0.0,
            )
        } else {
            panic!()
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(
                self.number - rhs.number,
                self.unit,
                0.0,
            )
        } else {
            panic!()
        }
    }
}

//impl Mul for Quantity {
//    type Output = Self;
//
//    fn mul(self, rhs: Quantity) -> Quantity {
//        Quantity::new(
//            self.number * rhs.number,
//            self.unit * rhs.unit,
//            0.0,
//        )
//    }
//}
//
//impl Div for Quantity {
//    type Output = Self;
//
//    fn div(self, rhs: Quantity) -> Quantity {
//        Quantity::new(
//            self.number / rhs.number,
//            self.unit / rhs.unit,
//            0.0,
//        )
//    }
//}

impl Quantity {
    //pub fn pow(&self, exp: i32) -> Self {
    //    Self::new(self.number.powi(exp), self.unit.pow(exp), 0.0)
    //}

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }
}
