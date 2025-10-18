use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{self, FromPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

//pub trait Numeric: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

//impl<T> Numeric for T where T: num_traits::Num + num_traits::NumOps + std::fmt::Display {}

/// A Decimal extended to have an associated uncertainty at the same scale as well as an extra
/// scaling factor of 10<sup><i>exponent</i></sup>.
#[derive(Copy, Clone, Debug)]
pub struct SciNum {
    number: Decimal,
    uncertainty_lo: u32,
    uncertainty_mid: u32,
    uncertainty_hi: u32,
    exponent: i32,
}

impl SciNum {
    fn new_with_exponent<T>(number: T, uncertainty: T, exponent: i32) -> Self
    where
        T: Into<Decimal>,
    {
        let num: Decimal = number.into();
        let mut uncert: Decimal = uncertainty.into();
        uncert.rescale(num.scale());
        let unpacked_uncert = uncert.unpack();
        SciNum {
            number: num,
            uncertainty_lo: unpacked_uncert.lo,
            uncertainty_mid: unpacked_uncert.mid,
            uncertainty_hi: unpacked_uncert.hi,
            exponent,
        }
    }

    pub fn new<T>(number: T, uncertainty: T) -> Self
    where
        T: Into<Decimal>,
    {
        Self::new_with_exponent(number, uncertainty, 0)
    }

    pub fn new_big<T>(number: T, uncertainty: T, exponent: i32) -> Self
    where
        T: Into<Decimal>,
    {
        Self::new_with_exponent(number, uncertainty, exponent)
    }

    pub fn exact<T>(number: T) -> Self
    where
        T: Into<Decimal>,
    {
        Self {
            number: number.into(),
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
            exponent: 0,
        }
    }

    pub fn exact_big<T>(number: T, exponent: i32) -> Self
    where
        T: Into<Decimal>,
    {
        Self {
            number: number.into(),
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
            exponent,
        }
    }

    pub fn number(&self) -> Self {
        Self {
            number: self.number,
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
            exponent: self.exponent,
        }
    }

    pub(crate) fn uncertainty_dec(&self) -> Decimal {
        Decimal::from_parts(
            self.uncertainty_lo,
            self.uncertainty_mid,
            self.uncertainty_hi,
            false,
            self.decimal_scale(),
        )
    }

    pub fn uncertainty(&self) -> Self {
        Self {
            number: self.uncertainty_dec(),
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
            exponent: self.exponent,
        }
    }

    pub(crate) fn decimal_scale(&self) -> u32 {
        self.number.scale()
    }

    pub(crate) fn exponent(&self) -> i32 {
        self.exponent
    }

    pub fn from_f64(number: f64, uncertainty: f64) -> Option<Self> {
        Some(Self::new(
            Decimal::from_f64(number)?,
            Decimal::from_f64(uncertainty)?,
        ))
    }

    pub(crate) fn relative_uncertainty_dec(&self) -> Decimal {
        self.uncertainty_dec() / self.number
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

    pub fn powf(self, rhs: f64) -> Self {
        let rhs = Self::from_f64(rhs, 0.0).unwrap();
        self.pow_with_correlation(rhs, Decimal::ZERO)
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
        impl From<$T> for Num256 {
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

impl PartialEq for SciNum {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for SciNum {}

impl PartialOrd for SciNum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SciNum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

macro_rules! impl_comparisons {
    ($t:ty) => {
        impl PartialEq<$t> for Num256 {
            fn eq(&self, other: &$t) -> bool {
                self.number == Decimal::from(*other)
            }
        }

        impl PartialOrd<$t> for Num256 {
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

impl Add for SciNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Add for &SciNum {
    type Output = SciNum;

    fn add(self, rhs: Self) -> SciNum {
        self.add_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Sub for SciNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Sub for &SciNum {
    type Output = SciNum;

    fn sub(self, rhs: Self) -> SciNum {
        self.sub_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Mul for SciNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mul_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Mul for &SciNum {
    type Output = SciNum;

    fn mul(self, rhs: Self) -> SciNum {
        self.mul_with_correlation(*rhs, Decimal::ZERO)
    }
}

impl Div for SciNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.div_with_correlation(rhs, Decimal::ZERO)
    }
}

impl Div for &SciNum {
    type Output = SciNum;

    fn div(self, rhs: Self) -> SciNum {
        self.div_with_correlation(*rhs, Decimal::ZERO)
    }
}

macro_rules! impl_arithmetic {
    ($t:ty) => {
        impl Add<$t> for Num256 {
            type Output = Self;

            fn add(self, rhs: $t) -> Number {
                self.add_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Add<Num256> for $t {
            type Output = Num256;

            fn add(self, rhs: Num256) -> Num256 {
                let num: Num256 = self.into();
                num.add_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Sub<$t> for Num256 {
            type Output = Self;

            fn sub(self, rhs: $t) -> Number {
                self.sub_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Sub<Num256> for $t {
            type Output = Num256;

            fn sub(self, rhs: Num256) -> Num256 {
                let num: Num256 = self.into();
                num.sub_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Mul<$t> for Num256 {
            type Output = Self;

            fn mul(self, rhs: $t) -> Number {
                self.mul_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Mul<Num256> for $t {
            type Output = Num256;

            fn mul(self, rhs: Num256) -> Num256 {
                let num: Num256 = self.into();
                num.mul_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Div<$t> for Num256 {
            type Output = Self;

            fn div(self, rhs: $t) -> Number {
                self.div_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Div<Num256> for $t {
            type Output = Num256;

            fn div(self, rhs: Num256) -> Num256 {
                let num: Num256 = self.into();
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

impl fmt::Display for SciNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.uncertainty == Decimal::ZERO {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{}+/-{}", self.number, self.uncertainty)
        }
    }
}

impl Number {
    pub const ZERO: SciNum = SciNum {
        number: Decimal::ZERO,
        uncertainty: Decimal::ZERO,
    };

    pub const ONE: SciNum = SciNum {
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
        let n = SciNum::new(20, 2);
        assert_eq!(n.relative_uncertainty(), dec!(0.1));

        let n2 = SciNum::new(500, 5);
        assert_eq!(n2.relative_uncertainty(), dec!(0.01));

        let n3 = SciNum::new(1000, 15);
        assert_eq!(n3.relative_uncertainty(), dec!(0.015));
    }

    #[test]
    fn addition() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 + n2;
        assert_eq!(result.number, dec!(50));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn addition_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 + n2;
        assert_eq!(result.number, dec!(50));
    }

    #[test]
    fn subtraction() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 - n2;
        assert_eq!(result.number, dec!(-10));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn subtraction_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 - n2;
        assert_eq!(result.number, dec!(-10));
    }

    #[test]
    fn multiplication() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 * n2;
        assert_eq!(result.number, dec!(600));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(116.619037896906).round_dp(5)
        );
    }

    #[test]
    fn multiplication_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 * n2;
        assert_eq!(result.number, dec!(600));
    }

    #[test]
    fn division() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 / n2;
        assert_eq!(result.number.round_dp(10), dec!(0.6666666667).round_dp(10));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.129576708774340).round_dp(5)
        );
    }

    #[test]
    fn division_with_int() {
        let n1 = SciNum::new(60, 0);
        let n2 = 30;
        let result: SciNum = n1 / n2;
        assert_eq!(result.number, dec!(2));
    }

    #[test]
    fn division_reversed() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n2 / n1;
        assert_eq!(result.number, dec!(1.5));
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.2915475947422).round_dp(5)
        );
    }

    #[test]
    fn exponentiation() {
        let n1 = SciNum::new(20, 2);

        let result = n1.powd(dec!(2));
        assert_eq!(result.number, dec!(400));
        assert_eq!(result.uncertainty, dec!(80));

        let result = n1.powi(2);
        assert_eq!(result.number, dec!(400));
        assert_eq!(result.uncertainty, dec!(80));
    }

    #[test]
    fn natural_log() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.ln();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.194365063161).round_dp(5)
        );
    }

    #[test]
    fn log_base10() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.log10();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.08441167440582).round_dp(5)
        );
    }

    #[test]
    fn exponential() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.exp();
        assert_eq!(
            result.uncertainty.round_dp(5),
            dec!(0.25238096660761).round_dp(5)
        );
    }
}
