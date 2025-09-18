use std::collections::HashMap;

use crate::{dimensions::Dimensions, id::Unit128, unit::{BaseUnit, Unit}};

#[derive(Debug)]
pub struct UnitRegistry {
    units: HashMap<Unit128, Unit>,
    unit_names: HashMap<String, Unit128>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
            unit_names: HashMap::new(),
        }
    }

    pub fn add(&mut self, unit: Unit) {
        let id = unit.id();
        let name = unit.name();
        self.units.insert(id, unit);
        self.unit_names.insert(name, id);
    }

    pub fn add_with_aliases(&mut self, unit: Unit, aliases: Vec<String>) {
        let id = unit.id();
        self.add(unit);
        for alias in aliases {
            self.unit_names.insert(alias, id);
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<Unit> {
        if let Some(id) = self.unit_names.get(name) {
            self.get_by_id(id)
        } else {
            None
        }
    }

    pub fn get_by_id(&self, id: &Unit128) -> Option<Unit> {
        self.units.get(id).cloned()
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        let mut reg = Self::new();
        reg.add_si();
        reg
    }
}

impl UnitRegistry {
    fn add_si(&mut self) {
        self.add(
            BaseUnit::new(
                String::from("s"),
                String::from("second"),
                Dimensions::new(1, 0, 0, 0, 0, 0, 0),
            ).into()
        );
        self.add_with_aliases(
            BaseUnit::new(
                String::from("m"),
                String::from("metre"),
                Dimensions::new(0, 1, 0, 0, 0, 0, 0),
            ).into(),
            vec![String::from("meter")],
        );
        self.add(
            BaseUnit::new(
                String::from("kg"),
                String::from("kilogram"),
                Dimensions::new(0, 0, 1, 0, 0, 0, 0),
            ).into()
        );
    }
}
