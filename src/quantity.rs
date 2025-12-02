// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{self, Display},
    ops::{Add, Div, Mul, Sub},
};

use num_traits::Num;
use rust_decimal::Decimal;

use crate::{dimensions::Dimensions, scinum::SciNum, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity<T>
where
    T: Num
{
    pub number: T,
    pub unit: Unit,
}

// Specific versions
pub type SciQuantity = Quantity<SciNum>;
pub type FloatQuantity = Quantity<f64>;
pub type DecQuantity = Quantity<Decimal>;

impl<T: Num> Quantity<T> {
    pub fn new(number: T, unit: Unit) -> Self {
        Self { number, unit }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }
}

impl<T: Num> Add for Quantity<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number + rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl<T: Num> Sub for Quantity<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number - rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl<T: Num> Mul for Quantity<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.number * rhs.number, (self.unit * rhs.unit).cancel_by_unit())
    }
}

impl<T: Num> Div for Quantity<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.number / rhs.number, (self.unit / rhs.unit).cancel_by_unit())
    }
}

// Additional methods that only apply to SciQuantity
impl SciQuantity {
    pub fn uncertainty(&self) -> Self {
        Self::new(self.number.uncertainty(), self.unit.clone())
    }

    /// Creates a new `SciQuantity` with the same number and unit but the provided uncertainty.
    ///
    /// Currently panics if the current number and the uncertainty have different values for
    /// `exponent`.
    pub fn with_uncertainty(mut self, uncertainty: SciNum) -> Self {
        if self.number.exponent != uncertainty.exponent {
            todo!()
        };
        self.number = self.number.with_uncertainty(uncertainty);
        self
    }

    /// Returns true if the `SciQuantity` has an uncertainty of zero.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.number.is_exact()
    }
}

/// Derives From and Into for types that already convert into a `SciNum`.
macro_rules! impl_from_for_sci_quant {
    ($t:ty) => {
        impl From<$t> for SciQuantity {
            fn from(n: $t) -> SciQuantity {
                SciQuantity {
                    number: n.into(),
                    unit: Unit::one(),
                }
            }
        }

        impl From<Quantity<$t>> for SciQuantity {
            fn from(q: Quantity<$t>) -> SciQuantity {
                SciQuantity {
                    number: q.number.into(),
                    unit: q.unit,
                }
            }
        }
    };
}

impl_from_for_sci_quant!(i8);
impl_from_for_sci_quant!(i16);
impl_from_for_sci_quant!(i32);
impl_from_for_sci_quant!(i64);
impl_from_for_sci_quant!(i128);
impl_from_for_sci_quant!(isize);
impl_from_for_sci_quant!(u8);
impl_from_for_sci_quant!(u16);
impl_from_for_sci_quant!(u32);
impl_from_for_sci_quant!(u64);
impl_from_for_sci_quant!(u128);
impl_from_for_sci_quant!(usize);
impl_from_for_sci_quant!(Decimal);

// Arithmetic functions for correlated uncertainties
impl SciQuantity {
    /// Adds two quantities and propagates the uncertainties as appropriate for the given
    /// correlation.
    pub fn add_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        if self.unit == rhs.unit {
            Self::new(
                self.number.add_with_correlation(rhs.number, correlation),
                self.unit,
            )
        } else {
            panic!()
        }
    }

    /// Subtracts two quantities and propagates the uncertainties as appropriate for the given
    /// correlation.
    pub fn sub_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        if self.unit == rhs.unit {
            Self::new(
                self.number.sub_with_correlation(rhs.number, correlation),
                self.unit,
            )
        } else {
            panic!()
        }
    }

    /// Multiplies two quantities and propagates the uncertainties as appropriate for the given
    /// correlation.
    pub fn mul_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal> + Copy,
    {
        Self::new(
            self.number.mul_with_correlation(rhs.number, correlation),
            self.unit * rhs.unit,
        )
    }

    /// Divides two quantities and propagates the uncertainties as appropriate for the given
    /// correlation.
    pub fn div_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal> + Copy,
    {
        Self::new(
            self.number.div_with_correlation(rhs.number, correlation),
            self.unit / rhs.unit,
        )
    }
}

impl<T: Num + Display> Display for Quantity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit.symbol(false))
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use std::str::FromStr;

    use crate::{scinum::py::PyIntoSciNum, unit::py::PyUnit};

    use super::*;
    use pyo3::prelude::*;
    use rust_decimal::Decimal;

    #[pyclass(frozen, name = "Quantity")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub(crate) struct PyQuantity(pub(crate) SciQuantity);

    impl PyQuantity {
        pub fn into_inner(self) -> SciQuantity {
            self.0
        }

        pub fn borrow_inner(&self) -> &SciQuantity {
            &self.0
        }

        pub fn owned_inner(&self) -> SciQuantity {
            self.0.clone()
        }
    }

    impl From<SciQuantity> for PyQuantity {
        fn from(value: SciQuantity) -> Self {
            Self(value)
        }
    }

    #[pymethods]
    impl PyQuantity {
        #[new]
        fn new(number: PyIntoSciNum, unit: PyUnit) -> Self {
            let number: SciNum = number.try_into().unwrap();
            Self(Quantity::new(number, unit.into_inner()))
        }

        fn __str__(&self) -> String {
            format!("{} {}", self.0.number, self.0.unit.symbol(true))
        }

        fn __repr__(&self) -> String {
            if self.0.is_exact() {
                format!("Quantity({}, {})", self.0.number, self.0.unit.symbol(true))
            } else {
                format!(
                    "Quantity({}, {}, uncertainty={})",
                    self.0.number.number(),
                    self.0.unit.symbol(true),
                    self.0.number.uncertainty()
                )
            }
        }

        fn __eq__(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn __add__(&self, other: &Self) -> Self {
            Self::from(self.owned_inner() + other.owned_inner())
        }

        fn __radd__(&self, other: &Self) -> Self {
            Self::from(other.owned_inner() + self.owned_inner())
        }

        fn __sub__(&self, other: &Self) -> Self {
            Self::from(self.owned_inner() - other.owned_inner())
        }

        fn __rsub__(&self, other: &Self) -> Self {
            Self::from(other.owned_inner() - self.owned_inner())
        }

        fn __mul__(&self, other: PyQuantityArithmeticEnum) -> Self {
            match other {
                PyQuantityArithmeticEnum::Quantity(q) => {
                    Self::from(self.owned_inner() * q.into_inner())
                }
                PyQuantityArithmeticEnum::Unit(u) => {
                    Self::from(self.owned_inner() * u.into_inner())
                }
                PyQuantityArithmeticEnum::Int(i) => Self::from(self.owned_inner() * i),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(self.owned_inner() * SciNum::from_f64_exact(f).unwrap())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(self.owned_inner() * SciNum::new_exact(d))
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(self.owned_inner() * SciNum::from_str(&s).unwrap())
                }
            }
        }

        fn __rmul__(&self, other: PyQuantityArithmeticEnum) -> Self {
            match other {
                PyQuantityArithmeticEnum::Quantity(q) => {
                    Self::from(q.into_inner() * self.owned_inner())
                }
                PyQuantityArithmeticEnum::Unit(u) => {
                    Self::from(u.into_inner() * self.owned_inner())
                }
                PyQuantityArithmeticEnum::Int(i) => Self::from(i * self.owned_inner()),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(SciNum::from_f64_exact(f).unwrap() * self.owned_inner())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(SciNum::new_exact(d) * self.owned_inner())
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(SciNum::from_str(&s).unwrap() * self.owned_inner())
                }
            }
        }

        fn __truediv__(&self, other: PyQuantityArithmeticEnum) -> Self {
            match other {
                PyQuantityArithmeticEnum::Quantity(q) => {
                    Self::from(self.owned_inner() / q.into_inner())
                }
                PyQuantityArithmeticEnum::Unit(u) => {
                    Self::from(self.owned_inner() / u.into_inner())
                }
                PyQuantityArithmeticEnum::Int(i) => Self::from(self.owned_inner() / i),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(self.owned_inner() / SciNum::from_f64_exact(f).unwrap())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(self.owned_inner() / SciNum::new_exact(d))
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(self.owned_inner() / SciNum::from_str(&s).unwrap())
                }
            }
        }

        fn __rtruediv__(&self, other: PyQuantityArithmeticEnum) -> Self {
            match other {
                PyQuantityArithmeticEnum::Quantity(q) => {
                    Self::from(q.into_inner() / self.owned_inner())
                }
                PyQuantityArithmeticEnum::Unit(u) => {
                    Self::from(u.into_inner() / self.owned_inner())
                }
                PyQuantityArithmeticEnum::Int(i) => Self::from(i / self.owned_inner()),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(SciNum::from_f64_exact(f).unwrap() / self.owned_inner())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(SciNum::new_exact(d) / self.owned_inner())
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(SciNum::from_str(&s).unwrap() / self.owned_inner())
                }
            }
        }

        /// Adds two quantities and propagates the uncertainties as appropriate for the given
        /// correlation.
        fn add_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
            Self::from(
                self.owned_inner()
                    .add_with_correlation(rhs.owned_inner(), correlation),
            )
        }

        /// Subtracts two quantities and propagates the uncertainties as appropriate for the given
        /// correlation.
        fn sub_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
            Self::from(
                self.owned_inner()
                    .sub_with_correlation(rhs.owned_inner(), correlation),
            )
        }

        /// Multiplies two quantities and propagates the uncertainties as appropriate for the given
        /// correlation.
        fn mul_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
            Self::from(
                self.owned_inner()
                    .mul_with_correlation(rhs.owned_inner(), correlation),
            )
        }

        /// Divides two quantities and propagates the uncertainties as appropriate for the given
        /// correlation.
        fn truediv_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
            Self::from(
                self.owned_inner()
                    .div_with_correlation(rhs.owned_inner(), correlation),
            )
        }

        fn with_uncertainty(&self, uncertainty: PyIntoSciNum) -> Self {
            let uncertainty: SciNum = uncertainty.try_into().unwrap();
            let new_inner: SciQuantity = self.owned_inner().with_uncertainty(uncertainty);
            Self(new_inner)
        }

        /// Alias for `with_uncertainty()`.
        #[inline]
        fn plus_minus(&self, uncertainty: PyIntoSciNum) -> Self {
            self.with_uncertainty(uncertainty)
        }
    }

    #[derive(Debug, FromPyObject)]
    enum PyQuantityArithmeticEnum {
        #[pyo3(transparent, annotation = "Quantity")]
        Quantity(PyQuantity),
        #[pyo3(transparent, annotation = "Unit")]
        Unit(PyUnit),
        #[pyo3(transparent, annotation = "int")]
        Int(isize),
        #[pyo3(transparent, annotation = "float")]
        Float(f64),
        #[pyo3(transparent, annotation = "Decimal")]
        Decimal(Decimal),
        #[pyo3(transparent, annotation = "str")]
        String(String),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        unit::{LinearUnit, LinearUnitType},
        unit128::Unit128,
    };

    use super::*;

    #[test]
    fn new() {
        let n = SciNum::new(5, 0);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.number, n);
        assert_eq!(q.unit, u);
    }

    #[test]
    fn dimensions() {
        let n = SciNum::new(5, 0);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u);
        assert_eq!(q.dimensions(), Dimensions::TIME);
    }

    #[test]
    fn uncertainty() {
        let n = SciNum::new(20, 1);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.uncertainty(), Quantity::new(SciNum::from(1), u));
    }

    #[test]
    fn mul() {
        let s = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let q1 = Quantity::new(SciNum::new(5, 0), s.clone());
        let q2 = Quantity::new(SciNum::new(8, 0), s.clone());
        assert_eq!(
            q1 * q2,
            Quantity::new(SciNum::new(40, 0), s.clone() * s.clone())
        );
    }

    #[test]
    fn div() {
        let s = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let q1 = Quantity::new(SciNum::new(40, 0), s.clone() * s.clone());
        let q2 = Quantity::new(SciNum::new(8, 0), s.clone());
        dbg!(&q1);
        dbg!(&q2);
        assert_eq!(q1 / q2, Quantity::new(SciNum::new(5, 0), s.clone()));
    }
}
