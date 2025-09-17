use std::fmt::Debug;
use std::ops::{Div, Mul};

use pyo3::prelude::*;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::id::Unit128;
use crate::prefix::Prefix;


#[pyclass(frozen, eq)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Unit {
    Base(BaseUnit),
    Unitless(UnitlessUnit),
    Derived(),
    Compound(),
    Logarithmic(),
    Temperature(),
}

impl Unit {
    pub fn id(&self) -> Unit128 {
        match self {
            Unit::Base(unit) => Unit128::new(1, 1, 10, 0, unit.dimensions(), 0x00),
            _ => Unit128(0xFFFF, 0xFFFF),
        }
    }

    pub fn symbol(&self) -> String {
        String::from("oops")
    }

    pub fn name(&self) -> String {
        match self {
            Unit::Base(base_unit) => base_unit.name(),
            _ => String::from("oops")
        }
    }

//    pub fn preceding_space(&self) -> bool;
//
    pub fn dimensions(&self) -> Dimensions {
        Dimensions::default()
    }
}


#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum LinearFactor {
    Base(BaseUnit, Frac),
    Unitless(UnitlessUnit, Frac),
    Derived(DerivedUnit, Frac),
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


#[pyclass(frozen)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct BaseUnit {
    symbol: String,
    name: String,
    dimensions: Dimensions,
    prefixed: bool,
}

//#[pymethods]
//impl BaseUnit {
//    #[new]
//    pub fn new(
//        symbol: String,
//        name: String,
//        dimensions: Dimensions,
//    ) -> Self {
//        Self {
//            symbol,
//            name,
//            dimensions,
//            prefixed: false,
//        }
//    }
//
//    pub fn __repr__(&self) -> String {
//        format!("BaseUnit({})", self.name())
//    }
//
//    pub fn __str__(&self) -> String {
//        self.symbol()
//    }
//}

impl BaseUnit {
    pub fn new(
        symbol: String,
        name: String,
        dimensions: Dimensions,
    ) -> Self {
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
    type Output = CompoundUnit;

    fn mul(self, rhs: Self) -> CompoundUnit {
        CompoundUnit::new(
            &[LinearFactor::Base(self, 1.into()), LinearFactor::Base(rhs, 1.into())]
        )
    }
}

impl From<BaseUnit> for Unit {
    fn from(value: BaseUnit) -> Self {
        Unit::Base(value)
    }
}


#[pyclass(frozen)]
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
        Dimensions::new(0,0,0,0,0,0,0)
    }
}


#[pyclass(frozen)]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct DerivedUnit {
    symbol: String,
    name: String,
    prefix: Option<Prefix>,
    def_number: f64,
    def_factors: Vec<LinearFactor>,
    def_uncertainty: f64,
}

impl DerivedUnit {
    pub fn new(
        symbol: String,
        name: String,
        //prefix: Option<Prefix>,
        def_number: f64,
        def_factors: &[LinearFactor],
        def_uncertainty: f64,
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
        self.def_factors.iter().map(|x| x.dimensions()).reduce(|acc, d| acc * d).unwrap()
    }
}


#[pyclass(frozen)]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct CompoundUnit {
    pub factors: Vec<LinearFactor>,
}

impl CompoundUnit {
    pub fn new(factors: &[LinearFactor]) -> Self {
        Self {factors: factors.to_vec()}
    }
}

impl CompoundUnit {
    fn symbol(&self) -> String {
        self.factors.iter().map(|x| x.symbol()).reduce(|acc, s| acc + " " + &s).unwrap()
    }

    fn name(&self) -> String {
        self.symbol()
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        self.factors.iter().map(|x| x.dimensions()).reduce(|acc, d| acc * d).unwrap()
    }
}

impl Mul for CompoundUnit {
    type Output = Self;

    fn mul(self, rhs: Self) -> CompoundUnit {
        let new_factors = [self.factors, rhs.factors].concat();
        CompoundUnit::new(&new_factors)
    }
}

impl Div for CompoundUnit {
    type Output = Self;

    fn div(self, rhs: Self) -> CompoundUnit {
        let mut new_factors = self.factors;
        for factor in rhs.factors {
            let new_factor = match factor {
                LinearFactor::Base(unit, exp) => LinearFactor::Base(unit, -exp),
                LinearFactor::Unitless(unit, exp) => LinearFactor::Unitless(unit, -exp),
                LinearFactor::Derived(unit, exp) => LinearFactor::Derived(unit, -exp),
            };
            new_factors.push(new_factor);
        }
        CompoundUnit::new(&new_factors)
    }
}
