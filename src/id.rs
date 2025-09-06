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
