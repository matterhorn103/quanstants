// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::ops::{Div, Mul, Neg};
use std::sync::Arc;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::prefix::Prefix;
use crate::scinum::SciNum;
use crate::unit128::Unit128;

#[derive(Copy, Clone, Debug)]
pub(crate) enum LinearUnitType {
    Unitless,
    Base,
    Derived,
    Compound,
}

// Intended to be stored on the heap, with user-facing units then carrying reference-counted smart
// pointers to them to allow reuse
// Base, unitless, derived, and compound units i.e. normal ones that work in multiplication
// A LinearUnit should not be cloned, it should be used and passed around only behind a pointer
#[derive(Debug)]
pub struct LinearUnit {
    pub(crate) id: Unit128,
    pub(crate) utype: LinearUnitType,
    pub(crate) dimensions: Dimensions,
    pub(crate) symbol: Option<String>, // Compound units have None for this
    pub(crate) name: Option<String>,   // Compound units have None for this
    pub(crate) prefix: Option<Prefix>, // Only possible for derived or base units
    pub(crate) number: SciNum,         // 1 for everything except derived units
    pub(crate) factors: Vec<LinearFactor>, // Empty for base units and unitless
}

// Generally LinearUnit just stores data without having its own methods
// Logic is implemented by the LinearFactor and Unit wrappers
impl LinearUnit {
    pub fn symbol(&self, use_superscripts: bool) -> String {
        match self.utype {
            LinearUnitType::Unitless => String::from(""),
            LinearUnitType::Base | LinearUnitType::Derived => self.symbol.clone().unwrap(),
            LinearUnitType::Compound => self
                .factors
                .iter()
                .map(|x| x.symbol(use_superscripts))
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    pub fn name(&self) -> String {
        match self.utype {
            LinearUnitType::Unitless => String::from(""),
            LinearUnitType::Base | LinearUnitType::Derived => self.name.clone().unwrap(),
            LinearUnitType::Compound => todo!(),
        }
    }
}

impl LinearUnit {
    #[allow(dead_code)]
    pub const UNITLESS: LinearUnit = LinearUnit {
        id: Unit128::UNITLESS,
        utype: LinearUnitType::Unitless,
        dimensions: Dimensions::DIMENSIONLESS,
        symbol: None,
        name: None,
        prefix: None,
        number: SciNum::ONE,
        factors: vec![],
    };
}

#[derive(Clone, Debug)]
pub struct LinearFactor {
    pub unit: Arc<LinearUnit>,
    pub exponent: Frac,
}

// Some logic is implemented on a per-LinearFactor basis to make it easier for a Unit to
// iterate over its factors
impl LinearFactor {
    /// Takes the inverse of the linear factor by multiplying the exponent by −1
    pub(crate) fn inverse(self) -> Self {
        Self {
            unit: self.unit,
            exponent: self.exponent.neg(),
        }
    }

    pub(crate) fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        // We maybe need to do more complicated logic for compound units,
        // but for everything else it works fine
        Self {
            unit: self.unit,
            exponent: self.exponent * exponent.into(),
        }
    }

    /// Returns the combined symbol of the unit and its exponent.
    /// If `use_superscripts` is `true`, uses Unicode superscript characters for the exponent.
    pub(crate) fn symbol(&self, use_superscripts: bool) -> String {
        // This will be fine as long as we don't allow LinearFactors to hold
        // a Compound unit with a non-unity exponent
        if self.exponent == 1 {
            self.unit.symbol(false)
        } else if use_superscripts {
            format!(
                "{}{}",
                self.unit.symbol(true),
                self.exponent.to_superscript()
            )
        } else {
            format!("{}{}", self.unit.symbol(false), self.exponent)
        }
    }
}

// This is the user-facing struct representing a linear unit
#[derive(Clone)]
pub struct Unit {
    pub id: Unit128,
    pub(crate) inner: Arc<LinearUnit>,
}

impl Unit {
    pub fn new(inner: LinearUnit) -> Self {
        Self {
            id: inner.id,
            inner: Arc::new(inner),
        }
    }

    pub fn unitless() -> Self {
        Self {
            id: Unit128::UNITLESS,
            inner: Arc::new(LinearUnit::UNITLESS),
        }
    }

    #[inline]
    pub fn is_base(&self) -> bool {
        matches!(self.inner.utype, LinearUnitType::Base)
    }

    #[inline]
    pub fn is_compound(&self) -> bool {
        matches!(self.inner.utype, LinearUnitType::Compound)
    }

    pub fn is_compound_base(&self) -> bool {
        todo!()
    }

    #[inline]
    pub fn is_dimensionless(&self) -> bool {
        self.inner.dimensions.is_dimensionless()
    }

    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        self.inner.dimensions
    }

    #[inline]
    pub fn symbol(&self, use_superscripts: bool) -> String {
        self.inner.symbol(use_superscripts)
    }

    #[inline]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    pub fn number(&self) -> SciNum {
        self.inner.number
    }

    #[inline]
    pub fn defining_factors(&self) -> Vec<LinearFactor> {
        self.inner.factors.clone()
    }

    pub fn to_factors(&self) -> Vec<LinearFactor> {
        match self.inner.utype {
            LinearUnitType::Base | LinearUnitType::Derived => {
                vec![LinearFactor {
                    unit: self.inner.clone(),
                    exponent: 1.into(),
                }]
            }
            LinearUnitType::Compound | LinearUnitType::Unitless => self.inner.factors.clone(),
        }
    }

    #[inline]
    fn to_inverse_factors(&self) -> Vec<LinearFactor> {
        self.to_factors().into_iter().map(|x| x.inverse()).collect()
    }

    /// Combines factors of a compound unit that contain identical units.
    /// 
    /// For example, `m s² m⁻¹` becomes `s²`, and `J K⁻¹ J` becomes `J² K⁻¹`
    /// 
    /// Has no effect for non-compound units.
    pub fn cancel_by_unit(&self) -> Self {
        let old_factors = self.to_factors();
        let mut factors_map: HashMap<Unit128, LinearFactor> = HashMap::with_capacity(old_factors.len());
        for old_factor in old_factors {
            let k = old_factor.unit.id;
            factors_map.entry(k)
            .and_modify(|new_factor| new_factor.exponent += old_factor.exponent)
            .or_insert(old_factor);
        }   
        let new_factors = factors_map.into_values().collect();

        Self::new(LinearUnit {
            id: self.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number(),
            factors: new_factors,
        })
    }
}

// Equality and ordering functions not covered by traits
impl Unit {
    /// Returns `true` if the two units are exactly the same.
    /// 
    /// `identical()` differs from `eq()` in that it returns `false` for two different units
    /// that have the same value.
    pub fn identical(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.id.normalize() == other.id.normalize()
    }
}

impl Eq for Unit {}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.normalize().cmp(&other.id.normalize())
    }
}

// Arithmetic not covered by traits
impl Unit {
    pub fn inverse(self) -> Self {
        Self::new(LinearUnit {
            id: self.id.inverse(),
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions().inverse(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number().inverse(),
            factors: self.to_inverse_factors(),
        })
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        let exponent: Frac = exponent.into();
        Self::new(LinearUnit {
            id: self.id.pow(exponent),
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions().pow(exponent),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number().powf(exponent.to_f64()),
            factors: self
                .to_factors()
                .into_iter()
                .map(|x| x.pow(exponent))
                .collect(),
        })
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(LinearUnit {
            id: self.id * rhs.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions() * rhs.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number() * rhs.number(),
            factors: self
                .to_factors()
                .into_iter()
                .chain(rhs.to_factors())
                .collect(),
        })
    }
}

impl Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(LinearUnit {
            id: self.id / rhs.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions() / rhs.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number() / rhs.number(),
            factors: self
                .to_factors()
                .into_iter()
                .chain(rhs.to_inverse_factors())
                .collect(),
        })
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unit {{ id: {:X}, inner: {} }}",
            self.id.to_bits(),
            self.inner.symbol(true)
        )
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol(true))
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use std::str::FromStr;

    use crate::{quantity::py::PyQuantity, unit128::py::PyUnitId};

    use super::*;
    use pyo3::prelude::*;
    use rust_decimal::Decimal;

    #[pyclass(frozen, name = "Unit")]
    #[derive(Clone, Debug)]
    pub struct PyUnit(Unit);

    impl PyUnit {
        pub fn into_inner(self) -> Unit {
            self.0
        }

        pub fn borrow_inner(&self) -> &Unit {
            &self.0
        }

        pub fn owned_inner(&self) -> Unit {
            self.0.clone()
        }
    }

    impl From<Unit> for PyUnit {
        fn from(value: Unit) -> Self {
            Self(value)
        }
    }

    #[pymethods]
    impl PyUnit {
        fn __eq__(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn __mul__(&self, other: &Self) -> Self {
            Self(self.owned_inner() * other.owned_inner())
        }

        // Only 3 * m is valid, not m * 3, so only define rmul and rtruediv for mixed ops
        // Operations between units and quantities are all handled by PyQuantity
        fn __rmul__(&self, other: PyUnitArithmeticEnum) -> PyQuantity {
            match other {
                PyUnitArithmeticEnum::Quantity(q) => {
                    PyQuantity::from(q.into_inner() * self.owned_inner())
                }
                PyUnitArithmeticEnum::Int(i) => (i * self.owned_inner()).into(),
                PyUnitArithmeticEnum::Float(f) => {
                    (SciNum::from_f64_exact(f).unwrap() * self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::Decimal(d) => (d * self.owned_inner()).into(),
                PyUnitArithmeticEnum::String(s) => {
                    (SciNum::from_str(&s).unwrap() * self.owned_inner()).into()
                }
            }
        }

        fn __truediv__(&self, other: &Self) -> Self {
            Self(self.owned_inner() / other.owned_inner())
        }

        fn __rtruediv__(&self, other: PyUnitArithmeticEnum) -> PyQuantity {
            match other {
                PyUnitArithmeticEnum::Quantity(q) => {
                    PyQuantity::from(q.into_inner() / self.owned_inner())
                }
                PyUnitArithmeticEnum::Int(i) => (i / self.owned_inner()).into(),
                PyUnitArithmeticEnum::Float(f) => {
                    (SciNum::from_f64_exact(f).unwrap() / self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::Decimal(d) => (d / self.owned_inner()).into(),
                PyUnitArithmeticEnum::String(s) => {
                    (SciNum::from_str(&s).unwrap() / self.owned_inner()).into()
                }
            }
        }

        fn __pow__(&self, other: i8, _modulo: &Bound<'_, PyAny>) -> Self {
            Self(self.owned_inner().pow(other))
        }

        #[getter]
        fn id(&self) -> PyUnitId {
            PyUnitId(self.0.id)
        }
    }

    #[derive(Debug, FromPyObject)]
    enum PyUnitArithmeticEnum {
        #[pyo3(transparent, annotation = "Quantity")]
        Quantity(PyQuantity),
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
    use super::*;

    #[test]
    fn equality() {
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
        let s2 = s.clone();
        assert_eq!(s, s2);
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
        let m = Unit::new(LinearUnit {
            id: Unit128::METRE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("m")),
            name: Some(String::from("metre")),
            prefix: None,
            number: SciNum::ONE,
            factors: Vec::new(),
        });
        let ms = m.clone() * s.clone();
        let mm = m.clone() * m.clone();
        assert_eq!(ms.symbol(false), "m s");
        assert_eq!(ms.id, Unit128::from_bits(0x11110C));
        assert_eq!(ms.dimensions(), Dimensions::new(1, 1, 0, 0, 0, 0, 0));
        assert_eq!(mm.symbol(false), "m m");
        assert_eq!(mm.id, Unit128::from_bits(0x12000C));
        assert_eq!(mm.dimensions(), Dimensions::new(0, 2, 0, 0, 0, 0, 0));
    }

    #[test]
    fn symbol() {
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
        dbg!(&s.inner.symbol);
        assert_eq!(s.symbol(false), "s");
    }

    #[test]
    fn symbol_compound() {
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
        let s2 = s.clone() * s.clone();
        dbg!(&s2.inner.symbol);
        assert_eq!(s2.symbol(false), "s s");
    }

    #[test]
    fn symbol_compound_cancelled() {
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
        let s2 = s.clone() * s.clone();
        dbg!(&s2.inner.symbol);
        let s2_cancelled = s2.cancel_by_unit();
        dbg!(&s2_cancelled.inner.symbol);
        assert_eq!(s2_cancelled.symbol(false), "s2");
    }

    #[test]
    fn debug() {
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
        assert_eq!(format!("{s:?}"), "Unit { id: 1100, inner: s }");
    }
}
