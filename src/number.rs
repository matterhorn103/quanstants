use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{self, FromPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

//pub trait Numeric: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

//impl<T> Numeric for T where T: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

#[derive(Copy, Clone, Debug)]
pub struct Number {
    pub number: Decimal,
    pub uncertainty: Decimal,
}

impl Number {
    pub fn new<T>(number: T, uncertainty: T) -> Self
    where
        T: Into<Decimal>,
    {
        Number {
            number: number.into(),
            uncertainty: uncertainty.into(),
        }
    }

    pub fn exact<T>(number: T) -> Self
    where
        T: Into<Decimal>,
    {
        Number {
            number: number.into(),
            uncertainty: Decimal::ZERO,
        }
    }

    pub fn from_f64(number: f64, uncertainty: f64) -> Option<Self> {
        Some(Number {
            number: Decimal::from_f64(number)?,
            uncertainty: Decimal::from_f64(uncertainty)?,
        })
    }

    pub fn relative_uncertainty(&self) -> Decimal {
        self.uncertainty / self.number
    }

    pub fn add_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty * rhs.uncertainty;
        let number = self.number + rhs.number;
        let uncertainty =
            ((self.uncertainty.powu(2)) + (rhs.uncertainty.powu(2)) + (dec!(2) * sigma_ab))
                .sqrt()
                .unwrap();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn sub_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty * rhs.uncertainty;
        let number = self.number - rhs.number;
        let uncertainty = ((self.uncertainty.powu(2)) + (rhs.uncertainty.powu(2))
            - (dec!(2) * sigma_ab))
            .sqrt()
            .unwrap();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn mul_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty * rhs.uncertainty;
        let number = self.number * rhs.number;
        let uncertainty = ((self.relative_uncertainty().powu(2))
            + (rhs.relative_uncertainty().powu(2))
            + (dec!(2) * sigma_ab / number))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn div_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty * rhs.uncertainty;
        let number = self.number / rhs.number;
        let uncertainty = ((self.relative_uncertainty().powu(2))
            + (rhs.relative_uncertainty().powu(2))
            - (dec!(2) * sigma_ab / number))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn powi(self, rhs: i64) -> Self {
        self.powd(rhs.into())
    }

    pub fn powd(self, rhs: Decimal) -> Self {
        self.pow_with_correlation(rhs.into(), Decimal::ZERO)
    }

    pub fn pow_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty * rhs.uncertainty;
        let number = self.number.powd(rhs.number);
        let uncertainty = ((self.relative_uncertainty() * rhs.number).powu(2)
            + (self.number.ln() * rhs.uncertainty).powu(2)
            + (dec!(2) * ((self.number.ln() * rhs.number) / self.number) * sigma_ab))
            .sqrt()
            .unwrap()
            * number.abs();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn ln(self) -> Self {
        let number = self.number.ln();
        let uncertainty = self.relative_uncertainty().abs();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn log10(self) -> Self {
        let number = self.number.log10();
        let uncertainty = (self.uncertainty / (Decimal::TEN.ln() * self.number)).abs();
        Self {
            number,
            uncertainty,
        }
    }

    pub fn exp(self) -> Self {
        let number = self.number.exp();
        let uncertainty = number.abs() * self.uncertainty.abs();
        Self {
            number,
            uncertainty,
        }
    }
}

impl From<Decimal> for Number {
    fn from(n: Decimal) -> Self {
        Self {
            number: n,
            uncertainty: Decimal::ZERO,
        }
    }
}

//impl FromPrimitive for Number {
//    fn from_i64(n: i64) -> Option<Self> {
//        Some(Self {
//            number: n.into(),
//            uncertainty: Decimal::ZERO,
//        })
//    }
//
//    fn from_u64(n: u64) -> Option<Self> {
//        Some(Self {
//            number: n.into(),
//            uncertainty: Decimal::ZERO,
//        })
//    }
//}

macro_rules! impl_from {
    ($T:ty) => {
        impl From<$T> for Number {
            fn from(t: $T) -> Self {
                Self {
                    number: t.into(),
                    uncertainty: Decimal::ZERO,
                }
            }
        }
    };
}

impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(i128);
impl_from!(isize);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);
impl_from!(usize);

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

macro_rules! impl_comparisons {
    ($t:ty) => {
        impl PartialEq<$t> for Number {
            fn eq(&self, other: &$t) -> bool {
                self.number == Decimal::from(*other)
            }
        }

        impl PartialOrd<$t> for Number {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                self.number.partial_cmp(&Decimal::from(*other))
            }
        }
    };
}

impl_comparisons!(i8);
impl_comparisons!(i16);
impl_comparisons!(i32);
impl_comparisons!(i64);
impl_comparisons!(i128);
impl_comparisons!(isize);
impl_comparisons!(u8);
impl_comparisons!(u16);
impl_comparisons!(u32);
impl_comparisons!(u64);
impl_comparisons!(u128);
impl_comparisons!(usize);

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Add for &Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Number {
        self.add_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Sub for &Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Number {
        self.sub_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mul_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Mul for &Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Number {
        self.mul_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.div_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Div for &Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Number {
        self.div_with_correlation(*rhs, Decimal::ZERO)
    }
}

macro_rules! impl_arithmetic {
    ($t:ty) => {
        impl Add<$t> for Number {
            type Output = Self;

            fn add(self, rhs: $t) -> Number {
                self.add_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Add<Number> for $t {
            type Output = Number;

            fn add(self, rhs: Number) -> Number {
                let num: Number = self.into();
                num.add_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Sub<$t> for Number {
            type Output = Self;

            fn sub(self, rhs: $t) -> Number {
                self.sub_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Sub<Number> for $t {
            type Output = Number;

            fn sub(self, rhs: Number) -> Number {
                let num: Number = self.into();
                num.sub_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Mul<$t> for Number {
            type Output = Self;

            fn mul(self, rhs: $t) -> Number {
                self.mul_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Mul<Number> for $t {
            type Output = Number;

            fn mul(self, rhs: Number) -> Number {
                let num: Number = self.into();
                num.mul_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Div<$t> for Number {
            type Output = Self;

            fn div(self, rhs: $t) -> Number {
                self.div_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Div<Number> for $t {
            type Output = Number;

            fn div(self, rhs: Number) -> Number {
                let num: Number = self.into();
                num.div_with_correlation(rhs, Decimal::ZERO)
            }
        }
    };
}

impl_arithmetic!(i8);
impl_arithmetic!(i16);
impl_arithmetic!(i32);
impl_arithmetic!(i64);
impl_arithmetic!(i128);
impl_arithmetic!(isize);
impl_arithmetic!(u8);
impl_arithmetic!(u16);
impl_arithmetic!(u32);
impl_arithmetic!(u64);
impl_arithmetic!(u128);
impl_arithmetic!(usize);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.uncertainty == Decimal::ZERO {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{}+/-{}", self.number, self.uncertainty)
        }
    }
}

impl Number {
    pub const ZERO: Number = Number {
        number: Decimal::ZERO,
        uncertainty: Decimal::ZERO,
    };

    pub const ONE: Number = Number {
        number: Decimal::ONE,
        uncertainty: Decimal::ZERO,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn relative_uncertainty() {
        let n = Number::new(20, 2);
        assert_eq!(n.relative_uncertainty(), dec!(0.1));

        let n2 = Number::new(500, 5);
        assert_eq!(n2.relative_uncertainty(), dec!(0.01));

        let n3 = Number::new(1000, 15);
        assert_eq!(n3.relative_uncertainty(), dec!(0.015));
    }

    #[test]
    fn addition() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let result = n1 + n2;
        assert_eq!(result.number, dec!(50));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn addition_with_int() {
        let n1 = Number::new(20, 0);
        let n2 = 30;
        let result: Number = n1 + n2;
        assert_eq!(result.number, dec!(50));
    }

    #[test]
    fn subtraction() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let result = n1 - n2;
        assert_eq!(result.number, dec!(-10));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn subtraction_with_int() {
        let n1 = Number::new(20, 0);
        let n2 = 30;
        let result: Number = n1 - n2;
        assert_eq!(result.number, dec!(-10));
    }

    #[test]
    fn multiplication() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let result = n1 * n2;
        assert_eq!(result.number, dec!(600));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(116.619037896906).round_dp(5)
        );
    }

    #[test]
    fn multiplication_with_int() {
        let n1 = Number::new(20, 0);
        let n2 = 30;
        let result: Number = n1 * n2;
        assert_eq!(result.number, dec!(600));
    }

    #[test]
    fn division() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let result = n1 / n2;
        assert_eq!(result.number.round_dp(10), dec!(0.6666666667).round_dp(10));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.129576708774340).round_dp(5)
        );
    }

    #[test]
    fn division_with_int() {
        let n1 = Number::new(60, 0);
        let n2 = 30;
        let result: Number = n1 / n2;
        assert_eq!(result.number, dec!(2));
    }

    #[test]
    fn division_reversed() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let result = n2 / n1;
        assert_eq!(result.number, dec!(1.5));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.2915475947422).round_dp(5)
        );
    }

    #[test]
    fn exponentiation() {
        let n1 = Number::new(20, 2);

        let result = n1.powd(dec!(2));
        assert_eq!(result.number, dec!(400));
        assert_eq!(result.uncertainty, dec!(80));

        let result = n1.powi(2);
        assert_eq!(result.number, dec!(400));
        assert_eq!(result.uncertainty, dec!(80));
    }

    #[test]
    fn natural_log() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.ln();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.194365063161).round_dp(5)
        );
    }

    #[test]
    fn log_base10() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.log10();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.08441167440582).round_dp(5)
        );
    }

    #[test]
    fn exponential() {
        let n1 = Number::new(20, 2);
        let n2 = Number::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.exp();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.25238096660761).round_dp(5)
        );
    }
}
