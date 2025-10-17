use std::{
    fmt,
    ops::{Div, Mul},
};

use crate::{dimensions::Dimensions, exponum::ExponentialNumber, fraction::Frac};

// A 128-bit representation of a unit consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Unit128 {
    pub num: u64,
    pub dim: u64,
}

impl Unit128 {
    pub fn new(
        factor: ExponentialNumber,
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
        factor: ExponentialNumber,
        dimensions: Dimensions,
        least_significant_byte: u8,
        reference: ExponentialNumber,
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
            | (((reference.mantissa as i64) * (reference.sign as i64)) as u64) << 40;
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

    pub fn factor(&self) -> ExponentialNumber {
        ExponentialNumber::new(
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
        let s = Unit128::new(ExponentialNumber::ONE, Dimensions::TIME, 0x00);
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
