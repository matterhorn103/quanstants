// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::ops::{Div, Mul, Neg};
use std::sync::Arc;

use indexmap::IndexMap;
use num_traits::{Inv, Pow};
use scinum::SciDecimal;

use crate::dimensions::Dimensions;
use crate::fraction::Frac;
use crate::prefix::Prefix;
use crate::quantity::Quantity;
use crate::unit128::Unit128;

#[derive(Copy, Clone, Debug)]
pub(crate) enum LinearUnitType {
    One,
    Base,
    Derived,
    Compound,
}

// Intended to be stored on the heap, with user-facing units then carrying
// reference-counted smart pointers to them to allow reuse
// Base, unitless, derived, and compound units i.e. normal ones that work in
// multiplication A LinearUnit should not be cloned, it should be used and
// passed around only behind a pointer
#[derive(Debug)]
pub struct LinearUnit {
    pub(crate) id: Unit128,
    pub(crate) utype: LinearUnitType,
    pub(crate) dimensions: Dimensions,
    pub(crate) symbol: Option<String>, // Compound units have None for this
    pub(crate) name: Option<String>,   // Compound units have None for this
    pub(crate) prefix: Option<Prefix>, // Only possible for derived or base units
    pub(crate) number: SciDecimal,         // 1 for everything except derived units
    pub(crate) factors: Vec<LinearFactor>, // Empty for base units and one
}

// Generally LinearUnit just stores data without having its own methods
// Logic is implemented by the LinearFactor and Unit wrappers
impl LinearUnit {
    pub fn symbol(&self, use_superscripts: bool) -> String {
        match self.utype {
            LinearUnitType::One => String::from(""),
            LinearUnitType::Base | LinearUnitType::Derived => self.symbol.clone().unwrap(),
            LinearUnitType::Compound => self
                .factors
                .iter()
                .map(|x| x.symbol(use_superscripts))
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    pub fn name(&self) -> String {
        match self.utype {
            LinearUnitType::One => String::from(""),
            LinearUnitType::Base | LinearUnitType::Derived => self.name.clone().unwrap(),
            LinearUnitType::Compound => todo!(),
        }
    }

    /// Returns `true` if the unit is a base unit or one.
    #[inline]
    pub fn is_base(&self) -> bool {
        matches!(self.utype, LinearUnitType::Base | LinearUnitType::One)
    }

    /// Returns `true` if the unit is a compound unit and all its factors are
    /// base units.
    #[inline]
    pub fn is_compound_base(&self) -> bool {
        matches!(self.utype, LinearUnitType::Compound)
            && self.factors.iter().all(|f| f.unit.is_base())
    }
}

impl LinearUnit {
    #[allow(dead_code)]
    pub const ONE: LinearUnit = LinearUnit {
        id: Unit128::ONE,
        utype: LinearUnitType::One,
        dimensions: Dimensions::DIMENSIONLESS,
        symbol: None,
        name: None,
        prefix: None,
        number: SciDecimal::ONE,
        factors: vec![],
    };
}

#[derive(Clone)]
pub struct LinearFactor {
    pub unit: Arc<LinearUnit>,
    pub exponent: Frac,
}

// Some logic is implemented on a per-LinearFactor basis to make it easier for a
// Unit to iterate over its factors
impl LinearFactor {
    /// Takes the inverse of the linear factor by multiplying the exponent by −1
    pub(crate) fn inverse(self) -> Self {
        Self {
            unit: self.unit,
            exponent: self.exponent.neg(),
        }
    }

    pub(crate) fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        // We maybe need to do more complicated logic for compound units,
        // but for everything else it works fine
        Self {
            unit: self.unit,
            exponent: self.exponent * exponent.into(),
        }
    }

    /// Returns the combined symbol of the unit and its exponent.
    /// If `use_superscripts` is `true`, uses Unicode superscript characters for
    /// the exponent.
    pub(crate) fn symbol(&self, use_superscripts: bool) -> String {
        // This will be fine as long as we don't allow LinearFactors to hold
        // a Compound unit with a non-unity exponent
        if self.exponent == 1 {
            self.unit.symbol(false)
        } else if use_superscripts {
            format!(
                "{}{}",
                self.unit.symbol(true),
                self.exponent.to_superscript()
            )
        } else {
            format!("{}{}", self.unit.symbol(false), self.exponent)
        }
    }

    /// Returns the equivalent of the factor as a tuple of a SciNum and its
    /// factors as base units.
    fn base_equivalent(self) -> (SciDecimal, Vec<LinearFactor>) {
        match self.unit.utype {
            LinearUnitType::Base | LinearUnitType::One => (SciDecimal::ONE, vec![self]),
            LinearUnitType::Derived | LinearUnitType::Compound => {
                // TODO
                // This would likely be faster if we just get the factor from the ID,
                // since the ID always has the appropriate numerical factor times for the base
                // rep.
                let prefix_value = match self.unit.prefix {
                    Some(p) => p.value(),
                    None => SciDecimal::ONE,
                };
                // e.g. if d = (c, -2) where c = 4.184 J-1
                // we want to return
                // (4.184, [{kg, -1}, {m, -2}, {s, 2}])^-2
                // = (0.0571..., [{kg, 2}, {m, 4}, {s, -4}])
                // First just work out the base equivalent of c
                let mut unit_number = self.unit.number * prefix_value;
                let mut unit_base_factors = Vec::new();
                //dbg!(self.unit.factors.clone());
                for f in self.unit.factors.clone() {
                    // First (and only) factor for example is {J, -1}
                    // Get base equivalent of {J, -1}
                    // = (1, [{kg, -1}, {m, -2}, {s, 2}])
                    let (f_base_num, mut f_base_factors) = f.base_equivalent();
                    //dbg!(f_base_factors.clone());
                    unit_number = unit_number * f_base_num;
                    unit_base_factors.append(&mut f_base_factors);
                }
                // Now we have the unit as its base equivalent
                // But need to return the whole factor as its base equivalent
                let number = unit_number.pow(self.exponent);
                let base_factors = unit_base_factors
                    .into_iter()
                    .map(|f| f.pow(self.exponent))
                    .collect();
                (number, base_factors)
            }
        }
    }
}

impl Debug for LinearFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LinearFactor {{ unit: {:X} ({}), exponent: {} }}",
            self.unit.id.to_bits(),
            self.unit.symbol(true),
            self.exponent,
        )
    }
}

// This is the user-facing struct representing a linear unit
#[derive(Clone)]
pub struct Unit {
    pub id: Unit128,
    pub(crate) inner: Arc<LinearUnit>,
}

impl Unit {
    pub fn new(inner: LinearUnit) -> Self {
        Self {
            id: inner.id,
            inner: Arc::new(inner),
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        matches!(self.inner.utype, LinearUnitType::One)
    }

    /// Returns `true` if the unit is a base unit or one.
    #[inline]
    pub fn is_base(&self) -> bool {
        self.inner.is_base()
    }

    #[inline]
    pub fn is_compound(&self) -> bool {
        matches!(self.inner.utype, LinearUnitType::Compound)
    }

    #[inline]
    pub fn is_compound_base(&self) -> bool {
        self.inner.is_compound_base()
    }

    #[inline]
    pub fn is_dimensionless(&self) -> bool {
        self.inner.dimensions.is_dimensionless()
    }

    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        self.inner.dimensions
    }

    #[inline]
    pub fn symbol(&self, use_superscripts: bool) -> String {
        self.inner.symbol(use_superscripts)
    }

    #[inline]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    pub fn number(&self) -> SciDecimal {
        self.inner.number
    }

    /// Returns the actual `LinearFactor` terms used to define the unit.
    #[inline]
    pub fn defining_factors(&self) -> Vec<LinearFactor> {
        self.inner.factors.clone()
    }

    /// Returns the effective unit factors of the unit in a `Vec`.
    ///
    /// For a base or derived unit, returns a `Vec` of a single `LinearFactor`,
    /// where that factor is the unit itself to the power of 1.
    ///
    /// For a compound unit, returns the factors that the unit is comprised of,
    /// which is the same as the unit's defining factors.
    ///
    /// For one, returns an empty `Vec`.
    pub fn to_factors(&self) -> Vec<LinearFactor> {
        match self.inner.utype {
            LinearUnitType::Base | LinearUnitType::Derived => {
                vec![LinearFactor {
                    unit: self.inner.clone(),
                    exponent: 1.into(),
                }]
            }
            LinearUnitType::Compound | LinearUnitType::One => self.inner.factors.clone(),
        }
    }

    #[inline]
    fn to_inverse_factors(&self) -> Vec<LinearFactor> {
        self.to_factors().into_iter().map(|x| x.inverse()).collect()
    }

    /// Combines factors of a compound unit that contain identical units.
    ///
    /// For example, `m s² m⁻¹` becomes `s²`, and `J K⁻¹ J` becomes `J² K⁻¹`
    ///
    /// Has no effect for non-compound units.
    pub fn cancelled_by_unit(self) -> Self {
        if !self.is_compound() {
            return self;
        }
        let old_factors = self.to_factors();
        // Use an IndexMap so that order is retained
        let mut factors_map: IndexMap<Unit128, LinearFactor> =
            IndexMap::with_capacity(old_factors.len());
        for old_factor in old_factors {
            let k = old_factor.unit.id;
            factors_map
                .entry(k)
                .and_modify(|new_factor| new_factor.exponent += old_factor.exponent)
                .or_insert(old_factor);
        }
        // Drop any terms which after cancelling are 0th order
        let new_factors = factors_map
            .into_values()
            .filter(|f| f.exponent != 0)
            .collect();

        Self::new(LinearUnit {
            id: self.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: SciDecimal::ONE,
            factors: new_factors,
        })
    }

    /// Combines factors of a compound unit that contain units of the same
    /// dimensionality. As this will generally result in a unit with a
    /// different value, returns a `Quantity` with the appropriate scaling
    /// factor.
    ///
    /// The unit kept for each dimension is that of the first term of that
    /// dimension.
    ///
    /// For example, `m ft` becomes `0.3048 m²`, and `ft m` becomes `3.2808398…
    /// ft²`.
    ///
    /// Units are combined if they have either identical dimensions, or one has
    /// the inverse dimensions of the other.
    /// This means that, for example, `s² Hz` becomes `s` (because `Hz = s⁻¹`),
    /// but `N m` does _not_ become `J` (even though `J = N m`).
    ///
    /// Has no effect for non-compound units.
    pub fn cancelled_by_dimension(self) -> Quantity<SciDecimal> {
        if !self.is_compound() {
            return self.value();
        }
        dbg!(&self);
        let old_factors = self.to_factors();
        dbg!(old_factors.clone());
        let mut num_factor = SciDecimal::ONE;
        // Use an IndexMap so that order is retained
        let mut factors_map: IndexMap<Dimensions, LinearFactor> =
            IndexMap::with_capacity(old_factors.len());
        for old_factor in old_factors {
            let k = old_factor.unit.dimensions;
            if factors_map.contains_key(&k) {
                factors_map.entry(k).and_modify(|new_factor| {
                    // `new_factor` is the target unit, u0, and `old_factor` is some derived unit,
                    // u1.
                    // u0 and u1 have terms of the same dimensions, so there is some c such that
                    // u1 = c * u0
                    // Thus, u1^n = (c * u0)^n = c^n * u0^n
                    // We want to combine some term in u0 with the u1 term
                    // u0^m * u1^n becomes u0^m * (c^n * u0^n) = c^n * u0^(m + n)
                    // First work out the multiplying factor c: u1 = c * u0, so c = u1 / u0
                    let conversion_factor =
                        old_factor.unit.id.factor() / new_factor.unit.id.factor();
                    // c^n
                    num_factor = num_factor * (conversion_factor).pow(old_factor.exponent);
                    // m + n
                    new_factor.exponent += old_factor.exponent;
                });
            // Alternatively, check for inverse
            } else if factors_map.contains_key(&k.inverse()) {
                factors_map.entry(k.inverse()).and_modify(|new_factor| {
                    // u1^n = (c * u0^-1)^n = c^n * u0^-n
                    // u0^m * u1^n becomes u0^m * (c^n * u0^-n) = c^n * u0^(m - n)
                    let conversion_factor =
                        old_factor.unit.id.factor() / new_factor.unit.id.factor();
                    num_factor = num_factor * (conversion_factor).pow(old_factor.exponent);
                    new_factor.exponent += -old_factor.exponent;
                });
            // Only then actually retain the unit without combining
            } else {
                factors_map.insert(k, old_factor);
            };
        }
        dbg!(&factors_map);
        // Drop any terms which after cancelling are 0th order
        let new_factors: Vec<LinearFactor> = factors_map
            .into_values()
            .filter(|f| f.exponent != 0)
            .collect();

        if new_factors.is_empty() && self.is_dimensionless() && num_factor == SciDecimal::ONE {
            Quantity::new(SciDecimal::ONE, Unit::one())
        } else {
            Quantity::new(
                num_factor,
                Unit::new(LinearUnit {
                    id: self.id, // Value is unchanged
                    utype: LinearUnitType::Compound,
                    dimensions: self.dimensions(),
                    symbol: None,
                    name: None,
                    prefix: None,
                    number: num_factor,
                    factors: new_factors,
                }),
            )
        }
    }

    /// Returns the equivalent of the unit as a quantity.
    #[inline]
    pub fn value(&self) -> Quantity<SciDecimal> {
        Quantity::new(SciDecimal::ONE, self.clone())
    }

    /// Returns the equivalent of the unit as a quantity in base units.
    pub fn in_base(&self) -> Quantity<SciDecimal> {
        if self.is_base() || self.is_compound_base() {
            self.value()
        } else {
            let mut num = SciDecimal::ONE;
            let mut factors = Vec::new();
            for f in self.to_factors() {
                // dbg!(f.clone());
                // dbg!(&f.unit.factors);
                let (f_base_num, f_base_factors) = f.base_equivalent();
                num = num * f_base_num;
                // dbg!(f_base_factors.clone());
                factors.append(&mut f_base_factors.into_iter().collect());
            }
            let new_unit = Unit::new(LinearUnit {
                id: Unit128::new(
                    self.id.factor() / num,
                    self.dimensions(),
                    (self.id.least_significant_byte() & 0xF0) | 0x0C, /* Set as generic compound
                                                                       * unit */
                ),
                utype: LinearUnitType::Compound,
                dimensions: self.dimensions(),
                symbol: None,
                name: None,
                prefix: None,
                number: SciDecimal::ONE,
                factors,
            });
            Quantity::new(num, new_unit)
        }
    }
}

// Equality and ordering functions not covered by traits
impl Unit {
    /// Returns `true` if the two units are exactly the same.
    ///
    /// `identical()` differs from `eq()` in that it returns `false` for two
    /// different units that have the same value.
    pub fn identical(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for Unit {
    /// Returns `true` if the two units have the same value (i.e. the same
    /// numerical factor and the same dimensions), even if they differ in other
    /// ways.
    fn eq(&self, other: &Self) -> bool {
        self.id.normalize() == other.id.normalize()
    }
}

impl Eq for Unit {}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.normalize().cmp(&other.id.normalize())
    }
}

// Arithmetic not covered by traits
impl Unit {
    pub fn inverse(self) -> Self {
        Self::new(LinearUnit {
            id: self.id.inverse(),
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions().inverse(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number().inv(),
            factors: self.to_inverse_factors(),
        })
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Self {
        let exponent: Frac = exponent.into();
        Self::new(LinearUnit {
            id: self.id.pow(exponent),
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions().pow(exponent),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number().pow(exponent),
            factors: self
                .to_factors()
                .into_iter()
                .map(|x| x.pow(exponent))
                .collect(),
        })
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(LinearUnit {
            id: self.id * rhs.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions() * rhs.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number() * rhs.number(),
            factors: self
                .to_factors()
                .into_iter()
                .chain(rhs.to_factors())
                .collect(),
        })
    }
}

impl Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(LinearUnit {
            id: self.id / rhs.id,
            utype: LinearUnitType::Compound,
            dimensions: self.dimensions() / rhs.dimensions(),
            symbol: None,
            name: None,
            prefix: None,
            number: self.number() / rhs.number(),
            factors: self
                .to_factors()
                .into_iter()
                .chain(rhs.to_inverse_factors())
                .collect(),
        })
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Unit {{ id: {}, symbol: "{}" }}"#,
            self.id,
            self.inner.symbol(true)
        )
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol(true))
    }
}

// Base units, just for internal use
#[allow(dead_code)]
impl Unit {
    pub(crate) fn one() -> Self {
        Self {
            id: Unit128::ONE,
            inner: Arc::new(LinearUnit::ONE),
        }
    }

    pub(crate) fn second() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::SECOND,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::TIME,
            symbol: Some(String::from("s")),
            name: Some(String::from("second")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn metre() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::METRE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("m")),
            name: Some(String::from("metre")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn kilogram() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::KILOGRAM,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::MASS,
            symbol: Some(String::from("kg")),
            name: Some(String::from("kilogram")),
            prefix: Some(Prefix::kilo),
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn ampere() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::AMPERE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::ELECTRIC_CURRENT,
            symbol: Some(String::from("A")),
            name: Some(String::from("ampere")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn kelvin() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::KELVIN,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::THERMODYNAMIC_TEMPERATURE,
            symbol: Some(String::from("K")),
            name: Some(String::from("kelvin")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn mole() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::MOLE,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::AMOUNT_OF_SUBSTANCE,
            symbol: Some(String::from("mol")),
            name: Some(String::from("mole")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }

    pub(crate) fn candela() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::CANDELA,
            utype: LinearUnitType::Base,
            dimensions: Dimensions::LUMINOUS_INTENSITY,
            symbol: Some(String::from("cd")),
            name: Some(String::from("candela")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Vec::new(),
        })
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use std::str::FromStr;

    use crate::{
        quantity::{SciQuantity, py::PyQuantity},
        unit128::py::PyUnitId,
    };

    use super::*;
    use pyo3::prelude::*;
    use rust_decimal::Decimal;

    #[pyclass(frozen, name = "Unit")]
    #[derive(Clone, Debug)]
    pub struct PyUnit(Unit);

    impl PyUnit {
        pub fn into_inner(self) -> Unit {
            self.0
        }

        pub fn borrow_inner(&self) -> &Unit {
            &self.0
        }

        pub fn owned_inner(&self) -> Unit {
            self.0.clone()
        }
    }

    impl From<Unit> for PyUnit {
        fn from(value: Unit) -> Self {
            Self(value)
        }
    }

    #[pymethods]
    impl PyUnit {
        fn __str__(&self) -> String {
            format!("{}", self.borrow_inner())
        }

        fn __repr__(&self) -> String {
            format!("{:?}", self.borrow_inner())
        }

        fn __eq__(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn __mul__(&self, other: &Self) -> Self {
            Self(self.owned_inner() * other.owned_inner())
        }

        // Only 3 * m is valid, not m * 3, so only define rmul and rtruediv for mixed
        // ops Operations between units and quantities are all handled by
        // PyQuantity
        fn __rmul__(&self, other: PyUnitArithmeticEnum) -> PyQuantity {
            match other {
                PyUnitArithmeticEnum::Quantity(q) => {
                    PyQuantity::from(q.into_inner() * self.owned_inner())
                }
                PyUnitArithmeticEnum::Int(i) => SciQuantity::from(i * self.owned_inner()).into(),
                PyUnitArithmeticEnum::Float(f) => {
                    (SciDecimal::from_f64_exact(f).unwrap() * self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::Decimal(d) => {
                    SciQuantity::from(d * self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::String(s) => {
                    (SciDecimal::from_str(&s).unwrap() * self.owned_inner()).into()
                }
            }
        }

        fn __truediv__(&self, other: &Self) -> Self {
            Self(self.owned_inner() / other.owned_inner())
        }

        fn __rtruediv__(&self, other: PyUnitArithmeticEnum) -> PyQuantity {
            match other {
                PyUnitArithmeticEnum::Quantity(q) => {
                    PyQuantity::from(q.into_inner() / self.owned_inner())
                }
                PyUnitArithmeticEnum::Int(i) => SciQuantity::from(i / self.owned_inner()).into(),
                PyUnitArithmeticEnum::Float(f) => {
                    (SciDecimal::from_f64_exact(f).unwrap() / self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::Decimal(d) => {
                    SciQuantity::from(d / self.owned_inner()).into()
                }
                PyUnitArithmeticEnum::String(s) => {
                    (SciDecimal::from_str(&s).unwrap() / self.owned_inner()).into()
                }
            }
        }

        fn __pow__(&self, other: i8, _modulo: &Bound<'_, PyAny>) -> Self {
            Self(self.owned_inner().pow(other))
        }

        #[getter]
        fn id(&self) -> PyUnitId {
            PyUnitId(self.0.id)
        }
    }

    #[derive(Debug, FromPyObject)]
    enum PyUnitArithmeticEnum {
        #[pyo3(transparent, annotation = "Quantity")]
        Quantity(PyQuantity),
        #[pyo3(transparent, annotation = "int")]
        Int(isize),
        #[pyo3(transparent, annotation = "float")]
        Float(f64),
        #[pyo3(transparent, annotation = "Decimal")]
        Decimal(Decimal),
        #[pyo3(transparent, annotation = "str")]
        String(String),
    }
}

#[cfg(test)]
mod tests {
    use scinum::sci;
    use std::str::FromStr;

    use super::*;

    // Functions to create additional units for testing

    fn hertz() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::HERTZ,
            utype: LinearUnitType::Derived,
            dimensions: Dimensions::TIME.inverse(),
            symbol: Some(String::from("Hz")),
            name: Some(String::from("hertz")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: Unit::second().to_inverse_factors(),
        })
    }

    fn joule() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::JOULE,
            utype: LinearUnitType::Derived,
            dimensions: Dimensions::new(-2, 2, 1, 0, 0, 0, 0),
            symbol: Some(String::from("J")),
            name: Some(String::from("joule")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: vec![
                LinearFactor {
                    unit: Unit::kilogram().inner,
                    exponent: Frac::new(1, 1),
                },
                LinearFactor {
                    unit: Unit::metre().inner,
                    exponent: Frac::new(2, 1),
                },
                LinearFactor {
                    unit: Unit::second().inner,
                    exponent: Frac::new(-2, 1),
                },
            ],
        })
    }

    fn foot() -> Unit {
        Unit::new(LinearUnit {
            id: Unit128::from_bits(0xBE7FC0000000000110001),
            utype: LinearUnitType::Derived,
            dimensions: Dimensions::LENGTH,
            symbol: Some(String::from("ft")),
            name: Some(String::from("foot")),
            prefix: None,
            number: sci!(0.3048),
            factors: Unit::metre().to_factors(),
        })
    }

    fn complicated_derived() -> Unit {
        // A unit d, defined as d = 1 c^-2, where c = 4.184 J-1
        let j = joule();
        let c_num = sci!(4.184);
        let c_dim = Dimensions::new(2, -2, -1, 0, 0, 0, 0);
        let c = Unit::new(LinearUnit {
            id: Unit128::new(c_num, c_dim, 0x0D),
            utype: LinearUnitType::Derived,
            dimensions: c_dim,
            symbol: Some(String::from("c")),
            name: Some(String::from("c")),
            prefix: None,
            number: c_num,
            factors: j.to_inverse_factors(),
        });
        let d_dim = c_dim.pow(-2);
        Unit::new(LinearUnit {
            id: Unit128::new(c_num.powi(-2), d_dim, 0x0D),
            utype: LinearUnitType::Derived,
            dimensions: d_dim,
            symbol: Some(String::from("d")),
            name: Some(String::from("d")),
            prefix: None,
            number: SciDecimal::ONE,
            factors: vec![LinearFactor {
                unit: c.inner.clone(),
                exponent: Frac::from(-2),
            }],
        })
    }

    #[test]
    fn equality() {
        let s = Unit::second();
        let s2 = s.clone();
        assert_eq!(s, s2);
    }

    #[test]
    fn mul() {
        let m = Unit::metre();
        let s = Unit::second();
        let ms = m.clone() * s.clone();
        let mm = m.clone() * m.clone();
        assert_eq!(ms.symbol(false), "m s");
        assert_eq!(ms.id, Unit128::from_bits(0x11110C));
        assert_eq!(ms.dimensions(), Dimensions::new(1, 1, 0, 0, 0, 0, 0));
        assert_eq!(mm.symbol(false), "m m");
        assert_eq!(mm.id, Unit128::from_bits(0x12000C));
        assert_eq!(mm.dimensions(), Dimensions::new(0, 2, 0, 0, 0, 0, 0));
    }

    #[test]
    fn div() {
        let s = Unit::second();
        let hz = hertz();
        let one_over_s = SciDecimal::ONE / s;
        assert_eq!(one_over_s, SciDecimal::ONE * hz);
        assert_eq!(one_over_s.to_string(), "1 s-1");
        assert_eq!(
            one_over_s.dimensions(),
            Dimensions::new(-1, 0, 0, 0, 0, 0, 0)
        );
    }

    #[test]
    fn symbol() {
        let s = Unit::second();
        assert_eq!(s.symbol(false), "s");
        let j = joule();
        assert_eq!(j.symbol(false), "J");
    }

    #[test]
    fn symbol_compound() {
        let s = Unit::second();
        let s2 = s.clone() * s.clone();
        assert_eq!(s2.symbol(false), "s s");
    }

    #[test]
    fn cancel_by_unit() {
        let s = Unit::second();
        let m = Unit::metre();
        let ms = (m.clone() * s.clone()).cancelled_by_unit();
        let mm = (m.clone() * m.clone()).cancelled_by_unit();
        let m_per_s = (m.clone() / s.clone()).cancelled_by_unit();
        let s_m_per_s = (s.clone() * (m.clone() / s.clone())).cancelled_by_unit();
        let m_ft = (m.clone() * foot()).cancelled_by_unit();
        let s_hz = (s.clone() * hertz()).cancelled_by_unit();
        assert_eq!(ms.symbol(false), "m s");
        assert_eq!(mm.symbol(false), "m2");
        assert_eq!(m_per_s.symbol(false), "m s-1");
        assert_eq!(s_m_per_s.symbol(false), "m");
        assert_eq!(m_ft.symbol(false), "m ft");
        assert_eq!(s_hz.symbol(false), "s Hz");
    }

    #[test]
    fn cancel_by_dimension() {
        let s = Unit::second();
        let m = Unit::metre();
        let ms = (m.clone() * s.clone()).cancelled_by_dimension();
        let mm = (m.clone() * m.clone()).cancelled_by_dimension();
        let s_m_per_s = (s.clone() * (m.clone() / s.clone())).cancelled_by_dimension();
        let m_ft = (m.clone() * foot()).cancelled_by_dimension();
        let s_hz = (s.clone() * hertz()).cancelled_by_dimension();
        assert_eq!(format!("{ms}"), "1 m s");
        assert_eq!(format!("{mm}"), "1 m2");
        assert_eq!(format!("{s_m_per_s}"), "1 m");
        assert_eq!(format!("{m_ft}"), "0.3048 m2");
        assert_eq!(format!("{s_hz}"), "1");
    }

    #[test]
    fn debug() {
        let s = Unit::second();
        assert_eq!(format!("{s:?}"), r#"Unit { id: 0x1100, symbol: "s" }"#);
    }

    #[test]
    fn in_base() {
        let j = joule();
        dbg!(j.defining_factors());
        let base = j.in_base();
        assert_eq!(base.to_string(), "1 kg m2 s-2");

        let s = Unit::second();
        let hz = hertz();
        let base = hz.in_base();
        assert_eq!(base, SciDecimal::ONE / s);
        assert_eq!(base.to_string(), "1 s-1");

        let m = Unit::metre();
        let ft = foot();
        let base = ft.in_base();
        assert_eq!(base, sci!(0.3048) * m);
        assert_eq!(base.to_string(), "0.3048 m");

        let d = complicated_derived();
        let base = d.in_base();
        assert_eq!(base.number, sci!(4.184).powi(-2));
        assert_eq!(base.unit.symbol(false), "kg2 m4 s-4");
    }
}
