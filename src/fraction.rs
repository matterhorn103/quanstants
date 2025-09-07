use std::ops::{Add, Sub, Mul, Div};

use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone, Copy)]
pub struct Frac16 {
    numerator: u8,
    denominator: i8,
}

impl Frac16 {
    pub fn from_int(n: i8) -> Self {
        Frac16 { numerator: n.unsigned_abs(), denominator: n.signum() }
    }

    pub fn to_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn to_int(self) -> i8 {
        (self.numerator as i16 / self.denominator as i16) as i8
    }

    /// Get the greatest common divisor of two numbers
    fn gcd(a: u8, b: u8) -> u8 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    /// Reduce the fraction to its simplest form
    pub fn reduce(self) -> Self {
        let gcd = Self::gcd(self.numerator, self.denominator.unsigned_abs());
        let num = self.numerator / gcd;
        let den = self.denominator / gcd as i8;
        
        Frac16 { numerator: num, denominator: den }
    }

    /// Check if the fraction represents zero
    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    /// Check if the fraction is negative
    pub fn is_negative(&self) -> bool {
        self.denominator.is_negative()
    }
}

impl PartialEq for Frac16 {
    fn eq(&self, other: &Self) -> bool {
        // Cross multiply to compare: a/b == c/d if a*d == b*c
        // Handle signs properly
        let left = (self.numerator as i16) * (other.denominator as i16);
        let right = (other.numerator as i16) * (self.denominator as i16);
        left == right
    }
}

impl Eq for Frac16 {}

impl Add for Frac16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let num = (self.numerator as i16) * (other.denominator as i16) + 
                  (other.numerator as i16) * (self.denominator as i16);
        let den = (self.denominator as i16) * (other.denominator as i16);
        
        // Handle potential overflow and sign
        let (final_num, final_den) = if num < 0 {
            ((-num) as u8, -(den as i8))
        } else {
            (num as u8, den as i8)
        };
        
        Frac16 { numerator: final_num, denominator: final_den }.reduce()
    }
}

impl Sub for Frac16 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let num = (self.numerator as i16) * (other.denominator as i16) - 
                  (other.numerator as i16) * (self.denominator as i16);
        let den = (self.denominator as i16) * (other.denominator as i16);
        
        // Handle potential overflow and sign
        let (final_num, final_den) = if num < 0 {
            ((-num) as u8, -(den as i8))
        } else {
            (num as u8, den as i8)
        };
        
        Frac16 { numerator: final_num, denominator: final_den }.reduce()
    }
}

impl Mul for Frac16 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let num = (self.numerator as u16) * (other.numerator as u16);
        let den = (self.denominator as i16) * (other.denominator as i16);
        
        Frac16 { 
            numerator: num as u8, 
            denominator: den as i8 
        }.reduce()
    }
}

impl Div for Frac16 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.numerator == 0 {
            panic!("Division by zero");
        }
        
        // Multiply by reciprocal
        let num = (self.numerator as u16) * (other.denominator.unsigned_abs() as u16);
        let den = (self.denominator as i16) * (other.numerator as i16);
        
        // Handle sign from flipping denominator of other
        let final_den = if other.denominator < 0 { -den } else { den };
        
        Frac16 { 
            numerator: num as u8, 
            denominator: final_den as i8 
        }.reduce()
    }
}

impl std::fmt::Display for Frac16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else if self.denominator == -1 {
            write!(f, "-{}", self.numerator)
        } else if self.denominator < 0 {
            write!(f, "-{}/{}", self.numerator, -self.denominator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl std::fmt::Debug for Frac16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frac16({}/{})", self.numerator, self.denominator)
    }
}

#[pymethods]
impl Frac16 {
    /// Panics if the denominator is zero
    #[new]
    pub fn new(numerator: i8, denominator: i8) -> Self {
        if denominator == 0 { panic!() };
        // Move sign of numerator to denominator
        let d = denominator * numerator.signum();
        let n = numerator.unsigned_abs();
        Frac16 { numerator: n, denominator: d }
    }

    fn __repr__(&self) -> String {
        format!("Frac16({}, {})", self.numerator, self.denominator)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}
