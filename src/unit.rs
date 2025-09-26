use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Div, Mul};
use std::sync::Arc;

use rust_decimal::Decimal;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::id::Unit128;
use crate::prefix::Prefix;


#[derive(Clone, Debug)]
pub(crate) struct LinearFactor {
    pub(crate) unit: Arc<LinearUnit>,
    pub(crate) exponent: Frac,
}


// Intended to be stored on the heap, with user-facing units then carrying reference-counted smart
// pointers to them to allow reuse
// Base, unitless, derived, and compound units i.e. normal ones that work in multiplication
// A LinearUnit should not be cloned, it should be used and passed around only behind a pointer
#[derive(Debug)]
pub(crate) struct LinearUnit {
    pub(crate) is_base: bool,
    pub(crate) dimensions: Dimensions,
    pub(crate) symbol: Option<String>, // Compound units have None for this
    pub(crate) name: Option<String>, // Compound units have None for this
    pub(crate) prefix: Option<Prefix>, // Only possible for derived units
    pub(crate) number: Decimal,
    pub(crate) factors: Option<Arc<Vec<LinearFactor>>>, // Base units and unitless indicated by None
    pub(crate) uncertainty: Decimal,
}

impl LinearUnit {
    pub(crate) fn is_dimensionless(&self) -> bool {
        self.dimensions.is_dimensionless()
    }

    pub(crate) fn id(&self) -> Unit128 {
        todo!()
    }
}


// This is the user-facing struct representing a linear unit
#[derive(Clone, Debug)]
pub struct Unit {
    pub id: Unit128,
    pub(crate) inner: Arc<LinearUnit>,
}

impl Unit {
    pub fn is_base(&self) -> bool {
        todo!()
    }

    pub fn is_compound_base(&self) -> bool {
        todo!()
    }

    pub fn is_dimensionless(&self) -> bool {
        self.inner.is_dimensionless()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.inner.dimensions
    }

    fn generate_symbol(&self) -> String {
        todo!()
    }

    pub fn symbol(&self) -> String {
        self.inner.symbol.clone().unwrap_or(self.generate_symbol())
    }

    fn generate_name(&self) -> String {
        todo!()
    }

    pub fn name(&self) -> String {
        self.inner.name.clone().unwrap_or(self.generate_name())
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


#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass(frozen, name = "Unit")]
    #[derive(Clone, Debug)]
    pub struct PyUnit(Unit);

    impl PyUnit {
        pub fn into_inner(self) -> Unit {
            self.0
        }
    }

    impl From<Unit> for PyUnit {
        fn from(value: Unit) -> Self {
            Self(value)
        }
    }
}
