use std::ops::{Add, Deref, Div, Mul, Neg, Sub};
//use derive_more::{Add, Sub, Mul, Div};
use num_rational::Ratio;

use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Frac(Ratio<i8>);

impl Deref for Frac {
    type Target = Ratio<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Frac {
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
    fn from(n: i8) -> Self {
        Self(Ratio::from_integer(n))
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

#[pymethods]
impl Frac {
    /// Panics if the denominator is zero
    #[new]
    pub fn new(numerator: i8, denominator: i8) -> Self {
        if denominator == 0 { panic!() };
        // Move sign of numerator to denominator
        Frac(Ratio::new(numerator, denominator))
    }

    fn __repr__(&self) -> String {
        format!("Frac16({}, {})", self.0.numer(), self.0.denom())
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
}
