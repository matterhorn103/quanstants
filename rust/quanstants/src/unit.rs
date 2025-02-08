use std::ops::{Div, Mul};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::LazyLock;
// use std::sync::atomic::AtomicU16;

use crate::dimensions::Dimensions;
use crate::quantity::{Linear, Quantity};


#[derive(Default)]
pub struct UnitRegistry {
    id_counter: u16,
    registry: HashMap<u16, Unit>,
    index_by_symbol: HashMap<String, u16>,
    index_by_name: HashMap<String, u16>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            registry: HashMap::new(),
            index_by_symbol: HashMap::new(),
            index_by_name: HashMap::new(),
        }
    }

    pub fn get_by_id(&self, id: u16) -> Option<&Unit> {
        self.registry.get(&id)
    }
    
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&Unit> {
        let id = self.index_by_symbol.get(symbol)?;
        self.get_by_id(*id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Unit> {
        let id = self.index_by_name.get(name)?;
        self.get_by_id(*id)
    }

    pub fn add_unit(
        &mut self,
        id: Option<u16>,
        unit_type: UnitType,
        symbol: String,
        name: String,
        alt_names: Vec<String>,
        dimensions: Dimensions,
    ) -> &Unit {
        let new_id = match id {
            Some(suggestion) => if self.registry.contains_key(&suggestion) {
                panic!()
            } else {suggestion},
            None => {
                while self.registry.contains_key(&self.id_counter) {
                    self.id_counter += 1;
                };
                self.id_counter
            },
        };
        let new_unit = Unit::new(
            new_id,
            unit_type,
            symbol.clone(),
            name.clone(),
            dimensions,
        );
        self.registry.insert(new_id, new_unit);
        self.index_by_symbol.insert(symbol, new_id);
        self.index_by_name.insert(name, new_id);
        for alt_name in alt_names {
            self.index_by_name.insert(alt_name, new_id);
        }
        self.registry.get(&new_id).unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Base,
    Simple,
    Unitless,
    Derived,
    Compound,
}

#[derive(Debug, PartialEq)]
pub struct Unit {
    id: u16,
    unit_type: UnitType,
    symbol: String,
    name: String,
    dimensions: Dimensions,
}

impl Unit {
    fn new(
        id: u16,
        unit_type: UnitType,
        symbol: String,
        name: String,
        dimensions: Dimensions,
    ) -> Self {
        Self {
            id,
            unit_type,
            symbol,
            name,
            dimensions,
        }
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn preceding_space(&self) -> bool {
        true
    }

    pub fn pow(&self, exp: i32) -> &Unit {
        self
    }
}

impl Linear for Unit {
    fn value(&self) -> Quantity {
        match self.unit_type {
            UnitType::Base => Quantity::new(1.0, self, 0.0),
            UnitType::Simple => todo!(),
            UnitType::Unitless => todo!(),
            UnitType::Derived => todo!(),
            UnitType::Compound => todo!(),
        }
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}


