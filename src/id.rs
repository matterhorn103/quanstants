use crate::dimensions::DimensionalWord;

// A UnitId consists of two 64-bit parts:
//   1. A 64-bit number in a custom format corresponding roughly to scientific notation
//   2. A 64-bit representation of the dimensions of the unit
// The numeric component is defined such that all zeroes for the first half of the ID does not
// indicate a factor of 0 but of 1 and therefore all coherent SI units are contained within the
// first 64 bits

// TODO proper hashing

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct NumericWord(u64);

impl NumericWord {
    pub fn exponent(&self) -> i8 {
        (self.0 & 0xFF) as i8
    }

    pub fn base(&self) -> u8 {
        let raw_base = ((self.0 >> 8) & 0x7F) as u8;
        if raw_base == 0 { 10 } else { raw_base }
    }

    pub fn sign(&self) -> u8 {
        ((self.0 >> 15) & 0x01) as u8
    }

    pub fn mantissa(&self) -> u64 {
        (self.0 >> 16) + 1
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnitId {
    pub num: NumericWord,
    pub dim: DimensionalWord,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypedUnitId {
    Base(UnitId),
    Unitless(UnitId),
    Derived(UnitId),
}
