use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Div, Mul};
use std::rc::Rc;
use std::sync::Arc;

use rust_decimal::Decimal;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::id::Unit128;
use crate::numeric::Numeric;
use crate::prefix::Prefix;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum UnitType {
    Base,
    Unitless,
    Derived,
    Compound,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum LinearFactor {
    Base(Rc<BaseUnit>, Frac),
    Unitless(Rc<UnitlessUnit>, Frac),
    Derived(Rc<DerivedUnit>, Frac),
}

impl LinearFactor {
    pub fn symbol(&self) -> String {
        match self {
            LinearFactor::Base(unit, exp) => unit.symbol() + "^" + &exp.to_string(),
            LinearFactor::Unitless(unit, exp) => unit.symbol() + "^" + &exp.to_string(),
            LinearFactor::Derived(unit, exp) => unit.symbol() + "^" + &exp.to_string(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            LinearFactor::Base(unit, exp) => unit.dimensions().pow(*exp),
            LinearFactor::Unitless(unit, exp) => unit.dimensions().pow(*exp),
            LinearFactor::Derived(unit, exp) => unit.dimensions().pow(*exp),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BaseUnit {
    symbol: String,
    name: String,
    dimensions: Dimensions,
    prefixed: bool,
}

impl BaseUnit {
    pub fn new(symbol: String, name: String, dimensions: Dimensions) -> Self {
        Self {
            symbol,
            name,
            dimensions,
            prefixed: false,
        }
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}

impl Mul for BaseUnit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Unit {
        Unit::new(&[
            LinearFactor::Base(self, 1.into()),
            LinearFactor::Base(rhs, 1.into()),
        ])
    }
}

//impl From<BaseUnit> for Unit {
//    fn from(value: BaseUnit) -> Self {
//        Unit::Base(value)
//    }
//}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UnitlessUnit;

//#[pymethods]
//impl UnitlessUnit {
//    pub fn __repr__(&self) -> String {
//        format!("BaseUnit({})", self.name())
//    }
//
//    pub fn __str__(&self) -> String {
//        self.symbol()
//    }
//}

impl UnitlessUnit {
    fn symbol(&self) -> String {
        String::from("(unitless)")
    }

    fn name(&self) -> String {
        String::from("unitless")
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions::new(0, 0, 0, 0, 0, 0, 0)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct DerivedUnit {
    symbol: String,
    name: String,
    prefix: Option<Prefix>,
    def_number: Decimal,
    def_factors: Vec<LinearFactor>,
    def_uncertainty: Decimal,
}

impl DerivedUnit {
    pub fn new(
        symbol: String,
        name: String,
        //prefix: Option<Prefix>,
        def_number: Decimal,
        def_factors: &[LinearFactor],
        def_uncertainty: Decimal,
    ) -> Self {
        Self {
            symbol,
            name,
            prefix: None,
            def_number,
            def_uncertainty,
            def_factors: def_factors.to_vec(),
        }
    }
}

impl DerivedUnit {
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        self.def_factors
            .iter()
            .map(|x| x.dimensions())
            .reduce(|acc, d| acc * d)
            .unwrap()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Unit {
    pub id: Unit128,
    pub factors: Vec<LinearFactor>,
}

impl Unit {
    pub fn new(factors: &[LinearFactor]) -> Self {
        todo!()
        //Self {
        //    factors: factors.to_vec(),
        //}
    }
}

impl From<BaseUnit> for Unit {
    fn from(value: BaseUnit) -> Self {
        Self::new(&[LinearFactor::Base(value, 1.into())])
    }
}

impl From<UnitlessUnit> for Unit {
    fn from(value: UnitlessUnit) -> Self {
        Self::new(&[LinearFactor::Unitless(value, 1.into())])
    }
}

impl From<DerivedUnit> for Unit {
    fn from(value: DerivedUnit) -> Self {
        Self::new(&[LinearFactor::Derived(value, 1.into())])
    }
}

impl Unit {
    pub fn id(&self) -> Unit128 {
        todo!()
    }

    pub fn symbol(&self) -> String {
        self.factors
            .iter()
            .map(|x| x.symbol())
            .reduce(|acc, s| acc + " " + &s)
            .unwrap()
    }

    pub fn name(&self) -> String {
        self.symbol()
    }

    pub fn preceding_space(&self) -> bool {
        true
    }

    pub fn dimensions(&self) -> Dimensions {
        self.factors
            .iter()
            .map(|x| x.dimensions())
            .reduce(|acc, d| acc * d)
            .unwrap()
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Unit {
        let new_factors = [self.factors, rhs.factors].concat();
        Unit::new(&new_factors)
    }
}

impl Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Unit {
        let mut new_factors = self.factors;
        for factor in rhs.factors {
            let new_factor = match factor {
                LinearFactor::Base(unit, exp) => LinearFactor::Base(unit, -exp),
                LinearFactor::Unitless(unit, exp) => LinearFactor::Unitless(unit, -exp),
                LinearFactor::Derived(unit, exp) => LinearFactor::Derived(unit, -exp),
            };
            new_factors.push(new_factor);
        }
        Unit::new(&new_factors)
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass(frozen, eq, name = "Unit")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub struct PyUnit(Unit);

    impl PyUnit {
        pub fn into_inner(self) -> Unit {
            self.0
        }
    }

    impl From<Unit> for PyUnit {
        fn from(value: Unit) -> Self {
            PyUnit(value)
        }
    }
}
