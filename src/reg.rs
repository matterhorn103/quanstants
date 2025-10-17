use std::{collections::HashMap, sync::Arc};

use crate::{
    dimensions::Dimensions, exponum::ExponentialNumber, number::Number, prefix::Prefix, unit::{LinearUnit, LinearUnitType, Unit}, unit128::Unit128,
};

#[derive(Debug)]
pub struct UnitRegistry {
    units: HashMap<Unit128, Unit>,
    string_map: HashMap<String, Unit128>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
            string_map: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        symbol: String,
        name: String,
        //number: Decimal,
        //factors: Option<Arc<Vec<LinearFactor>>>,
        //uncertainty: Decimal,
    ) {
        todo!()
        //let id = unit.id;
        //let name = unit.name();
        //let unit = LinearUnit {
        //    is_base: false,
        //    dimensions:
        //    prefix: None,
        //    ..
        //};
        //self.units.insert(id, unit);
        //self.string_map.insert(name, id);
    }

    pub fn add_base(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) {
        let id = Unit128::new(ExponentialNumber::new(1, 1, 10, 0), dimensions, 0x00);
        let inner_unit = LinearUnit {
            utype: LinearUnitType::Base,
            dimensions,
            symbol: Some(symbol),
            name: Some(name.clone()),
            prefix,
            number: Number::ONE,
            factors: vec![],
        };
        let unit = Unit {
            id,
            inner: Arc::new(inner_unit),
        };
        self.units.insert(id, unit);
        self.string_map.insert(name, id);
    }

    pub fn add_base_with_aliases(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        aliases: Vec<String>,
    ) {
        let id = Unit128::new(ExponentialNumber::new(1, 1, 10, 0), dimensions, 0x00);
        self.add_base(dimensions, symbol, name, prefix);
        for alias in aliases {
            self.string_map.insert(alias, id);
        }
    }

    pub fn get_by_name(&self, name: &str) -> Unit {
        let id = self.string_map.get(name).unwrap();
        self.get_by_id(id)
    }

    pub fn get_by_id(&self, id: &Unit128) -> Unit {
        self.units.get(id).unwrap().clone()
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
        self.add_base(
            Dimensions::new(1, 0, 0, 0, 0, 0, 0),
            String::from("s"),
            String::from("second"),
            None,
        );
        self.add_base_with_aliases(
            Dimensions::new(0, 1, 0, 0, 0, 0, 0),
            String::from("m"),
            String::from("metre"),
            None,
            vec![String::from("meter")],
        );
        self.add_base(
            Dimensions::new(0, 0, 1, 0, 0, 0, 0),
            String::from("kg"),
            String::from("kilogram"),
            Some(Prefix::kilo),
        );
        self.add_base_with_aliases(
            Dimensions::new(0, 0, 0, 1, 0, 0, 0),
            String::from("A"),
            String::from("ampere"),
            None,
            vec![String::from("amp")],
        );
        self.add_base(
            Dimensions::new(0, 0, 0, 0, 1, 0, 0),
            String::from("K"),
            String::from("kelvin"),
            None,
        );
        self.add_base(
            Dimensions::new(0, 0, 0, 0, 0, 1, 0),
            String::from("mol"),
            String::from("mole"),
            None,
        );
        self.add_base(
            Dimensions::new(0, 0, 0, 0, 0, 0, 1),
            String::from("cd"),
            String::from("candela"),
            None,
        );
    }
}
