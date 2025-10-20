use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use crate::{dimensions::Dimensions, number::SciNum, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity {
    pub number: SciNum,
    pub unit: Unit,
}

impl Quantity {
    pub fn new(number: impl Into<SciNum>, unit: Unit) -> Self {
        Self {
            number: number.into(),
            unit,
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }

    pub fn uncertainty(&self) -> Self {
        Self::new(self.number.uncertainty(), self.unit.clone())
    }
}

impl<T> From<T> for Quantity
where
    T: Into<SciNum>,
{
    fn from(value: T) -> Self {
        Self {
            number: value.into(),
            unit: Unit::unitless(),
        }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number + rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl Sub for Quantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number - rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.number * rhs.number, self.unit * rhs.unit)
    }
}

impl Mul<Unit> for Quantity {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit * rhs)
    }
}

impl<T> Mul<T> for Quantity
where
    T: Into<SciNum>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.number * rhs.into(), self.unit)
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.number / rhs.number, self.unit / rhs.unit)
    }
}

impl Div<Unit> for Quantity {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit / rhs)
    }
}

impl<T> Div<T> for Quantity
where
    T: Into<SciNum>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.number / rhs.into(), self.unit)
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol(false))
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::unit::py::PyUnit;

    use super::*;
    use pyo3::prelude::*;
    use rust_decimal::Decimal;

    #[pyclass(frozen, name = "Quantity")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub struct PyQuantity(Quantity);

    impl PyQuantity {
        pub fn into_inner(self) -> Quantity {
            self.0
        }

        pub fn borrow_inner(&self) -> &Quantity {
            &self.0
        }

        pub fn owned_inner(&self) -> Quantity {
            self.0.clone()
        }
    }

    impl From<Quantity> for PyQuantity {
        fn from(value: Quantity) -> Self {
            Self(value)
        }
    }

    #[pymethods]
    impl PyQuantity {
        #[new]
        fn new(number: Decimal, unit: PyUnit) -> Self {
            PyQuantity(Quantity::new(number, unit.into_inner()))
        }

        fn __str__(&self) -> String {
            format!("{} {}", self.0.number, self.0.unit.symbol(true))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        unit::{LinearUnit, LinearUnitType},
        unit128::Unit128,
    };

    use super::*;

    #[test]
    fn new() {
        let n = SciNum::new(5, 0);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.number, n);
        assert_eq!(q.unit, u);
    }

    #[test]
    fn dimensions() {
        let n = SciNum::new(5, 0);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u);
        assert_eq!(q.dimensions(), Dimensions::TIME);
    }

    #[test]
    fn uncertainty() {
        let n = SciNum::new(20, 1);
        let u = Unit {
            id: Unit128::SECOND,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::TIME,
                symbol: Some(String::from("s")),
                name: Some(String::from("second")),
                prefix: None,
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.uncertainty(), Quantity::new(SciNum::from(1), u));
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
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
        let q1 = Quantity::new(SciNum::new(5, 0), s.clone());
        let q2 = Quantity::new(SciNum::new(8, 0), s.clone());
        assert_eq!(
            q1 * q2,
            Quantity::new(SciNum::new(40, 0), s.clone() * s.clone())
        );
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
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
        let q1 = Quantity::new(SciNum::new(40, 0), s.clone() * s.clone());
        let q2 = Quantity::new(SciNum::new(8, 0), s.clone());
        assert_eq!(q1 / q2, Quantity::new(SciNum::new(5, 0), s.clone()));
    }
}
