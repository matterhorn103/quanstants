use std::cmp::Ordering;
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
pub(crate) struct LinearUnit {
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
    pub(crate) fn symbol(&self, use_superscripts: bool) -> String {
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

    pub(crate) fn name(&self) -> String {
        todo!()
    }
}

impl LinearUnit {
    #[allow(dead_code)]
    pub const UNITLESS: LinearUnit = LinearUnit {
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
pub(crate) struct LinearFactor {
    pub(crate) unit: Arc<LinearUnit>,
    pub(crate) exponent: Frac,
}

// Some logic is implemented on a per-LinearFactor basis to make it easier for a Unit to
// iterate over its factors
impl LinearFactor {
    pub(crate) fn inverse(self) -> Self {
        Self {
            unit: self.unit,
            exponent: self.exponent.neg(),
        }
    }

    pub(crate) fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        // We maybe need to not just do this simple logic for compound units but for everything else
        // it works fine
        Self {
            unit: self.unit,
            exponent: self.exponent * exponent.into(),
        }
    }

    pub(crate) fn symbol(&self, use_superscripts: bool) -> String {
        // This will be fine as long as we don't allow LinearFactors to hold a Compound unit with a
        // non-unity exponent
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
    pub fn unitless() -> Self {
        Self {
            id: Unit128::UNITLESS,
            inner: Arc::new(LinearUnit::UNITLESS),
        }
    }

    pub fn is_base(&self) -> bool {
        matches!(self.inner.utype, LinearUnitType::Base)
    }

    pub fn is_compound_base(&self) -> bool {
        todo!()
    }

    pub fn is_dimensionless(&self) -> bool {
        self.inner.dimensions.is_dimensionless()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.inner.dimensions
    }

    pub fn symbol(&self, use_superscripts: bool) -> String {
        self.inner.symbol(use_superscripts)
    }

    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn number(&self) -> SciNum {
        self.inner.number
    }

    fn to_factors(&self) -> Vec<LinearFactor> {
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

    fn to_inverse_factors(&self) -> Vec<LinearFactor> {
        self.to_factors().into_iter().map(|x| x.inverse()).collect()
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        let exponent: Frac = exponent.into();
        let new_inner = Arc::new(LinearUnit {
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
        });
        let new_id = self.id.pow(exponent);
        Unit {
            id: new_id,
            inner: new_inner,
        }
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

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let new_inner = Arc::new(LinearUnit {
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
        });
        let new_id = self.id * rhs.id;
        Self {
            id: new_id,
            inner: new_inner,
        }
    }
}

impl Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let new_inner = Arc::new(LinearUnit {
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
        });
        let new_id = self.id / rhs.id;
        Self {
            id: new_id,
            inner: new_inner,
        }
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
    use crate::{
        quantity::{py::PyQuantity, Quantity},
        unit128::py::PyUnitId,
    };

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

        fn __rmul__(&self, other: RArithmeticEnum) -> PyQuantity {
            match other {
                RArithmeticEnum::Quantity(py_quantity) => {
                    PyQuantity::from(py_quantity.into_inner() * self.owned_inner())
                }
                RArithmeticEnum::Int(integer) => Quantity::new(integer, self.owned_inner()).into(),
                RArithmeticEnum::Float(float) => {
                    Quantity::new(SciNum::from_f64(float, 0.0).unwrap(), self.owned_inner()).into()
                }
                RArithmeticEnum::Decimal(decimal) => {
                    Quantity::new(decimal, self.owned_inner()).into()
                }
                RArithmeticEnum::String(string) => Quantity::new(
                    Decimal::from_str_exact(&string).unwrap(),
                    self.owned_inner(),
                )
                .into(),
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

    #[derive(FromPyObject)]
    enum RArithmeticEnum {
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
        let s2 = s.clone();
        assert_eq!(s, s2);
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
        let m = Unit {
            id: Unit128::METRE,
            inner: Arc::new(LinearUnit {
                utype: LinearUnitType::Base,
                dimensions: Dimensions::LENGTH,
                symbol: Some(String::from("m")),
                name: Some(String::from("metre")),
                prefix: None,
                number: SciNum::ONE,
                factors: Vec::new(),
            }),
        };
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
        dbg!(&s.inner.symbol);
        assert_eq!(s.symbol(false), "s");
    }

    #[test]
    fn symbol_compound() {
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
        let s2 = s.clone() * s.clone();
        dbg!(&s2.inner.symbol);
        assert_eq!(s2.symbol(false), "s2");
    }

    #[test]
    fn debug() {
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
        assert_eq!(format!("{:?}", s), "Unit { id: 1100, inner: s }");
    }
}
