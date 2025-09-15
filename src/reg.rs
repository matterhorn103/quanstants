use std::collections::HashMap;

use pyo3::pyclass;

use crate::{id::{TypedUnitId, UnitId}, unit::BaseUnit};

#[pyclass]
#[derive(Clone, Debug)]
pub struct UnitRegistry {
    string_map: HashMap<String, TypedUnitId>,
    id_map: HashMap<UnitId, TypedUnitId>,
    base_units: HashMap<UnitId, BaseUnit>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            base_units: HashMap::new(),
            string_map: HashMap::new(),
            id_map: HashMap::new(),
        }
    }

    pub fn add_unit(&mut self, id: UnitId, unit: BaseUnit, aliases: Vec<String>) {
        let typed_id = TypedUnitId::Base(id);
        self.base_units.insert(id, unit);
        self.id_map.insert(id, typed_id);
        for alias in aliases {
            self.string_map.insert(alias, typed_id);
        }
    }

    pub fn get_unit(&self, name: &str) -> BaseUnit {
        let typed_id = self.string_map.get(name).unwrap();
        match typed_id {
            TypedUnitId::Base(unit_id) => self.base_units.get(unit_id).unwrap().clone(),
            TypedUnitId::Unitless(unit_id) => todo!(),
            TypedUnitId::Derived(unit_id) => todo!(),
        }
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
