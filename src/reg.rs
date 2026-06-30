// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use scinum::{SciDecimal, SciNum};
use std::collections::HashMap;

use crate::{
    defs::{DefFile, UnitDef, units::UnitModule},
    dimensions::Dimensions,
    error::QuanstantsError,
    fraction::Frac,
    prefix::Prefix,
    unit::{LinearFactor, LinearUnit, LinearUnitType, Unit},
    unit128::Unit128,
};

/// A struct to hold unit definitions and provide lookup mechanisms for them by
/// UoMID, name, and symbol.
#[derive(Debug)]
pub struct UnitRegistry {
    pub(crate) units: HashMap<Unit128, Unit>,
    pub(crate) string_map: HashMap<String, Unit>,
    pub(crate) symbol_map: HashMap<String, Unit>,
    pub(crate) sources: HashMap<Unit128, Option<String>>,
}

/// Constructors.
impl UnitRegistry {
    /// Creates a new `UnitRegistry` with minimal pre-population (just the SI
    /// base units).
    #[inline]
    pub fn new_minimal() -> Self {
        let mut reg = Self {
            units: HashMap::new(),
            string_map: HashMap::new(),
            symbol_map: HashMap::new(),
            sources: HashMap::new(),
        };
        reg.add_one();
        reg.load_si_base();
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

    /// Creates a new `UnitRegistry` pre-populated with:
    /// - the SI base units
    /// - the SI derived units
    /// - the non-SI units officially approved for use with the SI
    /// - common prefixed units
    pub fn new_populated() -> Self {
        let mut reg = Self::new();
        reg.load_module(UnitModule::SiCompatible)
            .expect("Internal `si_compatible.toml` file should be correct");
        reg.load_common_prefixed();
        reg
    }
}

/// Private associated helper functions to assist with the creation of new
/// `LinearUnit`s and wrapping `Unit`s.
impl UnitRegistry {
    /// Creates a new base `LinearUnit` and wraps it in a `Unit`, giving it the
    /// provided UoMID.
    fn new_base(
        id: Unit128,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) -> Unit {
        Unit::new(LinearUnit {
            id,
            utype: LinearUnitType::Base,
            dimensions,
            symbol: Some(symbol),
            name: Some(name),
            prefix,
            number: SciDecimal::ONE,
            factors: vec![],
        })
    }

    /// Creates a new derived `LinearUnit` and wraps it in a `Unit`, giving it the
    /// provided UoMID.
    fn new_derived(
        id: Unit128,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciDecimal,
        unit_factors: Vec<(Unit, Frac)>,
    ) -> Unit {
        Unit::new(LinearUnit {
            id,
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
        })
    }

    fn calculate_derived_id(
        prefix: Option<Prefix>,
        proportionality_factor: SciDecimal,
        unit_factors: &[(Unit, Frac)],
    ) -> Unit128 {
        // Build the compound unit representing the unit terms only, ignoring any prefix
        // or numerical factor
        // This is just the easiest way to obtain the proportionality factor that
        // results from expressing the unit terms in SI base units, which we need
        let id_factors: Vec<(Unit128, Frac)> = unit_factors.iter().map(|x| (x.0.id, x.1)).collect();
        let cmpd = Unit128::new_compound(&id_factors);
        if let Some(p) = prefix {
            if p.is_binary()
                && proportionality_factor == SciDecimal::ONE
                && cmpd.factor() == SciDecimal::ONE
            {
                // Encode binary prefix using binary scheme
                todo!()
            } else {
                // Roll all numerical factors, including the prefix, into just one
                Unit128::new(
                    p.value() * proportionality_factor * cmpd.factor(),
                    cmpd.dimensions(),
                )
            }
        } else {
            Unit128::new(proportionality_factor * cmpd.factor(), cmpd.dimensions())
        }
    }
}

/// Methods for adding units to the registry.
impl UnitRegistry {
    /// Adds the unit to string_map under the normalized lowercase form of the
    /// provided name.
    #[inline]
    fn insert_under_string(&mut self, name: String, unit: Unit) {
        let normalized = name.to_lowercase();
        self.string_map.insert(normalized, unit);
    }

    /// Adds the unit to symbol_map under the provided symbol only if not already
    /// present.
    #[inline]
    fn insert_under_symbol_checked(&mut self, symbol: String, unit: Unit) {
        self.symbol_map.entry(symbol).or_insert(unit);
    }

    /// Adds a definition for 1 to the registry.
    fn add_one(&mut self) {
        self.units.insert(Unit128::ONE, Unit::one());
    }

    /// Creates a base unit and inserts it into the registry.
    ///
    /// The UoMID is automatically calculated.
    pub fn add_base(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
    ) -> Unit128 {
        let id = Unit128::new(SciDecimal::ONE, dimensions);
        let unit = Self::new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Creates variations of a base unit with multiple alternative names and
    /// inserts them into the registry.
    ///
    /// The `name` and each `alt_name` then all refer to separate `Unit`s with
    /// identical values.
    ///
    /// Localized and translated names are equally valid spellings, so it is
    /// important that the unit returned from a lookup has the name expected
    /// by the user and not the "canonical" (here, English) one.
    /// This includes distinguishing between "metre" and "meter".
    ///
    /// Calling `Unit.name()` on the alternative units then returns a different
    /// name in each case. The symbol and value of each alternative unit is
    /// the same. The UoMID of each alternative unit is also identical, but
    /// lookup in the registry using the ID will always return the canonical
    /// unit.
    ///
    /// The UoMID is automatically calculated.
    pub fn add_base_with_alt_names(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        alt_names: Vec<String>,
    ) -> Unit128 {
        let id = Unit128::new(SciDecimal::ONE, dimensions);
        let unit = Self::new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        for n in alt_names {
            let alt_unit = Self::new_base(id, dimensions, symbol.clone(), n.clone(), prefix);
            self.insert_under_string(n, alt_unit);
        }
        // Getting the unit by symbol or ID should return the canonical form
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Creates a base unit and inserts it into the registry under `name` and
    /// one or more `aliases` that point to the same `Unit`.
    ///
    /// Unlike `alt_names`, only a single `Unit` is created, and each alias in
    /// `aliases` refers to the exact same `Unit`.
    /// This allows the unit to be found using several different names, assisting
    /// unit discovery.
    ///
    /// For example:
    /// - the kilogram can also be found under "kilo"
    /// - the ampere can also be found under "amp"
    /// - the percent can also be found under "per cent"
    /// - the dalton can also be found under "unified atomic mass unit"
    ///
    /// The unit obtained by the search is identical regardless of the alias used.
    /// Calling `Unit.name()` on a unit obtained using an alias always returns
    /// the canonical name.
    ///
    /// The UoMID is automatically calculated.
    pub fn add_base_with_aliases(
        &mut self,
        dimensions: Dimensions,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        aliases: Vec<String>,
    ) -> Unit128 {
        let id = Unit128::new(SciDecimal::ONE, dimensions);
        let unit = Self::new_base(id, dimensions, symbol.clone(), name.clone(), prefix);
        self.insert_under_string(name, unit.clone());
        for alias in aliases {
            self.insert_under_string(alias, unit.clone());
        }
        self.insert_under_symbol_checked(symbol, unit.clone());
        self.units.insert(id, unit);
        id
    }

    /// Creates a derived unit and inserts it into the registry.
    ///
    /// If the unit is prefixed:
    /// - the symbol and name provided should already include the prefix
    /// - BUT the proportionality factor should *not* already include the
    ///   value of the prefix.
    ///
    /// The overall value of the unit is then:
    /// `prefix * proportionality_factor * factors[0] * … * factors[-1])`
    ///
    /// For catalogued units, the correct ID should be provided.
    /// For anonymous units, a UoMID is generated automatically; to avoid
    /// conflicts, anonymous units are not added to `self.units` and cannot be
    /// looked up by their ID.
    pub fn add_derived(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciDecimal,
        unit_factors: Vec<(Unit, Frac)>,
    ) -> Unit128 {
        let new_id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = Self::new_derived(
            new_id,
            symbol.clone(),
            name.clone(),
            prefix,
            proportionality_factor,
            unit_factors,
        );
        self.insert_under_string(name, unit.clone());
        self.insert_under_symbol_checked(symbol, unit.clone());
        // Only insert if the ID was provided to avoid conflicts between
        // non-catalogued units
        if id.is_some() {
            self.units.insert(new_id, unit);
        }
        new_id
    }

    /// Creates variations of a derived unit with multiple alternative names and
    /// inserts them into the registry.
    ///
    /// The `name` and each `alt_name` then all refer to separate `Unit`s with
    /// identical values.
    ///
    /// Localized and translated names are equally valid spellings, so it is
    /// important that the unit returned from a lookup has the name expected
    /// by the user and not the "canonical" (here, English) one.
    /// This includes distinguishing between "metre" and "meter".
    ///
    /// Calling `Unit.name()` on the alternative units then returns a different
    /// name in each case. The symbol and value of each alternative unit is
    /// the same. The UoMID of each alternative unit is also identical, but
    /// lookup in the registry using the ID will always return the canonical
    /// unit.
    ///
    /// See [`UnitRegistry::add_derived`] for notes on prefixed units.
    ///
    /// For catalogued units, the correct ID should be provided.
    /// For anonymous units, a UoMID is generated automatically; to avoid
    /// conflicts, anonymous units are not added to `self.units` and cannot be
    /// looked up by their ID.
    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_with_alt_names(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciDecimal,
        unit_factors: Vec<(Unit, Frac)>,
        alt_names: Vec<String>,
    ) -> Unit128 {
        let new_id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = Self::new_derived(
            new_id,
            symbol.clone(),
            name.clone(),
            prefix,
            proportionality_factor,
            unit_factors.clone(),
        );
        self.insert_under_string(name, unit.clone());
        for n in alt_names {
            let alt_unit = Self::new_derived(
                new_id,
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
        // Only insert if the ID was provided to avoid conflicts between
        // non-catalogued units
        if id.is_some() {
            self.units.insert(new_id, unit);
        }
        new_id
    }

    /// Creates a derived unit and inserts it into the registry under `name` and
    /// one or more `aliases` that point to the same `Unit`.
    ///
    /// Unlike `alt_names`, only a single `Unit` is created, and each alias in
    /// `aliases` refers to the exact same `Unit`.
    /// This allows the unit to be found using several different names, assisting
    /// unit discovery.
    ///
    /// For example:
    /// - the kilogram can also be found under "kilo"
    /// - the ampere can also be found under "amp"
    /// - the percent can also be found under "per cent"
    /// - the dalton can also be found under "unified atomic mass unit"
    ///
    /// The unit obtained by the search is identical regardless of the alias used.
    /// Calling `Unit.name()` on a unit obtained using an alias always returns
    /// the canonical name.
    ///
    /// See [`UnitRegistry::add_derived`] for notes on prefixed units.
    ///
    /// For catalogued units, the correct ID should be provided.
    /// For anonymous units, a UoMID is generated automatically; to avoid
    /// conflicts, anonymous units are not added to `self.units` and cannot be
    /// looked up by their ID.
    #[allow(clippy::too_many_arguments)]
    pub fn add_derived_with_aliases(
        &mut self,
        id: Option<Unit128>,
        symbol: String,
        name: String,
        prefix: Option<Prefix>,
        proportionality_factor: SciDecimal,
        unit_factors: Vec<(Unit, Frac)>,
        aliases: Vec<String>,
    ) -> Unit128 {
        let new_id = match id {
            Some(id) => id, // If it's a catalogued derived unit it'll have had the ID provided
            None => Self::calculate_derived_id(prefix, proportionality_factor, &unit_factors),
        };
        let unit = Self::new_derived(
            new_id,
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
        // Only insert if the ID was provided to avoid conflicts between
        // non-catalogued units
        if id.is_some() {
            self.units.insert(new_id, unit);
        }
        new_id
    }

    /// Adds a `Prefix` to the provided `Unit`, gives it an ID with the catalogue
    /// number indicated, and inserts it into the registry under the prefixed name.
    ///
    /// Generally, a catalogue number of 1 should be used for a catalogued
    /// prefixed unit.
    ///
    /// Anonymous prefixed units shouldn't be added to the registry.
    ///
    /// Only possible with metric prefixes.
    ///
    /// # Panics
    ///
    /// Panics if the catalogue number is not between 1 and 7 inclusive.
    ///
    /// Panics if attempted with a binary prefix.
    fn add_prefixed(&mut self, prefix: Prefix, unit: Unit, catalogue_number: u8) {
        if prefix.is_binary() {
            panic!("Can't add a unit with a binary prefix to a unit registry!")
        }
        if catalogue_number == 0 || catalogue_number > 7 {
            panic!("Catalogue number must be between 1 and 7 inclusive!")
        }
        // Create the unit in the normal way, but need to adjust the ID afterwards.
        let mut new_unit = prefix * unit;
        new_unit.id = new_unit.id.with_catalogue_number(catalogue_number);
        self.insert_under_string(new_unit.name(), new_unit.clone());
        self.units.insert(new_unit.id, new_unit);
    }
}

/// Methods for loading unit definition modules and adding sets of pre-defined units.
impl UnitRegistry {
    /// Adds the SI base units to the registry.
    ///
    /// Two versions of the metre are added, with the two different spellings.
    fn load_si_base(&mut self) {
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

    /// Adds some of the most common prefixed units to the registry.
    ///
    /// Note that this method relies on the definitions of the SI base units, so
    /// [`UnitRegistry::load_si_base()`] should have been called prior.
    fn load_common_prefixed(&mut self) {
        self.add_prefixed(Prefix::nano, self.get_by_id(Unit128::SECOND).unwrap(), 0x01);
        self.add_prefixed(
            Prefix::micro,
            self.get_by_id(Unit128::SECOND).unwrap(),
            0x01,
        );
        self.add_prefixed(
            Prefix::milli,
            self.get_by_id(Unit128::SECOND).unwrap(),
            0x01,
        );

        self.add_prefixed(Prefix::nano, self.get_by_id(Unit128::METRE).unwrap(), 0x01);
        self.add_prefixed(Prefix::micro, self.get_by_id(Unit128::METRE).unwrap(), 0x01);
        self.add_prefixed(Prefix::milli, self.get_by_id(Unit128::METRE).unwrap(), 0x01);
        self.add_prefixed(Prefix::centi, self.get_by_id(Unit128::METRE).unwrap(), 0x01);
        self.add_prefixed(Prefix::deci, self.get_by_id(Unit128::METRE).unwrap(), 0x01);
        self.add_prefixed(Prefix::kilo, self.get_by_id(Unit128::METRE).unwrap(), 0x01);

        self.add_prefixed(Prefix::milli, self.get_by_id(Unit128::MOLE).unwrap(), 0x01);

        self.add_prefixed(Prefix::kilo, self.get_by_id(Unit128::HERTZ).unwrap(), 0x01);
        self.add_prefixed(Prefix::mega, self.get_by_id(Unit128::HERTZ).unwrap(), 0x01);
        self.add_prefixed(Prefix::giga, self.get_by_id(Unit128::HERTZ).unwrap(), 0x01);
        self.add_prefixed(Prefix::tera, self.get_by_id(Unit128::HERTZ).unwrap(), 0x01);

        self.add_prefixed(
            Prefix::hecto,
            self.get_by_id(Unit128::PASCAL).unwrap(),
            0x01,
        );
        self.add_prefixed(Prefix::kilo, self.get_by_id(Unit128::PASCAL).unwrap(), 0x01);

        self.add_prefixed(Prefix::kilo, self.get_by_id(Unit128::WATT).unwrap(), 0x01);
        self.add_prefixed(Prefix::mega, self.get_by_id(Unit128::WATT).unwrap(), 0x01);
        self.add_prefixed(Prefix::giga, self.get_by_id(Unit128::WATT).unwrap(), 0x01);

        self.add_prefixed(Prefix::kilo, self.get_by_id(Unit128::JOULE).unwrap(), 0x01);
        self.add_prefixed(Prefix::mega, self.get_by_id(Unit128::JOULE).unwrap(), 0x01);

        self.add_prefixed(Prefix::milli, self.get_by_id(Unit128::TESLA).unwrap(), 0x01);

        if let Some(gram) = self.get_by_id(Unit128::GRAM) {
            self.add_prefixed(Prefix::milli, gram, 0x01);
        }

        if let Some(electronvolt) = self.get_by_name("electronvolt") {
            self.add_prefixed(Prefix::mega, electronvolt.clone(), 0x01);
            self.add_prefixed(Prefix::giga, electronvolt, 0x01);
        }

        if let Some(litre) = self.get_by_name("litre") {
            self.add_prefixed(Prefix::micro, litre.clone(), 0x01);
            self.add_prefixed(Prefix::milli, litre, 0x01);
        }
    }

    /// Creates a `Unit` (base or derived, as appropriate) from the definition
    /// and inserts it into the registry.
    ///
    /// Returns an error if the definition is invalid, either because the
    /// definition is missing fields necessary for the type of unit, or
    /// because the units used in the definition cannot be found in the
    /// registry.
    pub(crate) fn add_from_def(&mut self, def: UnitDef) -> Result<Unit128, QuanstantsError> {
        let id = if def.base {
            if !def.alt_names.is_empty() {
                self.add_base_with_alt_names(
                    def.dimensions.ok_or(QuanstantsError::Definition(format!(
                        "Invalid dimensions for: {}",
                        def.name
                    )))?,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
                    def.name,
                    def.prefix,
                    def.alt_names,
                )
            } else if !def.aliases.is_empty() {
                self.add_base_with_aliases(
                    def.dimensions.ok_or(QuanstantsError::Definition(format!(
                        "Invalid dimensions for: {}",
                        def.name
                    )))?,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
                    def.name,
                    def.prefix,
                    def.aliases,
                )
            } else {
                self.add_base(
                    def.dimensions.ok_or(QuanstantsError::Definition(format!(
                        "Invalid dimensions for: {}",
                        def.name
                    )))?,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
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
            let mut unit_factors: Vec<(Unit, Frac)> = Vec::new();
            for (name, exp) in value.unit {
                unit_factors.push((
                    self.get_by_name(&name)
                        .ok_or(QuanstantsError::Lookup(name))?,
                    exp,
                ))
            }
            if !def.alt_names.is_empty() {
                self.add_derived_with_alt_names(
                    def.id,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
                    def.name,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.alt_names,
                )
            } else if !def.aliases.is_empty() {
                self.add_derived_with_aliases(
                    def.id,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
                    def.name,
                    def.prefix,
                    proportionality_factor,
                    unit_factors,
                    def.aliases,
                )
            } else {
                self.add_derived(
                    def.id,
                    def.symbol.ok_or(QuanstantsError::Definition(format!(
                        "Invalid symbol for: {}",
                        def.name
                    )))?,
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

    /// Add the units defined by the given unit definition module.
    pub fn load_module(&mut self, module: UnitModule) -> Result<(), QuanstantsError> {
        let def_file: DefFile =
            toml::from_str(module.toml()).expect("Files stored in binary, so they should work");
        // Be flexible in the order we process the definitions
        // This allows units to be defined in terms of related ones in the same
        // module e.g. a yard can be defined as three feet
        let mut pending: Vec<String> = def_file.units.keys().cloned().collect();
        // Keep going until all loaded
        while let Some(name) = pending.pop() {
            match self.add_from_def(
                def_file
                    .units
                    .get(&name)
                    .expect("Couldn't have ever made it into pending if it wasn't in the map")
                    .clone(),
            ) {
                Ok(_) => continue,
                Err(e) => match e {
                    QuanstantsError::Lookup(name) => {
                        // Maybe the unit that was looked for is in this module
                        // but hasn't been loaded yet, in which case we should
                        // come back to this one later, after we've loaded the
                        // other definitions in the file
                        if pending.contains(&name) {
                            pending.insert(0, name);
                        } else {
                            Err(QuanstantsError::Lookup(name))?
                        }
                    }
                    _ => Err(e)?, // Just propagate
                },
            }
        }
        Ok(())
    }
}

// Methods for unit access and lookup.
impl UnitRegistry {
    /// Returns the unit equivalent to 1.
    #[inline]
    pub fn one(&self) -> Unit {
        self.units.get(&Unit128::ONE).unwrap().clone()
    }

    /// Returns the unit stored in the registry under `name`, or `None` if there
    /// is no corresponding unit.
    ///
    /// `name` may be the unit's canonical name, or an alias.
    /// The ampere, for example, can also be obtained using `get_by_name("amp")`.
    #[inline]
    pub fn get_by_name(&self, name: &str) -> Option<Unit> {
        self.string_map.get(&name.to_lowercase()).cloned()
    }

    /// Returns the unit with the corresponding `symbol`, or `None` if there
    /// is no corresponding unit.
    ///
    /// Note that it is common for multiple units to have the same symbol. If
    /// this is the case, it is the first one added to the registry that is
    /// listed under the symbol, and it will be that one which is returned.
    ///
    /// This makes the behaviour of this function dependent on the order in
    /// which modules were loaded.
    /// In general, lookup by name is more reliable and less error-prone, so
    /// should be preferred.
    #[inline]
    pub fn get_by_symbol(&self, symbol: &str) -> Option<Unit> {
        self.symbol_map.get(symbol).cloned()
    }

    /// Returns the unit with the corresponding UoMID, or `None` if there
    /// is no corresponding unit.
    ///
    /// Note that only base and catalogued units are accessible by their IDs.
    #[inline]
    pub fn get_by_id(&self, id: Unit128) -> Option<Unit> {
        self.units.get(&id).cloned()
    }
}

impl Default for UnitRegistry {
    /// Creates a new, pre-populated `UnitRegistry`.
    /// Equivalent to [`UnitRegistry::new_populated`].
    fn default() -> Self {
        Self::new_populated()
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
                .get_by_id(Unit128(id))
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
        //    m.clone() * m.clone() * m.clone()// *
        // SciNum::new_exact(dec!(0.001))
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
