use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use num_traits;
use rust_decimal::{Decimal, MathematicalOps};

pub trait Numeric: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

impl<T> Numeric for T where T: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Number {
    number: Decimal,
    uncertainty: Decimal,
}

impl From<Decimal> for Number {
    fn from(value: Decimal) -> Self {
        Self {
            number: value,
            uncertainty: Decimal::ZERO,
        }
    }
}

impl Number {
    fn relative_uncertainty(&self) -> Decimal {
        self.uncertainty / self.number
    }

    fn add_with_correlation(self, rhs: Self, correlation: Decimal) -> Self {
        let sigma_ab = correlation * self.uncertainty * rhs.uncertainty;
        let number = self.number + rhs.number;
        let uncertainty = ((self.uncertainty.powu(2))
            + (rhs.uncertainty.powu(2))
            + (Decimal::new(2, 0) * sigma_ab))
            .sqrt()
            .unwrap();
        Self {
            number,
            uncertainty,
        }
    }

    fn sub_with_correlation(self, rhs: Self, correlation: Decimal) -> Self {
        let sigma_ab = correlation * self.uncertainty * rhs.uncertainty;
        let number = self.number - rhs.number;
        let uncertainty = ((self.uncertainty.powu(2)) + (rhs.uncertainty.powu(2))
            - (Decimal::new(2, 0) * sigma_ab))
            .sqrt()
            .unwrap();
        Self {
            number,
            uncertainty,
        }
    }

    fn mul_with_correlation(self, rhs: Self, correlation: Decimal) -> Self {
        let sigma_ab = correlation * self.uncertainty * rhs.uncertainty;
        let number = self.number * rhs.number;
        let uncertainty = ((self.relative_uncertainty().powu(2))
            + (rhs.relative_uncertainty().powu(2))
            + (Decimal::new(2, 0) * sigma_ab / number))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    fn div_with_correlation(self, rhs: Self, correlation: Decimal) -> Self {
        let sigma_ab = correlation * self.uncertainty * rhs.uncertainty;
        let number = self.number / rhs.number;
        let uncertainty = ((self.relative_uncertainty().powu(2))
            + (rhs.relative_uncertainty().powu(2))
            - (Decimal::new(2, 0) * sigma_ab / number))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    fn powi(self, rhs: i64) -> Self {
        self.powd(rhs.into())
    }

    fn powd(self, rhs: Decimal) -> Self {
        self.pow_with_correlation(rhs.into(), Decimal::ZERO)
    }

    fn pow_with_correlation(self, rhs: Self, correlation: Decimal) -> Self {
        let sigma_ab = correlation * self.uncertainty * rhs.uncertainty;
        let number = self.number / rhs.number;
        let uncertainty = ((self.relative_uncertainty().powu(2))
            + (rhs.relative_uncertainty().powu(2))
            - (Decimal::new(2, 0) * sigma_ab / number))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    fn ln(self) -> Self {
        let number = self.number.ln();
        let uncertainty = self.relative_uncertainty().abs();
        Self {
            number,
            uncertainty,
        }
    }

    fn log10(self) -> Self {
        let number = self.number.log10();
        let uncertainty = (self.uncertainty / (Decimal::TEN.ln() * self.number)).abs();
        Self {
            number,
            uncertainty,
        }
    }

    fn exp(self) -> Self {
        let number = self.number.exp();
        let uncertainty = number.abs() * self.uncertainty.abs();
        Self {
            number,
            uncertainty,
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_with_correlation(rhs, Decimal::ZERO)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+/-{}", self.number, self.uncertainty)
    }
}
