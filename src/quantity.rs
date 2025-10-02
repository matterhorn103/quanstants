use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use rust_decimal::Decimal;

use crate::{dimensions::Dimensions, number::Number, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity {
    pub number: Number,
    pub unit: Unit,
}

impl Quantity {
    pub fn new(number: impl Into<Number>, unit: Unit) -> Self {
        Self {
            number: number.into(),
            unit,
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }

    pub fn uncertainty(&self) -> Self {
        Self::new(
            self.number.uncertainty,
            self.unit.clone(),
        )
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
        Quantity::new(self.number * rhs.number, self.unit * rhs.unit)
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(self.number / rhs.number, self.unit / rhs.unit)
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{unit::{LinearUnit, LinearUnitType}, unit128::Unit128};

    use super::*;

    #[test]
    fn new() {
        let n = Number::new(5, 0);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.number, n);
        assert_eq!(q.unit, u);
    }

    #[test]
    fn dimensions() {
        let n = Number::new(5, 0);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u);
        assert_eq!(q.dimensions(), Dimensions::TIME);
    }

    #[test]
    fn uncertainty() {
        let n = Number::new(20, 1);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.uncertainty(), Quantity::new(Number::from(1), u));
    }

    #[test]
    fn mul() {
        let s = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let q1 = Quantity::new(Number::new(5, 0), s.clone());
        let q2 = Quantity::new(Number::new(8, 0), s.clone());
        assert_eq!(q1 * q2, Quantity::new(Number::new(40, 0), s.clone() * s.clone()));
    }

    #[test]
    fn div() {
        let s = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let q1 = Quantity::new(Number::new(40, 0), s.clone() * s.clone());
        let q2 = Quantity::new(Number::new(8, 0), s.clone());
        assert_eq!(q1 / q2, Quantity::new(Number::new(5, 0), s.clone()));
    }
}
