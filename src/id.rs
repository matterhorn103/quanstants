use std::num::ParseIntError;

use pyo3::{pyclass, pymethods, types::PyType, Bound, PyResult};

use crate::dimensions::DimensionalWord;

// A UnitId consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

// TODO proper hashing

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericWord(u64);

impl NumericWord {
    pub fn new(sign: i8, mantissa: u64, base: u8, exponent: i8) -> Self {
        Self(
            exponent as u64 |
            (if base == 10 { 0 } else { base as u64}) << 8 |
            (if sign.is_positive() { 0 } else { 1 }) << 15 |
            (mantissa - 1) << 16
        )
    }

    pub fn exponent(&self) -> i8 {
        (self.0 & 0xFF) as i8
    }

    pub fn base(&self) -> u8 {
        let raw_base = ((self.0 >> 8) & 0x7F) as u8;
        if raw_base == 0 { 10 } else { raw_base }
    }

    pub fn sign(&self) -> i8 {
        let b = ((self.0 >> 15) & 0x01) as u8;
        if b == 0 { 1 } else { -1 }
    }

    pub fn mantissa(&self) -> u64 {
        (self.0 >> 16) + 1
    }
}

#[pyclass(frozen, eq, hash)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit128 {
    pub num: NumericWord,
    pub dim: DimensionalWord,
}

impl Unit128 {
    pub fn new(num: u64, dim: u64) -> Self {
        Unit128 { num: NumericWord(num), dim: DimensionalWord(dim) }
    }

    pub fn from_hex(x: &str) -> Result<Self, ParseIntError> {
        let value = u128::from_str_radix(x, 16)?;
        let num = (value >> 64) as u64;
        let dim = (value & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64;
        Ok(Unit128::new(num, dim))
    }

    pub fn to_hex(&self) -> String {
        if self.num.0 == 0 {
            format!("{:X}", self.dim.0)
        } else {
            format!("{:X}{:X}", self.num.0, self.dim.0)
        }
    }
}

#[pymethods]
impl Unit128 {
    #[new]
    fn py_new(num: u64, dim: u64) -> Self {
        Unit128::new(num, dim)
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypedUnitId {
    Base(Unit128),
    Unitless(Unit128),
    Derived(Unit128),
}
