use std::{fmt, ops::{Add, Div, Mul, Sub}};

use crate::{dimensions::Dimensions, unit::LinearUnit, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct LinearQuantity {
    pub number: f64,
    pub unit: LinearUnit,
    pub uncertainty: f64,
}

impl LinearQuantity {
    pub fn new(number: f64, unit: LinearUnit, uncertainty: f64) -> Self {
        Self {
            number,
            unit,
            uncertainty,
        }
    }
}

impl fmt::Display for LinearQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol())
    }
}

impl Add for LinearQuantity {
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

impl Sub for LinearQuantity {
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

impl Mul for LinearQuantity {
    type Output = Self;

    fn mul(self, rhs: LinearQuantity) -> LinearQuantity {
        LinearQuantity::new(
            self.number * rhs.number,
            self.unit * rhs.unit,
            0.0,
        )
    }
}

impl Div for LinearQuantity {
    type Output = Self;

    fn div(self, rhs: LinearQuantity) -> LinearQuantity {
        LinearQuantity::new(
            self.number / rhs.number,
            self.unit / rhs.unit,
            0.0,
        )
    }
}

impl LinearQuantity {
    //pub fn pow(&self, exp: i32) -> Self {
    //    Self::new(self.number.powi(exp), self.unit.pow(exp), 0.0)
    //}

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }
}

//#[macro_export]
//macro_rules! qu {
//    ($number:expr, $unit:expr) => {
//        Quantity::new($number, Unit::from_str($unit), 0.0)
//    };
//    ($number:expr, $unit:expr, $uncertainty:expr) => {
//        Quantity::new($number, Unit::from_str($unit), $uncertainty)
//    };
//}
