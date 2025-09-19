use std::{num::ParseIntError, ops::{Add, Deref, Div, Mul, Neg, Sub}};
//use derive_more::{Add, Sub, Mul, Div};
use num_rational::Ratio;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Frac(Ratio<i8>);

impl Deref for Frac {
    type Target = Ratio<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Frac {
    pub fn new(numerator: i8, denominator: i8) -> Self {
        if denominator == 0 {
            panic!()
        };
        Frac(Ratio::new(numerator, denominator))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == Ratio::ZERO
    }

    pub fn is_negative(&self) -> bool {
        self.0 < Ratio::ZERO
    }

    pub fn to_superscript(&self) -> String {
        let s = self.to_string();
        let mut output = String::new();
        for ch in s.chars() {
            output.push(char_to_superscript(ch));
        }
        output
    }

    pub fn from_byte(b: u8) -> Self {
        if b < 32 {
            Self::from((b & 0x0F) as i8)
        } else {
            let num = (b & 0x0F) as i8;
            let den = if b < 128 {
                (b >> 4) as i8
            } else {
                // Sign extend denominator so that we regain the 8-bit rep from the 4-bit one
                ((b >> 4) | 0xF0) as i8
            };
            Self::new(num, den)
        }
    }

    pub fn to_byte(&self) -> u8 {
        if self.is_zero() {
            0
        } else {
            let num = self.numer().unsigned_abs();
            let den = if self.is_negative() {
                self.denom().abs().neg()
            } else {
                self.denom().abs()
            };
            (den as u8) << 4 | num
        }
    }

    pub fn from_hex(x: &str) -> Result<Self, ParseIntError> {
        let byte = u8::from_str_radix(x, 16)?;
        Ok(Self::from_byte(byte))
    }

    pub fn to_hex(&self) -> String {
        let byte = self.to_byte();
        format!("{:02X}", byte)
    }
}

// We should only use super/subscripts like these in the terminal, it's Unicode abuse
fn char_to_superscript(character: char) -> char {
    match character {
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '0' => '⁰',
        '-' => '⁻',
        _ => panic!(),
    }
}

impl From<i8> for Frac {
    fn from(value: i8) -> Self {
        Self(Ratio::from_integer(value))
    }
}

impl PartialEq<i8> for Frac {
    fn eq(&self, other: &i8) -> bool {
        self.is_integer() && *self == Frac::from(*other)
    }
}

impl Neg for Frac {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Frac {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<i8> for Frac {
    type Output = Self;
    fn add(self, other: i8) -> Self {
        Self(self.0 + other)
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Sub<i8> for Frac {
    type Output = Self;
    fn sub(self, other: i8) -> Self {
        Self(self.0 - other)
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Mul<i8> for Frac {
    type Output = Self;
    fn mul(self, other: i8) -> Self {
        Self(self.0 * other)
    }
}

impl Div for Frac {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl Div<i8> for Frac {
    type Output = Self;
    fn div(self, other: i8) -> Self {
        Self(self.0 / other)
    }
}

#[cfg(feature = "python")]
pub mod py {
    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "Frac")]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    pub struct PyFrac(Frac);

    #[pymethods]
    impl PyFrac {
        /// Panics if the denominator is zero
        #[new]
        fn new(numerator: i8, denominator: i8) -> Self {
            PyFrac(Frac::new(numerator, denominator))
        }

        fn __repr__(&self) -> String {
            format!("Frac({}, {})", self.0.0.numer(), self.0.0.denom())
        }

        fn __str__(&self) -> String {
            self.0.to_string()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn numer(&self) -> i8 {
            *self.0.numer()
        }

        fn denom(&self) -> i8 {
            *self.0.denom()
        }

        #[classmethod]
        fn from_byte(_cls: &Bound<'_, PyType>, b: u8) -> Self {
            PyFrac(Frac::from_byte(b))
        }

        fn to_byte(&self) -> u8 {
            self.0.to_byte()
        }

        #[classmethod]
        fn from_hex(_cls: &Bound<'_, PyType>, x: &str) -> PyResult<Self> {
            Ok(PyFrac(Frac::from_hex(x)?))
        }

        fn to_hex(&self) -> String {
            self.0.to_hex()
        }
    }
}
