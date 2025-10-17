use std::{
    fmt,
    num::ParseIntError,
    ops::{Div, Mul},
};

use rust_decimal::{Decimal, MathematicalOps};

use crate::{dimensions::Dimensions, error::QuanstantsError, fraction::Frac, number::Number};

// A 128-bit representation of a unit consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericFactor {
    pub sign: i8, // +1 for positive numbers, -1 for negative
    pub mantissa: u64, // m - 1 must fit into 48 bits i.e. a u48 (m up to 2^48, not 2^48 - 1)
    pub base: u8, // base must fit into 7 bits i.e. a u7 (up to 2^7 - 1)
    pub exponent: i8,
}

impl NumericFactor {
    const MAX_MANTISSA: u64 = (1 << 48);
    const MAX_BASE: u8 = (1 << 7) - 1;

    pub fn new(sign: i8, mantissa: u64, base: u8, exponent: i8) -> Self {
        Self::try_new(sign, mantissa, base, exponent).unwrap()
    }

    pub fn try_new(sign: i8, mantissa: u64, base: u8, exponent: i8) -> Result<Self, QuanstantsError> {
        if (mantissa <= Self::MAX_MANTISSA) && (base <= Self::MAX_BASE) {
            Ok(Self {
                sign: (sign >> 7) | 1,
                mantissa,
                base,
                exponent,
            })
        } else {
            Err(QuanstantsError::Range)
        }
    }

    pub fn try_pow<T: Into<Frac>>(self, exponent: T) -> Result<Self, QuanstantsError> {
        let exp = exponent.into().to_f64();
        let dec: Decimal = self.try_into()?;
        let new_dec = dec.checked_powf(exp).ok_or(QuanstantsError::Overflow)?;
        Self::try_from(new_dec)
    }
}

impl TryFrom<Number> for NumericFactor {
    type Error = QuanstantsError;

    fn try_from(n: Number) -> Result<Self, QuanstantsError> {
        n.number.try_into()
    }
}

impl TryFrom<Decimal> for NumericFactor {
    type Error = QuanstantsError;

    fn try_from(n: Decimal) -> Result<Self, QuanstantsError> {
        Ok(NumericFactor::new(
            if n.is_sign_positive() { 1 } else { -1 },
            n.mantissa().unsigned_abs() as u64,
            10,
            n.scale().try_into()?,
        ))
    }
}

impl TryFrom<NumericFactor> for i128 {
    type Error = QuanstantsError;

    fn try_from(n: NumericFactor) -> Result<i128, QuanstantsError> {
        if n.exponent.is_positive() {
            // The NumericFactor can be expressed as an integer
            let b: i128 = n.base.into();
            let e: u32 = n.exponent.try_into().expect("Already checked that exponent is positive");
            let exponential_term = b.checked_pow(e).ok_or(QuanstantsError::Overflow)?;
            let m: i128 = if n.sign.is_positive() { n.mantissa as i128 } else { -(n.mantissa as i128) }; // We know this will be fine
            // Even if the exponential term fit into an i128, might overflow when multiplied by m
            m.checked_mul(exponential_term).ok_or(QuanstantsError::Overflow)
        } else {
            // Not an int
            Err(QuanstantsError::Cast)
        }
    }
}

impl TryFrom<NumericFactor> for Decimal {
    type Error = QuanstantsError;

    fn try_from(n: NumericFactor) -> Result<Decimal, QuanstantsError> {
        if n.exponent.is_positive() {
            // The NumericFactor can be expressed as an integer, so let's do so
            let m: i128 = n.try_into()?; 
            match Decimal::try_from_i128_with_scale(m, 0) {
                    Ok(result) => Ok(result),
                    Err(_) => Err(QuanstantsError::Cast)
                }
        } else {
            if n.base != 10 {
                Err(QuanstantsError::Cast)
            } else {
                let e: u32 = n.exponent.unsigned_abs().into();
                let m: i128 = if n.sign.is_positive() { n.mantissa as i128 } else { -(n.mantissa as i128) };
                match Decimal::try_from_i128_with_scale(m, e) {
                    Ok(result) => Ok(result),
                    Err(_) => Err(QuanstantsError::Cast)
                }
            }
        }
    }
}

impl Mul for NumericFactor {
    type Output = Self;

    // At the moment, panics if the bases are different (obviously not ideal)
    fn mul(self, rhs: Self) -> Self {
        if self.base != rhs.base {
            panic!()
        } else {
            NumericFactor {
                sign: self.sign * rhs.sign,
                mantissa: self.mantissa * rhs.mantissa,
                base: self.base,
                exponent: self.exponent + rhs.exponent,
            }
        }
    }
}

impl Div for NumericFactor {
    type Output = Self;

    // At the moment, panics if the bases are different (obviously not ideal)
    // Also loses precision in the mantissa!
    fn div(self, rhs: Self) -> Self {
        if self.base != rhs.base {
            panic!()
        } else {
            NumericFactor {
                sign: self.sign * rhs.sign,
                mantissa: self.mantissa / rhs.mantissa,
                base: self.base,
                exponent: self.exponent - rhs.exponent,
            }
        }
    }
}

impl NumericFactor {
    #[allow(dead_code)]
    pub const ONE: NumericFactor = {
        NumericFactor {
            sign: 1,
            mantissa: 1,
            base: 10,
            exponent: 0,
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericReference {
    pub sign: i8,
    pub mantissa: u32,
    pub base: u8,
    pub exponent: i8,
}

impl NumericReference {
    pub fn new(sign: i8, mantissa: u32, base: u8, exponent: i8) -> Self {
        NumericReference {
            sign,
            mantissa,
            base,
            exponent,
        }
    }
}

impl NumericReference {
    #[allow(dead_code)]
    pub const ONE: NumericReference = {
        NumericReference {
            sign: 1,
            mantissa: 1,
            base: 10,
            exponent: 0,
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit128 {
    pub num: u64,
    pub dim: u64,
}

impl Unit128 {
    pub fn new(factor: NumericFactor, dimensions: Dimensions, least_significant_byte: u8) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = factor.exponent as u64
            | (if factor.base == 10 {
                0
            } else {
                factor.base as u64
            }) << 8
            | (if factor.sign.is_positive() { 0 } else { 1 }) << 15
            | (factor.mantissa - 1) << 16;
        Self { num, dim }
    }

    pub fn new_referenced(
        factor: NumericFactor,
        dimensions: Dimensions,
        least_significant_byte: u8,
        reference: NumericReference,
    ) -> Self {
        if factor.base != reference.base {
            panic!()
        }
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = factor.exponent as u64
            | (if factor.base == 10 {
                0
            } else {
                factor.base as u64
            }) << 8
            | (if factor.sign.is_positive() { 0 } else { 1 }) << 15
            | (factor.mantissa - 1) << 16
            | (reference.exponent as u64) << 32
            | ((i64::from(reference.mantissa) * (reference.sign as i64)) as u64) << 40;
        Self { num, dim }
    }

    pub fn is_referenced(&self) -> bool {
        // Tried to be efficient but logic is incorrect
        //((self.dim & 0b10000000) == 0b10000000) // 0xA* to 0xF* are for other systems entirely
        //|| ((self.dim & 0xF0) == 0) // 0x0* is for normal linear units

        // Just keep it simple for now
        (0x10..=0x9F).contains(&self.least_significant_byte())
    }

    pub fn least_significant_byte(&self) -> u8 {
        (self.dim & 0xFF) as u8
    }

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

    pub fn factor(&self) -> NumericFactor {
        NumericFactor::new(
            self.factor_sign(),
            self.factor_mantissa(),
            self.factor_base(),
            self.factor_exponent(),
        )
    }

    pub fn factor_exponent(&self) -> i8 {
        (self.num & 0xFF) as i8
    }

    pub fn factor_base(&self) -> u8 {
        let raw_base = ((self.num >> 8) & 0x7F) as u8;
        if raw_base == 0 {
            10
        } else {
            raw_base
        }
    }

    pub fn factor_mantissa(&self) -> u64 {
        if self.is_referenced() {
            (self.num >> 16) + 1
        } else {
            ((self.num & 0x00000000FFFF0000) >> 16) + 1
        }
    }

    pub fn factor_sign(&self) -> i8 {
        let b = ((self.num >> 15) & 0x01) as u8;
        if b == 0 {
            1
        } else {
            -1
        }
    }

    pub fn reference_exponent(&self) -> Option<i8> {
        if self.is_referenced() {
            Some(((self.num & 0x000000FF00000000) >> 16) as i8)
        } else {
            None
        }
    }

    pub fn reference_base(&self) -> u8 {
        let raw_base = ((self.num >> 8) & 0x7F) as u8;
        if raw_base == 0 {
            10
        } else {
            raw_base
        }
    }

    pub fn reference_mantissa(&self) -> Option<u32> {
        if self.is_referenced() {
            Some((((self.num & 0xFFFFFF0000000000) >> 16) as i32).unsigned_abs())
        } else {
            None
        }
    }

    pub fn reference_sign(&self) -> Option<i8> {
        if self.is_referenced() {
            Some((((self.num & 0xFFFFFF0000000000) >> 16) as i32).signum() as i8)
        } else {
            None
        }
    }

    pub fn normalize(self) -> Self {
        Self {
            num: self.num,
            dim: (self.dim & 0xFFFFFFFFFFFFFFF0) | 0xA,
        }
    }

    pub fn from_bits(b: u128) -> Self {
        Self {
            num: (b >> 64) as u64,
            dim: (b & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64,
        }
    }

    pub fn to_bits(self: Unit128) -> u128 {
        (self.num as u128) << 64 | self.dim as u128
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Unit128 {
        // Panics for referenced units
        let exp: Frac = exponent.into();
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor().try_pow(exp).unwrap(),
                self.dimensions().pow(exp),
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
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
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
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
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
        }
    }
}

impl fmt::Display for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.to_bits())
    }
}

impl Unit128 {
    #[allow(dead_code)]
    pub const UNITLESS: Unit128 = { Unit128 { num: 0x0, dim: 0x0 } };

    pub const ONE: Unit128 = Unit128::UNITLESS;

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

    pub const KILOGRAM: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x110000,
        }
    };

    pub const AMPERE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x11000000,
        }
    };

    pub const KELVIN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000,
        }
    };

    pub const MOLE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x110000000000,
        }
    };

    pub const CANDELA: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x110000000000,
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
    use super::*;

    #[test]
    fn new() {
        let s = Unit128::new(NumericFactor::ONE, Dimensions::TIME, 0x00);
        let _celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert_eq!(s.dimensions(), Dimensions::TIME);
    }

    #[test]
    fn lsb() {
        assert_eq!(Unit128::UNITLESS.least_significant_byte(), 0x00);
        assert_eq!(Unit128::SECOND.least_significant_byte(), 0x00);
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert_eq!(celsius.least_significant_byte(), 0x41);
    }

    #[test]
    fn is_referenced() {
        assert!(!Unit128::UNITLESS.is_referenced());
        assert!(!Unit128::SECOND.is_referenced());
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert!(celsius.is_referenced())
    }
}
