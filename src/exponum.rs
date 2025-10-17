use std::ops::{Div, Mul};

use rust_decimal::{Decimal, MathematicalOps};

use crate::{error::QuanstantsError, fraction::Frac, number::Number};

/// A non-zero number in exponential notation (equivalent to scientific notation when _b_ = 10).
/// 
/// Has the form _s_ _m_ _b_<sup><i>e</i></sup>, where:
/// - _s_ is +1 or −1
/// - _m_ is a positive non-zero integer between 1 and 2<sup>48</sup>
/// - _b_ is a positive non-zero integer between 1 and (2<sup>7</sup> − 1)
/// - _e_ is a signed integer
/// 
/// Though the in-memory representation uses 128 bits (for now), the restrictions allow the number
/// to be encoded within 64 bits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExponentialNumber {
    pub sign: i8,      // +1 for positive numbers, -1 for negative
    pub mantissa: u64, // m - 1 must fit into 48 bits i.e. a u48 (m up to 2^48, not 2^48 - 1)
    pub base: u8,      // base must fit into 7 bits i.e. a u7 (up to 2^7 - 1)
    pub exponent: i8,
}

impl ExponentialNumber {
    const MAX_MANTISSA: u64 = (1 << 48);
    const MAX_BASE: u8 = (1 << 7) - 1;

    pub fn new(sign: i8, mantissa: u64, base: u8, exponent: i8) -> Self {
        Self::try_new(sign, mantissa, base, exponent).unwrap()
    }

    pub fn try_new(
        sign: i8,
        mantissa: u64,
        base: u8,
        exponent: i8,
    ) -> Result<Self, QuanstantsError> {
        if (mantissa <= Self::MAX_MANTISSA) && (base <= Self::MAX_BASE) {
            Ok(Self {
                sign: (sign >> 7) | 1,
                mantissa,
                base,
                exponent,
            })
        } else {
            Err(QuanstantsError::Range)
        }
    }

    pub fn try_pow<T: Into<Frac>>(self, exponent: T) -> Result<Self, QuanstantsError> {
        let exp = exponent.into().to_f64();
        let dec: Decimal = self.try_into()?;
        let new_dec = dec.checked_powf(exp).ok_or(QuanstantsError::Overflow)?;
        Self::try_from(new_dec)
    }
}

impl TryFrom<Number> for ExponentialNumber {
    type Error = QuanstantsError;

    fn try_from(n: Number) -> Result<Self, QuanstantsError> {
        n.number.try_into()
    }
}

impl TryFrom<Decimal> for ExponentialNumber {
    type Error = QuanstantsError;

    fn try_from(n: Decimal) -> Result<Self, QuanstantsError> {
        Ok(ExponentialNumber::new(
            if n.is_sign_positive() { 1 } else { -1 },
            n.mantissa().unsigned_abs() as u64,
            10,
            n.scale().try_into()?,
        ))
    }
}

impl TryFrom<ExponentialNumber> for i128 {
    type Error = QuanstantsError;

    fn try_from(n: ExponentialNumber) -> Result<i128, QuanstantsError> {
        if n.exponent.is_positive() {
            // The NumericFactor can be expressed as an integer
            let b: i128 = n.base.into();
            let e: u32 = n
                .exponent
                .try_into()
                .expect("Already checked that exponent is positive");
            let exponential_term = b.checked_pow(e).ok_or(QuanstantsError::Overflow)?;
            let m: i128 = if n.sign.is_positive() {
                n.mantissa as i128
            } else {
                -(n.mantissa as i128)
            }; // We know this will be fine
               // Even if the exponential term fit into an i128, might overflow when multiplied by m
            m.checked_mul(exponential_term)
                .ok_or(QuanstantsError::Overflow)
        } else {
            // Not an int
            Err(QuanstantsError::Cast)
        }
    }
}

impl TryFrom<ExponentialNumber> for Decimal {
    type Error = QuanstantsError;

    fn try_from(n: ExponentialNumber) -> Result<Decimal, QuanstantsError> {
        if n.exponent.is_positive() {
            // The NumericFactor can be expressed as an integer, so let's do so
            let m: i128 = n.try_into()?;
            match Decimal::try_from_i128_with_scale(m, 0) {
                Ok(result) => Ok(result),
                Err(_) => Err(QuanstantsError::Cast),
            }
        } else if n.base != 10 {
            Err(QuanstantsError::Cast)
        } else {
            let e: u32 = n.exponent.unsigned_abs().into();
            let m: i128 = if n.sign.is_positive() {
                n.mantissa as i128
            } else {
                -(n.mantissa as i128)
            };
            match Decimal::try_from_i128_with_scale(m, e) {
                Ok(result) => Ok(result),
                Err(_) => Err(QuanstantsError::Cast),
            }
        }
    }
}

impl Mul for ExponentialNumber {
    type Output = Self;

    // At the moment, panics if the bases are different (obviously not ideal)
    fn mul(self, rhs: Self) -> Self {
        if self.base != rhs.base {
            panic!()
        } else {
            ExponentialNumber {
                sign: self.sign * rhs.sign,
                mantissa: self.mantissa * rhs.mantissa,
                base: self.base,
                exponent: self.exponent + rhs.exponent,
            }
        }
    }
}

impl Div for ExponentialNumber {
    type Output = Self;

    // At the moment, panics if the bases are different (obviously not ideal)
    // Also loses precision in the mantissa!
    fn div(self, rhs: Self) -> Self {
        if self.base != rhs.base {
            panic!()
        } else {
            ExponentialNumber {
                sign: self.sign * rhs.sign,
                mantissa: self.mantissa / rhs.mantissa,
                base: self.base,
                exponent: self.exponent - rhs.exponent,
            }
        }
    }
}

impl ExponentialNumber {
    #[allow(dead_code)]
    pub const ONE: ExponentialNumber = {
        ExponentialNumber {
            sign: 1,
            mantissa: 1,
            base: 10,
            exponent: 0,
        }
    };
}
