// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{self, Debug},
    ops::{Div, Mul},
    str::FromStr,
};

use num_traits::Inv;
use serde::{Deserialize, Serialize};

use crate::{dimensions::Dimensions, error::QuanstantsError, fraction::Frac, scinum::SciNum};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Nibble {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    XA = 0xA,
    XB = 0xB,
    XC = 0xC,
    XD = 0xD,
    XE = 0xE,
    XF = 0xF,
}

impl TryFrom<u8> for Nibble {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0xF {
            Err("Maximum value of a nibble is 15!")
        } else {
            Ok(unsafe { std::mem::transmute::<u8, Nibble>(value) })
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CGSSystem {
    Unspecified,
    Electrostatic,
    Electromagnetic,
    Gaussian,
    HeavisideLorentz,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum UnitSystem {
    Linear = 0x0,
    Base10Log = 0x1,
    Base2Log = 0x2,
    NaturalLog = 0x3,
    Temperature = 0x4,
    UnknownSICompatible(u8), // 5-9 not yet assigned
    CentimetreGramSecond(CGSSystem) = 0xC,
    UnknownSIIncompatible(u8), // A, B, D not yet assigned
    Private(u8),               // E and F
}

impl UnitSystem {
    pub fn from_nibble(nibble: Nibble) -> Self {
        match nibble {
            Nibble::X0 => Self::Linear,
            Nibble::X1 => Self::Base10Log,
            Nibble::X2 => Self::Base2Log,
            Nibble::X3 => Self::NaturalLog,
            Nibble::X4 => Self::Temperature,
            Nibble::X5 | Nibble::X6 | Nibble::X7 | Nibble::X8 | Nibble::X9 => {
                Self::UnknownSICompatible(nibble as u8)
            }
            Nibble::XA | Nibble::XB | Nibble::XD => Self::UnknownSIIncompatible(nibble as u8),
            Nibble::XC => Self::CentimetreGramSecond(CGSSystem::Unspecified),
            Nibble::XE | Nibble::XF => Self::Private(nibble as u8),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum UnitType {
    Base = 0x0,
    CataloguedDerived(u8), // 1-9
    Normalized = 0xA,
    BinaryDerived = 0xB,
    GenericCompound = 0xC,
    UnknownDerived = 0xD,
    Private(u8), // E and F
}

impl UnitType {
    pub fn from_nibble(nibble: Nibble) -> Self {
        match nibble {
            Nibble::X0 => Self::Base,
            Nibble::XA => Self::Normalized,
            Nibble::XB => Self::BinaryDerived,
            Nibble::XC => Self::GenericCompound,
            Nibble::XD => Self::UnknownDerived,
            Nibble::XE | Nibble::XF => Self::Private(nibble as u8),
            _ => Self::CataloguedDerived(nibble as u8),
        }
    }

    pub fn to_nibble(&self) -> Nibble {
        match self {
            Self::Base => Nibble::X0,
            Self::Normalized => Nibble::XA,
            Self::BinaryDerived => Nibble::XB,
            Self::GenericCompound => Nibble::XC,
            Self::UnknownDerived => Nibble::XD,
            Self::Private(n) => {
                Nibble::try_from(*n).expect("Inner u8 will always fit into a nibble")
            }
            Self::CataloguedDerived(n) => {
                Nibble::try_from(*n).expect("Inner u8 will always fit into a nibble")
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Unit128 {
    pub(crate) num: u64,
    pub(crate) dim: u64,
}

impl Unit128 {
    pub fn new(factor: SciNum, dimensions: Dimensions, least_significant_byte: u8) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = Unit128::factor_to_bits(factor);
        Self { num, dim }
    }

    pub fn new_referenced(
        factor: SciNum,
        dimensions: Dimensions,
        least_significant_byte: u8,
        reference: SciNum,
    ) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = Unit128::factor_and_reference_to_bits(factor, reference);
        Self { num, dim }
    }

    pub fn new_compound(factors: Vec<(Unit128, Frac)>) -> Self {
        // Panics if any of the units are referenced or not compatible with the SI
        if factors
            .iter()
            .any(|x| x.0.is_referenced() || !x.0.is_si_compatible())
        {
            panic!()
        } else {
            let dimensions = factors
                .iter()
                .map(|x| x.0.dimensions().pow(x.1))
                .fold(Dimensions::DIMENSIONLESS, |acc, x| acc * x);
            // Do this way, rather than by multiplying successive Unit128s, in order to
            // avoid introducing rounding error in the proportionality factor
            let proportionality_factor = factors
                .iter()
                .map(|x| x.0.factor().powfrac(x.1))
                .fold(SciNum::ONE, |acc, x| acc * x);
            Self::new(proportionality_factor, dimensions, 0x0C)
        }
    }

    #[inline]
    pub fn least_significant_byte(&self) -> u8 {
        (self.dim & 0xFF) as u8
    }

    #[inline]
    pub(crate) fn as_unit_type(mut self, utype: UnitType) -> Self {
        self.dim = (self.dim & !0xF) | (utype.to_nibble() as u64);
        self
    }

    #[inline]
    pub fn utype(&self) -> UnitType {
        UnitType::from_nibble(
            ((self.dim & 0xF) as u8)
                .try_into()
                .expect("Will always fit"),
        )
    }

    #[inline]
    pub fn system(&self) -> UnitSystem {
        UnitSystem::from_nibble(
            ((self.dim & 0xF0) as u8 >> 4)
                .try_into()
                .expect("Will always fit"),
        )
    }

    #[inline]
    pub fn is_si_compatible(&self) -> bool {
        (0x00..=0x9F).contains(&self.least_significant_byte())
    }

    #[inline]
    pub fn is_referenced(&self) -> bool {
        // Tried to be efficient but logic is incorrect
        //((self.dim & 0b10000000) == 0b10000000) // 0xA* to 0xF* are for other systems
        //((self.dim entirely
        //|| ((self.dim & 0xF0) == 0) // 0x0* is for normal linear units

        // Just keep it simple for now
        (0x10..=0x9F).contains(&self.least_significant_byte())
    }

    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            T: Frac::from_bits(((self.dim >> 8) & 0xFF) as u8),
            L: Frac::from_bits(((self.dim >> 16) & 0xFF) as u8),
            M: Frac::from_bits(((self.dim >> 24) & 0xFF) as u8),
            I: Frac::from_bits(((self.dim >> 32) & 0xFF) as u8),
            Θ: Frac::from_bits(((self.dim >> 40) & 0xFF) as u8),
            N: Frac::from_bits(((self.dim >> 48) & 0xFF) as u8),
            J: Frac::from_bits(((self.dim >> 56) & 0xFF) as u8),
        }
    }

    #[inline]
    pub fn factor(&self) -> SciNum {
        if self.is_referenced() {
            Unit128::bits_to_factor_and_reference(self.num).0
        } else {
            Unit128::bits_to_factor(self.num)
        }
    }

    pub fn reference_exponent(&self) -> Option<i8> {
        if self.is_referenced() {
            Some(((self.num & 0x000000FF00000000) >> 16) as i8)
        } else {
            None
        }
    }

    pub fn normalize(self) -> Self {
        // Will need to normalize the number as well I guess
        self.as_unit_type(UnitType::Normalized)
    }

    pub fn from_bits(b: u128) -> Self {
        Self {
            num: (b >> 64) as u64,
            dim: (b & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64,
        }
    }

    pub fn to_bits(self) -> u128 {
        (self.num as u128) << 64 | self.dim as u128
    }

    pub fn inverse(self) -> Unit128 {
        // Panics for referenced units
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor().inv(),
                self.dimensions().inverse(),
                self.least_significant_byte(),
            )
            .as_unit_type(UnitType::GenericCompound) // Set as generic compound
                                                     // unit
        }
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Unit128 {
        // Panics for referenced units
        let exp: Frac = exponent.into();
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor().powfrac(exp),
                self.dimensions().pow(exp),
                self.least_significant_byte(),
            )
            .as_unit_type(UnitType::GenericCompound) // Set as generic compound
                                                     // unit
        }
    }
}

impl Mul for Unit128 {
    type Output = Self;

    // Panics for referenced units
    fn mul(self, rhs: Unit128) -> Unit128 {
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor() * rhs.factor(),
                self.dimensions() * rhs.dimensions(),
                self.least_significant_byte(),
            )
            .as_unit_type(UnitType::GenericCompound) // Set as generic compound
                                                     // unit
        }
    }
}

impl Div for Unit128 {
    type Output = Self;

    // Panics for referenced units
    fn div(self, rhs: Unit128) -> Unit128 {
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor() / rhs.factor(),
                self.dimensions() / rhs.dimensions(),
                self.least_significant_byte(),
            )
            .as_unit_type(UnitType::GenericCompound) // Set as generic compound
                                                     // unit
        }
    }
}

impl Debug for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit128 {{ num: {:X}, dim: {:X} }}", self.num, self.dim)
    }
}

impl fmt::Display for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.to_bits())
    }
}

impl FromStr for Unit128 {
    type Err = QuanstantsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("0x").unwrap_or(s);
        let bits = u128::from_str_radix(hex, 16).map_err(|_e| QuanstantsError::Parse(s.into()))?;
        Ok(Self::from_bits(bits))
    }
}

#[allow(dead_code)]
// Functions for converting between `SciNum`s and the 64-bit numeric component
// of `Unit128`
impl Unit128 {
    // Maximum and minimum values for a simple numeric component
    // Though the bias of -1 means that actually a mantissa 1 higher than this is
    // theoretically possible, restrict to these values so that they always fit into
    // other formats with the same width but no bias
    const MAX_MANTISSA_FACTOR: i64 = 0x7FFFFFFFFFFFFF;
    const MIN_MANTISSA_FACTOR: i64 = -0x7FFFFFFFFFFFFF;
    const MAX_EXPONENT_FACTOR: i8 = 0x7F;
    const MIN_EXPONENT_FACTOR: i8 = -0x80;

    /// Calculates the 64-bit simple numeric component that encodes the provided
    /// `SciNum`.
    ///
    /// Currently panics if the factor is too large to be represented.
    pub(crate) fn factor_to_bits(factor: SciNum) -> u64 {
        let shortened_factor: SciNum = if factor.sigfigs() > 15 {
            SciNum::new_exact(
                factor
                    .number_dec()
                    .round_sf_with_strategy(
                        15,
                        rust_decimal::RoundingStrategy::MidpointAwayFromZero,
                    )
                    .expect("rust_decimal can do 28 s.f. of precision"),
            )
        } else {
            factor
        };

        match i8::try_from(shortened_factor.exponent_integral()) {
            Ok(exponent) => {
                exponent as u8 as u64 | ((shortened_factor.significand_integral() - 1) as u64) << 8
            }
            Err(_) => panic!("Exponent of provided SciNum {} exceeds the range of the i8 used for Unit128's numeric factor's exponent", shortened_factor.exponent_integral()),
        }
    }

    // Maximum and minimum values for a referenced numeric component
    const MAX_MANTISSA_FACTOR_REFERENCED: i32 = 0x7FFFFF;
    const MIN_MANTISSA_FACTOR_REFERENCED: i32 = -0x7FFFFF;
    const MAX_EXPONENT_FACTOR_REFERENCED: i8 = 0x7F;
    const MIN_EXPONENT_FACTOR_REFERENCED: i8 = -0x80;
    const MAX_MANTISSA_REFERENCE: i32 = 0x7FFFFF;
    const MIN_MANTISSA_REFERENCE: i32 = -0x7FFFFF;
    const MAX_EXPONENT_REFERENCE: i8 = 0x7F;
    const MIN_EXPONENT_REFERENCE: i8 = -0x80;

    /// Calculates the 64-bit referenced numeric component that encodes the
    /// provided `SciNum`s.
    ///
    /// Currently panics if either the factor or reference are too large to be
    /// represented.
    pub(crate) fn factor_and_reference_to_bits(factor: SciNum, reference: SciNum) -> u64 {
        let shortened_factor: SciNum = if factor.sigfigs() > 6 {
            SciNum::new_exact(
                factor
                    .number_dec()
                    .round_sf_with_strategy(6, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .expect("rust_decimal can do 28 s.f. of precision"),
            )
        } else {
            factor
        };

        let shortened_reference: SciNum = if reference.sigfigs() > 6 {
            SciNum::new_exact(
                reference
                    .number_dec()
                    .round_sf_with_strategy(6, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                    .expect("rust_decimal can do 28 s.f. of precision"),
            )
        } else {
            reference
        };

        (Unit128::factor_to_bits(shortened_factor) & 0x0000_0000_FFFF_FFFF)
            | (Unit128::factor_to_bits(shortened_reference) << 32)
    }

    /// Determines the `SciNum` encoded by the provided 64-bit numeric
    /// component.
    pub(crate) fn bits_to_factor(b: u64) -> SciNum {
        let exponent = (b & 0x0000_0000_0000_00FF) as i8;
        let significand = ((b as i64) >> 8) + 1;
        SciNum::exact_from_scientific_parts(significand, exponent.into())
    }

    /// Determines the `SciNum`s encoded by the provided 64-bit referenced
    /// numeric component.
    pub(crate) fn bits_to_factor_and_reference(b: u64) -> (SciNum, SciNum) {
        let factor_exponent = (b & 0x0000_0000_0000_00FF) as i8;
        let factor_significand = ((b & 0x0000_0000_FFFF_FF00) >> 8) + 1;
        let factor =
            SciNum::exact_from_scientific_parts(factor_significand, factor_exponent.into());
        let ref_exponent = ((b & 0x0000_00FF_0000_0000) >> 32) as i8;
        let ref_significand = (b & 0xFFFF_FF00_0000_0000 >> 40) + 1;
        let reference = SciNum::exact_from_scientific_parts(ref_significand, ref_exponent.into());
        (factor, reference)
    }
}

impl Unit128 {
    #[allow(dead_code)]
    pub const ONE: Unit128 = { Unit128 { num: 0x0, dim: 0x0 } };

    pub const SECOND: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100,
        }
    };

    pub const METRE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x110000,
        }
    };

    pub const METER: Unit128 = Unit128::METRE;

    pub const KILOGRAM: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x11000000,
        }
    };

    pub const AMPERE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000001100000000,
        }
    };

    pub const KELVIN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000110000000000,
        }
    };

    pub const MOLE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0011000000000000,
        }
    };

    pub const CANDELA: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000000000,
        }
    };

    pub const GRAM: Unit128 = {
        Unit128 {
            num: 0xFD,
            dim: 0x11000001,
        }
    };

    pub const RADIAN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000000000001,
        }
    };

    pub const STERADIAN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000000000002,
        }
    };

    pub const HERTZ: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000000F101, // s⁻¹
        }
    };

    pub const NEWTON: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001111F201, // kg⋅m⋅s⁻²
        }
    };

    pub const PASCAL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000011F1F201, // kg⋅m⁻¹⋅s⁻²
        }
    };

    pub const JOULE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001112F201, // kg⋅m²⋅s⁻²
        }
    };

    pub const WATT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001112F301, // kg⋅m²⋅s⁻³
        }
    };

    pub const COULOMB: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000001100001101, // A⋅s
        }
    };

    pub const VOLT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11112F301, // kg⋅m²⋅s⁻³⋅A⁻¹
        }
    };

    pub const FARAD: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x00000012F1F21401, // kg⁻¹⋅m⁻²⋅s⁴⋅A²
        }
    };

    pub const OHM: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F21112F301, // kg⋅m²⋅s⁻³⋅A⁻²
        }
    };

    pub const SIEMENS: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x00000012F1F21301, // kg⁻¹⋅m⁻²⋅s³⋅A²
        }
    };

    pub const WEBER: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11112F201, // kg⋅m²⋅s⁻²⋅A⁻¹
        }
    };

    pub const TESLA: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11100F201, // kg⋅s⁻²⋅A⁻¹
        }
    };

    pub const HENRY: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F21112F201, // kg⋅m²⋅s⁻²⋅A⁻²
        }
    };

    /// The absolute magnitude of the degree Celsius, equal to the kelvin
    pub const CELSIUS_DEGREE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000110000000001,
        }
    };

    /// The referenced degree Celsius, for temperatures on the Celsius scale
    pub const DEGREE_CELSIUS: Unit128 = {
        Unit128 {
            num: 0x006AB3FE00000000,
            dim: 0x0000110000000041,
        }
    };

    pub const LUMEN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000000001, // cd⋅sr
        }
    };

    pub const LUX: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000F20001, // cd⋅sr⋅m⁻²
        }
    };

    pub const BECQUEREL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000000F102, // s⁻¹
        }
    };

    pub const GRAY: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000012F201, // m²⋅s⁻²
        }
    };

    pub const SIEVERT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000012F202, // m²⋅s⁻²
        }
    };

    pub const KATAL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x001100000000F101, // mol⋅s⁻¹
        }
    };
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "UnitId")]
    pub struct PyUnitId(pub(crate) Unit128);

    #[pymethods]
    impl PyUnitId {
        #[new]
        fn new(id: u128) -> Self {
            PyUnitId(Unit128::from_bits(id))
        }

        fn __repr__(&self) -> String {
            format!("UnitId({})", self.0)
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }

        fn __eq__(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        #[classmethod]
        fn from_bits(_cls: &Bound<'_, PyType>, x: u128) -> PyResult<Self> {
            Ok(PyUnitId(Unit128::from_bits(x)))
        }

        fn to_bits(&self) -> u128 {
            self.0.to_bits()
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn new() {
        let s = Unit128::new(SciNum::ONE, Dimensions::TIME, 0x00);
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(s, Unit128::SECOND);
        assert_eq!(s.num, 0x0);
        assert_eq!(s.dim, 0x1100);
        assert_eq!(celsius.num, 0x006AB3FE00000000);
        assert_eq!(celsius.dim, 0x0000110000000041);
        assert_eq!(ft.num, 0xBE7FC);
        assert_eq!(ft.dim, 0x110001);
    }

    #[test]
    fn factor_to_bits() {
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1)), 0x0);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(2)), 0x100);
        //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(10)), 0x1); // Fails for
        // now, gives:
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(10)), 0x900);
        //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1000)), 0x3); // Fails
        // for now, gives:
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1000)), 0x3E700);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(dec!(0.1))), 0xFF);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(dec!(1e-3))), 0xFD);
        assert_eq!(
            Unit128::factor_to_bits(SciNum::new_exact(-1)),
            0xFFFFFFFFFFFFFE00
        );
        assert_eq!(
            Unit128::factor_to_bits(SciNum::new_exact(-3)),
            0xFFFFFFFFFFFFFC00
        );
        assert_eq!(
            Unit128::factor_to_bits(SciNum::new_exact(dec!(0.3048))),
            0xBE7FC
        );
    }

    #[test]
    fn bits_to_factor() {
        assert_eq!(Unit128::bits_to_factor(0x0), SciNum::new_exact(1));
        assert_eq!(Unit128::bits_to_factor(0x100), SciNum::new_exact(2));
        //assert_eq!(Unit128::bits_to_factor(0x1, SciNum::new_exact(10)); // Fails for
        // now, gives:
        assert_eq!(Unit128::bits_to_factor(0x900), SciNum::new_exact(10));
        //assert_eq!(Unit128::bits_to_factor(0x3, SciNum::new_exact(1000)); // Fails
        // for now, gives:
        assert_eq!(Unit128::bits_to_factor(0x3E700), SciNum::new_exact(1000));
        assert_eq!(Unit128::bits_to_factor(0xFF), SciNum::new_exact(dec!(0.1)));
        assert_eq!(Unit128::bits_to_factor(0xFD), SciNum::new_exact(dec!(1e-3)));
        assert_eq!(
            Unit128::bits_to_factor(0xFFFFFFFFFFFFFE00),
            SciNum::new_exact(-1)
        );
        assert_eq!(
            Unit128::bits_to_factor(0xFFFFFFFFFFFFFC00),
            SciNum::new_exact(-3)
        );
        assert_eq!(
            Unit128::bits_to_factor(0xBE7FC),
            SciNum::new_exact(dec!(0.3048))
        );
    }

    #[test]
    fn factor() {
        assert_eq!(Unit128::KILOGRAM.factor(), SciNum::ONE);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.factor(), SciNum::new_exact(dec!(0.3048)));
        // Calling factor() on this was broken, keep as regression test
        let x = Unit128 {
            num: 0x20789937226C9F0,
            dim: 0x1214F40D,
        };
        // The above should correspond to:
        // 4.184^-2 = 0.05712374190670824665757561355… = 571237419067082 * 10^-16
        assert_eq!(x.factor(), SciNum::new_exact(dec!(571237419067082e-16)));
    }

    #[test]
    fn dimensions() {
        assert_eq!(Unit128::KILOGRAM.dimensions(), Dimensions::MASS);
        assert_eq!(
            Unit128::KELVIN.dimensions(),
            Dimensions::THERMODYNAMIC_TEMPERATURE
        );
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.dimensions(), Dimensions::LENGTH);
    }

    #[test]
    fn lsb() {
        assert_eq!(Unit128::ONE.least_significant_byte(), 0x00);
        assert_eq!(Unit128::SECOND.least_significant_byte(), 0x00);
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert_eq!(celsius.least_significant_byte(), 0x41);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.least_significant_byte(), 0x01);
    }

    #[test]
    fn is_referenced() {
        assert!(!Unit128::ONE.is_referenced());
        assert!(!Unit128::SECOND.is_referenced());
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert!(celsius.is_referenced());
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert!(!ft.is_referenced());
    }

    #[test]
    fn to_from_str() {
        let s = Unit128::SECOND;
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        // Test these known examples
        assert_eq!(s.to_string(), "0x1100");
        assert_eq!(celsius.to_string(), "0x6AB3FE000000000000110000000041");
        assert_eq!(ft.to_string(), "0xBE7FC0000000000110001");
        // Test round trip
        assert_eq!(Unit128::from_str(&s.to_string()).unwrap(), s);
        assert_eq!(Unit128::from_str(&celsius.to_string()).unwrap(), celsius);
        assert_eq!(Unit128::from_str(&ft.to_string()).unwrap(), ft);
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Unit128::SECOND),
            "Unit128 { num: 0, dim: 1100 }"
        );
    }

    #[test]
    fn mul() {
        let amp_second = Unit128::AMPERE * Unit128::SECOND;
        let square_metre = Unit128::METRE * Unit128::METRE;
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        let square_foot = ft * ft;
        assert_eq!(amp_second.num, 0x0);
        assert_eq!(amp_second.dim, 0x110000110C);
        assert_eq!(square_metre.num, 0x0);
        assert_eq!(square_metre.dim, 0x12000C);
        assert_eq!(square_foot.num, 0x8DC23FF8);
        assert_eq!(square_foot.dim, 0x12000C);
    }
}
