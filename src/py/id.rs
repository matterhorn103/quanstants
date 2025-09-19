use std::num::ParseIntError;

use pyo3::{pyclass, pymethods, types::PyType, Bound, PyResult};

use crate::py::{dimensions::Dimensions, fraction::Frac};

// A UnitId consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

// TODO proper hashing

#[pyclass(frozen, eq, hash)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit128(pub u64, pub u64);

impl Unit128 {
    pub fn new(
        sign: i8,
        mantissa: u64,
        base: u8,
        exponent: i8,
        dimensions: Dimensions,
        least_significant_byte: u8,
    ) -> Self {
        let num = exponent as u64 |
                (if base == 10 { 0 } else { base as u64}) << 8 |
                (if sign.is_positive() { 0 } else { 1 }) << 15 |
                (mantissa - 1) << 16;
        let dim = least_significant_byte as u64 |
            (dimensions.T.to_byte() as u64) << 8 |
            (dimensions.L.to_byte() as u64) << 16 |
            (dimensions.M.to_byte() as u64) << 24 |
            (dimensions.I.to_byte() as u64) << 32 |
            (dimensions.Θ.to_byte() as u64) << 40 |
            (dimensions.N.to_byte() as u64) << 48 |
            (dimensions.J.to_byte() as u64) << 56;
        Self(num, dim)
    }

    pub fn sign(&self) -> i8 {
        let b = ((self.0 >> 15) & 0x01) as u8;
        if b == 0 { 1 } else { -1 }
    }

    pub fn mantissa(&self) -> u64 {
        (self.0 >> 16) + 1
    }
    
    pub fn base(&self) -> u8 {
        let raw_base = ((self.0 >> 8) & 0x7F) as u8;
        if raw_base == 0 { 10 } else { raw_base }
    }
    
    pub fn exponent(&self) -> i8 {
        (self.0 & 0xFF) as i8
    }
    
    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            T: Frac::from_byte(((self.1 >> 8) & 0xFF) as u8),
            L: Frac::from_byte(((self.1 >> 16) & 0xFF) as u8),
            M: Frac::from_byte(((self.1 >> 24) & 0xFF) as u8),
            I: Frac::from_byte(((self.1 >> 32) & 0xFF) as u8),
            Θ: Frac::from_byte(((self.1 >> 40) & 0xFF) as u8),
            N: Frac::from_byte(((self.1 >> 48) & 0xFF) as u8),
            J: Frac::from_byte(((self.1 >> 56) & 0xFF) as u8),
        }
    }
    
    pub fn least_significant_byte(&self) -> u8 {
        (self.1 & 0xFF) as u8
    }

    pub fn from_hex(x: &str) -> Result<Self, ParseIntError> {
        let value = u128::from_str_radix(x, 16)?;
        Ok(Unit128::from(value))
    }

    pub fn to_hex(self) -> String {
        format!("{:X}", u128::from(self))
    }
}

impl From<u128> for Unit128 {
    fn from(value: u128) -> Self {
        Self(
            (value >> 64) as u64,
            (value & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64,
        )
    }
}

impl From<Unit128> for u128 {
    fn from(value: Unit128) -> Self {
        (value.0 as u128) << 64 | value.1 as u128
    }
}

#[pymethods]
impl Unit128 {
    #[new]
    fn py_new(num: u64, dim: u64) -> Self {
        Unit128(num, dim)
    }

    #[classmethod]
    #[pyo3(name = "from_hex")]
    fn py_from_hex(_cls: &Bound<'_, PyType>, x: &str) -> PyResult<Self> {
        Ok(Self::from_hex(x)?)
    }

    #[pyo3(name = "to_hex")]
    fn py_to_hex(&self) -> String {
        self.to_hex()
    }

    fn __str__(&self) -> String {
        self.to_hex()
    }
}
