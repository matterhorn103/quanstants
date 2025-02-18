use std::fmt::Debug;
use std::ops::{Div, Mul};

use crate::dimensions::Dimensions;
use crate::quantity::Quantity;

pub trait Unit: Clone + Debug + PartialEq {
    fn symbol(&self) -> &str;

    fn name(&self) -> &str;

    fn preceding_space(&self) -> bool;

    fn dimensions(&self) -> Dimensions;
}

#[derive(Clone, Debug, PartialEq)]
pub struct BaseUnit {
    symbol: String,
    name: String,
    dimensions: Dimensions,
}

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
        }
    }
}

impl Unit for BaseUnit {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnitlessUnit;

impl Unit for UnitlessUnit {
    fn symbol(&self) -> &str {
        "(unitless)"
    }

    fn name(&self) -> &str {
        "unitless"
    }

    fn preceding_space(&self) -> bool {
        true
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions::new(0,0,0,0,0,0,0)
    }
}

#[derive(Clone, Debug)]
pub struct DerivedUnit<U: Unit> {
    symbol: String,
    name: String,
    value: Quantity<U>,
}

impl<U: Unit> DerivedUnit<U> {
    fn new(
        symbol: String,
        name: String,
        value: Quantity<U>,
    ) -> Self {
        Self {
            symbol,
            name,
            value,
        }
    }
}

pub struct Factor {
    unit: dyn Unit,
    exponent: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompoundUnit {
    factors: Vec<Factor<dyn Unit>>
}

impl Unit for CompoundUnit {
    fn symbol(&self) -> &str {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn preceding_space(&self) -> bool {
        todo!()
    }

    fn dimensions(&self) -> Dimensions {
        todo!()
    }
}
