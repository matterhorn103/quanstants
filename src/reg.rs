// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, sync::Arc};

use crate::{
    defs::{units::UnitModule, DefFile, UnitDef},
    dimensions::Dimensions,
    error::QuanstantsError,
    fraction::Frac,
    prefix::Prefix,
    scinum::SciNum,
    unit::{LinearFactor, LinearUnit, LinearUnitType, Unit},
    unit128::Unit128,
};

#[derive(Debug)]
pub struct UnitRegistry {
    pub(crate) units: HashMap<Unit128, Unit>,
    pub(crate) string_map: HashMap<String, Unit>,
    pub(crate) symbol_map: HashMap<String, Unit>,
    pub(crate) sources: HashMap<Unit128, Option<String>>,
}

impl UnitRegistry {
    /// Creates a new `UnitRegistry` with minimal pre-population (just the SI base units).
    #[inline]
    pub fn new_minimal() -> Self {
        let mut reg = Self {
            units: HashMap::new(),
            string_map: HashMap::new(),
            symbol_map: HashMap::new(),
            sources: HashMap::new(),
        };
        reg.add_unitless();
        reg.add_si_base();
        reg
    }

    /// Creates a new `UnitRegistry` pre-populated with:
    /// - the SI base units
    /// - the SI derived units
    pub fn new() -> Self {
        let mut reg = Self::new_minimal();
        reg.load_module(UnitModule::Si)
            .expect("Internal `si.toml` file should be correct");
        reg
    }
}

impl Default for UnitRegistry {
    /// Creates a new `UnitRegistry` pre-populated with:
    /// - the SI base units
    /// - the SI derived units
    /// - the non-SI units officially approved for use with the SI
    /// - common prefixed units
    fn default() -> Self {
        let mut reg = Self::new();
        reg.load_module(UnitModule::SiCompatible)
            .expect("Internal `si_compatible.toml` file should be correct");
        reg
    }
}

impl UnitRegistry {
    /// Adds the unit to string_map under the normalized lowercase form of the provided name.
    #[inline]
    fn insert_under_string(&mut self, name: String, unit: Unit) {
        let normalized = name.to_lowercase();
        self.string_map.insert(normalized, unit);
    }

    /// Adds the unit to symbol_map under the provided symbol if not already present.
    #[inline]
    fn insert_under_symbol_checked(&mut self, symbol: String, unit: Unit) {
        self.symbol_map.entry(symbol).or_insert(unit);
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
    ) -> Unit128 {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    pub fn add_base_with_alt_names(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        alt_names: Vec<String>,
    ) -> Unit128 {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        for n in alt_names {
            let alt_unit = self.new_base(id, dimensions, symbol.clone(), n.clone(), prefix);
            self.insert_under_string(n, alt_unit);
        }
        // Getting the unit by symbol or ID should return the canonical form
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    pub fn add_base_with_aliases(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        aliases: Vec<String>,
    ) -> Unit128 {
        let id = Unit128::new(SciNum::ONE, dimensions, 0x00);
        let unit = self.new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        for alias in aliases {
            self.insert_under_string(alias, unit.clone());
        }
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
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
        // Build the compound unit representing the unit terms only, ignoring any prefix or
        // numerical factor
        // This is just the easiest way to obtain the proportionality factor that results from
        // expressing the unit terms in SI base units, which we need
        let cmpd = Unit128::new_compound(unit_factors.iter().map(|x| (x.0.id, x.1)).collect());
        if let Some(p) = prefix {
            if p.is_binary()
                && proportionality_factor == SciNum::ONE
                && cmpd.factor() == SciNum::ONE
            {
                // Encode binary prefix using binary scheme
                todo!()
            } else {
                // Roll all numerical factors, including the prefix, into just one
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
    ) -> Unit128 {
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
            unit_factors,
        );
        self.insert_under_string(name, unit.clone());
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Adds a derived unit to the registry under multiple alternative names.
    ///
    /// The `name` and each `alt_name` then all refer to separate `Unit`s with identical values.
    ///
    /// Localized and translated names are equally valid spellings, so it is important that
    /// the unit returned from a lookup has the name expected by the user and not the "canonical"
    /// (English) one.
    /// This includes distinguishing between "metre" and "meter".
    ///
    /// Calling `Unit.name()` on the alternative units then returns a different name in each case.
    /// The symbol and value of each alternative unit is the same.
    /// The ID of each alternative unit is also identical, but lookup in the registry using the ID
    /// will always return the canonical unit.
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
    ) -> Unit128 {
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
        // Getting the unit by symbol or ID should return the canonical form
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Adds a derived unit to the registry along with aliases that point to the same unit.
    ///
    /// Unlike `alt_names`, `aliases` refer to the exact same `Unit`, they just allow the unit
    /// to be found using several different names.
    ///
    /// For example:
    /// - the "percent" unit can also be found under "per cent"
    /// - the "Dalton" unit can also be found under "unified atomic mass unit"
    ///
    /// Calling `Unit.name()` on the units always returns the canonical name.
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
    ) -> Unit128 {
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
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Creates a `Unit` (base or derived, as appropriate) from the definition and inserts it into
    /// the registry.
    ///
    /// Returns an error if the definition is invalid, either because the definition is missing
    /// fields necessary for the type of unit, or because the units used in the definition cannot
    /// be found in the registry.
    pub(crate) fn add_from_def(&mut self, def: UnitDef) -> Result<Unit128, QuanstantsError> {
        let id = if def.base {
            if !def.alt_names.is_empty() {
                self.add_base_with_alt_names(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name,
                    def.prefix,
                    def.alt_names,
                )
            } else if !def.aliases.is_empty() {
                self.add_base_with_aliases(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name,
                    def.prefix,
                    def.aliases,
                )
            } else {
                self.add_base(
                    def.dimensions
                        .ok_or(QuanstantsError::Definition("dimensions".to_string()))?,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name,
                    def.prefix,
                )
            }
        } else {
            let value = def
                .value
                .ok_or(QuanstantsError::Definition("value".to_string()))?;
            let proportionality_factor = match value.uncertainty {
                Some(u) => value.number.with_uncertainty(u),
                None => value.number,
            };
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
                    def.name,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.alt_names,
                )
            } else if !def.aliases.is_empty() {
                self.add_derived_with_aliases(
                    def.id,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.aliases,
                )
            } else {
                self.add_derived(
                    def.id,
                    def.symbol
                        .ok_or(QuanstantsError::Definition("symbol".to_string()))?,
                    def.name,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                )
            }
        };
        self.sources.insert(id, def.source);
        Ok(id)
    }

    pub fn load_module(&mut self, module: UnitModule) -> Result<(), QuanstantsError> {
        let def_file: DefFile =
            toml::from_str(module.toml()).expect("Files stored in binary, so they should work");
        for u in def_file.units.into_values() {
            self.add_from_def(u)?;
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
    pub fn get_by_symbol(&self, symbol: &str) -> Option<Unit> {
        self.symbol_map.get(symbol).cloned()
    }

    #[inline]
    pub fn get_by_id(&self, id: Unit128) -> Option<Unit> {
        self.units.get(&id).cloned()
    }
}

// Functions to add sets of units, for internal use only
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
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::{context::py::PyContext, defs::units::py::PyUnitModule, unit::py::PyUnit};

    use super::*;
    use pyo3::prelude::*;

    #[pyclass]
    pub(crate) struct PyUnits {
        pub(crate) parent: Py<PyContext>,
    }

    #[pymethods]
    impl PyUnits {
        // Square bracket notation lookup for units
        fn __getitem__(&self, py: Python, name: &str) -> PyUnit {
            self.parent
                .borrow(py)
                .0
                .units
                .get_by_name(name)
                .unwrap()
                .into()
        }

        fn get_by_name(&self, py: Python, name: &str) -> PyUnit {
            self.parent
                .borrow(py)
                .0
                .units
                .get_by_name(name)
                .unwrap()
                .into()
        }

        fn get_by_id(&self, py: Python, id: u128) -> PyUnit {
            self.parent
                .borrow(py)
                .0
                .units
                .get_by_id(Unit128::from_bits(id))
                .unwrap()
                .into()
        }

        fn list(&self, py: Python) -> Vec<String> {
            self.parent
                .borrow(py)
                .0
                .units
                .string_map
                .keys()
                .cloned()
                .collect()
        }

        fn load_module(&mut self, py: Python, module: PyUnitModule) {
            self.parent
                .borrow_mut(py)
                .0
                .units
                .load_module(module.into())
                .unwrap();
        }
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
        //assert_eq!(
        //    reg.get_by_name("litre").unwrap(),
        //    m.clone() * m.clone() * m.clone()// * SciNum::new_exact(dec!(0.001))
        //);
    }

    #[test]
    #[allow(non_snake_case)]
    fn get_by_symbol() {
        let reg = UnitRegistry::default();

        let _s = reg.get_by_symbol("s").unwrap();
        let _m = reg.get_by_symbol("m").unwrap();
        let _kg = reg.get_by_symbol("kg").unwrap();
        let _A = reg.get_by_symbol("A").unwrap();
        let _K = reg.get_by_symbol("K").unwrap();
        let _mol = reg.get_by_symbol("mol").unwrap();
        let _cd = reg.get_by_symbol("cd").unwrap();
    }
}
