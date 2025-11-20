// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::{
    dimensions::Dimensions,
    error::QuanstantsError,
    fraction::Frac,
    prefix::Prefix,
    scinum::SciNum,
    serde::UnitDef,
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
        reg.add_si_base();
        reg
    }

    /// Adds the unit to string_map under the normalized lowercase form of the provided name.
    #[inline]
    fn insert_under_string(&mut self, name: String, unit: Unit) {
        let normalized = name.to_lowercase();
        self.string_map.insert(normalized, unit);
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
        self.insert_under_string(name, unit.clone());
        self.units.insert(id, unit);
    }

    pub fn add_base_with_alt_names(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        alt_names: Vec<String>,
    ) {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        for n in alt_names {
            let alt_unit = self.new_base(id, dimensions, symbol.clone(), n.clone(), prefix);
            self.insert_under_string(n, alt_unit);
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
        self.insert_under_string(name, unit.clone());
        for alias in aliases {
            self.insert_under_string(alias, unit.clone());
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
        self.insert_under_string(name, unit.clone());
        self.units.insert(id, unit);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_with_alt_names(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciNum,
        unit_factors: Vec<(Unit, Frac)>,
        alt_names: Vec<String>,
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
        self.insert_under_string(name, unit.clone());
        for n in alt_names {
            let alt_unit = self.new_derived(
                id,
                symbol.clone(),
                n.clone(),
                prefix,
                proportionality_factor,
                unit_factors.clone(),
            );
            self.insert_under_string(n, alt_unit);
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
        self.insert_under_string(name, unit.clone());
        for alias in aliases {
            self.insert_under_string(alias, unit.clone());
        }
        self.units.insert(id, unit);
    }

    fn add_from_def(&mut self, def: UnitDef) -> Result<(), QuanstantsError> {
        if def.base {
            if !def.alt_names.is_empty() {
                self.add_base_with_alt_names(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                    def.alt_names,
                );
            } else if !def.aliases.is_empty() {
                self.add_base_with_aliases(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                    def.aliases,
                );
            } else {
                self.add_base(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                );
            }
        } else {
            let value = def
                .value
                .ok_or(QuanstantsError::Definition("value".to_string()))?;
            let proportionality_factor = value.number.with_uncertainty(value.uncertainty);
            // TODO this definitely shouldn't just panic if the unit isn't found
            let unit_factors: Vec<(Unit, Frac)> = value
                .unit
                .into_iter()
                .map(|f| (self.get_by_name(&f.0).unwrap(), f.1))
                .collect();
            if !def.alt_names.is_empty() {
                self.add_derived_with_alt_names(
                    def.id,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.alt_names,
                );
            } else if !def.aliases.is_empty() {
                self.add_derived_with_aliases(
                    def.id,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.aliases,
                );
            } else {
                self.add_derived(
                    def.id,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name
                        .ok_or(QuanstantsError::Definition("name".to_string()))?,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                );
            }
        }
        Ok(())
    }

    #[inline]
    pub fn unitless(&self) -> Unit {
        self.units.get(&Unit128::UNITLESS).unwrap().clone()
    }

    #[inline]
    pub fn get_by_name(&self, name: &str) -> Option<Unit> {
        self.string_map.get(&name.to_lowercase()).cloned()
    }

    #[inline]
    pub fn get_by_id(&self, id: Unit128) -> Option<Unit> {
        self.units.get(&id).cloned()
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        let mut reg = Self::new();
        reg.add_si_derived();
        reg
    }
}

impl UnitRegistry {
    fn add_unitless(&mut self) {
        self.units.insert(Unit128::UNITLESS, Unit::unitless());
    }

    fn add_si_base(&mut self) {
        self.add_base(
            Dimensions::new(1, 0, 0, 0, 0, 0, 0),
            String::from("s"),
            String::from("second"),
            None,
        );
        self.add_base_with_alt_names(
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

    fn add_si_derived(&mut self) {
        // This method should only be used if the SI base units have definitely already been added,
        // as it will panic otherwise
        // This should always be the case though as they are added in new()
        let second = self.units.get(&Unit128::SECOND).unwrap().clone();
        let metre = self.units.get(&Unit128::METRE).unwrap().clone();
        let kilogram = self.units.get(&Unit128::KILOGRAM).unwrap().clone();
        let ampere = self.units.get(&Unit128::AMPERE).unwrap().clone();
        let kelvin = self.units.get(&Unit128::KELVIN).unwrap().clone();
        let mole = self.units.get(&Unit128::MOLE).unwrap().clone();
        let candela = self.units.get(&Unit128::SECOND).unwrap().clone();

        // rad (radian) = 1
        self.add_derived(
            Some(Unit128::RADIAN),
            String::from("rad"),
            String::from("radian"),
            None,
            SciNum::ONE,
            vec![(self.unitless(), Frac::ONE)],
        );

        // sr (steradian) = 1
        self.add_derived(
            Some(Unit128::STERADIAN),
            String::from("sr"),
            String::from("steradian"),
            None,
            SciNum::ONE,
            vec![(self.unitless(), Frac::ONE)],
        );

        // Hz (hertz) = s⁻¹
        self.add_derived(
            Some(Unit128::HERTZ),
            String::from("Hz"),
            String::from("hertz"),
            None,
            SciNum::ONE,
            vec![(second.clone(), Frac::ONE)],
        );

        // N (newton) = kg⋅m⋅s⁻²
        self.add_derived(
            Some(Unit128::NEWTON),
            String::from("N"),
            String::from("newton"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::ONE),
                (second.clone(), Frac::from(-2)),
            ],
        );

        // Pa (pascal) = N⋅m⁻² = kg⋅m⁻¹⋅s⁻²
        self.add_derived(
            Some(Unit128::PASCAL),
            String::from("Pa"),
            String::from("pascal"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(-1)),
                (second.clone(), Frac::from(-2)),
            ],
        );

        // J (joule) = N⋅m = kg⋅m²⋅s⁻²
        self.add_derived(
            Some(Unit128::JOULE),
            String::from("J"),
            String::from("joule"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-2)),
            ],
        );

        // W (watt) = J⋅s⁻¹ = kg⋅m²⋅s⁻³
        self.add_derived(
            Some(Unit128::WATT),
            String::from("W"),
            String::from("watt"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-3)),
            ],
        );

        // C (coulomb) = A⋅s
        self.add_derived(
            Some(Unit128::COULOMB),
            String::from("C"),
            String::from("coulomb"),
            None,
            SciNum::ONE,
            vec![(ampere.clone(), Frac::ONE), (second.clone(), Frac::ONE)],
        );

        // V (volt) = W⋅A⁻¹ = kg⋅m²⋅s⁻³⋅A⁻¹
        self.add_derived(
            Some(Unit128::VOLT),
            String::from("V"),
            String::from("volt"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-3)),
                (ampere.clone(), Frac::from(-1)),
            ],
        );

        // F (farad) = C⋅V⁻¹ = kg⁻¹⋅m⁻²⋅s⁴⋅A²
        self.add_derived(
            Some(Unit128::FARAD),
            String::from("F"),
            String::from("farad"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::from(-1)),
                (metre.clone(), Frac::from(-2)),
                (second.clone(), Frac::from(4)),
                (ampere.clone(), Frac::from(2)),
            ],
        );

        // Ω (ohm) = V⋅A⁻¹ = kg⋅m²⋅s⁻³⋅A⁻²
        self.add_derived(
            Some(Unit128::OHM),
            String::from("Ω"),
            String::from("ohm"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-3)),
                (ampere.clone(), Frac::from(-2)),
            ],
        );

        // S (siemens) = Ω⁻¹ = kg⁻¹⋅m⁻²⋅s³⋅A²
        self.add_derived(
            Some(Unit128::SIEMENS),
            String::from("S"),
            String::from("siemens"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::from(-1)),
                (metre.clone(), Frac::from(-2)),
                (second.clone(), Frac::from(3)),
                (ampere.clone(), Frac::from(2)),
            ],
        );

        // Wb (weber) = V⋅s = kg⋅m²⋅s⁻²⋅A⁻¹
        self.add_derived(
            Some(Unit128::WEBER),
            String::from("Wb"),
            String::from("weber"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-2)),
                (ampere.clone(), Frac::from(-1)),
            ],
        );

        // T (tesla) = Wb⋅m⁻² = kg⋅s⁻²⋅A⁻¹
        self.add_derived(
            Some(Unit128::TESLA),
            String::from("T"),
            String::from("tesla"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (second.clone(), Frac::from(-2)),
                (ampere.clone(), Frac::from(-1)),
            ],
        );

        // H (henry) = Wb⋅A⁻¹ = kg⋅m²⋅s⁻²⋅A⁻²
        self.add_derived(
            Some(Unit128::HENRY),
            String::from("H"),
            String::from("henry"),
            None,
            SciNum::ONE,
            vec![
                (kilogram.clone(), Frac::ONE),
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-2)),
                (ampere.clone(), Frac::from(-2)),
            ],
        );

        // The absolute magnitude of the degree Celsius, equal to the kelvin
        self.add_derived(
            Some(Unit128::CELSIUS_DEGREE),
            String::from("°C"),
            String::from("Celsius degree"),
            None,
            SciNum::ONE,
            vec![(kelvin.clone(), Frac::ONE)],
        );

        // lm (lumen) = cd⋅sr = cd
        self.add_derived(
            Some(Unit128::LUMEN),
            String::from("lm"),
            String::from("lumen"),
            None,
            SciNum::ONE,
            vec![(candela.clone(), Frac::ONE)],
        );

        // lx (lux) = lm⋅m⁻² = cd⋅m⁻²
        self.add_derived(
            Some(Unit128::LUX),
            String::from("lx"),
            String::from("lux"),
            None,
            SciNum::ONE,
            vec![
                (candela.clone(), Frac::ONE),
                (metre.clone(), Frac::from(-2)),
            ],
        );

        // Bq (becquerel) = s⁻¹
        self.add_derived(
            Some(Unit128::BECQUEREL),
            String::from("Bq"),
            String::from("becquerel"),
            None,
            SciNum::ONE,
            vec![(second.clone(), Frac::from(-1))],
        );

        // Gy (gray) = J⋅kg⁻¹ = m²⋅s⁻²
        self.add_derived(
            Some(Unit128::GRAY),
            String::from("Gy"),
            String::from("gray"),
            None,
            SciNum::ONE,
            vec![
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-2)),
            ],
        );

        // Sv (sievert) = J⋅kg⁻¹ = m²⋅s⁻²
        self.add_derived(
            Some(Unit128::SIEVERT),
            String::from("Sv"),
            String::from("sievert"),
            None,
            SciNum::ONE,
            vec![
                (metre.clone(), Frac::from(2)),
                (second.clone(), Frac::from(-2)),
            ],
        );

        // kat (katal) = mol⋅s⁻¹
        self.add_derived(
            Some(Unit128::KATAL),
            String::from("kat"),
            String::from("katal"),
            None,
            SciNum::ONE,
            vec![(mole.clone(), Frac::ONE), (second.clone(), Frac::from(-1))],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn si_derived_units() {
        let reg = UnitRegistry::default();

        let s = reg.get_by_name("second").unwrap();
        let m = reg.get_by_name("metre").unwrap();
        let kg = reg.get_by_name("kilogram").unwrap();
        let A = reg.get_by_name("ampere").unwrap();
        let K = reg.get_by_name("kelvin").unwrap();
        let mol = reg.get_by_name("mole").unwrap();
        let cd = reg.get_by_name("candela").unwrap();

        assert_eq!(
            reg.get_by_name("newton").unwrap(),
            kg.clone() * m.clone() * s.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("pascal").unwrap(),
            kg.clone() * m.clone().pow(-1) * s.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("joule").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("watt").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-3)
        );
        assert_eq!(reg.get_by_name("coulomb").unwrap(), A.clone() * s.clone());
        assert_eq!(
            reg.get_by_name("volt").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-3) * A.clone().pow(-1)
        );
        assert_eq!(
            reg.get_by_name("farad").unwrap(),
            kg.clone().pow(-1) * m.clone().pow(-2) * s.clone().pow(4) * A.clone().pow(2)
        );
        assert_eq!(
            reg.get_by_name("ohm").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-3) * A.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("siemens").unwrap(),
            kg.clone().pow(-1) * m.clone().pow(-2) * s.clone().pow(3) * A.clone().pow(2)
        );
        assert_eq!(
            reg.get_by_name("weber").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-2) * A.clone().pow(-1)
        );
        assert_eq!(
            reg.get_by_name("tesla").unwrap(),
            kg.clone() * s.clone().pow(-2) * A.clone().pow(-1)
        );
        assert_eq!(
            reg.get_by_name("henry").unwrap(),
            kg.clone() * m.clone().pow(2) * s.clone().pow(-2) * A.clone().pow(-2)
        );
        assert_eq!(reg.get_by_name("Celsius degree").unwrap(), K.clone());
        assert_eq!(reg.get_by_name("lumen").unwrap(), cd.clone());
        assert_eq!(
            reg.get_by_name("lux").unwrap(),
            cd.clone() * m.clone().pow(-2)
        );
        assert_eq!(reg.get_by_name("becquerel").unwrap(), s.clone().pow(-1));
        assert_eq!(
            reg.get_by_name("gray").unwrap(),
            m.clone().pow(2) * s.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("sievert").unwrap(),
            m.clone().pow(2) * s.clone().pow(-2)
        );
        assert_eq!(
            reg.get_by_name("katal").unwrap(),
            mol.clone() * s.clone().pow(-1)
        );
    }
}
