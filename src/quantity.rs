use std::{
    fmt,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

use rust_decimal::Decimal;

use crate::{
    dimensions::Dimensions,
    number::Number,
    unit::Unit,
};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity {
    pub number: Number,
    pub unit: Unit,
}

impl Quantity {
    pub fn new<T: Into<Number>>(number: T, unit: Unit) -> Self {
        Self {
            number: number.into(),
            unit,
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }
}

impl From<Decimal> for Quantity {
    fn from(value: Decimal) -> Self {
        Self {
            number: value.into(),
            unit: Unit::unitless(),
        }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(self.number + rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(self.number - rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(
            self.number * rhs.number,
            self.unit * rhs.unit,
        )
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(
            self.number / rhs.number,
            self.unit / rhs.unit,
        )
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol())
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::unit::py::PyUnit;

    use super::*;
    use pyo3::prelude::*;
    use rust_decimal::Decimal;

    #[pyclass(name = "Quantity")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub struct PyQuantity(Quantity);

    #[pymethods]
    impl PyQuantity {
        #[new]
        fn new(number: Decimal, unit: PyUnit) -> Self {
            PyQuantity(Quantity::new(number, unit.into_inner()))
        }
    }
}
