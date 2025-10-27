// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::ops::{Div, Mul};

use crate::fraction::Frac;

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

    pub fn exponents(&self) -> [Frac; 7] {
        [self.T, self.L, self.M, self.I, self.Θ, self.N, self.J]
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Dimensions {
        let exp: Frac = exponent.into();
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

    pub fn is_dimensionless(&self) -> bool {
        self.exponents().iter().all(|&x| x.is_zero())
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
        let exponents = self.exponents();
        let symbols = ["T", "L", "M", "I", "Θ", "N", "J"];
        let output = if self.is_dimensionless() {
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

impl Dimensions {
    pub const DIMENSIONLESS: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const D: Dimensions = Dimensions::DIMENSIONLESS;

    pub const TIME: Dimensions = Dimensions {
        T: Frac::ONE,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const T: Dimensions = Dimensions::TIME;

    pub const LENGTH: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ONE,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const L: Dimensions = Dimensions::LENGTH;

    pub const MASS: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ONE,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const M: Dimensions = Dimensions::MASS;

    pub const ELECTRIC_CURRENT: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ONE,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const I: Dimensions = Dimensions::ELECTRIC_CURRENT;

    pub const THERMODYNAMIC_TEMPERATURE: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ONE,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const Θ: Dimensions = Dimensions::THERMODYNAMIC_TEMPERATURE;

    pub const AMOUNT_OF_SUBSTANCE: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ONE,
        J: Frac::ZERO,
    };

    pub const N: Dimensions = Dimensions::AMOUNT_OF_SUBSTANCE;

    pub const LUMINOUS_INTENSITY: Dimensions = Dimensions {
        T: Frac::ZERO,
        L: Frac::ZERO,
        M: Frac::ZERO,
        I: Frac::ZERO,
        Θ: Frac::ZERO,
        N: Frac::ZERO,
        J: Frac::ZERO,
    };

    pub const J: Dimensions = Dimensions::LUMINOUS_INTENSITY;
}

#[cfg(feature = "python")]
pub mod py {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "Dimensions")]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
    pub struct PyDimensions(Dimensions);

    #[pymethods]
    impl PyDimensions {
        #[new]
        #[allow(non_snake_case)]
        fn new(T: i8, L: i8, M: i8, I: i8, Θ: i8, N: i8, J: i8) -> Self {
            PyDimensions(Dimensions::new(T, L, M, I, Θ, N, J))
        }

        fn __repr__(&self) -> String {
            self.0.to_string()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __mul__(&self, other: Self) -> Self {
            PyDimensions(self.0.mul(other.0))
        }

        fn __truediv__(&self, other: Self) -> Self {
            PyDimensions(self.0.div(other.0))
        }

        fn pow(&self, other: i8) -> Self {
            PyDimensions(self.0.pow(other))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let dim = Dimensions::new(1, 2, 3, 0, -1, 0, 1);
        assert_eq!(dim.T, Frac::from(1));
        assert_eq!(dim.L, Frac::from(2));
        assert_eq!(dim.M, Frac::from(3));
        assert_eq!(dim.I, Frac::ZERO);
        assert_eq!(dim.Θ, Frac::from(-1));
        assert_eq!(dim.N, Frac::ZERO);
        assert_eq!(dim.J, Frac::from(1));
    }

    #[test]
    fn default() {
        let dim = Dimensions::default();
        assert_eq!(dim, Dimensions::new(0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn exponents() {
        let dim = Dimensions::new(1, 2, 0, -1, 0, 3, 0);
        let exp = dim.exponents();
        assert_eq!(
            exp,
            [
                Frac::from(1),
                Frac::from(2),
                Frac::from(0),
                Frac::from(-1),
                Frac::from(0),
                Frac::from(3),
                Frac::from(0)
            ]
        );
    }

    #[test]
    fn is_dimensionless_true() {
        let dim = Dimensions::new(0, 0, 0, 0, 0, 0, 0);
        assert!(dim.is_dimensionless());
    }

    #[test]
    fn is_dimensionless_false() {
        let dim = Dimensions::new(1, 0, 0, 0, 0, 0, 0);
        assert!(!dim.is_dimensionless());
    }

    #[test]
    fn dimensionless_const() {
        let dim = Dimensions::DIMENSIONLESS;
        assert_eq!(dim, Dimensions::new(0, 0, 0, 0, 0, 0, 0));
        assert_eq!(dim.T, Frac::ZERO);
        assert!(dim.is_dimensionless());
    }

    #[test]
    fn mul() {
        let dim1 = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        let dim2 = Dimensions::new(2, 0, 0, 2, 0, 0, 0);
        assert_eq!(dim1 * dim2, Dimensions::new(2, 1, 0, 4, 0, 0, 0))
    }

    #[test]
    fn div() {
        let dim1 = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        let dim2 = Dimensions::new(2, 0, 0, 2, 0, 0, 0);
        assert_eq!(dim1 / dim2, Dimensions::new(-2, 1, 0, 0, 0, 0, 0))
    }

    #[test]
    fn pow() {
        let dim = Dimensions::new(0, 1, 0, 2, 0, 0, 0);
        assert_eq!(dim.pow(2), Dimensions::new(0, 2, 0, 4, 0, 0, 0))
    }

    #[test]
    fn display() {
        let dim = Dimensions::new(1, -1, 0, 2, 0, -3, 0);
        assert_eq!(dim.to_string(), "TL⁻¹I²N⁻³");
        assert_eq!(Dimensions::DIMENSIONLESS.to_string(), "(dimensionless)");
    }
}
