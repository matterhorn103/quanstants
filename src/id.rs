// A UnitId consists of two 64-bit parts:
//   1. An IEEE 754 decimal64 value indicating a numerical factor
//   2. A 64-bit unit representation
// All zeroes for the first half of the ID does not indicate a factor of 0 but of 1 

// TODO proper hashing

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct UnitId {
    factor: u64,
    unit: u64,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TypedUnitId {
    Base(UnitId),
    Unitless(UnitId),
    Derived(UnitId),
}
