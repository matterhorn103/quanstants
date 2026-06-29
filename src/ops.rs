// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementations of mathematical operations for the main components of
//! `quanstants` ([`SciDecimal`], [`Unit`], and [`Quantity`]) with each other as well as
//! with the foreign types valid for those operations (`isize`, `f64`,
//! `Decimal`, and `String`). Operations between a type and itself are
//! implemented in the type's own file.
//!
//! Multiplication and division operations are implemented for the following:
//!
//! P * U -> U
//!
//! N */ U -> Q
//! U */ N -> Q
//!
//! N */ Q -> Q
//! Q */ N -> Q
//!
//! U */ Q -> Q
//! Q */ U -> Q
//!
//! where:
//! - N is either [`SciDecimal`] or [`SciFloat`]
//! - P is [`Prefix`]
//! - U is [`Unit`]
//! - Q is [`Quantity`]

use std::ops::{Div, Mul};

use num_traits::{Float, Inv, Num, Pow};
use scinum::{SciDecimal, SciFloat};

use crate::{
    fraction::Frac,
    prefix::Prefix,
    quantity::Quantity,
    unit::{LinearUnit, LinearUnitType, Unit},
    unit128::Unit128,
};

// P * U -> U
// Adding a prefix to an existing unprefixed unit to create a prefixed one

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
        let new_id = if self.is_binary() && rhs.id.is_coherent() {
            // Only use a binary exponent if there is no (decimal) factor currently
            Unit128::new_with_binary_prefix(self.equivalent_power() / 3, rhs.id.dimensions())
        } else {
            let dimensions = rhs.id.dimensions();
            let factor = rhs.id.factor() * self.value();
            Unit128::new(factor, dimensions)
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

// Quantity creation by multiplication or division between a unit and a number
// Numerical type `N` creates a `Quantity<N>`
// N */ U -> Q

macro_rules! impl_mul_div_with_unit {
    ($n:ty) => {
        impl Mul<Unit> for $n {
            type Output = Quantity<$n>;

            fn mul(self, rhs: Unit) -> Quantity<$n> {
                Quantity::new(self, rhs)
            }
        }

        impl Div<Unit> for $n {
            type Output = Quantity<$n>;

            fn div(self, rhs: Unit) -> Quantity<$n> {
                Quantity::new(self, rhs.inverse())
            }
        }
    };
}

// Don't implement for things like integers, which make very little sense as `N`
impl_mul_div_with_unit!(f32);
impl_mul_div_with_unit!(f64);
impl_mul_div_with_unit!(SciDecimal);
impl_mul_div_with_unit!(SciFloat);

// U */ N -> Q

impl<N: Num> Mul<N> for Unit {
    type Output = Quantity<N>;

    fn mul(self, rhs: N) -> Quantity<N> {
        Quantity::new(rhs, self)
    }
}

impl<N: Num + Inv<Output = N>> Div<N> for Unit {
    type Output = Quantity<N>;

    fn div(self, rhs: N) -> Quantity<N> {
        Quantity::new(rhs.inv(), self)
    }
}

// Arithmetic between the inner numeric type of a `SciNum` type and
// corresponding quantities
// N */ Q -> Q

// Can't do blanket implementation
macro_rules! impl_mul_div_with_sci_quant {
    ($n:ty) => {
        impl Mul<Quantity<$n>> for $n {
            type Output = Quantity<$n>;

            fn mul(self, rhs: Quantity<$n>) -> Quantity<$n> {
                Quantity::new(self * rhs.number, rhs.unit)
            }
        }

        impl Div<Quantity<$n>> for $n {
            type Output = Quantity<$n>;

            fn div(self, rhs: Quantity<$n>) -> Quantity<$n> {
                Quantity::new(self / rhs.number, rhs.unit.inverse())
            }
        }
    };
}

impl_mul_div_with_sci_quant!(SciDecimal);
impl_mul_div_with_sci_quant!(SciFloat);

// Q */ N -> Q

impl<N: Num> Mul<N> for Quantity<N> {
    type Output = Quantity<N>;

    fn mul(self, rhs: N) -> Quantity<N> {
        Quantity::new(self.number * rhs, self.unit)
    }
}

impl<N: Num + Inv<Output = N>> Div<N> for Quantity<N> {
    type Output = Quantity<N>;

    fn div(self, rhs: N) -> Quantity<N> {
        Quantity::new(self.number / rhs, self.unit)
    }
}

// U */ Q -> Q

impl<N: Num> Mul<Quantity<N>> for Unit {
    type Output = Quantity<N>;

    fn mul(self, rhs: Quantity<N>) -> Quantity<N> {
        Quantity::new(rhs.number, self * rhs.unit)
    }
}

impl<N: Num + Inv<Output = N>> Div<Quantity<N>> for Unit {
    type Output = Quantity<N>;

    fn div(self, rhs: Quantity<N>) -> Quantity<N> {
        Quantity::new(rhs.number.inv(), self / rhs.unit)
    }
}

// Q */ U -> Q

impl<N: Num> Mul<Unit> for Quantity<N> {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit * rhs)
    }
}

impl<N: Num> Div<Unit> for Quantity<N> {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit / rhs)
    }
}

// Other assorted mixed arithmetic

impl Pow<Frac> for SciDecimal {
    type Output = SciDecimal;

    fn pow(self, rhs: Frac) -> SciDecimal {
        self.pow(&rhs)
    }
}

impl Pow<&Frac> for SciDecimal {
    type Output = SciDecimal;

    fn pow(self, rhs: &Frac) -> SciDecimal {
        if rhs.is_integer() {
            self.powi(rhs.to_integer().into())
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix() {
        let millimetre = Prefix::milli * Unit::metre();
        assert_eq!(millimetre.id, Unit128(0xFD_00_00_00_00_00_01_00_00));
        let kilometre = Prefix::kilo * Unit::metre();
        assert_eq!(kilometre.id, Unit128(0x03_00_00_00_00_00_01_00_00));
        let kibisecond = Prefix::kibi * Unit::second();
        assert_eq!(
            kibisecond.id,
            Unit128(0x80000000_00000003_00_00_00_00_00_00_01_00)
        );
    }
}
