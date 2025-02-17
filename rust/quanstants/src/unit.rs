use std::fmt::Debug;
use std::ops::{Div, Mul};
use std::collections::HashMap;

use crate::dimensions::Dimensions;
use crate::quantity::{Linear, Quantity};

pub trait Unit: Clone + Debug {
    fn symbol(&self) -> &str;

    fn name(&self) -> &str;

    fn preceding_space(&self) -> bool;

    fn dimensions(&self) -> Dimensions;
}

#[derive(Clone, Debug)]
pub struct BaseUnit {
    symbol: String,
    name: String,
    dimensions: Dimensions,
}

impl BaseUnit {
    fn new(
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

#[derive(Clone, Debug)]
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
pub struct DerivedUnit {
    symbol: String,
    name: String,
    value: Quantity,
}

#[derive(Clone, Debug)]
pub struct CompoundUnit {

}
