use std::collections::HashMap;

use crate::{dimensions::Dimensions, id::{TypedUnitId, Unit128}, unit::{BaseUnit, Unit}};

#[derive(Debug)]
pub struct UnitRegistry {
    string_map: HashMap<String, TypedUnitId>,
    id_map: HashMap<Unit128, TypedUnitId>,
    base_units: HashMap<Unit128, BaseUnit>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            base_units: HashMap::new(),
            string_map: HashMap::new(),
            id_map: HashMap::new(),
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
        self.add(id, m, vec![String::from("meter")]);
    }

    pub fn add(&mut self, id: Unit128, unit: BaseUnit, aliases: Vec<String>) {
        let typed_id = TypedUnitId::Base(id);
        let name = unit.name();
        self.base_units.insert(id, unit);
        self.id_map.insert(id, typed_id);
        self.string_map.insert(name, typed_id);
        for alias in aliases {
            self.string_map.insert(alias, typed_id);
        }
    }

    pub fn get_by_name(&self, name: &str) -> BaseUnit {
        let typed_id = self.string_map.get(name).unwrap();
        match typed_id {
            TypedUnitId::Base(unit_id) => self.base_units.get(unit_id).unwrap().clone(),
            TypedUnitId::Unitless(_unit_id) => todo!(),
            TypedUnitId::Derived(_unit_id) => todo!(),
        }
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
