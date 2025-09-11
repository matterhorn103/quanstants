use std::ops::{Add, Deref, Div, Mul, Neg, Sub};
//use derive_more::{Add, Sub, Mul, Div};
use num_rational::Ratio;

use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Exponent(Ratio<i8>);

impl Deref for Exponent {
    type Target = Ratio<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Exponent {
    pub fn is_zero(&self) -> bool {
        self.0 == Ratio::ZERO
    }

    pub fn is_negative(&self) -> bool {
        self.0 < Ratio::ZERO
    }
}

impl From<i8> for Exponent {
    fn from(n: i8) -> Self {
        Self(Ratio::from_integer(n))
    }
}

impl PartialEq<i8> for Exponent {
    fn eq(&self, other: &i8) -> bool {
        self.is_integer() && *self == Exponent::from(*other)
    }
}

impl Neg for Exponent {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Exponent {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<i8> for Exponent {
    type Output = Self;
    fn add(self, other: i8) -> Self {
        Self(self.0 + other)
    }
}

impl Sub for Exponent {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Sub<i8> for Exponent {
    type Output = Self;
    fn sub(self, other: i8) -> Self {
        Self(self.0 - other)
    }
}

impl Mul for Exponent {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Mul<i8> for Exponent {
    type Output = Self;
    fn mul(self, other: i8) -> Self {
        Self(self.0 * other)
    }
}

impl Div for Exponent {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl Div<i8> for Exponent {
    type Output = Self;
    fn div(self, other: i8) -> Self {
        Self(self.0 / other)
    }
}

#[pymethods]
impl Exponent {
    /// Panics if the denominator is zero
    #[new]
    pub fn new(numerator: i8, denominator: i8) -> Self {
        if denominator == 0 { panic!() };
        // Move sign of numerator to denominator
        Exponent(Ratio::new(numerator, denominator))
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
