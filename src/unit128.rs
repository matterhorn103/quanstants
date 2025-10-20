use std::{
    fmt::{self, Debug},
    ops::{Div, Mul},
};

use rust_decimal::Decimal;

use crate::{dimensions::Dimensions, fraction::Frac, scinum::SciNum};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Unit128 {
    pub(crate) num: u64,
    pub(crate) dim: u64,
}

impl Unit128 {
    pub fn new(factor: SciNum, dimensions: Dimensions, least_significant_byte: u8) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = Unit128::factor_to_bits(factor);
        Self { num, dim }
    }

    pub fn new_referenced(
        factor: SciNum,
        dimensions: Dimensions,
        least_significant_byte: u8,
        reference: SciNum,
    ) -> Self {
        let dim = least_significant_byte as u64
            | (dimensions.T.to_bits() as u64) << 8
            | (dimensions.L.to_bits() as u64) << 16
            | (dimensions.M.to_bits() as u64) << 24
            | (dimensions.I.to_bits() as u64) << 32
            | (dimensions.Θ.to_bits() as u64) << 40
            | (dimensions.N.to_bits() as u64) << 48
            | (dimensions.J.to_bits() as u64) << 56;
        let num = Unit128::factor_and_reference_to_bits(factor, reference);
        Self { num, dim }
    }

    pub fn is_referenced(&self) -> bool {
        // Tried to be efficient but logic is incorrect
        //((self.dim & 0b10000000) == 0b10000000) // 0xA* to 0xF* are for other systems entirely
        //|| ((self.dim & 0xF0) == 0) // 0x0* is for normal linear units

        // Just keep it simple for now
        (0x10..=0x9F).contains(&self.least_significant_byte())
    }

    pub fn least_significant_byte(&self) -> u8 {
        (self.dim & 0xFF) as u8
    }

    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            T: Frac::from_bits(((self.dim >> 8) & 0xFF) as u8),
            L: Frac::from_bits(((self.dim >> 16) & 0xFF) as u8),
            M: Frac::from_bits(((self.dim >> 24) & 0xFF) as u8),
            I: Frac::from_bits(((self.dim >> 32) & 0xFF) as u8),
            Θ: Frac::from_bits(((self.dim >> 40) & 0xFF) as u8),
            N: Frac::from_bits(((self.dim >> 48) & 0xFF) as u8),
            J: Frac::from_bits(((self.dim >> 56) & 0xFF) as u8),
        }
    }

    pub fn factor(&self) -> SciNum {
        if self.is_referenced() {
            Unit128::bits_to_factor_and_reference(self.num).0
        } else {
            Unit128::bits_to_factor(self.num)
        }
    }

    pub fn reference_exponent(&self) -> Option<i8> {
        if self.is_referenced() {
            Some(((self.num & 0x000000FF00000000) >> 16) as i8)
        } else {
            None
        }
    }

    pub fn normalize(self) -> Self {
        // Will need to normalize the number as well I guess
        Self {
            num: self.num,
            dim: (self.dim & 0xFFFFFFFFFFFFFFF0) | 0xA,
        }
    }

    pub fn from_bits(b: u128) -> Self {
        Self {
            num: (b >> 64) as u64,
            dim: (b & 0x0000000000000000FFFFFFFFFFFFFFFF) as u64,
        }
    }

    pub fn to_bits(self: Unit128) -> u128 {
        (self.num as u128) << 64 | self.dim as u128
    }

    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Unit128 {
        // Panics for referenced units
        let exp: Frac = exponent.into();
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor().powfrac(exp),
                self.dimensions().pow(exp),
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
        }
    }
}

impl Mul for Unit128 {
    type Output = Self;

    // Panics for referenced units
    fn mul(self, rhs: Unit128) -> Unit128 {
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor() * rhs.factor(),
                self.dimensions() * rhs.dimensions(),
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
        }
    }
}

impl Div for Unit128 {
    type Output = Self;

    // Panics for referenced units
    fn div(self, rhs: Unit128) -> Unit128 {
        if self.is_referenced() {
            panic!()
        } else {
            Unit128::new(
                self.factor() / rhs.factor(),
                self.dimensions() / rhs.dimensions(),
                (self.least_significant_byte() & 0xF0) | 0x0C, // Set as generic compound unit
            )
        }
    }
}

impl Debug for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit128 {{ num: {:X}, dim: {:X} }}", self.num, self.dim)
    }
}

impl fmt::Display for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.to_bits())
    }
}

impl Unit128 {
    pub(crate) fn factor_to_bits(factor: SciNum) -> u64 {
        let dec = factor.number_dec();
        // Need to find a way to shorten factors with too much precision
        // (This doesn't work)
        //let dec = dec.trunc_with_scale(14);
        if dec.scale() > 14 {
            panic!()
        } else {
            -(dec.scale() as i64) as u64 | ((dec.mantissa() - 1) as u64) << 8
        }
    }

    pub(crate) fn factor_and_reference_to_bits(factor: SciNum, reference: SciNum) -> u64 {
        let dec_factor = factor.number_dec();
        let dec_ref = reference.number_dec();
        if dec_factor.scale() > 6 || dec_ref.scale() > 6 {
            panic!()
        } else {
            dec_factor.scale() as u64
                | ((dec_factor.mantissa() - 1) as u64) << 8
                | (dec_ref.scale() as u64) << 32
                | (dec_ref.mantissa() as u64) << 40
        }
    }

    pub(crate) fn bits_to_factor(b: u64) -> SciNum {
        let scale = (b & 0x0000_0000_0000_00FF) as u32;
        let mantissa = ((b >> 8) as i128) + 1;
        Decimal::from_i128_with_scale(mantissa, scale).into()
    }

    pub(crate) fn bits_to_factor_and_reference(b: u64) -> (SciNum, SciNum) {
        let factor_scale = (b & 0x0000_0000_0000_00FF) as u32;
        let factor_mantissa = (((b & 0x0000_0000_FFFF_FF00) >> 8) as i128) + 1;
        let factor: SciNum = Decimal::from_i128_with_scale(factor_mantissa, factor_scale).into();
        let ref_scale = ((b & 0x0000_00FF_0000_0000) >> 32) as u32;
        let ref_mantissa = (b >> 40) as i128;
        let reference: SciNum = Decimal::from_i128_with_scale(ref_mantissa, ref_scale).into();
        (factor, reference)
    }
}

impl Unit128 {
    #[allow(dead_code)]
    pub const UNITLESS: Unit128 = { Unit128 { num: 0x0, dim: 0x0 } };

    pub const ONE: Unit128 = Unit128::UNITLESS;

    pub const SECOND: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100,
        }
    };

    pub const METRE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x110000,
        }
    };

    pub const KILOGRAM: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x11000000,
        }
    };

    pub const AMPERE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000001100000000,
        }
    };

    pub const KELVIN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000110000000000,
        }
    };

    pub const MOLE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0011000000000000,
        }
    };

    pub const CANDELA: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000000000,
        }
    };

    pub const GRAM: Unit128 = {
        Unit128 {
            num: 0xFD,
            dim: 0x1100000D,
        }
    };

    pub const RADIAN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000000000001,
        }
    };

    pub const STERADIAN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000000000002,
        }
    };

    pub const HERTZ: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000000F101, // s-1
        }
    };

    pub const NEWTON: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001111F201, // kg m s-2
        }
    };

    pub const PASCAL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000000011F1F201, // kg m-1 s-2
        }
    };

    pub const JOULE: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001112F201, // kg m2 s-2
        }
    };

    pub const WATT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000001112F301, // kg m2 s-3
        }
    };

    pub const COULOMB: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x0000001100001101, // s A
        }
    };

    pub const VOLT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11112F301, // kg m2 s-3 A-1
        }
    };

    pub const FARAD: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x00000012F1F21401, // kg-1 m-2 s4 A2
        }
    };

    pub const OHM: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F21112F301, // kg m2 s-3 A-2
        }
    };

    pub const SIEMENS: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x00000012F1F21301, // kg-1 m-2 s3 A2
        }
    };

    pub const WEBER: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11112F201, // kg m2 s-2 A-1
        }
    };

    pub const TESLA: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11100F201, // kg s-2 A-1
        }
    };

    pub const HENRY: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000F11112F201, // kg m2 s-2 A-1
        }
    };

    /// The referenced degree Celsius, for relative temperatures
    pub const DEGREE_CELSIUS: Unit128 = {
        Unit128 {
            num: 0x006AB3FE00000000,
            dim: 0x0000110000000041,
        }
    };

    pub const LUMEN: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000000001, // cd sr
        }
    };

    pub const LUX: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x1100000000F20001, // cd sr m-2
        }
    };

    pub const BECQUEREL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000000F102, // s-1
        }
    };

    pub const GRAY: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000012F201, // m2 s-2
        }
    };

    pub const SIEVERT: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000012F202, // m2 s-2
        }
    };

    pub const KATAL: Unit128 = {
        Unit128 {
            num: 0x0,
            dim: 0x000000000000F101, // mol s-1
        }
    };
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "UnitId")]
    pub struct PyUnitId(pub(crate) Unit128);

    #[pymethods]
    impl PyUnitId {
        #[new]
        fn new(id: u128) -> Self {
            PyUnitId(Unit128::from_bits(id))
        }

        fn __repr__(&self) -> String {
            format!("UnitId({})", self.0)
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }

        fn __eq__(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        #[classmethod]
        fn from_bits(_cls: &Bound<'_, PyType>, x: u128) -> PyResult<Self> {
            Ok(PyUnitId(Unit128::from_bits(x)))
        }

        fn to_bits(&self) -> u128 {
            self.0.to_bits()
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn new() {
        let s = Unit128::new(SciNum::ONE, Dimensions::TIME, 0x00);
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(s, Unit128::SECOND);
        assert_eq!(s.num, 0x0);
        assert_eq!(s.dim, 0x1100);
        assert_eq!(celsius.num, 0x006AB3FE00000000);
        assert_eq!(celsius.dim, 0x0000110000000041);
        assert_eq!(ft.num, 0xBE7FC);
        assert_eq!(ft.dim, 0x110001);
    }

    #[test]
    fn factor_to_bits() {
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1)), 0x0);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(2)), 0x100);
        //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(10)), 0x1); // Fails for now
        //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1000)), 0x3); // Fails for now
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(dec!(0.1))), 0xFF);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(dec!(1e-3))), 0xFD);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(-1)), 0xFFFFFFFFFFFFFE00);
        assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(-3)), 0xFFFFFFFFFFFFFC00);
    }

    #[test]
    fn factor() {
        assert_eq!(Unit128::KILOGRAM.factor(), SciNum::ONE);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.factor(), SciNum::new_exact(dec!(0.3048)));
    }

    #[test]
    fn dimensions() {
        assert_eq!(Unit128::KILOGRAM.dimensions(), Dimensions::MASS);
        assert_eq!(
            Unit128::KELVIN.dimensions(),
            Dimensions::THERMODYNAMIC_TEMPERATURE
        );
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.dimensions(), Dimensions::LENGTH);
    }

    #[test]
    fn lsb() {
        assert_eq!(Unit128::UNITLESS.least_significant_byte(), 0x00);
        assert_eq!(Unit128::SECOND.least_significant_byte(), 0x00);
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert_eq!(celsius.least_significant_byte(), 0x41);
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert_eq!(ft.least_significant_byte(), 0x01);
    }

    #[test]
    fn is_referenced() {
        assert!(!Unit128::UNITLESS.is_referenced());
        assert!(!Unit128::SECOND.is_referenced());
        let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
        assert!(celsius.is_referenced());
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        assert!(!ft.is_referenced());
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Unit128::SECOND),
            "Unit128 { num: 0, dim: 1100 }"
        );
    }

    #[test]
    fn mul() {
        let amp_second = Unit128::AMPERE * Unit128::SECOND;
        let square_metre = Unit128::METRE * Unit128::METRE;
        let ft = Unit128::new(SciNum::new_exact(dec!(0.3048)), Dimensions::LENGTH, 0x01);
        let square_foot = ft * ft;
        assert_eq!(amp_second.num, 0x0);
        assert_eq!(amp_second.dim, 0x110000110C);
        assert_eq!(square_metre.num, 0x0);
        assert_eq!(square_metre.dim, 0x12000C);
        assert_eq!(square_foot.num, 0xB138FFF8);
        assert_eq!(square_foot.dim, 0x12000C);
    }
}
