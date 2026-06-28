// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::error::QuanstantsError;

/// A signed 7-bit integer.
///
/// Stored as the bit array in a `u8` with zero-padding i.e. the most
/// significant bit is always 0.
///
/// As such, the representation of negative numbers differs from the equivalent
/// `i8`, which would be sign-extended rather than zero-extended.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(non_camel_case_types)]
pub(crate) struct i7(u8);

impl i7 {
    pub(crate) const MAX: i8 = 63;
    pub(crate) const MIN: i8 = -64;

    /// Creates a new `i7` from an equivalent array of bits. The most
    /// significant bit is simply zeroed, so both zero- and sign-extended
    /// representations are treated the same.
    pub(crate) fn from_byte(b: u8) -> Self {
        Self(b & 0x7F)
    }

    /// Returns a zero-extended representation of the number.
    pub(crate) fn as_byte(self) -> u8 {
        self.0
    }

    /// Casts a (pre-validated) `i8` to the equivalent `i7`.
    ///
    /// This function relies on the `i8` being already known to fit
    /// into an `i7` (i.e. it is in the range `-63..=64`).
    pub(crate) fn from_i8_unchecked(n: i8) -> Self {
        Self(n as u8)
    }

    /// Casts a (pre-validated) `i16` to the equivalent `i7`.
    ///
    /// This function relies on the `i16` being already known to fit
    /// into an `i7` (i.e. it is in the range `-63..=64`).
    pub(crate) fn from_i16_unchecked(n: i16) -> Self {
        // Can't just cast to `i8` and then `u8`, need to move the sign bits
        let n = n as u16;
        let sign = ((n & 0b1000000000000000) >> 9) as u8; // Sign bit 15 moved to bit 6
        let value = (n & 0b0011_1111) as u8; // Only bits 5–0
        Self(sign | value)
    }
}

impl TryFrom<i8> for i7 {
    type Error = QuanstantsError;

    /// Attempts to convert an `i8` to an `i7`, failing if `n` is too large to be represented.
    fn try_from(n: i8) -> Result<Self, Self::Error> {
        if n >= i7::MIN && n <= i7::MAX {
            Ok(Self::from_i8_unchecked(n))
        } else {
            Err(QuanstantsError::Cast)
        }
    }
}

impl TryFrom<i16> for i7 {
    type Error = QuanstantsError;

    /// Attempts to convert an `i16` to an `i7`, failing if `n` is too large to be represented.
    fn try_from(n: i16) -> Result<Self, Self::Error> {
        if n >= (i7::MIN as i16) && n <= (i7::MAX as i16) {
            Ok(Self::from_i16_unchecked(n))
        } else {
            Err(QuanstantsError::Cast)
        }
    }
}

impl From<i7> for i8 {
    /// Casts to the equivalent `i8` (sign-extending).
    fn from(n: i7) -> Self {
        // Sign extend manually to get `i8`
        let sign = (n.0 & 0b01000000) << 1;
        (sign | n.0) as i8
    }
}

impl From<i7> for i16 {
    /// Casts to the equivalent `i16` (sign-extending).
    fn from(n: i7) -> Self {
        // Cast from `i8`
        i8::from(n) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i16_to_i7() {
        // Max possible value for an i7 is 63, min value is -64
        // Positive numbers first, should be straightforward
        assert_eq!(i7::from_i16_unchecked(0_i16), i7(0_u8));
        assert_eq!(i7::from_i16_unchecked(1_i16), i7(1_u8));
        assert_eq!(i7::from_i16_unchecked(1_i16), i7(0b0000001));
        assert_eq!(i7::from_i16_unchecked(63_i16), i7(63_u8));
        assert_eq!(i7::from_i16_unchecked(63_i16), i7(0b0111111));
        // Generate negative i7s for testing using two's complement and then
        // zeroing bit 7 (to reflect overflow)
        // Don't have to worry about actual overflow of the u8 since the value
        // is always too low for it to occur
        assert_eq!(i7::from_i16_unchecked(-1_i16), i7((!1_u8 + 1) & 0b01111111));
        assert_eq!(i7::from_i16_unchecked(-1_i16), i7(0b1111111));
        assert_eq!(i7::from_i16_unchecked(-2_i16), i7((!2_u8 + 1) & 0b01111111));
        assert_eq!(i7::from_i16_unchecked(-2_i16), i7(0b1111110));
        assert_eq!(
            i7::from_i16_unchecked(-63_i16),
            i7((!63_u8 + 1) & 0b01111111)
        );
        assert_eq!(i7::from_i16_unchecked(-63_i16), i7(0b1000001));
        assert_eq!(
            i7::from_i16_unchecked(-64_i16),
            i7((!64_u8 + 1) & 0b01111111)
        );
        assert_eq!(i7::from_i16_unchecked(-64_i16), i7(0b1000000));
    }

    #[test]
    fn test_i7_to_i16() {
        // Max possible value for an i7 is 63, min value is -64
        // Positive numbers first, should be straightforward
        assert_eq!(i16::from(i7(0_u8)), 0_i16);
        assert_eq!(i16::from(i7(1_u8)), 1_i16);
        assert_eq!(i16::from(i7(0b0000001)), 1_i16);
        assert_eq!(i16::from(i7(2_u8)), 2_i16);
        assert_eq!(i16::from(i7(63_u8)), 63_i16);
        assert_eq!(i16::from(i7(0b0111111)), 63_i16);
        assert_eq!(i16::from(i7(0b1111111)), -1_i16);
        assert_eq!(i16::from(i7(0x7F)), -1_i16);
        assert_eq!(i16::from(i7(0b1111110)), -2_i16);
        assert_eq!(i16::from(i7(0x7E)), -2_i16);
        assert_eq!(i16::from(i7(0b1000001)), -63_i16);
        assert_eq!(i16::from(i7(0b1000000)), -64_i16);
    }
}
