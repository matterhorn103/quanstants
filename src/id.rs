use std::{fmt, num::ParseIntError};

use crate::{dimensions::Dimensions, fraction::Frac};

// A UnitId consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

// TODO proper hashing

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericFactor {
    pub sign: i8,
    pub mantissa: u64,
    pub base: u8,
    pub exponent: i8,
}

impl NumericFactor {
    pub fn new(
        sign: i8,
        mantissa: u64,
        base: u8,
        exponent: i8,
    ) -> Self {
        NumericFactor { sign, mantissa, base, exponent }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericReference {
    pub sign: i8,
    pub mantissa: u32,
    pub base: u8,
    pub exponent: i8,
}

impl NumericReference {
    pub fn new(
        sign: i8,
        mantissa: u32,
        base: u8,
        exponent: i8,
    ) -> Self {
        NumericReference { sign, mantissa, base, exponent }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit128(pub u64, pub u64);

impl Unit128 {
    pub fn new(
        factor: NumericFactor,
        dimensions: Dimensions,
        least_significant_byte: u8,
    ) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = factor.exponent as u64
            | (if factor.base == 10 { 0 } else { factor.base as u64 }) << 8
            | (if factor.sign.is_positive() { 0 } else { 1 }) << 15
            | (factor.mantissa - 1) << 16;
        Self(num, dim)
    }

    pub fn new_referenced(
        factor: NumericFactor,
        dimensions: Dimensions,
        least_significant_byte: u8,
        reference: NumericReference,
    ) -> Self {
        if factor.base != reference.base { panic!() }
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = factor.exponent as u64
            | (if factor.base == 10 { 0 } else { factor.base as u64 }) << 8
            | (if factor.sign.is_positive() { 0 } else { 1 }) << 15
            | (factor.mantissa as u64 - 1) << 16
            | (reference.exponent as u64) << 32
            | ((i64::from(reference.mantissa) * (reference.sign as i64)) as u64) << 40;
        Self(num, dim)
    }

    pub fn is_referenced(&self) -> bool {
        ((self.1 & 0b10000000) == 0b10000000) // 0xA* to 0xF* are for other systems entirely
        || ((self.1 & 0xF0) == 0) // 0x0* is for normal linear units
    }

    pub fn least_significant_byte(&self) -> u8 {
        (self.1 & 0xFF) as u8
    }

    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            T: Frac::from_bits(((self.1 >> 8) & 0xFF) as u8),
            L: Frac::from_bits(((self.1 >> 16) & 0xFF) as u8),
            M: Frac::from_bits(((self.1 >> 24) & 0xFF) as u8),
            I: Frac::from_bits(((self.1 >> 32) & 0xFF) as u8),
            Θ: Frac::from_bits(((self.1 >> 40) & 0xFF) as u8),
            N: Frac::from_bits(((self.1 >> 48) & 0xFF) as u8),
            J: Frac::from_bits(((self.1 >> 56) & 0xFF) as u8),
        }
    }

    pub fn factor_exponent(&self) -> i8 {
        (self.0 & 0xFF) as i8
    }
    
    pub fn factor_base(&self) -> u8 {
        let raw_base = ((self.0 >> 8) & 0x7F) as u8;
        if raw_base == 0 {
            10
        } else {
            raw_base
        }
    }
    
    pub fn factor_mantissa(&self) -> u64 {
        if self.is_referenced() {
            (self.0 >> 16) + 1
        } else {
            ((self.0 & 0x00000000FFFF0000) >> 16) + 1
        }
    }
    
    pub fn factor_sign(&self) -> i8 {
        let b = ((self.0 >> 15) & 0x01) as u8;
        if b == 0 {
            1
        } else {
            -1
        }
    }

    pub fn reference_exponent(&self) -> Option<i8> {
        if self.is_referenced() {
            Some(((self.0 & 0x000000FF00000000) >> 16) as i8)
        } else {
            None
        }
    }

    pub fn reference_base(&self) -> u8 {
        let raw_base = ((self.0 >> 8) & 0x7F) as u8;
        if raw_base == 0 {
            10
        } else {
            raw_base
        }
    }

    pub fn reference_mantissa(&self) -> Option<u32> {
        if self.is_referenced() {
            Some((((self.0 & 0xFFFFFF0000000000) >> 16) as i32).unsigned_abs())
        } else {
            None
        }
    }

    pub fn reference_sign(&self) -> Option<i8> {
        if self.is_referenced() {
            Some((((self.0 & 0xFFFFFF0000000000) >> 16) as i32).signum() as i8)
        } else {
            None
        }
    }

    pub fn normalize(self) -> Self {
        Self(self.0, (self.1 & 0xFFFFFFFFFFFFFFF0) | 0xA)
    }

    pub fn from_bits(b: u128) -> Self {
        Self(
            (b >> 64) as u64,
            (b & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64,
        )
    }

    pub fn to_bits(self: Unit128) -> u128 {
        (self.0 as u128) << 64 | self.1 as u128
    }
}

impl fmt::Display for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.to_bits())
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "UnitId")]
    pub struct PyUnitId(Unit128);

    #[pymethods]
    impl PyUnitId {
        #[new]
        fn new(num: u64, dim: u64) -> Self {
            PyUnitId(Unit128(num, dim))
        }

        fn __str__(&self) -> String {
            format!("UnitId({})", self.0)
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
