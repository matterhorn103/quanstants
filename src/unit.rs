use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::ops::{Div, Mul, Neg};
use std::sync::Arc;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::number::Number;
use crate::prefix::Prefix;
use crate::unit128::Unit128;

#[derive(Clone, Debug)]
pub(crate) struct LinearFactor {
    pub(crate) unit: Arc<LinearUnit>,
    pub(crate) exponent: Frac,
}

impl LinearFactor {
    pub(crate) fn inverse(self) -> Self {
        LinearFactor {
            unit: self.unit,
            exponent: self.exponent.neg(),
        }
    }
}

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
    pub(crate) number: Number,
    pub(crate) factors: Vec<LinearFactor>, // Empty for base units and unitless
}

impl LinearUnit {
    // Note that for derived units there is no way to know whether the unit has been assigned its
    // own unique ID, so they are all returned with a least-significant bit of 0x0D as standard
    //pub(crate) fn generate_id(&self) -> Unit128 {
    //    let lsb = match self.utype {
    //        LinearUnitType::Unitless => 0x00,
    //        LinearUnitType::Base => 0x00,
    //        LinearUnitType::Derived => 0x0D,
    //        LinearUnitType::Compound => 0x0C,
    //    };
    // THIS WON'T WORK IN CURRENT STATE BECAUSE SELF.NUMBER IS FOR THE UNIT DEFINITION, NOT THE
    // ACTUAL UNDERLYING NUMERIC FACTOR
    //    Unit128::new(self.number.try_into().unwrap(), self.dimensions, lsb)
    //}
}

impl LinearUnit {
    #[allow(dead_code)]
    pub const UNITLESS: LinearUnit = LinearUnit {
        utype: LinearUnitType::Unitless,
        dimensions: Dimensions::DIMENSIONLESS,
        symbol: None,
        name: None,
        prefix: None,
        number: Number::ONE,
        factors: vec![],
    };
}

// This is the user-facing struct representing a linear unit
#[derive(Clone, Debug)]
pub struct Unit {
    pub id: Unit128,
    pub(crate) inner: Arc<LinearUnit>,
}

impl Unit {
    pub fn unitless() -> Self {
        Unit {
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

    fn generate_symbol(&self) -> &str {
        todo!()
    }

    pub fn symbol(&self) -> &str {
        self.inner.symbol.as_deref().unwrap_or_else(|| self.generate_symbol())
    }

    fn generate_name(&self) -> &str {
        todo!()
    }

    pub fn name(&self) -> &str {
        self.inner.name.as_deref().unwrap_or_else(|| self.generate_name())
    }

    pub fn number(&self) -> Number {
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

    fn mul(self, rhs: Unit) -> Unit {
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
                .chain(rhs.to_inverse_factors())
                .collect(),
        });
        let new_id = self.id * rhs.id;
        Unit {
            id: new_id,
            inner: new_inner,
        }
    }
}

impl Div for Unit {
    type Output = Self;

    fn div(self, rhs: Unit) -> Unit {
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
        Unit {
            id: new_id,
            inner: new_inner,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
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
                    Quantity::new(Number::from_f64(float, 0.0).unwrap(), self.owned_inner()).into()
                }
                RArithmeticEnum::Decimal(decimal) => {
                    Quantity::new(decimal, self.owned_inner()).into()
                }
                RArithmeticEnum::String(string) => {
                    Quantity::new(Decimal::from_str_exact(&string).unwrap(), self.owned_inner()).into()
                }
            }
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
    use crate::unit128::NumericFactor;
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
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        let s2 = s.clone();
        assert_eq!(s, s2);
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
                number: Number::ONE,
                factors: Vec::new(),
            }),
        };
        dbg!(&s.inner.symbol);
        assert_eq!(s.symbol(), "s");
    }
}
