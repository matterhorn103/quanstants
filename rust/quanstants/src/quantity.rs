use std::ops::{Add, Div, Mul, Sub};

use crate::{dimensions::Dimensions, unit::{CompoundUnit, Unit}};

//pub trait Linear {
//    fn value(&self) -> Quantity<Unit>;
//
//    fn dimensions(&self) -> Dimensions;
//}

#[derive(Clone, Debug)]
pub struct Quantity<U: Unit> {
    pub number: f64,
    pub unit: U,
    pub uncertainty: f64,
}

impl<U: Unit> Quantity<U> {
    pub fn new(number: f64, unit: U, uncertainty: f64) -> Self {
        Self {
            number,
            unit,
            uncertainty,
        }
    }
}

    //pub fn pow(&self, exp: i32) -> Self {
    //    Self::new(self.number.powi(exp), self.unit.pow(exp), 0.0)
    //}

//impl Linear for Quantity {
//    fn value(&self) -> Quantity {
//        self.clone()
//    }
//
//    fn dimensions(&self) -> Dimensions {
//        *self.unit.dimensions()
//    }
//}

impl<U: Unit> Add for Quantity<U> {
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

impl<T, U: Unit> Mul<Quantity<T>> for Quantity<U> {
    type Output = Quantity<CompoundUnit>;

    fn mul(self, rhs: Quantity<T>) -> Self::Output {
        // Would this be faster if Linear required number() and unit() implementations?
        Self::new(
            self.number * rhs.value().number,
            self.unit,
            0.0,
        )
    }
}
