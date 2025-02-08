use std::ops::{Add, Div, Mul, Sub};

use crate::{dimensions::Dimensions, unit::Unit};

pub trait Linear {
    fn value(&self) -> Quantity;

    fn dimensions(&self) -> Dimensions;
}

#[derive(Copy, Clone, Debug)]
pub struct Quantity<'a> {
    number: f64,
    unit: &'a Unit,
    uncertainty: f64,
}

impl<'a> Quantity<'a> {
    pub fn new(number: f64, unit: &'a Unit, uncertainty: f64) -> Self {
        Self {
            number,
            unit,
            uncertainty,
        }
    }

    pub fn pow(&self, exp: i32) -> Self {
        Self::new(self.number.powi(exp), self.unit.pow(exp), 0.0)
    }
}

impl<'a> Linear for Quantity<'a> {
    fn value(&self) -> Quantity {
        *self
    }

    fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }
}

impl<'a> Add for Quantity<'a> {
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

impl<'a, T: Linear> Mul<T> for Quantity<'a> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        // Would this be faster if Linear required number() and unit() implementations?
        Self::new(
            self.number * rhs.value().number,
            self.unit,
            0.0,
        )
    }
}
