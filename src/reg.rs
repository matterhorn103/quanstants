// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, sync::Arc};

use crate::{
    dimensions::Dimensions,
    prefix::Prefix,
    scinum::SciNum,
    unit::{LinearUnit, LinearUnitType, Unit},
    unit128::Unit128,
};

#[derive(Debug)]
pub struct UnitRegistry {
    units: HashMap<Unit128, Unit>,
    string_map: HashMap<String, Unit>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            units: HashMap::new(),
            string_map: HashMap::new(),
        };
        reg.add_unitless();
        reg
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

    fn add_unitless(&mut self) {
        self.units.insert(Unit128::UNITLESS, Unit::unitless());
    }

    pub fn add_base(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let inner_unit = LinearUnit {
            utype: LinearUnitType::Base,
            dimensions,
            symbol: Some(symbol),
            name: Some(name.clone()),
            prefix,
            number: SciNum::ONE,
            factors: vec![],
        };
        let unit = Unit {
            id,
            inner: Arc::new(inner_unit),
        };
        self.units.insert(id, unit.clone());
        self.string_map.insert(name, unit);
    }

    pub fn add_base_with_alt_spellings(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        alt_spellings: Vec<String>,
    ) {
        self.add_base(dimensions, symbol.clone(), name, prefix);
        for spelling in alt_spellings {
            self.add_base(dimensions, symbol.clone(), spelling, prefix);
        }
    }

    pub(crate) fn add_base_with_aliases(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        aliases: Vec<String>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        self.add_base(dimensions, symbol.clone(), name, prefix);
        for alias in aliases {
            self.string_map.insert(alias, self.get_by_id(&id).clone());
        }
    }

    pub fn get_by_name(&self, name: &str) -> Unit {
        self.string_map.get(name).unwrap().clone()
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
        self.add_base_with_alt_spellings(
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
