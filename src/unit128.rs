// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A "unit of measure ID", or "UoMID": a 128-bit encoding of the unit of a quantity,
//! designed for identification, interchange, and arithmetic.
//!
//! ## Design
//!
//! The choices made mean that UoMIDs of "normal" units (SI-compatible units with integer exponents
//! in the dimensional terms and on a linear scale rather than a logarithmic or temperature scale)
//! written out in hexadecimal can be broadly understood by a human,
//! and all SI base units and coherent derived units are in the range
//! `0x0` to `0xFFFFFFFFFFFFFFFF` with all zeros for the most significant 64 bits.
//!
//! ### Overall layout
//!
//! The bit layout of a UoMID is divided, from least to most significant, into:
//! - a **scheme component**, indicating how the rest of the UoMID should be interpreted
//! - a **dimensional component**, describing the exponent of each dimension term
//! - a **numeric component**, encoding one or two numbers that define the mathematical relation
//!   between a quantity in terms of the unit and a quantity in terms of the reference unit that the
//!   unit is defined by
//!
//! The **scheme component** is always specified by the least significant 8 bits (bits 7–0).
//!
//! The **dimensional** and **numeric** components have variable widths.
//!
//! For SI and SI-compatible units (the vast majority), in most cases the rest of the UoMID is
//! divided up as follows:
//! - The **dimensional component** comprises bits 63–8
//! - The **numeric component** comprises bits 127–64
//!
//! Sometimes, fractional exponents are necessary for the dimension terms, in which case:
//! - The **dimensional component** comprises an expanded range of bits 91–8
//! - The **numeric component** comprises only bits 127–92
//!
//! ### Scheme component
//!
//! The first byte (as in, bits 0–7) contains **flags**.
//!
//! - Width: 8 bits
//! - Layout: `|prrrsuuu|`
//! - Between them, these flags indicate:
//!     1. Unit system/compatibility
//!         - SI compatibility
//!         - Whether the unit is a catalogued, uniquely identifiable unit or
//!           simply a normalized representation in SI base units
//!     2. How to interpret the numeric component
//!         - Decimal vs binary numeric factor
//!         - Wide number vs two narrow numbers
//!         - Integer vs fractional exponents
//!     3. How to relate the unit to a quantity
//!         - Linear scales (normal units) vs non-linear scales (referenced units)
//!         - What equation the quantity obeys
//! - Bits 0–3 classify the unit (1. above).
//! - Bits 4–7 indicate the scheme for the rest of the UoMID (2. and 3. above).
//! - Bit 3 `s` indicates whether the unit is SI-compatible or not.
//!     - If `s == 0`:
//!         - The unit (and quantity) are compatible with the SI.
//!         - A value for `uuu` of a value of `0b000` indicates either that it is
//!           the base unit itself, or that the unit is not catalogued and cannot
//!           be uniquely identified, and so has the encoded value in SI base units.
//!         - Other values of `uuu` uniquely identify the unit as a specific catalogued unit.
//!     - If `s == 1`, the bits 0–2 indicate the alternative system that the unit belongs to.
//!       The way the other bits are interpreted is determined by this value. At present, which
//!       SI-incompatible system is represented by each possibility is not specified.
//!     - All 1s i.e. `0b1111` for `suuu` is a reserved value.
//!     - The upshot is: hex values of `0` to `7` for the least significant nibble are used for
//!       SI-compatible units, while `8` to `E` are for SI-incompatible units, with `F` reserved.
//!     - Note that non-SI units are not automatically SI-incompatible. A foot is not an SI unit,
//!       but it can be expressed in terms of SI units with no issue, and there are no problems
//!       with compatibility. CGS systems, however, *are* incompatible with the SI – naive
//!       conversion and arithmetic between CGS quantities and SI quantities is not possible.
//! - The way the rest of the UoMID should be interpreted is not yet defined for non-SI systems,
//!   and will in any case be specific to each alternative system. The rest of this documentation
//!   concerns itself only with SI-compatible units i.e. where bit 3 `s == 0`.
//! - Bits 6–4 `rrr` indicate the type of scale a quantity with the unit uses:
//!     - `0b000` indicates a normal linear scale; the numeric factor encodes a single number.
//!     - All other combinations indicate a specific non-linear scale, four of which are currently
//!       considered:
//!         - `0b001` -> logarithmic, base 10, *x* *κ* = (*y* × 10^(*k*⋅*x*)) *λ*
//!         - `0b010` -> logarithmic, base 2, *x* *β* = (*y* × 2^(*k*⋅*x*)) *λ*
//!         - `0b011` -> logarithmic, base *e*, *x* *ε* = (*y* × *e*^(*k*⋅*x*)) *λ*
//!         - `0b100` -> temperature, *x* *θ* = (*k*(*x* + *y*)) *λ*
//!     - See [`ScaleType`] for more details on the mathematical relationships.
//!     - The bit combinations are chosen to allow for mnemonics (for bits 7–4, with no fractional
//!       exponents,`0x1` indicates base 10, `0x2` base 2, `0x3` base *e*).
//!     - For temperature scales, the scale unit and the degree unit for a scale are related by a
//!       single bit flip (of bit 6).
//! - Bit 7 `p` is a flag that indicates whether an array of denominators for fractional exponents
//!   is present or not.
//!     - If `p == 1`, the least significant 28 bits of the numeric component are used to encode the
//!       denominators as 4-bit unsigned integers, and the fields of the numeric component are
//!       narrowed to compensate.
//! - UoMIDs in which the four least significant bits are all 1s are currently invalid, with those
//!   combinations reserved for special values.
//!     - For example, a least significant byte of `0xFF` might be used as a continuation byte
//!       should a variable-width encoding turn out to be necessary.
//!
//!
//! ### Dimensional component
//!
//! #### Without fractional dimensional exponents
//!
//! - Width: 56 bits
//! - Layout: `|JJJJJJJJJ|NNNNNNNN|ΘΘΘΘΘΘΘΘ|IIIIIIII|MMMMMMMM|LLLLLLLL|TTTTTTTT|`
//! - Bits 8–63 encode the exponents for each SI dimension with one byte per dimension,
//!   in the order shown above.
//! - Each byte is interpreted simply as a signed 8-bit integer `i8`.
//! - The commonly used integer exponents are thus easily memorized
//!   (¹, ², ³ are `0x01, `0x02`, `0x03` respectively; ⁻¹, ⁻², ⁻³ are `0xFF`, `0xFE`, `0xFD`)
//!   enabling manual comprehension and composition.
//!
//! #### With fractional dimensional exponents
//!
//! - Width: 84 bits
//! - Layout: `jjjj|nnnnθθθθ|iiiimmmm|lllltttt|JJJJJJJJJ|NNNNNNNN|ΘΘΘΘΘΘΘΘ|IIIIIIII|MMMMMMMM|LLLLLLLL|TTTTTTTT|`
//! - Bits 8–63 encode the numerators of the exponents for each SI dimension as signed 8-bit
//!   integers `i8`.
//! - Bits 91–64 encode the denominators of the exponents as unsigned 4-bit integers (i.e. `u4`)
//!
//!
//! ### Numeric component
//!
//! The numeric component may either be:
//! 1. A **simple numeric component**, encoding a single number
//! 2. A **referenced numeric component**, encoding two numbers
//!
//! In most cases the numeric component is simple, with referenced ones used for non-linear units
//! (logarithmic, temperature).
//!
//! The numbers are encoded as decimal floats in a BID fashion.
//! When the dimensional exponents do not need to be fractional and only a simple numeric component
//! needs to be encoded there is also the possibility of a binary-like float encoding.
//!
//! The layout of the numeric component is determined by the value of bits 7–4 in the scheme
//! component, and the layouts indicated by the possible values of that nibble are summarized below:
//!
//! | Bits 7–4 | Hex | Scale  | Bit width | Bit range | Sign bits | Binary bits | Exp. bits | Sig. bits | Sig. digits |
//! | -------- | --- | ------ | --------- | --------- | --------- | ----------- | --------- | --------- | ----------- |
//! | `0000`   | `0` | linear | 64        | 127–64    | 1         | 1           | 8         | 54        | 16          |
//! | `0001`   | `1` | log10  | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
//! | `0010`   | `2` | log2   | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
//! | `0011`   | `3` | ln     | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
//! | `0100`   | `4` | temp.  | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
//! | `0101`   | `5` |        |           |           |           |             |           |           |             |
//! | `0110`   | `6` |        |           |           |           |             |           |           |             |
//! | `0111`   | `7` |        |           |           |           |             |           |           |             |
//! | `1000`   | `8` | linear | 36        | 127–92    | 1         | 0           | 8         | 27        | 8           |
//! | `1001`   | `9` | log10  | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
//! | `1010`   | `A` | log2   | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
//! | `1011`   | `B` | ln     | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
//! | `1100`   | `C` | temp.  | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
//! | `1101`   | `D` |        |           |           |           |             |           |           |             |
//! | `1110`   | `E` |        |           |           |           |             |           |           |             |
//! | `1111`   | `F` |        |           |           |           |             |           |           |             |
//!
//! #### Simple numeric component
//!
//! ##### Without fractional dimensional exponents
//!
//! A 64-bit encoding of the proportionality factor, similar to both IEEE 754 floating point and
//! traditional scientific notation, with either a decimal or binary-like exponential factor.
//!
//! - Width: 64 bits
//! - Layout: `|wgaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|eeeeeeee|`
//! - `w` is a flag to indicate the use of the decimal (`0`) or the binary-like (`1`) encoding
//! - Encodes the proportionality factor *k* by:
//!     - Decimal encoding: *k* = (−1)^*g* (*a* + 1) × 10^*e*
//!     - Binary-like encoding: *k* = (−1)^*g* (*a* + 1) × 1024^(*e*/3)
//! - The bit pattern is much simpler than IEEE 754 `decimal64` but has the same precision (16 full
//!   decimal digits), achieved by reducing the exponent field to 8 bits (with the consequence that
//!   the range is somewhat reduced in comparison).
//! - `e` is the exponent, encoded (*with no bias*) by the least significant byte as an `i8`
//!     - Exponents can range from −128 to 127.
//!     - The base of the exponential term is indicated as decimal or binary by `w` as described.
//!     - Under the usual decimal scheme the value of the exponential term is simply 10^*e*.
//!     - Under the "binary-like" scheme the value of the exponential term is instead 1024^(*e*/3).
//!         - For traditional binary floating point it would be 2^*e*, hence "binary-like".
//!         - For a binary number the encoded `e` is thus actually `<exponent> * 3`; this is done so
//!           that analogous metric and binary prefixes are encoded by the same value/bit pattern
//!           e.g. kilo and kibi are both `0x03`.
//!         - The exponent in the binary form is thus constrained to multiples of 3.
//!         - This makes the maximum value (where `e == 126`) 1024^42 = 2^420 ≈ 10^126.
//!         - Thus, importantly, the range of the binary encoding falls within the range of the
//!           decimal encoding; this makes processing much easier.
//!         - This may seem too low, as `f64` has a max value of 1.80×10^308 and IEEE `decimal64`
//!           can go up to 1.0×10^385. However, the numeric component is only used to specify the
//!           value of units, not for the number of an actual quantity. The largest and smallest
//!           current SI prefixes are quetta = 10^30 and quecto = 10^−30 respectively, so the
//!           possible range allowed for by the design covers all realistically necessary factors of
//!           SI base units by some way.
//! - `a` encodes the significand as a 54-bit unsigned binary integer *with a bias of −1*.
//!     - Enables 16 full decimal digits of precision, matching IEEE `decimal64` and [`SciDecimal`],
//!       and enough to cover the maximum precision of `f64`.
//!     - The bias means that `a` is actually `<significand> - 1`.
//!     - The bias was chosen so that `+1` is encoded as `g = 0, a = 0` => 7 bytes of zeros.
//!     - The maximum value allowed for the significand is 10^16 − 1.
//!
//! A proportionality factor of +1 – the case for all SI base units, and products and combinations
//! thereof – has `w = 0, g = 0, a = 0, e = 0`, corresponding to 8 bytes of zeros, giving coherent
//! SI units nice short IDs.
//!
//! A proportionality factor of a power of 10, such as the decimal prefixes, have short and easily
//! understood encodings, for example:
//!
//! | Hex                | Value                                  |
//! | ------------------ | -------------------------------------- |
//! |               `00` | 1e0 = 1                                |
//! |              `100` | 2e0 = 2                                |
//! |               `01` | 1e1 = 10 (with 1 s.f.)                 |
//! |              `900` | 10e0 = 10 (with 2 s.f.)                |
//! |               `03` | 1e3 = 1000^1 = kilo                    |
//! |            `3E700` | 1000e0 = 1000 as well, but with 3 s.f. |
//! |               `06` | 1e6 = 1000^2 = mega                    |
//! |               `1E` | 1e30 = 1000^10 = quetta                |
//! |               `78` | 1e120 = ??!                            |
//! |               `7E` | 1e126 = maximum exponent               |
//! |               `FD` | 1e-3 = milli                           |
//! | `4000000000000000` | -1e0 = −1                              |
//! | `4000000000000100` | -2e0 = −2                              |
//! | `4000000000000003` | -1e3 = −1000                           |
//!
//! while the binary prefixes have encodings that match the corresponding decimal ones neatly, with
//! just a single bit flip (at bit 127):
//!
//! | Hex                | Value                          |
//! | ------------------ | ------------------------------ |
//! | `8000000000000003` | 1 × 1024 = kibi                |
//! | `8000000000000006` | 1 × 1024^2 = mebi              |
//! | `800000000000000C` | 1 × 1024^4 = tebi              |
//! | `800000000000001E` | 1 × 1024^10 = quebi            |
//! | `8000000000000078` | 1 × 1024^40 = ??!              |
//! | `800000000000007E` | 1 × 1024^42 = maximum exponent |
//! | `C000000000000003` | −1 × 1024^1 = −1024            |
//!
//! ##### With fractional dimensional exponents
//!
//! Broadly the same as the normal simple encoding, with the following differences:
//! - The bit width is reduced to 36 bits
//! - The significand is reduced to a bit width of 27 bits
//! - There is no binary flag bit `w` – binary numeric exponents are therefore not possible with
//!   anything other than a linear unit with integer dimensional exponents
//! - With 27 bits for the significand (which still uses a bias of −1), only 8 full decimal digits
//!   of precision is possible.
//! - The maximum value allowed for the significand is 10^8 − 1.
//!
//! - Width: 36 bits
//! - Layout: `|gaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaeeee|eeee`
//!
//! #### Referenced numeric component
//!
//! ##### Without fractional dimensional exponents
//!
//! Used to describe a non-linear or scale quantity: a quantity written as *x* *u*, where *x* is the
//! number and *u* is a non-linear unit, with the meaning that the quantity is a function
//! *Q*(*x*, *u*) = *u*(*x*), and *u* is a function of the form *f*(*x*, *y*, *k*, *λ*) where the
//! values of *y*, *k*, and *λ*  define the unit (see [`ScaleType`]).
//!
//! The applicable function *f* and therefore the appropriate interpretation of *y* and *k* is
//! indicated by the scheme component (see above).
//!
//! - Width: 64 bits
//! - Layout: `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
//! - Encoded in essentially the same way as the simple numeric component but as two 32-bit numbers.
//!     - The more significant half encodes *y* (the reference value of the scale),
//!       and the less significant half *k*.
//! - Neither number has a binary flag bit `w`; only decimal encoding is possible, and the base for
//!   the exponents is always 10.
//! - The exponents are reduced to a bit-width of 7 (with two's complement, making them effectively
//!   `i7`s)
//! - This allows 7 full decimal digits of precision in the significand,
//! - The four least significant bytes encode *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e*
//! - The four most significant bytes encode *y* as *y* = (−1)^*h* (*b* + 1) × 10^*f*
//! - With 24 bits for the significand (which still uses a bias of −1), 7 full decimal digits of
//!   precision is possible, matching IEEE 754's `decimal32` and similar to `f32`.
//! - The maximum value allowed for the significand is 10^7 − 1.
//!
//! ##### With fractional dimensional exponents
//!
//! Broadly the same as the normal referenced encoding, with the following differences:
//!     - The bit width of each number is reduced to 18 bits.
//!
//! - Width: 36 bits
//! - Layout: `hbbbbbbb|bbbfffff|ffgaaaaa|aaaaaeee|eeee`
//! - The exponents are, as before, encoded with 7 rather than 8 bits.
//! - With 10 remaining bits for the significand (which still uses a bias of −1), only 3 full
//!   decimal digits of precision is possible.
//! - The maximum value allowed for the significand is 999.

use std::{
    fmt,
    ops::{Div, Mul},
    str::FromStr,
};

use num_traits::{Float, Inv, Pow};
use scinum::{SciDecimal, SciNum};
use serde::{Deserialize, Serialize};

use crate::{dimensions::Dimensions, error::QuanstantsError, fraction::Frac, i7::i7};

/// The classification of a unit as SI-compatible or not, with a further distinction between
/// well-known catalogued units and unit definitions normalized in terms of SI base units.
///
/// Represents the information encoded by bits 3–0 of a UoMID (see [`Unit128`]).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SICompatibility {
    Normalized,
    Catalogued(u8),
    Incompatible(u8),
}

/// A 128-bit representation of a unit of measure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub(crate) struct Uomid(pub(crate) u128);

#[allow(dead_code)]
impl Uomid {
    /// Returns the value of bits 7–0.
    ///
    /// The scheme component contains information on:
    /// 1. SI compatibility
    /// 2. Anonymity/catalogue number
    /// 3. How the other 120 bits should be interpreted
    /// 4. How the encoded numbers relate the unit to base units mathematically
    #[inline]
    pub fn scheme_component(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns an enum indicating whether the encoded unit is SI compatible,
    /// and if so, whether the unit is anonymous/defined in base units or a catalogued unit.
    ///
    /// Note that non-SI units are not automatically SI-incompatible. A foot is not an SI unit,
    /// but it can be expressed in terms of SI units with no issue, and there are no problems
    /// with compatibility. CGS systems, however, *are* incompatible with the SI – naive
    /// conversion and arithmetic between CGS quantities and SI quantities is not possible.
    #[inline]
    pub fn si_compatibility(&self) -> SICompatibility {
        let compat_nibble = (self.0 & 0x0F) as u8;
        match compat_nibble {
            0 => SICompatibility::Normalized,
            1..8 => SICompatibility::Catalogued(compat_nibble),
            8..16 => SICompatibility::Incompatible(compat_nibble),
            16 => panic!("0x0F is reserved and should never occur!"),
            _ => unreachable!("Impossible for four bits to have a value greater than 16"),
        }
    }

    /// Returns `true` if the encoded unit is compatible with the SI.
    ///
    /// SI-compatibility is indicated by the SI flag bit, bit 3.
    ///
    /// Note that non-SI units are not automatically SI-incompatible. A foot is not an SI unit,
    /// but it can be expressed in terms of SI units with no issue, and there are no problems
    /// with compatibility. CGS systems, however, *are* incompatible with the SI – naive
    /// conversion and arithmetic between CGS quantities and SI quantities is not possible.
    #[inline]
    pub fn is_si_compatible(&self) -> bool {
        self.0 & 0b1000 == 0
    }

    /// Returns `true` if the encoded unit is both SI-compatible and linear, `false` otherwise.
    ///
    /// For SI-compatible units, a linear unit must have `000` for bits 6–4.
    #[inline]
    pub fn is_linear(&self) -> bool {
        self.is_si_compatible() && (self.0 & 0b01110000 == 0)
    }

    /// Returns `true` if the encoded unit is both SI-compatible and a scale unit, `false` otherwise.
    ///
    /// For SI-compatible units, a scale unit is any `Unit128` with anything other
    /// than `000` for bits 6–4.
    #[inline]
    pub fn is_scale(&self) -> bool {
        self.is_si_compatible() && (self.0 & 0b01110000 != 0)
    }
}

#[allow(dead_code)]
// Associated functions for converting between `SciDecimal`s and the numeric component
// of an SI-compatible `Uomid`, which varies in width and layout.
impl Uomid {
    /// Calculates a numeric component that encodes the provided [`SciDecimal`] with
    /// the desired widths of significand and exponent, then returns it as a zero-padded `u64`.
    ///
    /// If the significand is larger than `sig_max`, `n` is first rounded to
    /// `digits` significant figures.
    ///
    /// If `i7_exponent` is `true`, uses 7 bits for the exponent, otherwise the
    /// exponent is encoded as an `i8`.
    ///
    /// The number is encoded using a total bit width of
    /// `(binary_bit) + sign_bit + significand_bits + exponent_bits`
    /// i.e. `significand_bits` + 8, 9, or 10 depending on the arguments.
    ///
    /// The encoded number is returned as a `u64`, with the least significant bits
    /// used to encode the number and the rest as zeros.
    ///
    /// Fails if `n` is too large or small to be represented (it exceeds the maximum or minimum
    /// value for an `i8` or `i7`) or is not normal (i.e. it is 0 or infinity or NaN).
    fn num_to_bits(
        n: SciDecimal,
        digits: u8,
        sig_max: u64,
        significand_bits: u8,
        i7_exponent: bool,
    ) -> Result<u64, QuanstantsError> {
        if !n.is_normal() {
            return Err(QuanstantsError::Range);
        }

        // Done like this, with the maximum value passed by the caller, rather
        // than calling `SciDecimal.sf()`, so that the maximum value can be
        // compiled in as a constant and comparison is fast, rather than
        // constantly calculating the number of sig figs at runtime
        let rounded: SciDecimal = if n.significand() > sig_max {
            n.round_sf(digits, scinum::RoundingMode::HalfUp)
        } else {
            n
        };

        // Check that the exponents are representable as `i8` or `i7` as appropriate
        // That allows us to cast without worry
        let allowed_exponents = if i7_exponent {
            (i7::MIN as i16)..=(i7::MAX as i16)
        } else {
            (i8::MIN as i16)..=(i8::MAX as i16)
        };
        if !allowed_exponents.contains(&n.exponent()) {
            return Err(QuanstantsError::Range);
        }

        // Index of the sign bit. Zero indexing means sign bit isn't included in the count
        let sign_bit_pos = significand_bits + if i7_exponent { 7 } else { 8 };

        if i7_exponent {
            Ok(
                0_u64 << sign_bit_pos + 1 // Comes as most significant bit if present
                | (rounded.sign() as u64) << sign_bit_pos
                | (rounded.significand() - 1) << 7
                | i7::from_i16_unchecked(rounded.exponent()).as_byte() as u64,
            )
        } else {
            // Integer casting rules (https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric)
            // signed <-> unsigned at same size => no-op
            // larger to smaller => truncates (always)
            // unsigned smaller to larger => zero-extends
            // signed smaller to larger => sign-extends
            // The last point means that `i8 as u8 as u64` has a different result than direct `i8 as u64`
            Ok(
                0_u64 << sign_bit_pos + 1 // Comes as most significant bit if present
                | (rounded.sign() as u64) << sign_bit_pos
                | (rounded.significand() - 1) << 8
                | rounded.exponent() as i8 as u8 as u64,
            )
        }
    }

    /// Determines the [`SciDecimal`] encoded by a zero-padded numeric component
    /// with the specified layout.
    ///
    /// If `binary_bit` is `true`, the exponent is interpreted according to the
    /// "binary-like" encoding.
    ///
    /// If `i7_exponent` is `true`, extracts 7 bits for the exponent, otherwise
    /// the exponent is interpreted as an `i8`.
    fn bits_to_num(
        b: u64,
        binary_bit: bool,
        significand_bits: u8,
        i7_exponent: bool,
    ) -> SciDecimal {
        let sign_bit_pos = significand_bits + if i7_exponent { 7 } else { 8 };
        let binary_mask: u64 = if binary_bit {
            1_u64 << (sign_bit_pos + 1)
        } else {
            0
        };
        let sign_mask: u64 = 1_u64 << sign_bit_pos;
        let exponent_mask: u64 = if i7_exponent {
            (1_u64 << 7) - 1
        } else {
            (1_u64 << 8) - 1
        };
        // For the simple case, the masks are:
        // BINARY_MASK = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        // SIGN_MASK = 0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        // EXPONENT_MASK = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111
        let binary = if binary_bit {
            (b & binary_mask) != 0
        } else {
            false
        };

        let neg = (b & sign_mask) != 0;
        let significand =
            ((b & !sign_mask & !binary_mask) >> (if i7_exponent { 7 } else { 8 })) + 1;
        let sig_signed = if neg {
            -(significand as i64)
        } else {
            significand as i64
        };

        let exp: i16 = if i7_exponent {
            i7::from_byte((b & exponent_mask) as u8).into()
        } else {
            (b & exponent_mask) as i8 as i16 // Casting larger to smaller truncates, smaller to larger sign extends when signed
        };
        if !binary {
            SciDecimal::new(sig_signed, exp)
        } else {
            // First just make the significand into a number i.e. *n* = (−1)^*g* (*a* + 1)
            let n = SciDecimal::new(sig_signed, 0);
            // Create the exponent term as a second SciDecimal
            // Per the specification, the exponent is encoded as the actual value multiplied by 3
            let e = SciDecimal::new(1024, 0).powi(exp as i32 / 3);
            // The result will always fit into a `SciDecimal`,
            // but full precision will not always be possible
            n * e
        }
    }
}

impl From<Unit128> for Uomid {
    fn from(value: Unit128) -> Self {
        // Valid UoMIDs are a superset of valid Unit128 values
        Self(value.0)
    }
}

impl From<ScaleUnit128> for Uomid {
    fn from(value: ScaleUnit128) -> Self {
        // Valid UoMIDs are a superset of valid ScaleUnit128 values
        Self(value.0)
    }
}

/// A 128-bit representation of a linear (non-scale) SI-compatible unit.
///
/// A `Unit128` is simply a [`Uomid`] (Unit of Measure ID) enforced to be a
/// valid value for a linear unit compatible with the SI.
///
/// The vast majority of units normally encountered are linear. Commonly
/// encountered quantities with scale units are temperatures in Celsius and
/// Fahrenheit, logarithmic quantities in decibel, and other logarithmic scales
/// such as pH and stellar magnitude. Use [`ScaleUnit128`] for (SI-compatible)
/// scale units.
///
/// Note that non-SI units are not automatically SI-incompatible. Most units,
/// even those belonging to another system, are still expressible in terms of SI
/// base units. The litre is not a unit in the SI, but is of course compatible
/// with it. The foot is not an SI unit, but it can be expressed in terms of SI
/// units with no issue, and there are no problems with compatibility.
///
/// A unit system is incompatible with the SI if it is defined using a different
/// set of base dimensions. CGS systems, however, *are* incompatible with the SI
/// – naive conversion and arithmetic between CGS quantities and SI quantities
/// is not possible.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Unit128(pub(crate) u128);
// No huge advantage to be had from wrapping a `Uomid` as a newtype, so just
// wrap a `u128` (for now, anyway)

impl Unit128 {
    /// Creates a new anonymous unit defined in terms of SI base units.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is too large or small to be encoded (i.e. it has an
    /// exponent > [`Unit128::MAX_EXPONENT`] or < [`Unit128::MIN_EXPONENT`])
    /// or because it is not normal (i.e. it is 0 or infinity or NaN).
    pub fn new(factor: SciDecimal, dimensions: Dimensions) -> Self {
        if !dimensions.all_integer() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Self(
            (Unit128::factor_to_bits(factor).expect("Caller should not pass an unrepresentable value") as u128) << 64
                | (*dimensions.J.numer() as u8 as u128) << 56
                | (*dimensions.N.numer() as u8 as u128) << 48
                | (*dimensions.Θ.numer() as u8 as u128) << 40
                | (*dimensions.I.numer() as u8 as u128) << 32
                | (*dimensions.M.numer() as u8 as u128) << 24
                | (*dimensions.L.numer() as u8 as u128) << 16
                | (*dimensions.T.numer() as u8 as u128) << 8
                // Integer exponents    => bit 7 = 0
                // Linear unit          => bits 6-4 = 000
                // SI compatible        => bit 3 = 0
                // Uncatalogued         => bits 2-0 = 000
                | 0x00,
        )
    }

    /// Creates a new anonymous unit with a factor encoded using the binary-like representation.
    ///
    /// If the unit has fractional dimensional exponents, falls back to creating
    /// a unit with a decimal factor instead, as it is not possible to encode a
    /// binary factor with fractional exponents.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is too large or small to be encoded (i.e. its absolute
    /// value does not lie between [`Unit128::MAX_BIN_FACTOR`] and
    /// [`Unit128::MAX_POS_BIN_FACTOR`])
    /// or because it is not normal (i.e. it is 0 or infinity or NaN).
    pub(crate) fn new_with_binary_factor(factor: f64, dimensions: Dimensions) -> Self {
        if !dimensions.all_integer() {
            Unit128::new(factor.into(), dimensions)
        } else {
            Self(
                (Unit128::binary_factor_to_bits(factor).expect("Caller should not pass an unrepresentable value") as u128) << 64
                    | (*dimensions.J.numer() as u8 as u128) << 56
                    | (*dimensions.N.numer() as u8 as u128) << 48
                    | (*dimensions.Θ.numer() as u8 as u128) << 40
                    | (*dimensions.I.numer() as u8 as u128) << 32
                    | (*dimensions.M.numer() as u8 as u128) << 24
                    | (*dimensions.L.numer() as u8 as u128) << 16
                    | (*dimensions.T.numer() as u8 as u128) << 8
                    // Integer exponents    => bit 7 = 0
                    // Linear unit          => bits 6-4 = 000
                    // SI compatible        => bit 3 = 0
                    // Uncatalogued         => bits 2-0 = 000
                    | 0x00,
            )
        }
    }

    /// Creates a new anonymous unit with a factor of 1024^(exponent), encoded using
    /// the binary-like representation.
    ///
    /// If the unit has fractional dimensional exponents, falls back to creating
    /// a unit with a decimal factor instead, as it is not possible to encode a
    /// binary factor with fractional exponents.
    ///
    /// # Panics
    ///
    /// Panics if `exponent` is > 42 or < -42.
    pub fn new_with_binary_prefix(exponent: i8, dimensions: Dimensions) -> Self {
        if exponent > 42 || exponent < -42 {
            panic!("Exponent must be within the range -42..=42")
        }
        if !dimensions.all_integer() {
            Unit128::new(1024_f64.powi(exponent.into()).into(), dimensions)
        } else {
            Self(
                ((exponent * 3) as u8 as u128) << 64
                    | (*dimensions.J.numer() as u8 as u128) << 56
                    | (*dimensions.N.numer() as u8 as u128) << 48
                    | (*dimensions.Θ.numer() as u8 as u128) << 40
                    | (*dimensions.I.numer() as u8 as u128) << 32
                    | (*dimensions.M.numer() as u8 as u128) << 24
                    | (*dimensions.L.numer() as u8 as u128) << 16
                    | (*dimensions.T.numer() as u8 as u128) << 8
                    // Integer exponents    => bit 7 = 0
                    // Linear unit          => bits 6-4 = 000
                    // SI compatible        => bit 3 = 0
                    // Uncatalogued         => bits 2-0 = 000
                    | 0x00,
            )
        }
    }

    /// Creates a new compound unit from a set of factors.
    ///
    /// # Panics
    ///
    /// This function panics if the proportionality factor becomes so large or
    /// small that it can no longer be represented.
    pub fn new_compound(factors: &[(Unit128, Frac)]) -> Self {
        let dimensions = factors
            .iter()
            .map(|x| x.0.dimensions().pow(x.1))
            .fold(Dimensions::DIMENSIONLESS, |acc, x| acc * x);
        // Do this way, rather than by multiplying successive Unit128s, in order to
        // avoid constant conversion back and forth to SciDecimal
        let proportionality_factor = factors
            .iter()
            .map(|x| x.0.factor().pow(x.1))
            .fold(SciDecimal::ONE, |acc, x| acc * x);
        Self::new(proportionality_factor, dimensions)
    }

    /// Returns an equivalent unit but with the catalogue number set to the provided value.
    ///
    /// The catalogue number must be between 1 and 7 inclusive.
    ///
    /// # Panics
    ///
    /// Panics if `catalogue_number` is 0 or > 7.
    pub(crate) fn with_catalogue_number(self, catalogue_number: u8) -> Self {
        Unit128(self.0 | catalogue_number as u128)
    }

    /// Attempts to create a unit from the corresponding UomID in the form of a
    /// 128-bit integer.
    ///
    /// Fails if the UoMID does not correspond to a linear, SI-compatible unit.
    pub fn from_raw(b: u128) -> Result<Self, QuanstantsError> {
        if Uomid(b).is_linear() {
            Ok(Self(b))
        } else {
            Err(QuanstantsError::InvalidId)
        }
    }

    /// Returns the wrapped UoMID as a 128-bit integer.
    #[inline]
    pub fn as_raw(&self) -> u128 {
        self.0
    }

    /// Returns the proportionality factor.
    #[inline]
    pub fn factor(&self) -> SciDecimal {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Unit128::bits_to_factor((self.0 >> 64) as u64)
    }

    /// Returns the SI dimension terms of the unit.
    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Dimensions {
            T: Frac::from(((self.0 >> 8) & 0xFF) as u8 as i8),
            L: Frac::from(((self.0 >> 16) & 0xFF) as u8 as i8),
            M: Frac::from(((self.0 >> 24) & 0xFF) as u8 as i8),
            I: Frac::from(((self.0 >> 32) & 0xFF) as u8 as i8),
            Θ: Frac::from(((self.0 >> 40) & 0xFF) as u8 as i8),
            N: Frac::from(((self.0 >> 48) & 0xFF) as u8 as i8),
            J: Frac::from(((self.0 >> 56) & 0xFF) as u8 as i8),
        }
    }

    /// Returns `true` if the unit has a proportionality factor of 1.
    #[inline]
    pub fn is_coherent(&self) -> bool {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        // Don't need to actually extract the factor, just inspect the 64 bits
        // of the numeric component - if the factor is 1, all must be 0
        (self.0 >> 64) == 0
    }

    /// Returns `true` if the unit has a non-integer exponent in any dimension.
    #[inline]
    pub(crate) fn has_fractional_dimensions(&self) -> bool {
        (self.0 & 0b10000000) != 0
    }

    /// Discards the number uniquely identifying it to afford a normalized
    /// representation defined only in terms of the SI base units.
    #[inline]
    pub fn normalize(self) -> Self {
        // Zero bits 2–0
        Self(self.0 & !0b111)
    }

    /// Calculates the inverse of the unit.
    pub fn inverse(self) -> Unit128 {
        Unit128::new(self.factor().inv(), self.dimensions().inverse())
    }

    /// Raises the unit to the given power.
    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Unit128 {
        let exp: Frac = exponent.into();
        Unit128::new(self.factor().pow(exp), self.dimensions().pow(exp))
    }
}

impl Mul for Unit128 {
    type Output = Self;

    /// Multiplies two linear, SI-compatible units.
    fn mul(self, rhs: Unit128) -> Unit128 {
        Unit128::new(
            self.factor() * rhs.factor(),
            self.dimensions() * rhs.dimensions(),
        )
    }
}

impl Div for Unit128 {
    type Output = Self;

    /// Divides the unit by `rhs`, where both are linear, SI-compatible units.
    fn div(self, rhs: Unit128) -> Unit128 {
        Unit128::new(
            self.factor() / rhs.factor(),
            self.dimensions() / rhs.dimensions(),
        )
    }
}

impl fmt::Debug for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit128(0x{:X})", self.as_raw())
    }
}

impl fmt::Display for Unit128 {
    /// Writes the unit as the hexadecimal representation of the 128-bit UoMID.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.as_raw())
    }
}

impl FromStr for Unit128 {
    type Err = QuanstantsError;

    /// Creates a new unit from a hexadecimal string representation of the 128-bit UoMID.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // from_str_radix doesn't like anything other than digits
        // We allow prefix and underscores just like Rust literals and TOML hexadecimal
        let hex = s.strip_prefix("0x").unwrap_or(s).replace("_", "");
        let bits = u128::from_str_radix(&hex, 16).map_err(|_e| QuanstantsError::Parse(s.into()))?;
        Self::from_raw(bits)
    }
}

#[allow(dead_code)]
impl Unit128 {
    // Maximum and minimum values for a simple 64-bit numeric component, which just encodes
    // the proportionality factor _k_.

    /// The highest possible significand for the proportionality factor.
    pub const MAX_SIGNIFICAND: u64 = 10_u64.pow(16) - 1;

    /// The highest possible significand for the proportionality factor.
    pub const MAX_SIGNED_SIGNIFICAND: i64 = Unit128::MAX_SIGNIFICAND as i64;

    /// The lowest possible significand for the proportionality factor.
    pub const MIN_SIGNED_SIGNIFICAND: i64 = -Unit128::MAX_SIGNED_SIGNIFICAND;

    /// The highest possible exponent for the proportionality factor.
    pub const MAX_EXPONENT: i8 = i8::MAX;

    /// The lowest possible exponent for the proportionality factor.
    pub const MIN_EXPONENT: i8 = i8::MIN;

    /// The highest proportionality factor that can be represented.
    pub const MAX_FACTOR: SciDecimal =
        SciDecimal::new(Unit128::MAX_SIGNED_SIGNIFICAND, i8::MAX as i16);

    /// The lowest proportionality factor that can be represented.
    pub const MIN_FACTOR: SciDecimal =
        SciDecimal::new(Unit128::MIN_SIGNED_SIGNIFICAND, i8::MAX as i16);

    /// The smallest positive proportionality factor that can be represented.
    pub const MIN_POS_FACTOR: SciDecimal = SciDecimal::new(1_i64, i8::MIN as i16);

    /// The smallest negative proportionality factor that can be represented.
    pub const MAX_NEG_FACTOR: SciDecimal = SciDecimal::new(-1_i64, i8::MIN as i16);

    /// Calculates the 64-bit numeric component that encodes the provided [`SciDecimal`].
    ///
    /// A number defined by *k* = (−1)^*g* (*a* + 1) × 10^*e* is encoded by a form of BID decimal
    /// floating point, with (from most to least significant):
    ///
    /// - a binary bit, `w = 0`
    /// - a sign bit, `g`
    /// - a 54-bit significand with a bias of −1, `a`
    /// - an `i8` exponent, `e`
    ///
    /// in the layout: `|wgaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|eeeeeeee|`
    ///
    /// A binary-like encoding is possible when `w = 1`, but not using this function.
    ///
    /// The significand is limited to 16 full decimal digits of precision.
    /// This matches [`SciDecimal`] and IEEE 754's `decimal64`, so there is no loss of precision.
    ///
    /// Fails if `factor` is too large or small to be represented (*e* > [`MAX_EXPONENT_SIMPLE`] or
    /// < [`MIN_EXPONENT_SIMPLE`]) or is not normal (i.e. it is 0 or infinity or NaN).
    pub(crate) fn factor_to_bits(factor: SciDecimal) -> Result<u64, QuanstantsError> {
        Uomid::num_to_bits(factor, 16, Self::MAX_SIGNIFICAND, 54, false)
    }

    /// The highest proportionality factor that can be represented when using the binary-like encoding.
    pub const MAX_BIN_FACTOR: f64 =
        (Unit128::MAX_SIGNED_SIGNIFICAND as f64) * f64::from_bits(0x5A30000000000000); // Representation of 1024^42 as an f64 per IEEE 754

    /// The lowest proportionality factor that can be represented when using the binary-like encoding.
    pub const MIN_BIN_FACTOR: f64 = -Unit128::MAX_BIN_FACTOR;

    /// The smallest positive proportionality factor that can be represented when using the binary-like encoding.
    pub const MIN_POS_BIN_FACTOR: f64 = 1.0f64 * f64::from_bits(0x25B0000000000000); // Representation of 1024^-42 as an f64 per IEEE 754

    /// The smallest negative proportionality factor that can be represented when using the binary-like encoding.
    pub const MAX_NEG_BIN_FACTOR: f64 = -Unit128::MIN_POS_BIN_FACTOR;

    /// Calculates the 64-bit numeric component that encodes the provided `f64`
    /// using the binary-like encoding scheme.
    ///
    /// A number defined by *k* = (−1)^*g* (*a* + 1) × 1024^(*e*/3) is encoded by a form of BID decimal
    /// floating point, with (from most to least significant):
    ///
    /// - a binary bit, `w = 1`
    /// - a sign bit, `g`
    /// - a 54-bit significand with a bias of −1, `a`
    /// - an `i8` exponent, `e`
    ///
    /// in the layout: `|wgaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|eeeeeeee|`
    ///
    /// The significand is limited to that of an `f64`, which is just slightly
    /// less than 16 full decimal digits of precision (53 bits = 15.96 digits)
    ///
    /// Fails if `factor` is too large or small to be represented (its absolute
    /// value lies outside of the range [`MIN_POS_BIN_FACTOR`] to [`MAX_BIN_FACTOR`])
    /// or is not normal (i.e. it is 0 or infinity or NaN).
    ///
    /// The binary prefixes have convenient representations that match the corresponding
    /// decimal prefixes:
    ///
    /// | Symbol | Prefix | Value           | Encoded exponent | Hex                |
    /// | ------ | ------ | --------------- | ---------------- | ------------------ |
    /// | Ki     | kibi   |  1024^1 = 2^10  |                3 | `8000000000000003` |
    /// | Mi     | mebi   |  1024^2 = 2^20  |                6 | `8000000000000006` |
    /// | Gi     | gibi   |  1024^3 = 2^30  |                9 | `8000000000000009` |
    /// | Ti     | tebi   |  1024^4 = 2^40  |               12 | `800000000000000C` |
    /// | Pi     | pebi   |  1024^5 = 2^50  |               15 | `800000000000000F` |
    /// | Ei     | exbi   |  1024^6 = 2^60  |               18 | `8000000000000012` |
    /// | Zi     | zebi   |  1024^7 = 2^70  |               21 | `8000000000000015` |
    /// | Yi     | yobi   |  1024^8 = 2^80  |               24 | `8000000000000018` |
    /// | Ri     | robi   |  1024^9 = 2^90  |               27 | `800000000000001B` |
    /// | Qi     | quebi  | 1024^10 = 2^100 |               30 | `800000000000001E` |
    /// |        | MAX    | 1024^42 = 2^420 |              126 | `800000000000007E` |
    pub(crate) fn binary_factor_to_bits(factor: f64) -> Result<u64, QuanstantsError> {
        if !factor.is_normal() {
            return Err(QuanstantsError::Range);
        } else if (factor.is_sign_positive()
            && ((factor > Unit128::MAX_BIN_FACTOR) || (factor < Unit128::MIN_POS_BIN_FACTOR)))
            || (factor.is_sign_negative()
                && ((factor < Unit128::MIN_BIN_FACTOR) || (factor > Unit128::MAX_NEG_BIN_FACTOR)))
        {
            return Err(QuanstantsError::Range);
        }
        let bits = factor.to_bits();
        let sign_bit = bits >> 63;
        // Significand (called mantissa by `f64` docs) is bits 51–0 of an `f64`
        // plus an implicit leading 1, which we add back to get the real significand
        let mut significand = (bits & 0x000F_FFFF_FFFF_FFFF) | 1_u64 << 52;
        // Exponent is bits 62–52, with a bias of 1023, which we remove to get the real exponent
        let mut exp_2 = ((bits >> 52) & 0x7FF) as i16 - 1023;
        // This exponent was a power of 2, we need a power of 1024.
        // 1024 = 2^10 therefore 1024^a = 2^(a*10)
        // So we need the exponent with a base of 2 to be a multiple of 10 in
        // order to convert it to a base of 1024.
        // If we were to do a left shift on the significand to achieve this,
        // how many bits would we need to shift by?
        let remainder = exp_2 % 10;
        // If already a multiple of 10, no need to do anything
        if remainder != 0 {
            // Negative remainder means we'd have to left shift by 10 - |remainder|
            let r = if remainder.is_positive() {
                remainder
            } else {
                10 - remainder.abs()
            };
            // We have 54 bits available for the significand - how many are already being used?
            let digits = match significand.checked_ilog2() {
                Some(n) => n + 1,
                None => 0,
            };
            let spare = 54 - digits;
            // If we have enough spare to increase the significand precision while
            // decreasing the exponent, do it; otherwise, do the opposite
            if spare >= r as u32 {
                // Multiply significand by 2^remainder by shifting
                significand = significand << r;
                // Decrease exponent appropriately
                exp_2 -= r;
            } else {
                // Divide significand by necessary amount by shifting
                // Loses precision in the process, but this is unavoidable
                significand = significand >> (10 - r);
                exp_2 += 10 - r;
            }
        }
        // Now can actually convert to a base 1024 exponent
        debug_assert_eq!(exp_2 % 10, 0);
        let exp_1024 = exp_2 / 10;
        Ok(
            1_u64 << 63 // Binary bit set to 1 to indicate binary encoding
            | sign_bit << 62 // Sign bit follows binary bit
            | (significand - 1) << 8 // Apply our bias
            | (exp_1024 * 3) as i8 as u8 as u64, // Encoded as 3 times the exponent
        )
    }

    // Maximum and minimum values for a narrow 36-bit numeric component, which just encodes
    // the proportionality factor _k_.
    // Used when the dimensional component encodes fractional exponents and is therefore widened.

    /// The highest possible significand for the proportionality factor when the unit has fractional dimensional exponents.
    pub const MAX_SIGNIFICAND_NARROW: u64 = 10_u64.pow(8) - 1;

    /// The highest possible significand for the proportionality factor when the unit has fractional dimensional exponents.
    pub const MAX_SIGNED_SIGNIFICAND_NARROW: i64 = Unit128::MAX_SIGNIFICAND_NARROW as i64;

    /// The lowest possible significand for the proportionality factor when the unit has fractional dimensional exponents.
    pub const MIN_SIGNED_SIGNIFICAND_NARROW: i64 = -Unit128::MAX_SIGNED_SIGNIFICAND_NARROW;

    /// The highest possible exponent for the proportionality factor when the unit has fractional dimensional exponents.
    pub const MAX_EXPONENT_NARROW: i8 = i7::MAX;

    /// The lowest possible exponent for the proportionality factor when the unit has fractional dimensional exponents.
    pub const MIN_EXPONENT_NARROW: i8 = i7::MIN;

    /// The highest proportionality factor that can be represented when the unit has fractional dimensional exponents.
    pub const MAX_FACTOR_NARROW: SciDecimal =
        SciDecimal::new(Unit128::MAX_SIGNED_SIGNIFICAND_NARROW, i7::MAX as i16);

    /// The lowest proportionality factor that can be represented when the unit has fractional dimensional exponents.
    pub const MIN_FACTOR_NARROW: SciDecimal =
        SciDecimal::new(Unit128::MIN_SIGNED_SIGNIFICAND_NARROW, i7::MAX as i16);

    /// The smallest positive proportionality factor that can be represented when the unit has fractional dimensional exponents.
    pub const MIN_POS_FACTOR_NARROW: SciDecimal = SciDecimal::new(1_i64, i7::MIN as i16);

    /// The smallest negative proportionality factor that can be represented when the unit has fractional dimensional exponents.
    pub const MAX_NEG_FACTOR_NARROW: SciDecimal = SciDecimal::new(-1_i64, i7::MIN as i16);

    /// Calculates the 36-bit numeric component that encodes the provided [`SciDecimal`],
    /// with zero padding up to 64 bits.
    ///
    /// A number defined by *k* = (−1)^*g* (*a* + 1) × 10^*e* is encoded by a form of BID decimal
    /// floating point, with (from most to least significant):
    ///
    /// - a sign bit, `g`
    /// - a 27-bit significand with a bias of −1, `a`
    /// - an `i8` exponent, `e`
    ///
    /// in the layout: `|00000000|00000000|00000000|0000gaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|eeeeeeee|`
    ///
    /// The significand is limited to 8 full decimal digits of precision.
    /// If the number has more than 8 significant figures it is first rounded.
    ///
    /// Fails if `factor` is too large or small to be represented
    /// (*e* > [`MAX_EXPONENT_SIMPLE_NARROW`] or < [`MIN_EXPONENT_SIMPLE_NARROW`])
    /// or is not normal (i.e. it is 0 or infinity or NaN).
    pub(crate) fn factor_to_bits_narrow(factor: SciDecimal) -> Result<u64, QuanstantsError> {
        Uomid::num_to_bits(factor, 8, Self::MAX_SIGNIFICAND_NARROW, 27, false)
    }

    /// Determines the [`SciDecimal`] encoded by a 64-bit numeric component.
    ///
    /// If the numeric component uses the binary encoding, the result may lose
    /// some precision.
    pub(crate) fn bits_to_factor(b: u64) -> SciDecimal {
        Uomid::bits_to_num(b, true, 54, false)
    }

    /// Determines the [`SciDecimal`] encoded by a zero-padded 36-bit numeric component.
    pub(crate) fn bits_to_factor_narrow(b: u64) -> SciDecimal {
        Uomid::bits_to_num(b, false, 27, false)
    }
}

impl Unit128 {
    #[allow(dead_code)]

    /// Unity, the "base unit" for dimensionless quantities (i.e numbers).
    pub const ONE: Unit128 = Unit128(0x00);

    /// The SI base unit of time, symbol **s**.
    pub const SECOND: Unit128 = Unit128(0x01_00);

    /// The SI base unit of length, symbol **m**.
    pub const METRE: Unit128 = Unit128(0x01_00_00);

    /// An alias for [`Unit128::METRE`].
    pub const METER: Unit128 = Unit128::METRE;

    /// The SI base unit of mass, symbol **kg**.
    ///
    /// Note that though the kilogram is the base unit of mass in the SI,
    /// prefixes are used with the root form, the gram.
    ///
    /// When such prefixed forms are expressed as a UoMID, however, the prefix
    /// is treated as being appended to the kilogram.
    pub const KILOGRAM: Unit128 = Unit128(0x01_00_00_00);

    /// The SI base unit of electric current, symbol **A**.
    pub const AMPERE: Unit128 = Unit128(0x00_00_00_01_00_00_00_00);

    /// The SI base unit of thermodynamic temperature, symbol **K**.
    pub const KELVIN: Unit128 = Unit128(0x00_00_01_00_00_00_00_00);

    /// The SI base unit of amount of substance, symbol **mol**.
    pub const MOLE: Unit128 = Unit128(0x00_01_00_00_00_00_00_00);

    /// The SI base unit of luminous intensity, symbol **cd**.
    pub const CANDELA: Unit128 = Unit128(0x01_00_00_00_00_00_00_00);

    /// The gram, an SI derived unit of mass, symbol **g**, equal to 10⁻³ kg.
    ///
    /// For historical reasons the kilogram is the base unit of mass in the SI,
    /// and the gram is a derived unit defined in terms of kg.
    ///
    /// For its UoMID representation, the gram is encoded as if it were a
    /// prefixed form of the base unit i.e. as if it were a "millikilogram".
    ///
    /// As such a "millikilogram" does not exist and the gram is *the* canonical
    /// unit with the value 1 × 10⁻³ kg, the gram does not get a catalogue number
    /// and is considered the base/normalized representation and so the three
    /// least significant bits are all 0.
    pub const GRAM: Unit128 = Unit128(0xFD_00_00_00_00_01_00_00_00);

    /// The SI derived unit of plane angle, symbol **rad**, equal to 1.
    ///
    /// Note that unlike most unit libraries and representations, but following
    /// the SI, both the UoMID specification and quanstants do not consider the
    /// radian and steradian base units – they are just dimensionless derived
    /// units.
    ///
    /// From the SI Brochure (9th Ed.):
    /// > The radian is the coherent unit for plane angle. One radian is the angle subtended at the centre of a
    /// > circle by an arc that is equal in length to the radius. This suggests rad = m/m but this representation is
    /// > not intrinsic and may be misleading since angle is not the same kind of quantity as other length ratios.
    /// > An alternative definition is that a right angle is equal to π/2 rad. The radian is also the coherent unit
    /// > for phase angle. For periodic phenomena, the phase angle increases by 2π rad in one period.
    pub const RADIAN: Unit128 = Unit128(0x01);

    /// The SI derived unit of solid angle, symbol **sr**, equal to 1.
    ///
    /// Note that unlike most unit libraries and representations, but following
    /// the SI, both the UoMID specification and quanstants do not consider the
    /// radian and steradian base units – they are just dimensionless derived
    /// units.
    ///
    /// From the SI Brochure (9th Ed.):
    /// > The steradian is the coherent unit for solid angle. One steradian is the solid angle subtended at the centre
    /// > of a sphere by an area of the surface that is equal to the squared radius. This suggests sr = m2/m2, but this
    /// > representation is not intrinsic and may be misleading since solid angle is not the same kind of quantity as
    /// > other area ratios. An alternative definition is that a complete sphere subtends 4π sr about its centre.
    pub const STERADIAN: Unit128 = Unit128(0x02);

    /// The SI derived unit of frequency, symbol **Hz**, equal to s⁻¹.
    ///
    /// Per the SI:
    /// > The hertz shall only be used for periodic phenomena and the becquerel shall only be used for stochastic
    /// processes in activity referred to a radionuclide.
    ///
    /// For all other purposes, s⁻¹ should be used.
    pub const HERTZ: Unit128 = Unit128(0x00_00_00_00_00_00_FF_01);

    /// The SI derived unit of force, symbol **N**, equal to kg⋅m⋅s⁻².
    pub const NEWTON: Unit128 = Unit128(0x00_00_00_00_01_01_FF_01);

    /// The SI derived unit of pressure and stress, symbol **Pa**, equal to kg⋅m⁻¹⋅s⁻².
    pub const PASCAL: Unit128 = Unit128(0x00_00_00_00_01_FF_FE_01);

    /// The SI derived unit of energy, work, and amount of heat, symbol **J**, equal to kg⋅m²⋅s⁻².
    pub const JOULE: Unit128 = Unit128(0x00_00_00_00_01_02_FE_01);

    /// The SI derived unit of power and radiant flux, symbol **W**, equal to kg⋅m²⋅s⁻³.
    pub const WATT: Unit128 = Unit128(0x00_00_00_00_01_02_FD_01);

    /// The SI derived unit of electric charge, symbol **C**, equal to A⋅s.
    pub const COULOMB: Unit128 = Unit128(0x00_00_00_01_00_00_01_01);

    /// The SI derived unit of electric potential difference (voltage), symbol **V**, equal to kg⋅m²⋅s⁻³⋅A⁻¹.
    pub const VOLT: Unit128 = Unit128(0x00_00_00_FF_01_02_FD_01);

    /// The SI derived unit of capacitance, symbol **F**, equal to kg⁻¹⋅m⁻²⋅s⁴⋅A².
    pub const FARAD: Unit128 = Unit128(0x00_00_00_02_FF_FE_14_01);

    /// The SI derived unit of electric resistance, symbol **Ω**, equal to kg⋅m²⋅s⁻³⋅A⁻².
    pub const OHM: Unit128 = Unit128(0x00_00_00_FE_01_02_FD_01);

    /// The SI derived unit of electric conductance, symbol **S**, equal to kg⁻¹⋅m⁻²⋅s³⋅A².
    pub const SIEMENS: Unit128 = Unit128(0x00_00_00_02_FF_FE_03_01);

    /// The SI derived unit of magnetic flux, symbol **Wb**, equal to kg⋅m²⋅s⁻²⋅A⁻¹.
    pub const WEBER: Unit128 = Unit128(0x00_00_00_FF_01_02_FE_01);

    /// The SI derived unit of magnetic flux density, symbol **T**, equal to kg⋅s⁻²⋅A⁻¹.
    pub const TESLA: Unit128 = Unit128(0x00_00_00_FF_01_00_FE_01);

    /// The SI derived unit of inductance, symbol **H**, equal to kg⋅m²⋅s⁻²⋅A⁻².
    pub const HENRY: Unit128 = Unit128(0x00_00_00_FE_01_02_FE_01);

    /// The absolute magnitude of the degree Celsius, equal to the kelvin.
    ///
    /// This is an absolute, linear unit, used for representing temperature
    /// differences and intervals.
    ///
    /// For the corresponding scale unit, used for representing temperatures on
    /// the Celsius scale, see [`ScaleUnit128::DEGREE_CELSIUS`].
    pub const CELSIUS_DEGREE: Unit128 = Unit128(
        // Numeric component gets referenced layout:
        // `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
        // *y* = (−1)^*h* (*b* + 1) × 10^*f* and *k* = (−1)^*g* (*a* + 1) × 10^*e*
        // For °C:
        // reference y = 273.15 = 27315e-2
        //   => h = 0 (positive)
        //      b = 27315 − 1 = 0b1101010_10110010
        //      f = −2 = 0b11111110 as i8 = 0b1111110 as i7
        //   so encoded as 0b00000000_00110101_01011001_01111110 = 0x00_35_59_7E
        // constant (proportionality factor) k = 1
        //   => g = 0,
        //      b = 1 − 1 = 0,
        //      f = 0 = 0b0 as i8 = 0b0 as i7
        //   so encoded as 4 bytes of zeros 0x00_00_00_00
        //   and encoded for the degree magnitude (a linear unit) as 8 bytes of zeros
        // reference unit ꟛ = kelvin => dim = 0x00_00_01_00_00_00_00
        // Celsius gets catalogue number 1 assigned => lsb = 0x01 for degree magnitude, 0x41 for scale
        // Putting it together (everything below is hex):
        // reference:                   0035597E
        // constant:                             00000000
        // reference unit:                                00_00_01_00_00_00_00
        // least significant byte:                                             n1 (where n = 0 or 4)
        // UoMID for degree magnitude:                    00_00_01_00_00_00_00_01
        // UoMID for scale unit:        0035597E_00000000_00_00_01_00_00_00_00_41
        0x00_00_01_00_00_00_00_01,
    );

    /// The SI derived unit of luminous flux, symbol **lm**, equal to cd⋅sr.
    pub const LUMEN: Unit128 = Unit128(0x01_00_00_00_00_00_00_01);

    /// The SI derived unit of illuminance, symbol **lx**, equal to cd⋅sr⋅m⁻².
    pub const LUX: Unit128 = Unit128(0x01_00_00_00_00_FE_00_01);

    /// The SI derived unit of radioactivity, symbol **Bq**, equal to s⁻¹.
    ///
    /// Per the SI:
    /// > The hertz shall only be used for periodic phenomena and the becquerel shall only be used for stochastic
    /// processes in activity referred to a radionuclide.
    ///
    /// For all other purposes, s⁻¹ should be used.
    ///
    /// Note that according to the SI Brochure (9th Ed.):
    /// > Activity referred to a radionuclide is sometimes incorrectly called radioactivity.
    pub const BECQUEREL: Unit128 = Unit128(0x00_00_00_00_00_00_FF_02);

    /// The SI derived unit of absorbed dose and kerma, symbol **Gy**, equal to m²⋅s⁻².
    pub const GRAY: Unit128 = Unit128(0x00_00_00_00_00_02_FE_01);

    /// The SI derived unit of dose equivalent, symbol **Sv**, equal to m²⋅s⁻².
    ///
    /// The sievert is intended to represent the health risk of ionizing radiation,
    /// whereas the gray is used for the physical absorbed dose.
    ///
    /// See CIPM Recommendation 2 on the use of the sievert (PV, 2002, 70, 205).
    pub const SIEVERT: Unit128 = Unit128(0x00_00_00_00_00_02_FE_02);

    /// The SI derived unit of catalytic activity, symbol **kat**, equal to mol⋅s⁻¹.
    pub const KATAL: Unit128 = Unit128(0x00_01_00_00_00_00_FF_01);
}

/// The mathematical relationship between a quantity in terms of a unit and
/// the equivalent reference value.
///
/// Represents the information encoded by bits 6–4 of a UoMID (see [`Uomid`]).
///
/// A "normal" quantity is understood to be the *product* of a number _x_ and a unit _λ_;
/// for example, "3 ft" is understood to mean 3 × ft and to convert, ft can be
/// expressed as ft = 0.3408 m and thus 3 ft = 3 × 0.3408 m = 1.0224 m
///
/// However, in order to generalize this to "non-linear" or "scale" units -- where
/// a quantity of the form _x⋅u_ with that unit _u_ indicates a specific point on a
/// scale, we must instead express quantities as *functions* of a number and a unit,
/// where the unit is itself a function, and the value of the quantity is given by:
///
/// _Q_(_x_, _u_) = _u_(_x_)
///
/// where _x_ is the number of the quantity.
///
/// For each type of quantity the type of unit is defined by a specific functional form,
/// and an individual unit is defined by the values of the parameters of that function.
///
/// For a linear quantity _Λ_:
/// - _Λ_(_x_, _λ_) = _λ_(_x_, _k_, *λ*₀), where:
///     - *λ*₀ is the other linear unit (usually SI) used to define the unit
///     - _k_ is the proportionality factor, the number of the value of the unit
///       when expressed in *λ*₀
///     - The value of an individual unit is fully defined by its values of _k_ and *λ*₀
///
/// For a temperature on a scale, _Θ_:
/// - _Θ_(_x_, _θ_) = _θ_(_x_, _y_, _k_, _λ_), where:
///     - _λ_ is the (linear) unit of the reference absolute temperature
///     - _y_ is the number of the reference absolute temperature (i.e. the value in _λ_ at 0 _θ_)
///     - _k_ is the proportionality factor of the degree of the scale – the size of the degree
///       when expressed in _λ_
/// - A temperature on the scale _Θ_ is thus expressed as a linear quantity _Λ_ by:
///     - _Θ_(_x_, _θ_) = _Λ_(_k_(_x_ + _y_), _λ_)
/// - An individual temperature scale is thus fully defined by _λ_, _y_, and _k_,
///   which can be linked to:
///     - the magnitude of the scale's degree _d_ = _Λ_(_k_, _λ_)
///     - the reference absolute temperature _r_ = _Λ_(_y_, _λ_)
/// - For Celsius, _λ_ is kelvin, _y_ = 273.15, and _k_ = 1
/// - For Fahrenheit, if _λ_ is the degree Rankine, _y_ = 459.67, and _k_ = 1
///   i.e. −459.67 °F is 1 × (−459.67 + 459.67) = 0 °R
///     - The degree Rankine is, despite the name, a linear unit of temperature,
///       with _k_ = 5/9 and *λ*₀ = kelvin
///     - Fahrenheit is therefore equivalently described by _y_ = 459.67, _k_ = 5/9
///
/// For a base-10 logarithmic quantity _Κ_:
/// - _Κ_(_x_, _κ_) = _κ_(_x_, _y_, _k_, _λ_), where:
///     - _λ_ is the (linear) unit of the reference quantity
///     - _y_ is the number of the reference quantity when expressed in _λ_, or
///       equivalently, the number of the value in _λ_ when _x_ = 0
///     - _k_ is the proportionality factor
/// - _Κ_ is thus expressed as a linear quantity _Λ_ by:
///     - _Κ_(_x_, _κ_) = _Λ_(_y_ × 10^(*k*⋅*x*), _λ_)
/// - A logarithmic unit is thus fully defined by _λ_, _y_, and _k_
/// - For a power level in decibel:
///     - *x* dB = 10⋅log(*P*/*P*₀) dB
///     - Thus *y* = *P*₀ and *k* = 10
///     - The absolute value can be reconstructed from the level by:
///       *P* = *y* × 10^(*k*⋅*x*) = *P*₀ × 10^(10⋅*x*)
///  - Any logarithmic power quantity in dB has *k* = 10 regardless of
///    reference value, and any logarithmic root-power quantity has *k* = 20
///  - Different reference values effectively produce different units, as the
///    way to back-calculate the absolute value is dependent on the reference.
///    This is why suffixes are commonly appended to dB to indicate the reference.
///    With an unspecified reference the logarithmic quantity cannot be expressed
///    as an absolute quantity, only as a ratio relative to the unknown reference.
///    This is handled in quanstants by treating a logarithmic quantity defined
///    in plain, unreferenced dB as having *y* = 1.
///
/// Similarly, a base-2 log quantity can be expressed as:
/// - _Β_(_x_, _β_) = _β_(_x_, _y_, _k_, _λ_) = _Λ_(_y_ × 2^(*k*⋅*x*), _λ_)
///
/// …and a natural log quantity as:
/// - _Ε_(_x_, _ε_) = _ε_(_x_, _y_, _k_, _λ_) = _Λ_(_y_ × *e*^(*k*⋅*x*), _λ_)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum ScaleType {
    Base10Log = 1,
    Base2Log = 2,
    NaturalLog = 3,
    Temperature = 4,
}

/// A 128-bit representation of an SI-compatible scale unit.
///
/// A `ScaleUnit128` is simply a [`Uomid`] (Unit of Measure ID) enforced to be a
/// valid value for a scale unit compatible with the SI.
///
/// The vast majority of units normally encountered are linear. Commonly
/// encountered quantities with scale units are temperatures in Celsius and
/// Fahrenheit, logarithmic quantities in decibel, and other logarithmic scales
/// such as pH and stellar magnitude. Use [`Unit128`] for (SI-compatible)
/// linear units.
///
/// Note that non-SI units are not automatically SI-incompatible. Most units,
/// even those belonging to another system, are still expressible in terms of SI
/// base units. The litre is not a unit in the SI, but is of course compatible
/// with it. The foot is not an SI unit, but it can be expressed in terms of SI
/// units with no issue, and there are no problems with compatibility.
///
/// A unit system is incompatible with the SI if it is defined using a different
/// set of base dimensions. CGS systems, however, *are* incompatible with the SI
/// – naive conversion and arithmetic between CGS quantities and SI quantities
/// is not possible.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ScaleUnit128(pub(crate) u128);

// Whether to enable arithmetic for scale units is an open question.
// As they always have a value in linear units, arithmetic could in theory
// be performed via that linear value – but the result is then no longer
// a scale quantity, which may be surprising behaviour.

impl ScaleUnit128 {
    /// Creates a new anonymous scale unit defined in terms of SI base units.
    ///
    /// # Panics
    ///
    /// Panics if either `reference` or `factor` is too large or small to be encoded
    /// (i.e. it is > [`Unit128::MAX_EXPONENT_REFERENCED`] or < [`Unit128::MIN_EXPONENT_REFERENCED`])
    /// or because it is not normal (i.e. it is 0 or infinity or NaN).
    pub fn new(
        scale_type: ScaleType,
        reference: SciDecimal,
        factor: SciDecimal,
        dimensions: Dimensions,
    ) -> Self {
        if !dimensions.all_integer() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Self(
            (Self::reference_and_factor_to_bits(reference, factor).expect("Caller should not pass an unrepresentable value") as u128) << 64
                | (*dimensions.J.numer() as u8 as u128) << 56
                | (*dimensions.N.numer() as u8 as u128) << 48
                | (*dimensions.Θ.numer() as u8 as u128) << 40
                | (*dimensions.I.numer() as u8 as u128) << 32
                | (*dimensions.M.numer() as u8 as u128) << 24
                | (*dimensions.L.numer() as u8 as u128) << 16
                | (*dimensions.T.numer() as u8 as u128) << 8
                // Integer exponents    => bit 7 = 0
                // Scale unit           => bits 6-4 = ?
                | (scale_type as u8 as u128) << 4
                // SI compatible        => bit 3 = 0
                // Uncatalogued         => bits 2-0 = 000
                | 0x0,
        )
    }

    /// Defines a new anonymous temperature scale unit through a degree size and
    /// a zero point, both in kelvin.
    ///
    /// The zero point is generally a positive absolute temperature (though it
    /// is not required to be).
    ///
    /// # Panics
    ///
    /// Also panics if either `zero` or `degree` is too large or small to be encoded
    /// (i.e. it is > [`Unit128::MAX_EXPONENT_REFERENCED`] or < [`Unit128::MIN_EXPONENT_REFERENCED`])
    /// or because it is not normal (i.e. it is 0 or infinity or NaN).
    pub fn new_temp_scale(zero: SciDecimal, degree: SciDecimal) -> Self {
        Self::new(
            ScaleType::Temperature,
            zero,
            degree,
            Dimensions::THERMODYNAMIC_TEMPERATURE,
        )
    }

    /// Attempts to create a scale unit from the corresponding UomID in the form
    /// of a 128-bit integer.
    ///
    /// Fails if the UoMID does not correspond to an SI-compatible scale unit.
    pub fn from_raw(b: u128) -> Result<Self, QuanstantsError> {
        if Uomid(b).is_scale() {
            Ok(Self(b))
        } else {
            Err(QuanstantsError::InvalidId)
        }
    }

    /// Returns the wrapped UoMID as a 128-bit integer.
    #[inline]
    pub fn as_raw(&self) -> u128 {
        self.0
    }

    /// Returns the proportionality factor of the unit.
    #[inline]
    pub fn factor(&self) -> SciDecimal {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        } else {
            Self::bits_to_reference_and_factor((self.as_raw() >> 64) as u64).1
        }
    }

    /// Returns the number of the reference value.
    pub fn reference(&self) -> SciDecimal {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Self::bits_to_reference_and_factor((self.as_raw() >> 64) as u64).0
    }

    /// Returns the SI dimension terms of the reference unit.
    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        if self.has_fractional_dimensions() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Dimensions {
            T: Frac::from(((self.0 >> 8) & 0xFF) as u8 as i8),
            L: Frac::from(((self.0 >> 16) & 0xFF) as u8 as i8),
            M: Frac::from(((self.0 >> 24) & 0xFF) as u8 as i8),
            I: Frac::from(((self.0 >> 32) & 0xFF) as u8 as i8),
            Θ: Frac::from(((self.0 >> 40) & 0xFF) as u8 as i8),
            N: Frac::from(((self.0 >> 48) & 0xFF) as u8 as i8),
            J: Frac::from(((self.0 >> 56) & 0xFF) as u8 as i8),
        }
    }

    /// Returns `true` if the unit has a non-integer exponent in any dimension.
    #[inline]
    pub(crate) fn has_fractional_dimensions(&self) -> bool {
        (self.as_raw() & 0b10000000) != 0
    }

    /// Discards the number uniquely identifying it to afford a normalized
    /// representation defined only in terms of the SI base units.
    #[inline]
    pub fn normalize(self) -> Self {
        // Zero bits 2–0
        Self(self.0 & !0b111)
    }
}

impl fmt::Debug for ScaleUnit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScaleUnit128(0x{:X})", self.as_raw())
    }
}

impl fmt::Display for ScaleUnit128 {
    /// Writes the unit as the hexadecimal representation of the 128-bit UoMID.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.as_raw())
    }
}

impl FromStr for ScaleUnit128 {
    type Err = QuanstantsError;

    /// Creates a new unit from a hexadecimal string representation of the 128-bit UoMID.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("0x").unwrap_or(s);
        let bits = u128::from_str_radix(hex, 16).map_err(|_e| QuanstantsError::Parse(s.into()))?;
        Self::from_raw(bits)
    }
}

#[allow(dead_code)]
impl ScaleUnit128 {
    // Maximum and minimum values for a referenced numeric component, which encodes
    // both a "reference" _y_ and a "factor" _k_.
    pub const MAX_SIGNIFICAND: u64 = 10_u64.pow(7) - 1;

    /// Calculates the 64-bit referenced numeric component that encodes the
    /// provided [`SciDecimal`]s.
    ///
    /// A reference defined by *y* = (−1)^*h* (*b* + 1) × 10^*f* and a proportionality factor
    /// defined by *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e* are encoded by a form of BID decimal
    /// floating point, with:
    ///
    /// - two sign bits, `h` and `g`
    /// - two 24-bit significands with a bias of −1, `b` and `a`
    /// - two `i7` exponents, `f` and `e`
    ///
    /// in the layout: `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
    ///
    /// The significands are limited to 7 full decimal digits of precision.
    /// If either of the numbers have more than 7 significant figures they are first rounded.
    ///
    /// Fails if either number is too large or small to be represented (*e* > [`MAX_EXPONENT_REFERENCED`]
    /// or < [`MIN_EXPONENT_REFERENCED`]) or is not normal (i.e. it is 0 or infinity or NaN).
    pub(crate) fn reference_and_factor_to_bits(
        reference: SciDecimal,
        factor: SciDecimal,
    ) -> Result<u64, QuanstantsError> {
        let ref_bits = Uomid::num_to_bits(reference, 7, Self::MAX_SIGNIFICAND, 24, true)?;

        let factor_bits = Uomid::num_to_bits(factor, 7, Self::MAX_SIGNIFICAND, 24, true)?;

        Ok(ref_bits << 32 | factor_bits)
    }

    // Maximum value for a narrow referenced numeric component, which encodes
    // both a "reference" _y_ and a "factor" _k_.
    // Used when the dimensional component encodes fractional exponents and is therefore widened.
    pub const MAX_SIGNIFICAND_NARROW: u64 = 10_u64.pow(3) - 1;

    /// Calculates the 36-bit referenced numeric component that encodes the
    /// provided [`SciDecimal`]s, with zero padding up to 64 bits.
    ///
    /// A reference defined by *y* = (−1)^*h* (*b* + 1) × 10^*f* and a proportionality factor
    /// defined by *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e* are encoded by a form of BID decimal
    /// floating point, with:
    ///
    /// - two sign bits, `h` and `g`
    /// - two 10-bit significands with a bias of −1, `b` and `a`
    /// - two `i7` exponents, `f` and `e`
    ///
    /// in the layout: `|00000000|00000000|00000000|0000hbbb|bbbbbbbf|ffffffga|aaaaaaaa|aeeeeeee|`
    ///
    /// The significands are limited to 3 full decimal digits of precision.
    /// If either of the numbers have more than 3 significant figures they are first rounded.
    ///
    /// Fails if either number is too large or small to be represented
    /// (*e* > [`MAX_EXPONENT_REFERENCED_NARROW`] or < [`MIN_EXPONENT_REFERENCED_NARROW`])
    /// or is not normal (i.e. it is 0 or infinity or NaN).
    pub(crate) fn reference_and_factor_to_bits_narrow(
        reference: SciDecimal,
        factor: SciDecimal,
    ) -> Result<u64, QuanstantsError> {
        let ref_bits = Uomid::num_to_bits(reference, 3, Self::MAX_SIGNIFICAND_NARROW, 10, true)?;

        let factor_bits = Uomid::num_to_bits(factor, 3, Self::MAX_SIGNIFICAND_NARROW, 10, true)?;

        Ok(ref_bits << 18 | factor_bits)
    }

    /// Determines the [`SciDecimal`]s encoded by a 64-bit referenced numeric component.
    pub(crate) fn bits_to_reference_and_factor(b: u64) -> (SciDecimal, SciDecimal) {
        // Reference and factor have same layout, just bit shifted
        let r = b >> 32; // Most significant 32 bits
        let f = b & 0x00000000_FFFFFFFF; // Least significant 32 bits
        let reference = Uomid::bits_to_num(r, false, 24, true);
        let factor = Uomid::bits_to_num(f, false, 24, true);
        (reference, factor)
    }

    /// Determines the [`SciDecimal`]s encoded by a zero-padded 36-bit referenced numeric component.
    pub(crate) fn bits_to_reference_and_factor_narrow(b: u64) -> (SciDecimal, SciDecimal) {
        // Reference and factor have same layout, just bit shifted
        let r = b >> 18; // Second least significant 18 bits, above that is only zeros
        let f = b & 0b11_11111111_11111111; // Least significant 18 bits
        let reference = Uomid::bits_to_num(r, false, 24, true);
        let factor = Uomid::bits_to_num(f, false, 24, true);
        (reference, factor)
    }
}

impl ScaleUnit128 {
    /// The scale unit of the degree Celsius, symbol **°C**, for temperatures on the Celsius scale.
    ///
    /// A temperature on the Celsius scale *x* °C is related to the absolute temperature
    /// in kelvin by *x* °C = (*x* + 273.15) K
    ///
    /// For the corresponding absolute, linear unit, equal to the kelvin and
    /// used for representing temperature differences and intervals, see
    /// [`Unit128::CELSIUS_DEGREE`].
    pub const DEGREE_CELSIUS: ScaleUnit128 = ScaleUnit128(
        // Numeric component gets referenced layout:
        // `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
        // *y* = (−1)^*h* (*b* + 1) × 10^*f* and *k* = (−1)^*g* (*a* + 1) × 10^*e*
        // For °C:
        // reference y = 273.15 = 27315e-2
        //   => h = 0 (positive)
        //      b = 27315 − 1 = 0b1101010_10110010
        //      f = −2 = 0b11111110 as i8 = 0b1111110 as i7
        //   so encoded as 0b00000000_00110101_01011001_01111110 = 0x00_35_59_7E
        // constant (proportionality factor) k = 1
        //   => g = 0,
        //      b = 1 − 1 = 0,
        //      f = 0 = 0b0 as i8 = 0b0 as i7
        //   so encoded as 4 bytes of zeros 0x00_00_00_00
        //   and encoded for the degree magnitude (a linear unit) as 8 bytes of zeros
        // reference unit ꟛ = kelvin => dim = 0x00_00_01_00_00_00_00
        // Celsius gets catalogue number 1 assigned => lsb = 0x01 for degree magnitude, 0x41 for scale
        // Putting it together (everything below is hex):
        // reference:                   0035597E
        // constant:                             00000000
        // reference unit:                                00_00_01_00_00_00_00
        // least significant byte:                                             n1 (where n = 0 or 4)
        // UoMID for degree magnitude:                    00_00_01_00_00_00_00_01
        // UoMID for scale unit:        0035597E_00000000_00_00_01_00_00_00_00_41
        0x0035597E_00000000_00_00_01_00_00_00_00_41,
    );
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "UnitId")]
    pub struct PyUnitId(pub(crate) Unit128);

    #[pymethods]
    impl PyUnitId {
        #[new]
        fn new(id: u128) -> Self {
            PyUnitId(Unit128(id))
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

        fn to_int(&self) -> u128 {
            self.0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use scinum::sci;
    //use serde::de::Unexpected::Unit;

    use super::*;

    #[test]
    fn factor_to_bits() {
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(1, 0)).unwrap(),
            0x00
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(2, 0)).unwrap(),
            0x100
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(10, 0)).unwrap(), // 10 (2 sf)
            0x900,
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(1, 1)).unwrap(), // 1e1 = 10 (1 sf)
            0x01,
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(1000, 0)).unwrap(), // 1000 (4 sf)
            0x3E700
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(1, 3)).unwrap(), // 1e3 = 1000 (1 sf)
            0x03
        );
        assert_eq!(Unit128::factor_to_bits(sci!(0.1)).unwrap(), 0xFF);
        assert_eq!(Unit128::factor_to_bits(sci!(1e-3)).unwrap(), 0xFD);
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(-1, 0)).unwrap(),
            0x4000_0000_0000_0000
        );
        assert_eq!(
            Unit128::factor_to_bits(SciDecimal::new(-3, 0)).unwrap(),
            0x4000_0000_0000_0200
        );
        assert_eq!(Unit128::factor_to_bits(sci!(0.3048)).unwrap(), 0xBE7FC);
    }

    #[test]
    fn binary_factor_to_bits() {
        assert_eq!(Unit128::binary_factor_to_bits(1_f64).unwrap(), 0x00);
        assert_eq!(Unit128::binary_factor_to_bits(2_f64).unwrap(), 0x100);
        // Test the encodings of the binary prefixes
        // Ki kibi  =  1024^1 =  2^10
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(10)).unwrap(),
            0x8000000000000003
        );
        // Mi mebi  =  1024^2 =  2^20
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(20)).unwrap(),
            0x8000000000000006
        );
        // Gi gibi  =  1024^3 =  2^30
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(30)).unwrap(),
            0x8000000000000009
        );
        // Ti tebi  =  1024^4 =  2^40
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(40)).unwrap(),
            0x800000000000000C
        );
        // Pi pebi  =  1024^5 =  2^50
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(50)).unwrap(),
            0x800000000000000F
        );
        // Ei exbi  =  1024^6 =  2^60
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(60)).unwrap(),
            0x8000000000000012
        );
        // Zi zebi  =  1024^7 =  2^70
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(70)).unwrap(),
            0x8000000000000015
        );
        // Yi yobi  =  1024^8 =  2^80
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(80)).unwrap(),
            0x8000000000000018
        );
        // Ri robi  =  1024^9 =  2^90
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(90)).unwrap(),
            0x800000000000001B
        );
        // Qi quebi = 1024^10 = 2^100
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(100)).unwrap(),
            0x800000000000001E
        );
        // Neg exponent
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(-10)).unwrap(),
            0x80000000000000FD,
        );
        // Max exponent (largest divisible by 3)
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(420)).unwrap(),
            0x800000000000007E,
        );
        // Max value = 2^54 × 2^420 = 18014398509481984 × 1024^(126/3) = 0x3FFFFFFFFFFFFF × 1024^(0x7E/3)
        assert_eq!(
            1_u64 << 63 // Binary bit
            | 0x3FFF_FFFF_FFFF_FF << 8
            | 0x7E,
            0xBFFFFFFFFFFFFF7E,
        );
        assert_eq!(
            Unit128::binary_factor_to_bits(Unit128::MAX_BIN_FACTOR).unwrap(),
            // 0b10111111_11111111_11111111_11111111_11111111_11111111_11111111_01111110
            // i.e. binary bit 1, positive sign, significand all 1s, largest allowed exponent
            0xBFFFFFFFFFFFFF7E,
        );
        // Larger than the max value should fail
        assert!(Unit128::binary_factor_to_bits(Unit128::MAX_BIN_FACTOR + 1_f64).is_err());
        // Zero should fail
        assert!(Unit128::binary_factor_to_bits(0_f64).is_err());
        // An exponent not divisible by 3 should result in an increased precision...
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(14)).unwrap(),
            // Exponent 2^14 becomes 2^10 = 1024^(3/3)
            // Significand 0b1 = 1 becomes 0b10000 = 16 becomes 15 with our bias
            0x8000000000000F03,
        );
        // ...unless the precision is too large, in which case rounding occurs
        assert_eq!(
            Unit128::binary_factor_to_bits(2_f64.pow(53) * 2_f64.pow(14)).unwrap(),
            // Exponent 2^14 becomes 2^20 = 1024^(6/3)
            // Significand 2^53 becomes 2^47 becomes 2^47 - 1 with our bias
            1_u64 << 63 // Binary bit
            | 0x007F_FFFF_FFFF_FF << 8
            | 0x06,
        );
    }

    #[test]
    fn ref_and_factor_to_bits() {
        assert_eq!(
            ScaleUnit128::reference_and_factor_to_bits(sci!(273.15), SciDecimal::ONE).unwrap(),
            0x0035597E_00000000,
        )
    }

    #[test]
    fn bits_to_factor() {
        assert_eq!(Unit128::bits_to_factor(0x0), SciDecimal::new(1, 0));
        assert_eq!(Unit128::bits_to_factor(0x100), SciDecimal::new(2, 0));
        //assert_eq!(Unit128::bits_to_factor(0x1, SciNum::new_exact(10)); // Fails for
        // now, gives:
        assert_eq!(Unit128::bits_to_factor(0x900), SciDecimal::new(10, 0));
        //assert_eq!(Unit128::bits_to_factor(0x3, SciNum::new_exact(1000)); // Fails
        // for now, gives:
        assert_eq!(Unit128::bits_to_factor(0x3E700), SciDecimal::new(1000, 0));
        assert_eq!(Unit128::bits_to_factor(0xFF), sci!(0.1));
        assert_eq!(Unit128::bits_to_factor(0xFD), sci!(1e-3));
        assert_eq!(
            Unit128::bits_to_factor(0x4000_0000_0000_0000),
            SciDecimal::new(-1, 0)
        );
        assert_eq!(
            Unit128::bits_to_factor(0x4000_0000_0000_0200),
            SciDecimal::new(-3, 0)
        );
        assert_eq!(Unit128::bits_to_factor(0xBE7FC), sci!(0.3048));
    }

    #[test]
    fn bits_to_ref_and_factor() {
        assert_eq!(
            ScaleUnit128::bits_to_reference_and_factor(0x0035597E_00000000),
            (sci!(273.15), SciDecimal::ONE),
        )
    }

    #[test]
    fn new() {
        let s = Unit128::new(SciDecimal::ONE, Dimensions::TIME);
        assert_eq!(s, Unit128::SECOND);
        assert_eq!(s.0, Unit128::SECOND.0);
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        // proportionality factor k = 0.3048 = 3048e-4
        //   => g = 0,
        //      b = 3048 − 1 = 3047 = 0xBE7,
        //      f = −4 = 0xFC as i8,
        //   so encoded as 0x00000000000BE7_FC
        // reference unit ꟛ = metre => dim = 0x00_00_00_00_00_10_00
        // least significant byte = 0x00
        // Note that creating a unit like this does not give the catalogued version
        // Actual foot has least significant byte = 0x01
        assert_eq!(ft.0, 0xBE7FC_00000000_00010000);
    }

    #[test]
    fn new_scale() {
        let celsius = ScaleUnit128::new(
            ScaleType::Temperature,
            sci!(273.15),
            SciDecimal::ONE,
            Dimensions::THERMODYNAMIC_TEMPERATURE,
        );
        // Note that creating a unit like this does not give the catalogued version
        // Actual celsius is 0x0035597E_00000000_00_00_01_00_00_00_00_41
        assert_eq!(celsius.0, 0x0035597E_00000000_00_00_01_00_00_00_00_40);
        assert_eq!(celsius, ScaleUnit128::DEGREE_CELSIUS.normalize());
    }

    #[test]
    fn factor() {
        assert_eq!(Unit128::KILOGRAM.factor(), SciDecimal::ONE);
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert_eq!(ft.factor(), sci!(0.3048));
        assert_eq!(ScaleUnit128::DEGREE_CELSIUS.factor(), SciDecimal::ONE);
        // Calling factor() on this was broken, keep as a good example of a
        // number with maximum precision and as a regression test
        // Should correspond to:
        // 4.184^-2 = 0.05712374190670824665757561355… = 5712374190670825 * 10^-17
        let x = Unit128(0x144B5FC27583E8EF_00_00_00_00_02_04_FC_00);
        assert_eq!(x.factor(), sci!(5712374190670825e-17));
    }

    #[test]
    fn reference() {
        assert_eq!(ScaleUnit128::DEGREE_CELSIUS.reference(), sci!(273.15))
    }

    #[test]
    fn dimensions() {
        assert_eq!(Unit128::KILOGRAM.dimensions(), Dimensions::MASS);
        assert_eq!(
            Unit128::KELVIN.dimensions(),
            Dimensions::THERMODYNAMIC_TEMPERATURE
        );
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert_eq!(ft.dimensions(), Dimensions::LENGTH);
    }

    #[test]
    fn is_si_compatible() {
        assert!(Uomid::from(Unit128::ONE).is_si_compatible());
        assert!(Uomid::from(Unit128::SECOND).is_si_compatible());
        assert!(Uomid::from(ScaleUnit128::DEGREE_CELSIUS).is_si_compatible());
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert!(Uomid::from(ft).is_si_compatible());
        // Make up some unit that's deliberately not compatible
        // Key thing is that bit 3 is a 1
        assert!(!Uomid(0x28390A).is_si_compatible());
    }

    #[test]
    fn is_scale() {
        assert!(!Uomid::from(Unit128::ONE).is_scale());
        assert!(!Uomid::from(Unit128::SECOND).is_scale());
        assert!(Uomid::from(ScaleUnit128::DEGREE_CELSIUS).is_scale());
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert!(!Uomid::from(ft).is_scale());
    }

    #[test]
    fn is_linear() {
        assert!(Uomid::from(Unit128::ONE).is_linear());
        assert!(Uomid::from(Unit128::SECOND).is_linear());
        assert!(!Uomid::from(ScaleUnit128::DEGREE_CELSIUS).is_linear());
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert!(Uomid::from(ft).is_linear());
    }

    #[test]
    fn to_from_str() {
        // Test these known examples including round trip
        let s = Unit128::SECOND;
        assert_eq!(s.to_string(), "0x100");
        assert_eq!(Unit128::from_str(&s.to_string()).unwrap(), s);

        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        assert_eq!(ft.to_string(), "0xBE7FC0000000000010000");
        assert_eq!(Unit128::from_str(&ft.to_string()).unwrap(), ft);

        let celsius = ScaleUnit128::DEGREE_CELSIUS;
        assert_eq!(celsius.to_string(), "0x35597E000000000000010000000041");
        assert_eq!(
            ScaleUnit128::from_str(&celsius.to_string()).unwrap(),
            celsius
        );
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Unit128::SECOND), "Unit128(0x100)");
    }

    #[test]
    fn mul() {
        let amp_second = Unit128::AMPERE * Unit128::SECOND;
        let square_metre = Unit128::METRE * Unit128::METRE;
        let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH);
        let square_foot = ft * ft;
        assert_eq!(amp_second.0, 0x100000100);
        assert_eq!(square_metre.0, 0x20000);
        // 0.3048^2 = 0.09290304 = 9290304e-8
        // 9290304 − 1 = 0x8DC23F
        // -8 as i8 = two's complement of 0b1000 = 0b11110111 + 1 = 0b11111000 = 0xF8
        assert_eq!(square_foot.0, 0x8DC23FF8_00_00_00_00_00_02_00_00);
    }
}
