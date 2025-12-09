// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

//! Implementations of mathematical operations for the main components of
//! `quanstants` (`SciNum`, `Unit`, and `Quantity`) with each other as well as
//! with the foreign types valid for those operations (`isize`, `f64`,
//! `Decimal`, and `String`). Operations between a type and itself are
//! implemented in the type's own file.
//!
//! Multiplication and division operations are implemented for the following
//! (where N is a numeric type -- meaning `SciNum` or a type that implements
//! `Into<SciNum>` -- P is `Prefix`, U is `Unit`, and Q is `Quantity`):
//!
//! N */ U -> Q
//! U */ N -> Q
//!
//! P * U -> U
//!
//! N */ Q -> Q
//! Q */ N -> Q
//!
//! U */ Q -> Q
//! Q */ U -> Q

use std::ops::{Div, Mul};

use num_traits::{Inv, Num};
use rust_decimal::Decimal;

use crate::{
    prefix::Prefix,
    quantity::{Quantity, SciQuantity},
    scinum::SciNum,
    unit::{LinearUnit, LinearUnitType, Unit},
    unit128::Unit128,
};

// N */ U -> Q

macro_rules! impl_mul_div_with_unit {
    ($t:ty) => {
        impl Mul<Unit> for $t {
            type Output = Quantity<$t>;

            fn mul(self, rhs: Unit) -> Quantity<$t> {
                Quantity::new(self, rhs)
            }
        }

        impl Div<Unit> for $t {
            type Output = Quantity<$t>;

            fn div(self, rhs: Unit) -> Quantity<$t> {
                Quantity::new(self, rhs.inverse())
            }
        }
    };
}

impl_mul_div_with_unit!(SciNum);
impl_mul_div_with_unit!(i8);
impl_mul_div_with_unit!(i16);
impl_mul_div_with_unit!(i32);
impl_mul_div_with_unit!(i64);
impl_mul_div_with_unit!(i128);
impl_mul_div_with_unit!(isize);
impl_mul_div_with_unit!(u8);
impl_mul_div_with_unit!(u16);
impl_mul_div_with_unit!(u32);
impl_mul_div_with_unit!(u64);
impl_mul_div_with_unit!(u128);
impl_mul_div_with_unit!(usize);
impl_mul_div_with_unit!(Decimal);

// U */ N -> Q

impl<T: Num> Mul<T> for Unit {
    type Output = Quantity<T>;

    fn mul(self, rhs: T) -> Quantity<T> {
        Quantity::new(rhs, self)
    }
}

impl<T: Num + Inv<Output = T>> Div<T> for Unit {
    type Output = Quantity<T>;

    fn div(self, rhs: T) -> Quantity<T> {
        Quantity::new(rhs.inv(), self)
    }
}

// P * U -> U

impl Mul<Unit> for Prefix {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        if rhs.inner.prefix.is_some() {
            panic!("Cannot prefix an already prefixed unit!")
        }
        if matches!(
            rhs.inner.utype,
            LinearUnitType::One | LinearUnitType::Compound
        ) {
            panic!("Cannot prefix a compound unit or Unitless!")
        }
        let new_id = if self.is_binary() {
            // Only use a binary exponent if there is no (decimal) factor currently
            if rhs.id.num == 0 {
                // Set as unit with binary factor
                let dim = rhs.id.dim & !0xF | 0xB;
                let num = self.equivalent_power() as u8 as u64; // Go via u8 so that it gets padded with zeros
                Unit128 { num, dim }
            } else {
                // Set as non-unique derived unit
                let least_significant_byte: u8 = rhs.id.least_significant_byte() & !0x0F | 0x0D;
                let dimensions = rhs.id.dimensions();
                let factor = rhs.id.factor() * self.value();
                Unit128::new(factor, dimensions, least_significant_byte)
            }
        } else {
            // Set as non-unique derived unit
            let dim = rhs.id.dim & !0xF | 0xD;
            let old_exponent = rhs.id.num as u8 as i8; // Go via u8 so that it gets truncated
            // Increase/decrease decimal exponent appropriately
            let num = rhs.id.num & !0xF | ((old_exponent + self.equivalent_power()) as u8 as u64);
            Unit128 { num, dim }
        };
        Unit::new(LinearUnit {
            id: new_id,
            utype: LinearUnitType::Derived,
            dimensions: rhs.dimensions(),
            symbol: Some(self.symbol() + &rhs.symbol(false)),
            name: Some(self.name() + &rhs.name()),
            prefix: Some(self),
            number: rhs.number(),
            factors: rhs.to_factors(),
        })
    }
}

// N */ Q -> Q

macro_rules! impl_mul_div_with_sci_quant {
    ($t:ty) => {
        impl Mul<SciQuantity> for $t {
            type Output = SciQuantity;

            fn mul(self, rhs: SciQuantity) -> SciQuantity {
                Quantity::new(self * rhs.number, rhs.unit)
            }
        }

        impl Div<SciQuantity> for $t {
            type Output = SciQuantity;

            fn div(self, rhs: SciQuantity) -> SciQuantity {
                Quantity::new(self / rhs.number, rhs.unit.inverse())
            }
        }
    };
}

impl_mul_div_with_sci_quant!(SciNum);
impl_mul_div_with_sci_quant!(i8);
impl_mul_div_with_sci_quant!(i16);
impl_mul_div_with_sci_quant!(i32);
impl_mul_div_with_sci_quant!(i64);
impl_mul_div_with_sci_quant!(i128);
impl_mul_div_with_sci_quant!(isize);
impl_mul_div_with_sci_quant!(u8);
impl_mul_div_with_sci_quant!(u16);
impl_mul_div_with_sci_quant!(u32);
impl_mul_div_with_sci_quant!(u64);
impl_mul_div_with_sci_quant!(u128);
impl_mul_div_with_sci_quant!(usize);
//impl_mul_div_with_sci_quant!(Decimal);

// Q */ N -> Q

// for generic Q

impl<T: Num> Mul<T> for Quantity<T> {
    type Output = Quantity<T>;

    fn mul(self, rhs: T) -> Quantity<T> {
        Quantity::new(self.number * rhs, self.unit)
    }
}

impl<T: Num + Inv<Output = T>> Div<T> for Quantity<T> {
    type Output = Quantity<T>;

    fn div(self, rhs: T) -> Quantity<T> {
        Quantity::new(self.number / rhs, self.unit)
    }
}

// for SciQ

macro_rules! impl_mul_div_for_sci_quant {
    ($t:ty) => {
        impl Mul<$t> for SciQuantity {
            type Output = SciQuantity;

            fn mul(self, rhs: $t) -> SciQuantity {
                Quantity::new(self.number * rhs, self.unit)
            }
        }

        impl Div<$t> for SciQuantity {
            type Output = SciQuantity;

            fn div(self, rhs: $t) -> SciQuantity {
                Quantity::new(self.number / rhs, self.unit)
            }
        }
    };
}

impl_mul_div_for_sci_quant!(i8);
impl_mul_div_for_sci_quant!(i16);
impl_mul_div_for_sci_quant!(i32);
impl_mul_div_for_sci_quant!(i64);
impl_mul_div_for_sci_quant!(i128);
impl_mul_div_for_sci_quant!(isize);
impl_mul_div_for_sci_quant!(u8);
impl_mul_div_for_sci_quant!(u16);
impl_mul_div_for_sci_quant!(u32);
impl_mul_div_for_sci_quant!(u64);
impl_mul_div_for_sci_quant!(u128);
impl_mul_div_for_sci_quant!(usize);
//impl_mul_div_with_sci_quant!(Decimal);

// U */ Q -> Q

impl Mul<SciQuantity> for Unit {
    type Output = SciQuantity;

    fn mul(self, rhs: SciQuantity) -> SciQuantity {
        SciQuantity::new(rhs.number, self * rhs.unit)
    }
}

impl Div<SciQuantity> for Unit {
    type Output = SciQuantity;

    fn div(self, rhs: SciQuantity) -> SciQuantity {
        SciQuantity::new(rhs.number.inv(), self / rhs.unit)
    }
}

// Q */ U -> Q

impl Mul<Unit> for SciQuantity {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit * rhs)
    }
}

impl Div<Unit> for SciQuantity {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::context::Context;

    use super::*;

    #[test]
    fn prefix() {
        let ctx = Context::new();
        let millimetre = Prefix::milli * ctx.metre();
        assert_eq!(
            millimetre.id,
            Unit128 {
                num: 0xFD,
                dim: 0x11000D
            }
        );
        let kilometre = Prefix::kilo * ctx.metre();
        assert_eq!(
            kilometre.id,
            Unit128 {
                num: 0x03,
                dim: 0x11000D
            }
        );
        let kibisecond = Prefix::kibi * ctx.second();
        assert_eq!(
            kibisecond.id,
            Unit128 {
                num: 0x03,
                dim: 0x110B
            }
        );
    }
}
