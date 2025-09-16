// A UnitId consists of two 64-bit parts:
//   1. A 64-bit decimal in the format |mm|mm|mm|mm|mm|mm|s+b|ee|
//   2. A 64-bit unit representation
// All zeroes for the first half of the ID does not indicate a factor of 0 but of 1 

// TODO proper hashing

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NumericalFactor(u64);

impl NumericalFactor {
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

//impl From<u64> for DecimalFactor {
//    fn from(value: u64) -> Self {
//        // 8 least significant bits are the exponent
//        let exponent = (value & 0xFF) as i8;
//        // Next 7 bits are the base
//        let base = ((value >> 8) & 0x7F) as u8;
//        let base = if base == 0 { 10 } else { base };
//        // Next bit is the sign
//        let sign = ((value >> 15) & 0x01) as u8;
//        // Most significant 48 bits are the mantissa - 1
//        let mantissa = (value >> 16) + 1;
//        DecimalFactor { mantissa, sign, base, exponent }
//    }
//}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UnitId {
    factor: NumericalFactor,
    unit: u64,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypedUnitId {
    Base(UnitId),
    Unitless(UnitId),
    Derived(UnitId),
}
