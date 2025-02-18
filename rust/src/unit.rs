use std::fmt::Debug;
use std::ops::{Div, Mul};

use pyo3::prelude::*;

use crate::dimensions::Dimensions;
use crate::prefix::Prefix;

pub trait Unit: Clone + Debug + PartialEq {
    fn symbol(&self) -> String;

    fn name(&self) -> String;

    fn preceding_space(&self) -> bool;

    fn dimensions(&self) -> Dimensions;
}


#[derive(Clone, Debug, PartialEq)]
pub enum LinearFactor {
    Base(BaseUnit, i8),
    Unitless(UnitlessUnit, i8),
    Derived(DerivedUnit, i8),
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


#[pyclass]
#[derive(Clone, Debug, PartialEq)]
pub struct BaseUnit {
    symbol: String,
    name: String,
    dimensions: Dimensions,
}

#[pymethods]
impl BaseUnit {
    #[new]
    pub fn new(
        symbol: String,
        name: String,
        dimensions: Dimensions,
    ) -> Self {
        Self {
            symbol,
            name,
            dimensions,
        }
    }
}

impl Unit for BaseUnit {
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
            &[LinearFactor::Base(self, 1), LinearFactor::Base(rhs, 1)]
        )
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitlessUnit;

impl Unit for UnitlessUnit {
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


#[derive(Clone, Debug, PartialEq)]
pub struct DerivedUnit {
    symbol: String,
    name: String,
    //prefix: Option<Prefix>,
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
            //prefix,
            def_number,
            def_uncertainty,
            def_factors: def_factors.to_vec(),
        }
    }
}

impl Unit for DerivedUnit {
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


#[derive(Clone, Debug, PartialEq)]
pub struct CompoundUnit {
    pub factors: Vec<LinearFactor>,
}

impl CompoundUnit {
    pub fn new(factors: &[LinearFactor]) -> Self {
        Self {factors: factors.to_vec()}
    }
}

impl Unit for CompoundUnit {
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
