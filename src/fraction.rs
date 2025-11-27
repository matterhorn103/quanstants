// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use num_rational::Ratio;
use num_traits::ToPrimitive;
use std::{
    fmt,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
    str::FromStr,
};

use crate::error::QuanstantsError;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
pub struct Frac(Ratio<i8>);

impl Deref for Frac {
    type Target = Ratio<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Frac {
    pub fn new(numerator: i8, denominator: i8) -> Self {
        // Like Ratio::new(), panics if the denominator is zero
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

    pub fn from_bits(b: u8) -> Self {
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

    pub fn to_bits(&self) -> u8 {
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

    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().expect("Should only be None if numer and denom not expressible as i64, which is impossible for us")
    }
}

impl Frac {
    pub const ZERO: Frac = Frac(Ratio::ZERO);

    pub const ONE: Frac = Frac(Ratio::ONE);
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

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_integer() {
            write!(f, "{}", self.to_integer())
        } else {
            write!(f, "{}⁄{}", self.numer(), self.denom())
        }
    }
}

impl FromStr for Frac {
    type Err = QuanstantsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(&['/', '⁄']).collect();
        let den: i8 = match parts.len() {
            1 => Ok(1),
            2 => i8::from_str(parts[0]).map_err(|_e| QuanstantsError::Parse(parts[0].to_string())),
            _ => Err(QuanstantsError::Parse(s.to_string())),
        }?;
        let num: i8 =
            i8::from_str(parts[0]).map_err(|_e| QuanstantsError::Parse(parts[0].to_string()))?;
        Ok(Self::new(num, den))
    }
}

// We should only use super/subscripts like these in the terminal!
// Formatting of superscripts and subscripts, like other rich text formatting,
// should not be handled by changing the encoded characters but in other ways
// e.g. OpenType font features
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
        '⁄' => '⁄',
        _ => panic!(),
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "Frac")]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    pub struct PyFrac(Frac);

    #[pymethods]
    impl PyFrac {
        #[new]
        fn new(numerator: i8, denominator: i8) -> Self {
            // TODO raise an error if denominator is zero
            PyFrac(Frac::new(numerator, denominator))
        }

        fn __repr__(&self) -> String {
            format!("Frac({}, {})", self.0 .0.numer(), self.0 .0.denom())
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
        fn from_bits(_cls: &Bound<'_, PyType>, b: u8) -> Self {
            PyFrac(Frac::from_bits(b))
        }

        #[allow(clippy::wrong_self_convention)]
        fn to_bits(&self) -> u8 {
            self.0.to_bits()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let f = Frac::new(3, 4);
        assert_eq!(*f.numer(), 3);
        assert_eq!(*f.denom(), 4);
    }

    #[test]
    fn new_neg() {
        let f = Frac::new(-3, 4);
        assert_eq!(*f.numer(), -3);
        assert_eq!(*f.denom(), 4);
    }

    #[test]
    fn new_normalized_neg() {
        let f = Frac::new(3, -4); // Sign should move to numerator (that's how Ratio normalizes)
        assert_eq!(*f.numer(), -3);
        assert_eq!(*f.denom(), 4);
    }

    #[test]
    fn new_reduced() {
        let f = Frac::new(6, 8); // Should be reduced to 3/4
        assert_eq!(*f.numer(), 3);
        assert_eq!(*f.denom(), 4);
    }

    #[test]
    #[should_panic]
    fn new_zero_denominator() {
        Frac::new(1, 0);
    }

    #[test]
    fn is_zero() {
        assert!(Frac::new(0, 1).is_zero());
        assert!(Frac::new(0, 5).is_zero());
        assert!(!Frac::new(1, 2).is_zero());
        assert!(!Frac::new(-1, 2).is_zero());
    }

    #[test]
    fn is_negative() {
        assert!(Frac::new(-1, 2).is_negative());
        assert!(Frac::new(1, -2).is_negative());
        assert!(!Frac::new(1, 2).is_negative());
        assert!(!Frac::new(0, 1).is_negative());
        assert!(!Frac::new(-2, -3).is_negative());
    }

    #[test]
    fn from_i8() {
        let f = Frac::from(5);
        assert_eq!(*f.numer(), 5);
        assert_eq!(*f.denom(), 1);

        let f = Frac::from(-3);
        assert_eq!(*f.numer(), -3);
        assert_eq!(*f.denom(), 1);

        let f = Frac::from(0);
        assert!(f.is_zero());
    }

    #[test]
    fn partial_eq_i8() {
        assert!(Frac::new(6, 2) == 3);
        assert!(Frac::new(-4, 2) == -2);
        assert!(Frac::new(0, 1) == 0);
        assert!((Frac::new(3, 2) != 1));
        assert!((Frac::new(5, 2) != 2));
    }

    #[test]
    fn neg() {
        let f = Frac::new(3, 4);
        let neg_f = -f;
        assert_eq!(*neg_f.numer(), -3);
        assert_eq!(*neg_f.denom(), 4);

        let f = Frac::new(-5, 2);
        let neg_f = -f;
        assert_eq!(*neg_f.numer(), 5);
        assert_eq!(*neg_f.denom(), 2);
    }

    #[test]
    fn add() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a + b;
        assert_eq!(result, Frac::new(5, 6));

        let a = Frac::new(3, 4);
        let result = a + 2;
        assert_eq!(result, Frac::new(11, 4));
    }

    #[test]
    fn sub() {
        let a = Frac::new(3, 4);
        let b = Frac::new(1, 4);
        let result = a - b;
        assert_eq!(result, Frac::new(1, 2));

        let a = Frac::new(5, 2);
        let result = a - 2;
        assert_eq!(result, Frac::new(1, 2));
    }

    #[test]
    fn mul() {
        let a = Frac::new(2, 3);
        let b = Frac::new(3, 4);
        let result = a * b;
        assert_eq!(result, Frac::new(1, 2));

        let a = Frac::new(3, 4);
        let result = a * 2;
        assert_eq!(result, Frac::new(3, 2));
    }

    #[test]
    fn div() {
        let a = Frac::new(3, 4);
        let b = Frac::new(2, 3);
        let result = a / b;
        assert_eq!(result, Frac::new(9, 8));

        let a = Frac::new(3, 4);
        let result = a / 2;
        assert_eq!(result, Frac::new(3, 8));
    }

    #[test]
    fn display() {
        let f = Frac::new(1, 2);
        assert_eq!(f.to_string(), "1⁄2");

        let f = Frac::new(2, 1);
        assert_eq!(f.to_string(), "2");

        let f = Frac::new(0, 2);
        assert_eq!(f.to_string(), "0");
    }

    #[test]
    fn test_char_to_superscript() {
        assert_eq!(char_to_superscript('0'), '⁰');
        assert_eq!(char_to_superscript('1'), '¹');
        assert_eq!(char_to_superscript('9'), '⁹');
        assert_eq!(char_to_superscript('-'), '⁻');
        assert_eq!(char_to_superscript('⁄'), '⁄');
    }

    #[test]
    #[should_panic]
    fn char_to_superscript_invalid() {
        char_to_superscript('a'); // Should panic on invalid character
    }

    #[test]
    fn to_superscript() {
        let f = Frac::new(1, 2);
        assert_eq!(f.to_superscript(), "¹⁄²");

        let f = Frac::new(-3, 4);
        assert_eq!(f.to_superscript(), "⁻³⁄⁴");
    }

    #[test]
    fn from_bits_zeroes() {
        // All zeroes is defined as being 0 even though 0 would properly be represented as 0/1
        let f = Frac::from_bits(0x00);
        assert!(f.is_zero());
    }

    #[test]
    fn from_bits_integers() {
        // Test positive whole numbers
        let f = Frac::from_bits(0x05);
        assert_eq!(f, Frac::from(5));

        let f = Frac::from_bits(0x0F);
        assert_eq!(f, Frac::from(15));
    }

    #[test]
    fn from_bits() {
        // Test positive fractions
        let f = Frac::from_bits(0x21); // num=1, den=2
        assert_eq!(f, Frac::new(1, 2));

        let f = Frac::from_bits(0x71); // num=1, den=7
        assert_eq!(f, Frac::new(1, 7));

        let f = Frac::from_bits(0x43); // num=3, den=4
        assert_eq!(f, Frac::new(3, 4));
    }

    #[test]
    fn from_bits_neg() {
        // Test negative integers
        let f = Frac::from_bits(0xF2); // num=2, den=-1
        assert_eq!(f, Frac::from(-2));

        // Test negative fractions
        let f = Frac::from_bits(0xE3); // num=3, den=-2
        assert_eq!(f, Frac::new(-3, 2));
    }

    #[test]
    fn to_bits_zero() {
        let f = Frac::new(0, 1);
        assert_eq!(f.to_bits(), 0x00);
    }

    #[test]
    fn to_bits_positive() {
        let f = Frac::new(1, 2);
        // Should encode as positive with num=1, den=2
        assert_eq!(f.to_bits(), 0x21);
    }

    #[test]
    fn to_bits_negative() {
        let f = Frac::new(-1, 2);
        assert_eq!(f.to_bits(), 0xE1);
    }

    #[test]
    fn bits_roundtrip() {
        for n in 0i8..=15 {
            for d in -7i8..=7 {
                if d == 0 {
                    continue;
                }
                let f = Frac::new(n, d);
                let bits = f.to_bits();
                let f2 = Frac::from_bits(bits);
                assert_eq!(f, f2, "Failed roundtrip for {n}/{d}");
            }
        }
    }

    #[test]
    fn default() {
        let f = Frac::default();
        assert!(f.is_zero());
    }

    #[test]
    fn ordering() {
        let a = Frac::new(1, 3);
        let b = Frac::new(1, 2);
        let c = Frac::new(2, 3);

        assert!(a < b);
        assert!(b < c);
        assert!(a < c);

        let neg = Frac::new(-1, 2);
        assert!(neg < a);
    }

    #[test]
    fn equality() {
        let a = Frac::new(2, 4);
        let b = Frac::new(1, 2);
        assert_eq!(a, b); // Should be equal after reduction

        let c = Frac::new(3, 6);
        assert_eq!(b, c);

        let d = Frac::new(4, 2);
        assert_eq!(d, 2);
    }

    #[test]
    fn deref() {
        let f = Frac::new(3, 4);
        // Test that we can call Ratio methods through Deref
        assert!(!f.is_integer());
    }
}
