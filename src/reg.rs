// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, sync::Arc};

use crate::{
    dimensions::Dimensions,
    fraction::Frac,
    prefix::Prefix,
    scinum::SciNum,
    unit::{LinearFactor, LinearUnit, LinearUnitType, Unit},
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

    fn add_unitless(&mut self) {
        self.units.insert(Unit128::UNITLESS, Unit::unitless());
    }

    fn new_base(
        &mut self,
        id: Unit128,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) -> Unit {
        let inner_unit = LinearUnit {
            utype: LinearUnitType::Base,
            dimensions,
            symbol: Some(symbol),
            name: Some(name),
            prefix,
            number: SciNum::ONE,
            factors: vec![],
        };
        Unit {
            id,
            inner: Arc::new(inner_unit),
        }
    }

    pub fn add_base(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol, name.clone(), prefix);
        self.string_map.insert(name, unit.clone());
        self.units.insert(id, unit);
    }

    pub fn add_base_with_alt_spellings(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        alt_spellings: Vec<String>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.string_map.insert(name, unit.clone());
        for spelling in alt_spellings {
            let alt_unit = self.new_base(id, dimensions, symbol.clone(), spelling.clone(), prefix);
            self.string_map.insert(spelling, alt_unit);
        }
        // Getting the unit by ID should return the canonical form
        self.units.insert(id, unit);
    }

    pub fn add_base_with_aliases(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        aliases: Vec<String>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol, name.clone(), prefix);
        self.string_map.insert(name, unit.clone());
        for alias in aliases {
            self.string_map.insert(alias, unit.clone());
        }
        self.units.insert(id, unit);
    }

    fn new_derived(
        &mut self,
        id: Unit128,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: Vec<(Unit, Frac)>,
    ) -> Unit {
        let inner_unit = LinearUnit {
            utype: LinearUnitType::Derived,
            dimensions: id.dimensions(),
            symbol: Some(symbol),
            name: Some(name),
            prefix,
            number: proportionality_factor,
            factors: unit_factors
                .into_iter()
                .map(|x| LinearFactor {
                    unit: x.0.inner,
                    exponent: x.1,
                })
                .collect(),
        };
        Unit {
            id,
            inner: Arc::new(inner_unit),
        }
    }

    fn calculate_derived_id(
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: &[(Unit, Frac)],
    ) -> Unit128 {
        let cmpd = Unit128::new_compound(unit_factors.iter().map(|x| (x.0.id, x.1)).collect());
        if let Some(p) = prefix {
            if p.is_binary()
                && proportionality_factor == SciNum::ONE
                && cmpd.factor() == SciNum::ONE
            {
                // Encode binary prefix
                todo!()
            } else {
                Unit128::new(
                    p.value() * proportionality_factor * cmpd.factor(),
                    cmpd.dimensions(),
                    0x0D,
                )
            }
        } else {
            Unit128::new(
                proportionality_factor * cmpd.factor(),
                cmpd.dimensions(),
                0x0D,
            )
        }
    }

    /// Adds a derived unit to the registry.
    ///
    /// The symbol and name of the unit should include the prefix, if there is one.
    ///
    /// On the other hand, the proportionality factor should not include the value of the prefix.
    /// The overall value of the unit is then (prefix * number * factors[0] * … * factors[-1])
    pub fn add_derived(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: Vec<(Unit, Frac)>,
    ) {
        let id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = self.new_derived(
            id,
            symbol,
            name.clone(),
            prefix,
            proportionality_factor,
            unit_factors,
        );
        self.string_map.insert(name, unit.clone());
        self.units.insert(id, unit);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_with_alt_spellings(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: Vec<(Unit, Frac)>,
        alt_spellings: Vec<String>,
    ) {
        let id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = self.new_derived(
            id,
            symbol.clone(),
            name.clone(),
            prefix,
            proportionality_factor,
            unit_factors.clone(),
        );
        self.string_map.insert(name, unit.clone());
        for spelling in alt_spellings {
            let alt_unit = self.new_derived(
                id,
                symbol.clone(),
                spelling.clone(),
                prefix,
                proportionality_factor,
                unit_factors.clone(),
            );
            self.string_map.insert(spelling, alt_unit);
        }
        // Getting the unit by ID should return the canonical form
        self.units.insert(id, unit);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_with_aliases(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: Vec<(Unit, Frac)>,
        aliases: Vec<String>,
    ) {
        let id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = self.new_derived(
            id,
            symbol.clone(),
            name.clone(),
            prefix,
            proportionality_factor,
            unit_factors.clone(),
        );
        self.string_map.insert(name, unit.clone());
        for alias in aliases {
            self.string_map.insert(alias, unit.clone());
        }
        self.units.insert(id, unit);
    }

    pub fn unitless(&self) -> Unit {
        self.units.get(&Unit128::UNITLESS).unwrap().clone()
    }

    pub fn get_by_name(&self, name: &str) -> Option<Unit> {
        self.string_map.get(name).cloned()
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
        // Don't need to add unitless as done for all new regs

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
