// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{self, Display},
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{Num, Zero};
use scinum::{SciNum, SciDecimal};

use crate::{dimensions::Dimensions, unit::Unit};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Quantity<N>
where
    N: Num,
{
    pub number: N,
    pub unit: Unit,
}

impl<N: Num> Quantity<N> {
    pub fn new(number: N, unit: Unit) -> Self {
        Self { number, unit }
    }

    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        self.unit.dimensions()
    }

    /// Returns `true` if the quantity's unit has the dimensions of a simple number.
    #[inline]
    pub fn is_dimensionless(&self) -> bool {
        self.dimensions().is_dimensionless()
    }

    /// Returns `true` if the quantity's unit is simply one.
    #[inline]
    pub fn is_unitless(&self) -> bool {
        self.unit.is_one()
    }
}

impl<N: Num> Add for Quantity<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number + rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl<N: Num> Sub for Quantity<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            Self::new(self.number - rhs.number, self.unit)
        } else {
            panic!()
        }
    }
}

impl<N: Num> Mul for Quantity<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.number * rhs.number,
            (self.unit * rhs.unit).cancelled_by_unit(),
        )
    }
}

impl<N: Num> Div for Quantity<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(
            self.number / rhs.number,
            (self.unit / rhs.unit).cancelled_by_unit(),
        )
    }
}

// Additional methods that only apply when the numeric type is SciNum
//impl<T: SciNum> Quantity<T> {
impl Quantity<SciDecimal> {
    pub fn uncertainty(&self) -> Self {
        Self::new(self.number.uncertainty(), self.unit.clone())
    }

    /// Creates a new `SciQuantity` with the same number and unit but the
    /// provided uncertainty.
    pub fn with_uncertainty(mut self, uncertainty: SciDecimal) -> Self {
        self.number = self.number.with_uncertainty(uncertainty);
        self
    }

    /// Returns true if the `SciQuantity` has an uncertainty of zero.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.number.is_exact()
    }

    /// If the quantity has a compound unit, returns a new quantity with the
    /// terms that contain identical units combined.
    ///
    /// For example, `3 m s² m⁻¹` becomes `3 s²`,
    /// and `0.78 J K⁻¹ J` becomes `0.78 J² K⁻¹`
    ///
    /// Has no effect for quantities with non-compound units.
    pub fn cancelled_by_unit(self) -> Self {
        Self { number: self.number, unit: self.unit.cancelled_by_unit() }
    }

    /// If the quantity has a compound unit, returns a new quantity with the
    /// terms that contain units of the same dimensionality combined.
    ///
    /// The unit kept for each dimension is that of the first term of that
    /// dimension.
    ///
    /// For example, `1 m ft` becomes `0.3048 m²`,
    /// and `1 ft m` becomes `3.2808398… ft²`.
    ///
    /// Units are combined if they have either identical dimensions, or one has
    /// the inverse dimensions of the other.
    /// This means that, for example, `s² Hz` becomes `s` (because `Hz = s⁻¹`),
    /// but `N m` does _not_ become `J` (even though `J = N m`).
    ///
    /// Has no effect for quantities with non-compound units.
    pub fn cancelled_by_dimension(self) -> Self {
        let cancelled = self.unit.cancelled_by_dimension();
        Self { number: self.number * cancelled.number, unit: cancelled.unit }
    }

    // This ought to be generic
    /// Returns the value of the `SciQuantity` when expressed in base units.
    pub fn in_base(&self) -> Self {
        if self.unit.is_base() || self.unit.is_compound_base() {
            self.clone()
        } else {
            self.number * self.unit.in_base()
        }
    }

    // This ought to be generic
    /// Returns the value of the `Quantity` when expressed in the given unit.
    /// 
    /// Returns `None` if the units have different dimensionality.
    pub fn in_unit(&self, unit: &Unit) -> Option<Self> {
        if self.number.is_zero() && self.number.is_exact() {
            return Some(Self { number: SciDecimal::zero(), unit: unit.clone() })
        };
        // Original quantity q0 = n0 * u0
        // The desired new unit is u1
        // If there is a number n1 such that u0 = n1 * u1,
        // the new quantity is then q1 = n0 * (n1 * u1) = (n0 * n1) * u1
        // Find n1 by: n1 = u0 / u1
        let ratio = (self.unit.value() / unit.value()).cancelled_by_dimension();
        if ratio.is_unitless() {
            Some(Self { number: self.number * ratio.number, unit: unit.clone() })
        } else {
            None
        }
    }
}

/// Blanket implementation of From and Into for numeric types to enable direct
/// conversion to corresponding unitless quantities
impl<N: Num> From<N> for Quantity<N> {
    fn from(n: N) -> Self {
        Quantity {
            number: n,
            unit: Unit::one(),
        }
    }
}

/// Derives From and Into for integer types that already convert into a `SciDecimal`.
macro_rules! impl_from_for_sci_quant {
    ($t:ty) => {
        impl From<$t> for Quantity<SciDecimal> {
            fn from(n: $t) -> Quantity<SciDecimal> {
                Quantity {
                    number: n.into(),
                    unit: Unit::one(),
                }
            }
        }

        impl From<Quantity<$t>> for Quantity<SciDecimal> {
            fn from(q: Quantity<$t>) -> Quantity<SciDecimal> {
                Quantity {
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
impl_from_for_sci_quant!(u8);
impl_from_for_sci_quant!(u16);
impl_from_for_sci_quant!(u32);
impl_from_for_sci_quant!(u64);

//// Arithmetic functions for correlated uncertainties
//impl SciQuantity {
//    /// Adds two quantities and propagates the uncertainties as appropriate for
//    /// the given correlation.
//    pub fn add_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
//    where
//        T: Into<Decimal>,
//    {
//        if self.unit == rhs.unit {
//            Self::new(
//                self.number.add_with_correlation(rhs.number, correlation),
//                self.unit,
//            )
//        } else {
//            panic!()
//        }
//    }
//
//    /// Subtracts two quantities and propagates the uncertainties as appropriate
//    /// for the given correlation.
//    pub fn sub_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
//    where
//        T: Into<Decimal>,
//    {
//        if self.unit == rhs.unit {
//            Self::new(
//                self.number.sub_with_correlation(rhs.number, correlation),
//                self.unit,
//            )
//        } else {
//            panic!()
//        }
//    }
//
//    /// Multiplies two quantities and propagates the uncertainties as
//    /// appropriate for the given correlation.
//    pub fn mul_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
//    where
//        T: Into<Decimal> + Copy,
//    {
//        Self::new(
//            self.number.mul_with_correlation(rhs.number, correlation),
//            (self.unit * rhs.unit).cancelled_by_unit(),
//        )
//    }
//
//    /// Divides two quantities and propagates the uncertainties as appropriate
//    /// for the given correlation.
//    pub fn div_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
//    where
//        T: Into<Decimal> + Copy,
//    {
//        Self::new(
//            self.number.div_with_correlation(rhs.number, correlation),
//            (self.unit / rhs.unit).cancelled_by_unit(),
//        )
//    }
//}

impl<N: Num + Display> Display for Quantity<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_unitless() {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{} {}", self.number, self.unit.symbol(false))
        }
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use std::str::FromStr;

    use crate::{num::py::{PyIntoSciDecimal, PySciDecimal}, unit::py::PyUnit};

    use super::*;
    use pyo3::prelude::*;
    use bigdecimal::BigDecimal;

    #[pyclass(frozen, name = "Quantity")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub(crate) struct PyQuantity(Quantity<SciDecimal>);

    impl PyQuantity {
        pub fn into_inner(self) -> Quantity<SciDecimal> {
            self.0
        }

        pub fn borrow_inner(&self) -> &Quantity<SciDecimal> {
            &self.0
        }

        pub fn owned_inner(&self) -> Quantity<SciDecimal> {
            self.0.clone()
        }
    }

    impl From<Quantity<SciDecimal>> for PyQuantity {
        fn from(n: Quantity<SciDecimal>) -> Self {
            Self(n)
        }
    }

    #[pymethods]
    impl PyQuantity {
        #[new]
        fn new(number: PyIntoSciDecimal, unit: PyUnit) -> Self {
            let number: SciDecimal = number.try_into().unwrap();
            Self(Quantity::new(number, unit.into_inner()))
        }

        #[getter]
        fn number(&self) -> PySciDecimal {
            self.borrow_inner().number.into()
        }

        #[getter]
        fn uncertainty(&self) -> Self {
            self.borrow_inner().uncertainty().into()
        }

        #[getter]
        fn unit(&self) -> PyUnit {
            self.borrow_inner().unit.clone().into()
        }

        fn __str__(&self) -> String {
            format!("{} {}", self.0.number, self.0.unit.symbol(true))
        }

        fn __repr__(&self) -> String {
            let inner = self.borrow_inner();
            let unit_symbol = if inner.is_unitless() { "(unitless)".to_string() } else { inner.unit.symbol(true) };
            if inner.is_exact() {
                format!("Quantity({}, {})", inner.number, unit_symbol)
            } else {
                format!(
                    "Quantity({}, {}, uncertainty={})",
                    inner.number.number(),
                    unit_symbol,
                    inner.number.uncertainty()
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
                PyQuantityArithmeticEnum::Int(i) => Self::from(self.owned_inner() * SciDecimal::from(i)),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(self.owned_inner() * SciDecimal::from_f64(f).unwrap())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(self.owned_inner() * SciDecimal::from(d))
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(self.owned_inner() * SciDecimal::from_str(&s).unwrap())
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
                PyQuantityArithmeticEnum::Int(i) => Self::from(SciDecimal::from(i) * self.owned_inner()),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(SciDecimal::from_f64(f).unwrap() * self.owned_inner())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(SciDecimal::from(d) * self.owned_inner())
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(SciDecimal::from_str(&s).unwrap() * self.owned_inner())
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
                PyQuantityArithmeticEnum::Int(i) => Self::from(self.owned_inner() / SciDecimal::from(i)),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(self.owned_inner() / SciDecimal::from_f64(f).unwrap())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(self.owned_inner() / SciDecimal::from(d))
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(self.owned_inner() / SciDecimal::from_str(&s).unwrap())
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
                PyQuantityArithmeticEnum::Int(i) => Self::from(SciDecimal::from(i) / self.owned_inner()),
                PyQuantityArithmeticEnum::Float(f) => {
                    Self::from(SciDecimal::from_f64(f).unwrap() / self.owned_inner())
                }
                PyQuantityArithmeticEnum::Decimal(d) => {
                    Self::from(SciDecimal::from(d) / self.owned_inner())
                }
                PyQuantityArithmeticEnum::String(s) => {
                    Self::from(SciDecimal::from_str(&s).unwrap() / self.owned_inner())
                }
            }
        }

        ///// Adds two quantities and propagates the uncertainties as appropriate
        ///// for the given correlation.
        //fn add_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
        //    Self::from(
        //        self.owned_inner()
        //            .add_with_correlation(rhs.owned_inner(), correlation),
        //    )
        //}

        ///// Subtracts two quantities and propagates the uncertainties as
        ///// appropriate for the given correlation.
        //fn sub_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
        //    Self::from(
        //        self.owned_inner()
        //            .sub_with_correlation(rhs.owned_inner(), correlation),
        //    )
        //}

        ///// Multiplies two quantities and propagates the uncertainties as
        ///// appropriate for the given correlation.
        //fn mul_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
        //    Self::from(
        //        self.owned_inner()
        //            .mul_with_correlation(rhs.owned_inner(), correlation),
        //    )
        //}

        ///// Divides two quantities and propagates the uncertainties as
        ///// appropriate for the given correlation.
        //fn truediv_with_correlation(&self, rhs: &Self, correlation: Decimal) -> Self {
        //    Self::from(
        //        self.owned_inner()
        //            .div_with_correlation(rhs.owned_inner(), correlation),
        //    )
        //}

        fn with_uncertainty(&self, uncertainty: PyIntoSciDecimal) -> Self {
            let uncertainty: SciDecimal = uncertainty.try_into().unwrap();
            let new_inner: Quantity<SciDecimal> = self.owned_inner().with_uncertainty(uncertainty);
            Self(new_inner)
        }

        /// Alias for `with_uncertainty()`.
        #[inline]
        fn plus_minus(&self, uncertainty: PyIntoSciDecimal) -> Self {
            self.with_uncertainty(uncertainty)
        }

        /// If the quantity has a compound unit, returns a new quantity with the
        /// terms that contain identical units combined.
        ///
        /// For example, `3 m s² m⁻¹` becomes `3 s²`,
        /// and `0.78 J K⁻¹ J` becomes `0.78 J² K⁻¹`
        ///
        /// Has no effect for quantities with non-compound units.
        fn cancelled_by_unit(&self) -> Self {
            self.owned_inner().cancelled_by_unit().into()
        }

        /// If the quantity has a compound unit, returns a new quantity with the
        /// terms that contain units of the same dimensionality combined.
        ///
        /// The unit kept for each dimension is that of the first term of that
        /// dimension.
        ///
        /// For example, `1 m ft` becomes `0.3048 m²`,
        /// and `1 ft m` becomes `3.2808398… ft²`.
        ///
        /// Units are combined if they have either identical dimensions, or one has
        /// the inverse dimensions of the other.
        /// This means that, for example, `s² Hz` becomes `s` (because `Hz = s⁻¹`),
        /// but `N m` does _not_ become `J` (even though `J = N m`).
        ///
        /// Has no effect for quantities with non-compound units.
        fn cancelled_by_dimension(&self) -> Self {
            self.owned_inner().cancelled_by_dimension().into()
        }

        /// Returns the value of the quantity when expressed in base units.
        fn in_base(&self) -> Self {
            self.owned_inner().in_base().into()
        }

        /// Returns `true` if the quantity's unit has the dimensions of a simple number.
        fn is_dimensionless(&self) -> bool {
            self.borrow_inner().is_dimensionless()
        }
    }

    #[derive(Debug, FromPyObject)]
    enum PyQuantityArithmeticEnum {
        #[pyo3(transparent, annotation = "Quantity")]
        Quantity(PyQuantity),
        #[pyo3(transparent, annotation = "Unit")]
        Unit(PyUnit),
        #[pyo3(transparent, annotation = "int")]
        Int(i64),
        #[pyo3(transparent, annotation = "float")]
        Float(f64),
        #[pyo3(transparent, annotation = "Decimal")]
        Decimal(BigDecimal),
        #[pyo3(transparent, annotation = "str")]
        String(String),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use scinum::sci;

    use crate::{
        prefix::Prefix, unit::{LinearUnit, LinearUnitType}, unit128::Unit128
    };

    use super::*;

    #[test]
    fn new() {
        let n = SciDecimal::new(5, 0);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.number, n);
        assert_eq!(q.unit, u);
    }

    #[test]
    fn dimensions() {
        let n = SciDecimal::new(5, 0);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u);
        assert_eq!(q.dimensions(), Dimensions::TIME);
    }

    #[test]
    fn uncertainty() {
        let n = SciDecimal::new_with_uncertainty(20, 1, 0);
        let u = Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let q = Quantity::new(n, u.clone());
        assert_eq!(q.uncertainty(), Quantity::new(SciDecimal::ONE, u));
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
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let q1 = Quantity::new(SciDecimal::new(5, 0), s.clone());
        let q2 = Quantity::new(SciDecimal::new(8, 0), s.clone());
        assert_eq!(
            q1 * q2,
            Quantity::new(SciDecimal::new(40, 0), s.clone() * s.clone())
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
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let q1 = Quantity::new(SciDecimal::new(40, 0), s.clone() * s.clone());
        let q2 = Quantity::new(SciDecimal::new(8, 0), s.clone());
        assert_eq!(q1 / q2, Quantity::new(SciDecimal::new(5, 0), s.clone()));
    }

    #[test]
    fn in_base() {
        let m = Unit::new(LinearUnit {
            id: Unit128::METRE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("m")),
            name: Some(String::from("metre")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let ft = Unit::new(LinearUnit {
            id: Unit128::from_bits(0xBE7FC0000000000110001),
            utype: LinearUnitType::Derived,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("ft")),
            name: Some(String::from("foot")),
            prefix: None,
            number: SciDecimal::from_str("0.3048").unwrap(),
            factors: m.to_factors(),
        });
        let q1: Quantity<SciDecimal> = sci!(0.3048) * m;
        let q2: Quantity<SciDecimal> = SciDecimal::ONE * ft;
        assert_eq!(q1.in_base(), q2.in_base());
    }

    #[test]
    fn in_unit() {
        let m = Unit::new(LinearUnit {
            id: Unit128::METRE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("m")),
            name: Some(String::from("metre")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        });
        let km = Prefix::kilo * m.clone();
        let q: Quantity<SciDecimal> = SciDecimal::new(3000, 0) * m;
        assert_eq!(q.in_unit(&km).unwrap(), SciDecimal::new(3, 0) * km);
    }
}
