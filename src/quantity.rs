use std::{
    fmt, hash::Hash, ops::{Add, Div, Mul, Sub}
};

use crate::{dimensions::Dimensions, numeric::Numeric, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity<T: Numeric> {
    pub number: T,
    pub unit: Unit,
    pub uncertainty: T,
}

impl<T: Numeric> Quantity<T> {
    pub fn new(number: T, unit: Unit, uncertainty: T) -> Self {
        Self {
            number,
            unit,
            uncertainty,
        }
    }
}

//impl<T: Numeric> Hash for Quantity<T> {
//    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//        self.number.hash(state);
//        self.unit.id;
//        self.uncertainty.hash(state);
//    }
//}

impl<T: Numeric> fmt::Display for Quantity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol())
    }
}

impl<T: Numeric> Add for Quantity<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(self.number + rhs.number, self.unit, self.uncertainty)
        } else {
            panic!()
        }
    }
}

impl<T: Numeric> Sub for Quantity<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self::new(self.number - rhs.number, self.unit, self.uncertainty)
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

impl<T: Numeric> Quantity<T> {
    //pub fn pow(&self, exp: i32) -> Self {
    //    Self::new(self.number.powi(exp), self.unit.pow(exp), 0.0)
    //}

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
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
    pub struct PyQuantity(Quantity<Decimal>);

    #[pymethods]
    impl PyQuantity {
        #[new]
        fn new(number: Decimal, unit: PyUnit, uncertainty: Decimal) -> Self {
            PyQuantity(Quantity::new(number, unit.into_inner(), uncertainty))
        }
    }

    #[pyclass(name = "FQuantity")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub struct PyFQuantity(Quantity<f64>);

    #[pymethods]
    impl PyFQuantity {
        #[new]
        fn new(number: f64, unit: PyUnit, uncertainty: f64) -> Self {
            PyFQuantity(Quantity::new(number, unit.into_inner(), uncertainty))
        }
    }
}
