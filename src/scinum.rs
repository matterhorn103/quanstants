// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::{
    fmt::{self, Debug},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use num_traits::FromPrimitive;
use regex::Regex;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

use crate::{error::QuanstantsError, fraction::Frac};

/// A decimal float in scientific notation with an associated uncertainty.
///
/// Represents a number of the form _m_ × 10<sup><i>n</i></sup>
///
/// Essentially a `Decimal` from `rust_decimal` extended to have an uncertainty.
///
/// A `SciNum` also contains an associated exponent, which is the exponent for an
/// additional scaling factor of 10<sup><i>exponent</i></sup>, which applies to both the number
/// and uncertainty.
/// For now, the scaling factor exponent must always be 0, so the range of representable values is
/// exactly the same as `rust_decimal::Decimal`.
#[derive(Copy, Clone, serde_with::DeserializeFromStr, serde_with::SerializeDisplay)]
pub struct SciNum {
    negative: bool,
    number_scale: u8,
    number_lo: u32,
    number_mid: u32,
    number_hi: u32,
    exponent: i16,
    uncertainty_scale: u8,
    uncertainty_lo: u32,
    uncertainty_mid: u32,
    uncertainty_hi: u32,
}

impl SciNum {
    /// Creates a `SciNum` from any `Decimal`-compatible type.
    pub fn new<T>(number: T, uncertainty: T) -> Self
    where
        T: Into<Decimal>,
    {
        let number: Decimal = number.into();
        let uncertainty: Decimal = uncertainty.into();
        // Make sure uncertainty is on same scale as number
        //uncertainty.rescale(number.scale());
        let number = number.unpack();
        let uncertainty = uncertainty.unpack();
        Self {
            negative: number.negative,
            number_scale: number.scale as u8,
            number_lo: number.lo,
            number_mid: number.mid,
            number_hi: number.hi,
            exponent: 0,
            uncertainty_scale: uncertainty.scale as u8,
            uncertainty_lo: uncertainty.lo,
            uncertainty_mid: uncertainty.mid,
            uncertainty_hi: uncertainty.hi,
        }
    }

    /// Creates a `SciNum` with an uncertainty of zero from any `Decimal`-compatible type.
    pub fn new_exact<T>(number: T) -> Self
    where
        T: Into<Decimal>,
    {
        let number: Decimal = number.into();
        let number = number.unpack();
        Self {
            negative: number.negative,
            number_scale: number.scale as u8,
            number_lo: number.lo,
            number_mid: number.mid,
            number_hi: number.hi,
            exponent: 0,
            uncertainty_scale: 0,
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
        }
    }

    /// Creates an exact `SciNum` from parts corresponding to _m_ × 10<sup><i>n</i></sup>.
    ///
    /// Currently, this will panic if the exponent is large or small enough to cause the overall
    /// number to exceed `Decimal::MAX`.
    pub fn exact_from_scientific_parts<T>(significand: T, exponent: i16) -> Self
    where
        T: Into<Decimal>,
    {
        let significand: Decimal = significand.into();
        if exponent == 0 {
            Self::new_exact(significand)
        } else if exponent.is_positive() {
            Self::new_exact(significand * Decimal::from(10_u32.pow(exponent as u32)))
        } else {
            Self::new_exact(significand / Decimal::from(10_u32.pow(exponent.unsigned_abs() as u32)))
        }
    }

    /// Creates a new `SciNum` with the same number but the provided uncertainty.
    ///
    /// Currently panics if the current `SciNum` and the uncertainty have different values for
    /// `exponent`.
    pub fn with_uncertainty(mut self, uncertainty: Self) -> Self {
        if self.exponent != uncertainty.exponent {
            panic!()
        };
        self.uncertainty_scale = uncertainty.number_scale;
        self.uncertainty_lo = uncertainty.number_lo;
        self.uncertainty_mid = uncertainty.number_mid;
        self.uncertainty_hi = uncertainty.number_hi;
        self
    }

    /// Returns the number as an exact `SciNum` without its uncertainty.
    #[inline]
    pub fn number(&self) -> Self {
        Self {
            negative: self.negative,
            number_scale: self.number_scale,
            number_lo: self.number_lo,
            number_mid: self.number_mid,
            number_hi: self.number_hi,
            exponent: self.exponent,
            uncertainty_scale: 0,
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
        }
    }

    /// Returns the number as a `Decimal` without its uncertainty.
    #[inline]
    pub(crate) fn number_dec(&self) -> Decimal {
        Decimal::from_parts(
            self.number_lo,
            self.number_mid,
            self.number_hi,
            self.negative,
            self.number_scale as u32,
        )
    }

    /// Returns the absolute uncertainty as an exact `SciNum`.
    /// The uncertainty is always positive.
    #[inline]
    pub fn uncertainty(&self) -> Self {
        Self {
            negative: false,
            number_scale: self.number_scale,
            number_lo: self.uncertainty_lo,
            number_mid: self.uncertainty_mid,
            number_hi: self.uncertainty_hi,
            exponent: self.exponent,
            uncertainty_scale: 0,
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
        }
    }

    /// Returns the uncertainty as a `Decimal`.
    /// The uncertainty is always positive.
    #[inline]
    pub(crate) fn uncertainty_dec(&self) -> Decimal {
        Decimal::from_parts(
            self.uncertainty_lo,
            self.uncertainty_mid,
            self.uncertainty_hi,
            false,
            self.uncertainty_scale as u32,
        )
    }

    /// Returns the relative uncertainty as a `Decimal`.
    ///
    /// The relative uncertainty is always positive.
    #[inline]
    pub(crate) fn relative_uncertainty_dec(&self) -> Decimal {
        self.uncertainty_dec() / self.number_dec().abs()
    }

    /// Returns the significand _m_ of the number when represented with _m_ as an integer.
    ///
    /// Corresponds to representation of the number as `mmmmm × 10^nn`.
    #[inline]
    pub fn significand_integral(&self) -> i128 {
        let unsigned = (self.number_hi as u128) << 64
            | (self.number_mid as u128) << 32
            | self.number_lo as u128;
        if self.negative {
            -(unsigned as i128)
        } else {
            unsigned as i128
        }
    }

    /// Returns the exponent _n_ of the number when represented with _m_ as an integer.
    ///
    /// Corresponds to representation of the number as `mmmmm × 10^nn`.
    #[inline]
    pub fn exponent_integral(&self) -> i16 {
        self.exponent - (i16::from(self.number_scale))
    }

    /// Returns the significand _m_ of the number when represented with normalized notation
    /// i.e. with 10 > _m_ >= 1.
    ///
    /// Corresponds to `iffff` when the number is notated as `i.ffff × 10^nn`.
    #[inline]
    pub fn significand_normalized(&self) -> i128 {
        let unsigned = (self.number_hi as i128) << 64
            | (self.number_mid as i128) << 32
            | self.number_lo as i128;
        if self.negative {
            -unsigned
        } else {
            unsigned
        }
    }

    /// Returns a tuple of the integer, fractional, and exponent parts of the significand _m_ of the
    /// number when represented with normalized notation i.e. with 10 > _m_ >= 1.
    ///
    /// Corresponds to `(i, ffff, nn)` when the number is notated as `i.ffff × 10^nn`.
    #[inline]
    pub fn scientific_parts_normalized_split(&self) -> (i8, Option<i128>, i16) {
        if self.is_zero() {
            return (0, None, 0);
        }
        let significand = self.significand_integral();
        // Work out the number of places the decimal point needs to move to the left in the
        // significand to get the correct representation
        let (shifted_places, divisor, exponent) = if self.number_dec().abs() < Decimal::ONE {
            // For small numbers decimal already provides us with the scale
            let shifted_places = significand.abs().ilog10() as i16;
            let divisor = 10_i128.pow(shifted_places as u32);
            let exponent = -(self.number_scale as i16) + shifted_places;
            (shifted_places, divisor, exponent)
        } else {
            // For large integers Decimal's scale is 0 so we take the base 10 logarithm
            let shifted_places = (significand.abs().ilog10() as i16) + (self.number_scale as i16);
            let divisor = 10_i128.pow(shifted_places as u32);
            let exponent = self.exponent_integral() + shifted_places;
            (shifted_places, divisor, exponent)
        };
        let int_part = significand / divisor;
        let frac_part = significand.abs() % divisor;
        let exp_part = exponent;

        dbg!(self);
        dbg!(significand);
        dbg!(self.number_scale);
        dbg!(shifted_places);
        dbg!(divisor);
        dbg!(int_part);
        dbg!(frac_part);
        dbg!(exp_part);
        println!("{int_part}.{frac_part}e{exp_part}");

        (int_part as i8, Some(frac_part), exp_part)
    }

    /// Returns the exponent _n_ of the number when represented with normalized notation
    /// i.e. with 10 > _m_ >= 1.
    ///
    /// Corresponds to `nn` when the number is notated as `i.ffff × 10^nn`.
    #[inline]
    pub fn exponent_normalized(&self) -> i16 {
        todo!()
    }

    /// Returns the number of significant decimal digits in the significand.
    /// 0 is considered to have 0 significant figures.
    #[inline]
    pub fn sigfigs(&self) -> u32 {
        // This might not be the same thing
        let significand = self.significand_integral();
        if significand == 0 {
            0
        } else {
            significand.abs().ilog10() + 1
        }
    }

    /// Returns the scale of the last significant place.
    ///
    /// For example:
    /// - 0.02 returns -2
    /// - 0.020 returns -3
    /// - 2 returns 0
    /// - 200 returns 2 or 1 or 0, depending on the precision of the number
    #[inline]
    pub fn precision(&self) -> i32 {
        // For now, the exponent is guaranteed to be zero, so equal to the scale of the decimal rep
        -(i32::from(self.number_scale))
    }

    /// Returns true if the `SciNum` has an uncertainty of zero.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.uncertainty_lo | self.uncertainty_mid | self.uncertainty_hi == 0
    }

    /// Returns true if the `SciNum` is equal to zero, regardless of any uncertainty.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.number_lo | self.number_mid | self.number_hi == 0
    }

    /// Returns true if the sign bit is negative.
    /// Zero is considered positive.
    #[inline(always)]
    //#[must_use]
    pub const fn is_sign_negative(&self) -> bool {
        self.negative
    }

    /// Returns true if the sign bit is positive.
    /// Zero is also considered positive.
    #[inline(always)]
    //#[must_use]
    pub const fn is_sign_positive(&self) -> bool {
        !self.negative
    }

    /// Creates a `SciNum` from floats via `Decimal::from_f64()`.
    pub fn from_f64(number: f64, uncertainty: f64) -> Option<Self> {
        Some(Self::new(
            Decimal::from_f64(number)?,
            Decimal::from_f64(uncertainty)?,
        ))
    }

    /// Creates an exact `SciNum` from floats via `Decimal::from_f64()`.
    pub fn from_f64_exact(number: f64) -> Option<Self> {
        Some(Self::new_exact(
            Decimal::from_f64(number)?,
        ))
    }

    pub fn add_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty_dec() * rhs.uncertainty_dec();
        let number = self.number_dec() + rhs.number_dec();
        let uncertainty = if self.is_exact() && rhs.is_exact() {
            Decimal::ZERO
        } else {
            ((self.uncertainty_dec().powu(2))
                + (rhs.uncertainty_dec().powu(2))
                + (dec!(2) * sigma_ab))
                .sqrt()
                .unwrap()
        };
        Self::new(number, uncertainty)
    }

    pub fn sub_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty_dec() * rhs.uncertainty_dec();
        let number = self.number_dec() - rhs.number_dec();
        let uncertainty = if self.is_exact() && rhs.is_exact() {
            Decimal::ZERO
        } else {
            ((self.uncertainty_dec().powu(2)) + (rhs.uncertainty_dec().powu(2))
                - (dec!(2) * sigma_ab))
                .sqrt()
                .unwrap()
        };
        Self::new(number, uncertainty)
    }

    pub fn mul_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty_dec() * rhs.uncertainty_dec();
        let number = self.number_dec() * rhs.number_dec();
        let uncertainty = if self.is_exact() && rhs.is_exact() {
            Decimal::ZERO
        } else {
            ((self.relative_uncertainty_dec().powu(2))
                + (rhs.relative_uncertainty_dec().powu(2))
                + (dec!(2) * sigma_ab / number))
                .sqrt()
                .unwrap()
                * number.abs()
        };
        Self::new(number, uncertainty)
    }

    pub fn div_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty_dec() * rhs.uncertainty_dec();
        let number = self.number_dec() / rhs.number_dec();
        let uncertainty = if self.is_exact() && rhs.is_exact() {
            Decimal::ZERO
        } else {
            ((self.relative_uncertainty_dec().powu(2)) + (rhs.relative_uncertainty_dec().powu(2))
                - (dec!(2) * sigma_ab / number))
                .sqrt()
                .unwrap()
                * number.abs()
        };
        Self::new(number, uncertainty)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        Self::ONE / self
    }

    #[inline]
    pub fn powi(self, rhs: i64) -> Self {
        self.powd(rhs.into())
    }

    #[inline]
    pub fn powd(self, rhs: Decimal) -> Self {
        self.pow_with_correlation(rhs.into(), Decimal::ZERO)
    }

    #[inline]
    pub fn powf(self, rhs: f64) -> Self {
        let rhs = Self::from_f64(rhs, 0.0).unwrap();
        self.pow_with_correlation(rhs, Decimal::ZERO)
    }

    #[inline]
    pub fn powfrac(self, rhs: Frac) -> Self {
        let n: Decimal = (*rhs.numer()).into();
        let d: Decimal = (*rhs.denom()).into();
        let rhs = n / d;
        self.powd(rhs)
    }

    pub fn pow_with_correlation<T>(self, rhs: Self, correlation: T) -> Self
    where
        T: Into<Decimal>,
    {
        let sigma_ab = correlation.into() * self.uncertainty_dec() * rhs.uncertainty_dec();
        let number = self.number_dec().powd(rhs.number_dec());
        let uncertainty = if self.is_exact() && rhs.is_exact() {
            Decimal::ZERO
        } else {
            ((self.relative_uncertainty_dec() * rhs.number_dec()).powu(2)
                + (self.number_dec().ln() * rhs.uncertainty_dec()).powu(2)
                + (dec!(2)
                    * ((self.number_dec().ln() * rhs.number_dec()) / self.number_dec())
                    * sigma_ab))
                .sqrt()
                .unwrap()
                * number.abs()
        };
        Self::new(number, uncertainty)
    }

    pub fn ln(self) -> Self {
        let number = self.number_dec().ln();
        let uncertainty = self.relative_uncertainty_dec().abs();
        Self::new(number, uncertainty)
    }

    pub fn log10(self) -> Self {
        let number = self.number_dec().log10();
        let uncertainty = (self.uncertainty_dec() / (Decimal::TEN.ln() * self.number_dec())).abs();
        Self::new(number, uncertainty)
    }

    pub fn exp(self) -> Self {
        let number = self.number_dec().exp();
        let uncertainty = number.abs() * self.uncertainty_dec();
        Self::new(number, uncertainty)
    }
}

impl From<Decimal> for SciNum {
    #[inline]
    fn from(n: Decimal) -> Self {
        let n = n.unpack();
        Self {
            negative: n.negative,
            number_scale: n.scale as u8,
            number_lo: n.lo,
            number_mid: n.mid,
            number_hi: n.hi,
            exponent: 0,
            uncertainty_scale: 0,
            uncertainty_lo: 0,
            uncertainty_mid: 0,
            uncertainty_hi: 0,
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
        impl From<$T> for SciNum {
            fn from(t: $T) -> Self {
                Self::new_exact(t)
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
        self.number_dec() == other.number_dec()
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
        self.number_dec().cmp(&other.number_dec())
    }
}

macro_rules! impl_comparisons {
    ($t:ty) => {
        impl PartialEq<$t> for SciNum {
            fn eq(&self, other: &$t) -> bool {
                self.number_dec() == Decimal::from(*other)
            }
        }

        impl PartialOrd<$t> for SciNum {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                self.number_dec().partial_cmp(&Decimal::from(*other))
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
impl_comparisons!(Decimal);

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
        impl Add<$t> for SciNum {
            type Output = SciNum;

            fn add(self, rhs: $t) -> SciNum {
                self.add_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Add<SciNum> for $t {
            type Output = SciNum;

            fn add(self, rhs: SciNum) -> SciNum {
                let num: SciNum = self.into();
                num.add_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Sub<$t> for SciNum {
            type Output = Self;

            fn sub(self, rhs: $t) -> SciNum {
                self.sub_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Sub<SciNum> for $t {
            type Output = SciNum;

            fn sub(self, rhs: SciNum) -> SciNum {
                let num: SciNum = self.into();
                num.sub_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Mul<$t> for SciNum {
            type Output = Self;

            fn mul(self, rhs: $t) -> SciNum {
                self.mul_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Mul<SciNum> for $t {
            type Output = SciNum;

            fn mul(self, rhs: SciNum) -> SciNum {
                let num: SciNum = self.into();
                num.mul_with_correlation(rhs, Decimal::ZERO)
            }
        }

        impl Div<$t> for SciNum {
            type Output = Self;

            fn div(self, rhs: $t) -> SciNum {
                self.div_with_correlation(rhs.into(), Decimal::ZERO)
            }
        }

        impl Div<SciNum> for $t {
            type Output = SciNum;

            fn div(self, rhs: SciNum) -> SciNum {
                let num: SciNum = self.into();
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

impl Debug for SciNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SciNum")
            .field("number", &self.number_dec())
            .field("uncertainty", &self.uncertainty_dec())
            .finish()
    }
}

impl fmt::Display for SciNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_exact() {
            // Up to five decimal places, display normally
            if (self < &dec!(1e6)) && (self >= &dec!(1e-5)) {
                write!(f, "{}", self.number_dec())
            // Otherwise, use scientific notation
            } else {
                let (int, frac, exp) = self.scientific_parts_normalized_split();
                // Fractional part might not have any places at all (e.g. 2e6)
                let frac_string = match frac {
                    Some(n) => n.to_string(),
                    None => String::new(),
                };
                write!(f, "{int}.{frac_string}e{exp}")
            }
        } else {
            // TODO Need to add support for ASCII +/-
            // Default representation should be parentheses notation in future though
            write!(f, "{}±{}", self.number(), self.uncertainty())
        }
    }
}

impl FromStr for SciNum {
    type Err = QuanstantsError;

    /// Parses a string and attempts to create a corresponding `SciNum`.
    /// Does not currently support uncertainties.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(-?\d+(?:[.,]\d+)?)(?:[eE]([+-]?\d+))?$").unwrap();
        let caps = re.captures(s).ok_or(QuanstantsError::Parse)?;
        let number_str = caps.get(1).ok_or(QuanstantsError::Parse)?.as_str();
        let number = Decimal::from_str(number_str).map_err(|_e| QuanstantsError::Parse)?;
        let exponent_str = caps.get(2).map(|m| m.as_str()).unwrap_or("0");
        let exponent = i16::from_str(exponent_str).map_err(|_e| QuanstantsError::Parse)?;
        Ok(Self::exact_from_scientific_parts(number, exponent))
    }
}

impl SciNum {
    /// A constant representing 0.
    pub const ZERO: SciNum = SciNum {
        negative: false,
        number_scale: 0,
        number_lo: 0,
        number_mid: 0,
        number_hi: 0,
        exponent: 0,
        uncertainty_scale: 0,
        uncertainty_lo: 0,
        uncertainty_mid: 0,
        uncertainty_hi: 0,
    };

    /// A constant representing 1.
    pub const ONE: SciNum = SciNum {
        negative: false,
        number_scale: 0,
        number_lo: 1,
        number_mid: 0,
        number_hi: 0,
        exponent: 0,
        uncertainty_scale: 0,
        uncertainty_lo: 0,
        uncertainty_mid: 0,
        uncertainty_hi: 0,
    };

    /// The largest supported number.
    ///
    /// Identical to Decimal::MAX for the time being, until SciNum supports non-zero exponents.
    pub const MAX: SciNum = SciNum {
        negative: false,
        number_scale: 0,
        number_lo: 4_294_967_295,
        number_mid: 4_294_967_295,
        number_hi: 4_294_967_295,
        exponent: 0,
        uncertainty_scale: 0,
        uncertainty_lo: 0,
        uncertainty_mid: 0,
        uncertainty_hi: 0,
    };

    /// The smallest supported number.
    ///
    /// Identical to Decimal::MIN for the time being, until SciNum supports non-zero exponents.
    pub const MIN: SciNum = SciNum {
        negative: true,
        number_scale: 0,
        number_lo: 4_294_967_295,
        number_mid: 4_294_967_295,
        number_hi: 4_294_967_295,
        exponent: 0,
        uncertainty_scale: 0,
        uncertainty_lo: 0,
        uncertainty_mid: 0,
        uncertainty_hi: 0,
    };
}

// Constants taken from rust_decimal
#[allow(dead_code)]
pub mod dec {
    // Sign mask for the flags field. A value of zero in this bit indicates a
    // positive Decimal value, and a value of one in this bit indicates a
    // negative Decimal value.
    pub(crate) const SIGN_MASK: u32 = 0x8000_0000;
    pub(crate) const UNSIGN_MASK: u32 = 0x4FFF_FFFF;

    // Scale mask for the flags field. This byte in the flags field contains
    // the power of 10 to divide the Decimal value by. The scale byte must
    // contain a value between 0 and 28 inclusive.
    pub const SCALE_MASK: u32 = 0x00FF_0000;
    pub const U8_MASK: u32 = 0x0000_00FF;
    pub const U32_MASK: u64 = u32::MAX as _;

    // Number of bits scale is shifted by.
    pub const SCALE_SHIFT: u32 = 16;
    // Number of bits sign is shifted by.
    pub const SIGN_SHIFT: u32 = 31;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new_from_int() {
        let n = SciNum::new(20, 2);
        assert_eq!(n.number(), SciNum::new(20, 0));
        assert_eq!(n.uncertainty(), SciNum::new(2, 0));
    }

    #[test]
    fn new_exact_from_int() {
        let n = SciNum::new_exact(30);
        assert_eq!(n.number(), SciNum::new_exact(30));
        assert_eq!(n.uncertainty(), SciNum::new_exact(0));
    }

    #[test]
    fn new_from_dec() {
        let n = SciNum::new(dec!(30), dec!(5));
        assert_eq!(n.number(), SciNum::new(dec!(30), dec!(0)));
        assert_eq!(n.uncertainty(), SciNum::new(dec!(5), dec!(0)));
    }

    #[test]
    fn new_exact_from_dec() {
        let n = SciNum::new_exact(dec!(20));
        assert_eq!(n.number(), SciNum::new_exact(dec!(20)));
        assert_eq!(n.uncertainty(), SciNum::new_exact(dec!(0)));
    }

    #[test]
    fn exact_from_scientific_parts() {
        let n = SciNum::exact_from_scientific_parts(67, 0);
        assert_eq!(n, SciNum::new_exact(dec!(67)));
        let n2 = SciNum::exact_from_scientific_parts(236, 3);
        assert_eq!(n2, SciNum::new_exact(dec!(2.36e5)));
        let n3 = SciNum::exact_from_scientific_parts(236, -6);
        assert_eq!(n3, SciNum::new_exact(dec!(2.36e-4)));
    }

    #[test]
    #[should_panic] // For now, not supported yet
    fn exact_from_scientific_parts_large() {
        let _n = SciNum::exact_from_scientific_parts(236, 40);
    }

    #[test]
    #[should_panic] // For now, not supported
    fn exact_from_scientific_parts_small() {
        let _n = SciNum::exact_from_scientific_parts(49, -76);
    }

    #[test]
    fn num_dec() {
        let n = SciNum::new(20, 2);
        assert_eq!(n.number_dec(), dec!(20));
    }

    #[test]
    fn uncert_dec() {
        let n = SciNum::new(30, 5);
        assert_eq!(n.uncertainty_dec(), dec!(5));
    }

    #[test]
    fn relative_uncertainty() {
        let n = SciNum::new(20, 2);
        assert_eq!(n.relative_uncertainty_dec(), dec!(0.1));

        let n2 = SciNum::new(500, 5);
        assert_eq!(n2.relative_uncertainty_dec(), dec!(0.01));

        let n3 = SciNum::new(1000, 15);
        assert_eq!(n3.relative_uncertainty_dec(), dec!(0.015));
    }

    #[test]
    fn sigfigs() {
        let n = SciNum::new_exact(dec!(123.45));
        assert_eq!(n.sigfigs(), 5);

        let n2 = SciNum::new_exact(dec!(0.00123));
        assert_eq!(n2.sigfigs(), 3);

        let n3 = SciNum::new_exact(dec!(1234));
        assert_eq!(n3.sigfigs(), 4);
    }

    #[test]
    fn sigfigs_trailing_zeros() {
        let n = SciNum::new_exact(dec!(123.4500));
        assert_eq!(n.sigfigs(), 7);

        let n2 = SciNum::new_exact(dec!(0.001230));
        assert_eq!(n2.sigfigs(), 4);

        let n3 = SciNum::new_exact(dec!(1230));
        assert_eq!(n3.sigfigs(), 4);
    }

    #[test]
    fn precision() {
        assert_eq!(SciNum::new_exact(dec!(0.02)).precision(), -2);
        assert_eq!(SciNum::new_exact(dec!(0.020)).precision(), -3);
        assert_eq!(SciNum::new_exact(dec!(2)).precision(), 0);
        //assert_eq!(SciNum::new_exact(dec!(2e3)).precision(), 3); // Fails for now
    }

    #[test]
    fn is_exact() {
        let n1 = SciNum::new_exact(dec!(45.1));
        let n2 = SciNum::new(500, 5);
        assert!(n1.is_exact());
        assert!(!n2.is_exact());
    }

    #[test]
    fn addition_fn_exact() {
        let n1 = SciNum::new_exact(40);
        let n2 = SciNum::new_exact(dec!(5.1));
        let result = n1.add_with_correlation(n2, 0);
        assert_eq!(result.number_dec(), dec!(45.1));
    }

    #[test]
    fn addition_fn() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1.add_with_correlation(n2, 0);
        assert_eq!(result.number_dec(), dec!(50));
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn addition_op() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 + n2;
        assert_eq!(result.number_dec(), dec!(50));
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn addition_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 + n2;
        assert_eq!(result.number_dec(), dec!(50));
    }

    #[test]
    fn subtraction() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 - n2;
        assert_eq!(result.number_dec(), dec!(-10));
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(5.3851648071345).round_dp(5)
        );
    }

    #[test]
    fn subtraction_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 - n2;
        assert_eq!(result.number_dec(), dec!(-10));
    }

    #[test]
    fn multiplication() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 * n2;
        assert_eq!(result.number_dec(), dec!(600));
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(116.619037896906).round_dp(5)
        );
        let ft = SciNum::new_exact(dec!(0.3048));
        let square_ft = ft * ft;
        assert_eq!(square_ft.number_dec(), dec!(0.09290304));
    }

    #[test]
    fn multiplication_with_int() {
        let n1 = SciNum::new(20, 0);
        let n2 = 30;
        let result: SciNum = n1 * n2;
        assert_eq!(result.number_dec(), dec!(600));
    }

    #[test]
    fn division() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n1 / n2;
        assert_eq!(
            result.number_dec().round_dp(10),
            dec!(0.6666666667).round_dp(10)
        );
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(0.129576708774340).round_dp(5)
        );
    }

    #[test]
    fn division_with_int() {
        let n1 = SciNum::new(60, 0);
        let n2 = 30;
        let result: SciNum = n1 / n2;
        assert_eq!(result.number_dec(), dec!(2));
    }

    #[test]
    fn division_reversed() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let result = n2 / n1;
        assert_eq!(result.number_dec(), dec!(1.5));
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
            dec!(0.2915475947422).round_dp(5)
        );
    }

    #[test]
    fn exponentiation() {
        let n1 = SciNum::new(20, 2);

        let result = n1.powd(dec!(2));
        assert_eq!(result.number_dec(), dec!(400));
        assert_eq!(result.uncertainty_dec(), dec!(80));

        let result = n1.powi(2);
        assert_eq!(result.number_dec(), dec!(400));
        assert_eq!(result.uncertainty_dec(), dec!(80));
    }

    #[test]
    fn natural_log() {
        let n1 = SciNum::new(20, 2);
        let n2 = SciNum::new(30, 5);
        let ratio = n1 / n2;
        let result = ratio.ln();
        assert_eq!(
            result.uncertainty_dec().round_dp(5),
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
            result.uncertainty_dec().round_dp(5),
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
            result.uncertainty_dec().round_dp(5),
            dec!(0.25238096660761).round_dp(5)
        );
    }

    #[test]
    fn debug() {
        let n = SciNum::new(20, 2);
        assert_eq!(format!("{n:?}"), "SciNum { number: 20, uncertainty: 2 }");
    }

    #[test]
    fn display() {
        // Small integers display normally
        assert_eq!(SciNum::new_exact(20).to_string(), "20");
        // Up to 5 places displays normally
        assert_eq!(SciNum::new_exact(99999).to_string(), "99999");
        assert_eq!(SciNum::new_exact(dec!(0.00001)).to_string(), "0.00001");
        // Above 6 places uses scientific notation
        assert_eq!(SciNum::new_exact(1295891).to_string(), "1.295891e6");
        assert_eq!(SciNum::new_exact(dec!(0.000000432)).to_string(), "4.32e-7");
        // Explicit zeros should be treated as significant
        assert_eq!(SciNum::new_exact(1295800).to_string(), "1.295800e6");
        // Here they shouldn't be but the problem is that Decimal does treat them as significant...
        //assert_eq!(SciNum::new_exact(dec!(1.2958e6)).to_string(), "1.2958e6");
        // Check uncertainty formatting
        assert_eq!(SciNum::new(20, 2).to_string(), "20±2");
        // TODO: More uncertainty display tests
    }

    #[test]
    fn from_str() {
        // Integer
        assert_eq!(SciNum::from_str("42").unwrap(), SciNum::new_exact(dec!(42)));
        // Negative float
        assert_eq!(
            SciNum::from_str("-3.14").unwrap(),
            SciNum::new_exact(dec!(-3.14))
        );
        // Scientific notation
        assert_eq!(
            SciNum::from_str("1.5e8").unwrap(),
            SciNum::new_exact(dec!(1.5e8))
        );
        // TODO large exponent fails with overflow error
        //assert_eq!(SciNum::from_str("1.5e10").unwrap(), SciNum::new_exact(dec!(1.5e10)));
        // Scientific notation with negative exponent
        assert_eq!(
            SciNum::from_str("2e-5").unwrap(),
            SciNum::new_exact(dec!(2e-5))
        );
        // Negative number with positive exponent
        assert_eq!(
            SciNum::from_str("-6.022e6").unwrap(),
            SciNum::new_exact(dec!(-6.022e6))
        );
        // TODO large exponent fails with overflow error
        //assert_eq!(SciNum::from_str("-6.022e23").unwrap(), SciNum::new_exact(dec!(-6.022e23)));
        // Capital E for exponent
        assert_eq!(
            SciNum::from_str("1.5E8").unwrap(),
            SciNum::new_exact(dec!(1.5E8))
        );
        // Make sure incorrectly formatted string fails
        assert!(SciNum::from_str("not a number").is_err());
    }
}
