use std::fmt;
use std::ops::{Div, Mul};

use pyo3::prelude::*;

use crate::py::fraction::Frac;


#[pyclass(str, frozen, eq, hash)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[allow(non_snake_case)]
pub struct Dimensions {
    pub T: Frac,
    pub L: Frac,
    pub M: Frac,
    pub I: Frac,
    pub Θ: Frac,
    pub N: Frac,
    pub J: Frac,
}

impl Dimensions {
    #[allow(non_snake_case)]
    pub fn new<T: Into<Frac>>(T: T, L: T, M: T, I: T, Θ: T, N: T, J: T) -> Self {
        Self {
            T: T.into(),
            L: L.into(),
            M: M.into(),
            I: I.into(),
            Θ: Θ.into(),
            N: N.into(),
            J: J.into(),
        }
    }

    pub fn pow(&self, exp: Frac) -> Dimensions {
        Dimensions {
            T: self.T * exp,
            L: self.L * exp,
            M: self.M * exp,
            I: self.I * exp,
            Θ: self.Θ * exp,
            N: self.N * exp,
            J: self.J * exp,
        }
    }
}

impl Mul for Dimensions {
    type Output = Self;

    fn mul(self, other: Dimensions) -> Dimensions {
        Self {
            T: self.T + other.T,
            L: self.L + other.L,
            M: self.M + other.M,
            I: self.I + other.I,
            Θ: self.Θ + other.Θ,
            N: self.N + other.N,
            J: self.J + other.J,
        }
    }
}

impl Div for Dimensions {
    type Output = Self;

    fn div(self, other: Dimensions) -> Dimensions {
        Self {
            T: self.T - other.T,
            L: self.L - other.L,
            M: self.M - other.M,
            I: self.I - other.I,
            Θ: self.Θ - other.Θ,
            N: self.N - other.N,
            J: self.J - other.J,
        }
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exponents = [self.T, self.L, self.M, self.I, self.Θ, self.N, self.J];
        let symbols = ["T", "L", "M", "I", "Θ", "N", "J"];
        let output = if exponents.iter().all(|&x| x.is_zero()) {
            String::from("(dimensionless)")
        } else {
            let mut string = String::new();
            for i in 0..7 {
                if !exponents[i].is_zero() {
                    string.push_str(symbols[i]);
                    if exponents[i] != 1 {
                        string.push_str(&exponents[i].to_superscript());
                    }
                }
            }
            string
        };
        write!(f, "{output}")
    }
}

#[pymethods]
impl Dimensions {
    #[new]
    #[allow(non_snake_case)]
    fn py_new(T: i8, L: i8, M: i8, I: i8, Θ: i8, N: i8, J: i8) -> Self {
        Self::new(T, L, M, I, Θ, N, J)
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __mul__(&self, other: Self) -> Self {
        self.mul(other)
    }

    fn __truediv__(&self, other: Self) -> Self {
        self.div(other)
    }

    #[pyo3(name = "pow")]
    fn py_pow(&self, other: i8) -> Self {
        self.pow(other.into())
    }
}
