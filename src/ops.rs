//! Implementations of mathematical operations for the main components of `quanstants` (`SciNum`,
//! `Unit`, and `Quantity`) with each other as well as with the foreign types valid for those
//! operations (`isize`, `f64`, `Decimal`, and `String`).
//! Operations between a type and itself are implemented in the type's own file.
//! 
//! Multiplication and division operations are implemented for the following (where N is a numeric
//! type -- meaning `SciNum` or a type that implements `Into<SciNum>` -- U is `Unit`, and Q is
//! `Quantity`):
//! 
//! N */ U -> Q
//! U */ N -> Q
//! 
//! N */ Q -> Q
//! Q */ N -> Q
//! 
//! U */ Q -> Q
//! Q */ U -> Q

use std::ops::{Div, Mul};

use rust_decimal::Decimal;

use crate::{quantity::Quantity, scinum::SciNum, unit::Unit};

// N */ U -> Q

macro_rules! impl_mul_div_with_unit {
    ($t:ty) => {
        impl Mul<Unit> for $t {
            type Output = Quantity;

            fn mul(self, rhs: Unit) -> Quantity {
                Quantity::new(self.into(), rhs)
            }
        }

        impl Div<Unit> for $t {
            type Output = Quantity;

            fn div(self, rhs: Unit) -> Quantity {
                Quantity::new(self.into(), rhs.inverse())
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

impl<T> Mul<T> for Unit
where
    T: Into<SciNum>
{
    type Output = Quantity;

    fn mul(self, rhs: T) -> Quantity {
        let num: SciNum = rhs.into();
        Quantity::new(num, self)
    }
}

impl<T> Div<T> for Unit
where
    T: Into<SciNum>
{
    type Output = Quantity;

    fn div(self, rhs: T) -> Quantity {
        let num: SciNum = rhs.into();
        Quantity::new(num.inverse(), self)
    }
}

// N */ Q -> Q

macro_rules! impl_mul_div_with_quantity {
    ($t:ty) => {
        impl Mul<Quantity> for $t {
            type Output = Quantity;

            fn mul(self, rhs: Quantity) -> Quantity {
                Quantity::new(self * rhs.number, rhs.unit)
            }
        }

        impl Div<Quantity> for $t {
            type Output = Quantity;

            fn div(self, rhs: Quantity) -> Quantity {
                Quantity::new(self / rhs.number, rhs.unit.inverse())
            }
        }
    };
}

impl_mul_div_with_quantity!(SciNum);
impl_mul_div_with_quantity!(i8);
impl_mul_div_with_quantity!(i16);
impl_mul_div_with_quantity!(i32);
impl_mul_div_with_quantity!(i64);
impl_mul_div_with_quantity!(i128);
impl_mul_div_with_quantity!(isize);
impl_mul_div_with_quantity!(u8);
impl_mul_div_with_quantity!(u16);
impl_mul_div_with_quantity!(u32);
impl_mul_div_with_quantity!(u64);
impl_mul_div_with_quantity!(u128);
impl_mul_div_with_quantity!(usize);
//impl_mul_div_with_quantity!(Decimal);

// Q */ N -> Q

impl<T> Mul<T> for Quantity
where
    T: Into<SciNum>
{
    type Output = Quantity;

    fn mul(self, rhs: T) -> Quantity {
        let num: SciNum = rhs.into();
        Quantity::new(self.number * num, self.unit)
    }
}

impl<T> Div<T> for Quantity
where
    T: Into<SciNum>
{
    type Output = Quantity;

    fn div(self, rhs: T) -> Quantity {
        let num: SciNum = rhs.into();
        Quantity::new(self.number / num, self.unit)
    }
}

// U */ Q -> Q

impl Mul<Quantity> for Unit {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(rhs.number, self * rhs.unit)
    }
}

impl Div<Quantity> for Unit {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(rhs.number.inverse(), self / rhs.unit)
    }
}

// Q */ U -> Q

impl Mul<Unit> for Quantity {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit * rhs)
    }
}

impl Div<Unit> for Quantity {
    type Output = Self;

    fn div(self, rhs: Unit) -> Self {
        Self::new(self.number, self.unit / rhs)
    }
}
