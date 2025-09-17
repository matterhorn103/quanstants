use std::collections::HashMap;

use crate::{dimensions::Dimensions, id::Unit128, unit::{BaseUnit, Unit}};

#[derive(Debug)]
pub struct UnitRegistry {
    units: HashMap<Unit128, Unit>,
    string_map: HashMap<String, Unit128>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            units: HashMap::new(),
            string_map: HashMap::new(),
        };
        reg.add_metre();
        reg
    }

    fn add_metre(&mut self) {
        let m = BaseUnit::new(
            String::from("m"),
            String::from("metre"),
            Dimensions::new(0, 1, 0, 0, 0, 0, 0),
        );
        let id = Unit128::new(0, 0x110000);
        self.add(id, m.into(), vec![String::from("meter")]);
    }

    pub fn add(&mut self, id: Unit128, unit: Unit, aliases: Vec<String>) {
        let name = unit.name();
        self.units.insert(id, unit);
        self.string_map.insert(name, id);
        for alias in aliases {
            self.string_map.insert(alias, id);
        }
    }

    pub fn get_by_name(&self, name: &str) -> Unit {
        let id = self.string_map.get(name).unwrap();
        self.units.get(id).unwrap().clone()
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
