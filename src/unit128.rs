// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    fmt,
    ops::{Div, Mul},
    str::FromStr,
};

use num_traits::{Float, Inv, Pow};
use scinum::{SciDecimal, SciNum};
use serde::{Deserialize, Serialize};

use crate::{dimensions::Dimensions, error::QuanstantsError, fraction::Frac};

/// The mathematical relationship between a written quantity with this unit and
/// the equivalent reference value.
///
/// Represents the information encoded by bits 6–4 of a UoMID (see [`Unit128`]).
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
///     - _k_ is some scaling factor
/// - _Κ_ is thus expressed as a linear quantity _Λ_ by:
///     - _Κ_(_x_, _κ_) = _Λ_(_y_ × 10^(*k*⋅*x*), _λ_)
/// - A logarithmic unit is thus fully defined by _λ_, _y_, and _k_
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

/// A 128-bit encoding of the unit of a quantity,
/// designed for identification, interchange, and arithmetic.
///
/// The unit of a quantity can be entirely described by a 128-bit integer,
/// here called the "unit of measure ID", or "UoMID".
///
/// ## Design
///
/// The choices made mean that UoMIDs of "normal" units (SI-compatible units with integer exponents
/// in the dimensional terms and on a linear scale rather than a logarithmic or temperature scale)
/// written out in hexadecimal can be broadly understood by a human,
/// and all SI base units and coherent derived units are in the range
/// `0x0` to `0xFFFFFFFFFFFFFFFF` with all zeros for the most significant 64 bits.
///
/// ### Overall layout
///
/// The bit layout of a UoMID is divided, from least to most significant, into:
/// - a **scheme component**, indicating how the rest of the UoMID should be interpreted
/// - a **dimensional component**, describing the exponent of each dimension term
/// - a **numeric component**, encoding one or two numbers that define the mathematical relation
///   between a quantity in terms of the unit and a quantity in terms of the reference unit that the
///   unit is defined by
///
/// The **scheme component** is always specified by the least significant 8 bits (bits 7–0).
///
/// The **dimensional** and **numeric** components have variable widths.
///
/// For SI and SI-compatible units (the vast majority), in most cases the rest of the UoMID is
/// divided up as follows:
/// - The **dimensional component** comprises bits 63–8
/// - The **numeric component** comprises bits 127–64
///
/// Sometimes, fractional exponents are necessary for the dimension terms, in which case:
/// - The **dimensional component** comprises an expanded range of bits 91–8
/// - The **numeric component** comprises only bits 127–92
///
/// ### Scheme component
///
/// The first byte (as in, bits 0–7) contains **flags**.
///
/// - Width: 8 bits
/// - Layout: `|prrrsuuu|`
/// - Between them, these flags indicate:
///     1. Unit system/compatibility
///         - SI compatibility
///         - Whether the unit is a catalogued, uniquely identifiable unit or
///           simply a normalized representation in SI base units
///     2. How to interpret the numeric component
///         - Decimal vs binary numeric factor
///         - Wide number vs two narrow numbers
///         - Integer vs fractional exponents
///     3. How to relate the unit to a quantity
///         - Linear scales (normal units) vs non-linear scales (referenced units)
///         - What equation the quantity obeys
/// - Bits 0–3 classify the unit (1. above).
/// - Bits 4–7 indicate the scheme for the rest of the UoMID (2. and 3. above).
/// - Bit 3 `s` indicates whether the unit is SI-compatible or not.
///     - If `s == 0`:
///         - The unit (and quantity) are compatible with the SI.
///         - A value for `uuu` of a value of `0b000` indicates either that it is
///           the base unit itself, or that the unit is not catalogued and cannot
///           be uniquely identified, and so has the encoded value in SI base units.
///         - Other values of `uuu` uniquely identify the unit as a specific catalogued unit.
///     - If `s == 1`, the bits 0–2 indicate the alternative system that the unit belongs to.
///       The way the other bits are interpreted is determined by this value. At present, which
///       SI-incompatible system is represented by each possibility is not specified.
///     - All 1s i.e. `0b1111` for `suuu` is a reserved value.
///     - The upshot is: hex values of `0` to `7` for the least significant nibble are used for
///       SI-compatible units, while `8` to `E` are for SI-incompatible units, with `F` reserved.
///     - Note that non-SI units are not automatically SI-incompatible. A foot is not an SI unit,
///       but it can be expressed in terms of SI units with no issue, and there are no problems
///       with compatibility. CGS systems, however, *are* incompatible with the SI – naive
///       conversion and arithmetic between CGS quantities and SI quantities is not possible.
/// - The way the rest of the UoMID should be interpreted is not yet defined for non-SI systems,
///   and will in any case be specific to each alternative system. The rest of this documentation
///   concerns itself only with SI-compatible units i.e. where bit 3 `s == 0`.
/// - Bits 6–4 `rrr` indicate the type of scale a quantity with the unit uses:
///     - `0b000` indicates a normal linear scale; the numeric factor encodes a single number.
///     - All other combinations indicate a specific non-linear scale, four of which are currently
///       considered:
///         - `0b001` -> logarithmic, base 10, *x* *κ* = (*y* × 10^(*k*⋅*x*)) *λ*
///         - `0b010` -> logarithmic, base 2, *x* *β* = (*y* × 2^(*k*⋅*x*)) *λ*
///         - `0b011` -> logarithmic, base *e*, *x* *ε* = (*y* × *e*^(*k*⋅*x*)) *λ*
///         - `0b100` -> temperature, *x* *θ* = (*k*(*x* + *y*)) *λ*
///     - See [`ScaleType`] for more details on the mathematical relationships.
///     - The bit combinations are chosen to allow for mnemonics (for bits 7–4, with no fractional
///       exponents,`0x1` indicates base 10, `0x2` base 2, `0x3` base *e*).
///     - For temperature scales, the scale unit and the degree unit for a scale are related by a
///       single bit flip (of bit 6).
/// - Bit 7 `p` is a flag that indicates whether an array of denominators for fractional exponents
///   is present or not.
///     - If `p == 1`, the least significant 28 bits of the numeric component are used to encode the
///       denominators as 4-bit unsigned integers, and the fields of the numeric component are
///       narrowed to compensate.
/// - UoMIDs in which the four least significant bits are all 1s are currently invalid, with those
///   combinations reserved for special values.
///     - For example, a least significant byte of `0xFF` might be used as a continuation byte
///       should a variable-width encoding turn out to be necessary.
///
///
/// ### Dimensional component
///
/// #### Without fractional dimensional exponents
///
/// - Width: 56 bits
/// - Layout: `|JJJJJJJJJ|NNNNNNNN|ΘΘΘΘΘΘΘΘ|IIIIIIII|MMMMMMMM|LLLLLLLL|TTTTTTTT|`
/// - Bits 8–63 encode the exponents for each SI dimension with one byte per dimension,
///   in the order shown above.
/// - Each byte is interpreted simply as a signed 8-bit integer `i8`.
/// - The commonly used integer exponents are thus easily memorized
///   (¹, ², ³ are `0x01, `0x02`, `0x03` respectively; ⁻¹, ⁻², ⁻³ are `0xFF`, `0xFE`, `0xFD`)
///   enabling manual comprehension and composition.
///
/// #### With fractional dimensional exponents
///
/// - Width: 84 bits
/// - Layout: `jjjj|nnnnθθθθ|iiiimmmm|lllltttt|JJJJJJJJJ|NNNNNNNN|ΘΘΘΘΘΘΘΘ|IIIIIIII|MMMMMMMM|LLLLLLLL|TTTTTTTT|`
/// - Bits 8–63 encode the numerators of the exponents for each SI dimension as signed 8-bit
///   integers `i8`.
/// - Bits 91–64 encode the denominators of the exponents as unsigned 4-bit integers (i.e. `u4`)
///
///
/// ### Numeric component
///
/// The numeric component may either be:
/// 1. A **simple numeric component**, encoding a single number
/// 2. A **referenced numeric component**, encoding two numbers
///
/// In most cases the numeric component is simple, with referenced ones used for non-linear units
/// (logarithmic, temperature).
///
/// The numbers are encoded as decimal floats in a BID fashion.
/// When the dimensional exponents do not need to be fractional and only a simple numeric component
/// needs to be encoded there is also the possibility of a binary-like float encoding.
///
/// The layout of the numeric component is determined by the value of bits 7–4 in the scheme
/// component, and the layouts indicated by the possible values of that nibble are summarized below:
///
/// | Bits 7–4 | Hex | Scale  | Bit width | Bit range | Sign bits | Binary bits | Exp. bits | Sig. bits | Sig. digits |
/// | -------- | --- | ------ | --------- | --------- | --------- | ----------- | --------- | --------- | ----------- |
/// | `0000`   | `0` | linear | 64        | 127–64    | 1         | 1           | 8         | 54        | 16          |
/// | `0001`   | `1` | log10  | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
/// | `0010`   | `2` | log2   | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
/// | `0011`   | `3` | ln     | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
/// | `0100`   | `4` | temp.  | 64        | 127–64    | 1         | 0           | 7         | 24        | 7           |
/// | `0101`   | `5` |        |           |           |           |             |           |           |             |
/// | `0110`   | `6` |        |           |           |           |             |           |           |             |
/// | `0111`   | `7` |        |           |           |           |             |           |           |             |
/// | `1000`   | `8` | linear | 36        | 127–92    | 1         | 0           | 8         | 27        | 8           |
/// | `1001`   | `9` | log10  | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
/// | `1010`   | `A` | log2   | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
/// | `1011`   | `B` | ln     | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
/// | `1100`   | `C` | temp.  | 36        | 127–92    | 1         | 0           | 7         | 10        | 3           |
/// | `1101`   | `D` |        |           |           |           |             |           |           |             |
/// | `1110`   | `E` |        |           |           |           |             |           |           |             |
/// | `1111`   | `F` |        |           |           |           |             |           |           |             |
///
/// #### Simple numeric component
///
/// ##### Without fractional dimensional exponents
///
/// A 64-bit encoding of the proportionality factor, similar to both IEEE 754 floating point and
/// traditional scientific notation, with either a decimal or binary-like exponential factor.
///
/// - Width: 64 bits
/// - Layout: `|wgaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaaaaa|eeeeeeee|`
/// - `w` is a flag to indicate the use of the decimal (`0`) or the binary-like (`1`) encoding
/// - Encodes the proportionality factor *k* by:
///     - Decimal encoding: *k* = (−1)^*g* (*a* + 1) × 10^*e*
///     - Binary-like encoding: *k* = (−1)^*g* (*a* + 1) × 1024^(*e*/3)
/// - The bit pattern is much simpler than IEEE 754 `decimal64` but has the same precision (16 full
///   decimal digits), achieved by reducing the exponent field to 8 bits (with the consequence that
///   the range is somewhat reduced in comparison).
/// - `e` is the exponent, encoded (*with no bias*) by the least significant byte as an `i8`
///     - Exponents can range from −128 to 127.
///     - The base of the exponential term is indicated as decimal or binary by `w` as described.
///     - Under the usual decimal scheme the value of the exponential term is simply 10^*e*.
///     - Under the "binary-like" scheme the value of the exponential term is instead 1024^(*e*/3).
///         - For traditional binary floating point it would be 2^*e*, hence "binary-like".
///         - For a binary number the encoded `e` is thus actually `<exponent> * 3`; this is done so
///           that analogous metric and binary prefixes are encoded by the same value/bit pattern
///           e.g. kilo and kibi are both `0x03`.
///         - The exponent in the binary form is thus constrained to multiples of 3.
///         - This makes the maximum value (where `e == 126`) 1024^42 = 2^420 ≈ 10^126.
///         - Thus, importantly, the range of the binary encoding falls within the range of the
///           decimal encoding; this makes processing much easier.
///         - This may seem too low, as `f64` has a max value of 1.80×10^308 and IEEE `decimal64`
///           can go up to 1.0×10^385. However, the numeric component is only used to specify the
///           value of units, not for the number of an actual quantity. The largest and smallest
///           current SI prefixes are quetta = 10^30 and quecto = 10^−30 respectively, so the
///           possible range allowed for by the design covers all realistically necessary factors of
///           SI base units by some way.
/// - `a` encodes the significand as a 54-bit unsigned binary integer *with a bias of −1*.
///     - Enables 16 full decimal digits of precision, matching IEEE `decimal64` and [`SciDecimal`],
///       and enough to cover the maximum precision of `f64`.
///     - The bias means that `a` is actually `<significand> - 1`.
///     - The bias was chosen so that `+1` is encoded as `g = 0, a = 0` => 7 bytes of zeros.
///     - The maximum value allowed for the significand is 10^16 − 1.
///
/// A proportionality factor of +1 – the case for all SI base units, and products and combinations
/// thereof – has `w = 0, g = 0, a = 0, e = 0`, corresponding to 8 bytes of zeros, giving coherent
/// SI units nice short IDs.
///
/// A proportionality factor of a power of 10, such as the decimal prefixes, have short and easily
/// understood encodings, for example:
///
/// | Hex                | Value                                  |
/// | ------------------ | -------------------------------------- |
/// |               `00` | 1e0 = 1                                |
/// |              `100` | 2e0 = 2                                |
/// |               `01` | 1e1 = 10 (with 1 s.f.)                 |
/// |              `900` | 10e0 = 10 (with 2 s.f.)                |
/// |               `03` | 1e3 = 1000^1 = kilo                    |
/// |            `3E700` | 1000e0 = 1000 as well, but with 3 s.f. |
/// |               `06` | 1e6 = 1000^2 = mega                    |
/// |               `1E` | 1e30 = 1000^10 = quetta                |
/// |               `78` | 1e120 = ??!                            |
/// |               `7E` | 1e126 = maximum exponent               |
/// |               `FD` | 1e-3 = milli                           |
/// | `4000000000000000` | -1e0 = −1                              |
/// | `4000000000000100` | -2e0 = −2                              |
/// | `4000000000000003` | -1e3 = −1000                           |
///
/// while the binary prefixes have encodings that match the corresponding decimal ones neatly, with
/// just a single bit flip (at bit 127):
///
/// | Hex                | Value                          |
/// | ------------------ | ------------------------------ |
/// | `8000000000000003` | 1 × 1024 = kibi                |
/// | `8000000000000006` | 1 × 1024^2 = mebi              |
/// | `800000000000000C` | 1 × 1024^4 = tebi              |
/// | `800000000000001E` | 1 × 1024^10 = quebi            |
/// | `8000000000000078` | 1 × 1024^40 = ??!              |
/// | `800000000000007E` | 1 × 1024^42 = maximum exponent |
/// | `C000000000000003` | −1 × 1024^1 = −1024            |
///
/// ##### With fractional dimensional exponents
///
/// Broadly the same as the normal simple encoding, with the following differences:
/// - The bit width is reduced to 36 bits
/// - The significand is reduced to a bit width of 27 bits
/// - There is no binary flag bit `w` – binary numeric exponents are therefore not possible with
///   anything other than a linear unit with integer dimensional exponents
/// - With 27 bits for the significand (which still uses a bias of −1), only 8 full decimal digits
///   of precision is possible.
/// - The maximum value allowed for the significand is 10^8 − 1.
///
/// - Width: 36 bits
/// - Layout: `|gaaaaaaa|aaaaaaaa|aaaaaaaa|aaaaeeee|eeee`
///
/// #### Referenced numeric component
///
/// ##### Without fractional dimensional exponents
///
/// Used to describe a non-linear or scale quantity: a quantity written as *x* *u*, where *x* is the
/// number and *u* is a non-linear unit, with the meaning that the quantity is a function
/// *Q*(*x*, *u*) = *u*(*x*), and *u* is a function of the form *f*(*x*, *y*, *k*, *λ*) where the
/// values of *y*, *k*, and *λ*  define the unit (see [`ScaleType`]).
///
/// The applicable function *f* and therefore the appropriate interpretation of *y* and *k* is
/// indicated by the scheme component (see above).
///
/// - Width: 64 bits
/// - Layout: `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
/// - Encoded in essentially the same way as the simple numeric component but as two 32-bit numbers.
///     - The more significant half encodes *y* (the reference value of the scale),
///       and the less significant half *k*.
/// - Neither number has a binary flag bit `w`; only decimal encoding is possible, and the base for
///   the exponents is always 10.
/// - The exponents are reduced to a bit-width of 7 (with two's complement, making them effectively
///   `i7`s)
/// - This allows 7 full decimal digits of precision in the significand,
/// - The four least significant bytes encode *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e*
/// - The four most significant bytes encode *y* as *y* = (−1)^*h* (*b* + 1) × 10^*f*
/// - With 24 bits for the significand (which still uses a bias of −1), 7 full decimal digits of
///   precision is possible, matching IEEE 754's `decimal32` and similar to `f32`.
/// - The maximum value allowed for the significand is 10^7 − 1.
///
/// ##### With fractional dimensional exponents
///
/// Broadly the same as the normal referenced encoding, with the following differences:
///     - The bit width of each number is reduced to 18 bits.
///
/// - Width: 36 bits
/// - Layout: `hbbbbbbb|bbbfffff|ffgaaaaa|aaaaaeee|eeee`
/// - The exponents are, as before, encoded with 7 rather than 8 bits.
/// - With 10 remaining bits for the significand (which still uses a bias of −1), only 3 full
///   decimal digits of precision is possible.
/// - The maximum value allowed for the significand is 999.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Unit128(pub u128);

impl Unit128 {
    /// Creates a new anonymous unit defined in terms of SI base units.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is too large or small to be encoded
    /// (i.e. it is > [`Unit128::MAX_EXPONENT_SIMPLE`] or < [`Unit128::MIN_EXPONENT_SIMPLE`])
    /// or because it is not finite (i.e. it is infinity or NaN).
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

    /// Creates a new anonymous scale unit defined in terms of SI base units.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is too large or small to be encoded
    /// (i.e. it is > [`Unit128::MAX_EXPONENT_REFERENCED`] or < [`Unit128::MIN_EXPONENT_REFERENCED`])
    /// or because it is not finite (i.e. it is infinity or NaN).
    pub fn new_scale(
        scale_type: ScaleType,
        reference: SciDecimal,
        constant: SciDecimal,
        dimensions: Dimensions,
    ) -> Self {
        if !dimensions.all_integer() {
            todo!("Fractional dimensional exponents are not yet implemented!")
        }
        Self(
            (Unit128::reference_and_constant_to_bits(reference, constant).expect("Caller should not pass an unrepresentable value") as u128) << 64
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

    /// Creates a new compound unit from a set of factors.
    ///
    /// # Panics
    ///
    /// This function panics if any of the units are scale units or are not
    /// SI-compatible, or if the proportionality factor is so large or small that
    /// it can no longer be represented.
    pub fn new_compound(factors: &[(Unit128, Frac)]) -> Self {
        // If a unit is not a scale unit and it is compatible with the SI then
        // it has a least significant byte in the range 0x00 to 0x07.
        if factors
            .iter()
            .any(|x| x.0.is_scale() || !x.0.is_si_compatible())
        {
            panic!("Compound units may only be created from linear, SI-compatible units!")
        };
        let dimensions = factors
            .iter()
            .map(|x| x.0.dimensions().pow(x.1))
            .fold(Dimensions::DIMENSIONLESS, |acc, x| acc * x);
        // Do this way, rather than by multiplying successive Unit128s, in order to
        // avoid constant conversion back and forth to SciDecimal
        let proportionality_factor = factors
            .iter()
            .map(|x| {
                x.0.factor()
                    .expect("We already checked it's a linear SI unit")
                    .pow(x.1)
            })
            .fold(SciDecimal::ONE, |acc, x| acc * x);
        Self::new(proportionality_factor, dimensions)
    }

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

    /// Returns an enum indicating whether the unit is SI compatible, and if so, whether the unit
    /// is anonymous/defined in base units or a catalogued unit.
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

    /// Returns `true` if the unit is compatible with the SI.
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

    /// Returns `true` if the unit is a linear unit, not a scale unit.
    ///
    /// For SI-compatible units, a linear unit must have `000` for bits 6–4.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible.
    #[inline]
    pub fn is_linear(&self) -> bool {
        if !self.is_si_compatible() {
            panic!("This method is not implemented for units incompatible with the SI")
        }
        self.0 & 0b01110000 != 0
    }

    /// Returns `true` if the unit is a scale unit, not a linear unit.
    ///
    /// For SI-compatible units, a scale unit is any `Unit128` with anything other
    /// than `000` for bits 6–4.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible.
    #[inline]
    pub fn is_scale(&self) -> bool {
        !self.is_linear()
    }

    /// Returns the SI dimension terms of the unit.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible.
    #[inline]
    pub fn dimensions(&self) -> Dimensions {
        if !self.is_si_compatible() {
            panic!("This method is not implemented for units incompatible with the SI")
        }
        if (self.0 & 0b10000000) != 0 {
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

    /// Returns the proportionality factor of a linear, SI-compatible unit, or `None` otherwise.
    #[inline]
    pub fn factor(&self) -> Option<SciDecimal> {
        if !self.is_si_compatible() {
            None
        } else if (self.0 & 0b10000000) != 0 {
            todo!("Fractional dimensional exponents are not yet implemented!")
        } else if self.is_linear() {
            Some(Unit128::bits_to_factor((self.0 >> 64) as u64))
        } else {
            None
        }
    }

    /// Returns the reference and constant values of a SI-compatible scale unit, or `None` otherwise.
    pub fn reference(&self) -> Option<(SciDecimal, SciDecimal)> {
        if !self.is_si_compatible() {
            None
        } else if (self.0 & 0b10000000) != 0 {
            todo!("Fractional dimensional exponents are not yet implemented!")
        } else if self.is_scale() {
            Some(Self::bits_to_reference_and_constant((self.0 >> 64) as u64))
        } else {
            None
        }
    }

    /// For a catalogued SI-compatible unit, discards the number uniquely identifying to afford a
    /// normalized representation defined only in terms of the SI base units.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible.
    pub fn normalize(self) -> Self {
        if !self.is_si_compatible() {
            panic!("This method is not implemented for units incompatible with the SI")
        }
        // Zero bits 2–0
        Self(self.0 & !0b111)
    }

    /// Calculates the inverse of a linear, SI-compatible unit.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible, or if it is a scale unit.
    pub fn inverse(self) -> Unit128 {
        if !self.is_si_compatible() {
            panic!("This operation is not possible for units incompatible with the SI")
        } else if self.is_scale() {
            todo!("This method is not implemented for scale units")
            // Whether to enable arithmetic for scale units is an open question (see Mul)
        } else {
            Unit128::new(
                self.factor()
                    .expect("We already checked it's a linear SI unit")
                    .inv(),
                self.dimensions().inverse(),
            )
        }
    }

    /// Raises a linear, SI-compatible unit to the given power.
    ///
    /// # Panics
    ///
    /// Panics if the unit is not SI-compatible, or if it is a scale unit.
    pub fn pow<T: Into<Frac>>(self, exponent: T) -> Unit128 {
        if !self.is_si_compatible() {
            panic!("This operation is not possible for units incompatible with the SI")
        } else if self.is_scale() {
            todo!("This method is not implemented for scale units")
            // Whether to enable arithmetic for scale units is an open question (see Mul)
        } else {
            let exp: Frac = exponent.into();
            Unit128::new(
                self.factor()
                    .expect("We already checked it's a linear SI unit")
                    .pow(exp),
                self.dimensions().pow(exp),
            )
        }
    }
}

impl Mul for Unit128 {
    type Output = Self;

    /// Multiplies two linear, SI-compatible units.
    ///
    /// # Panics
    ///
    /// Panics if either unit is not SI-compatible, or is a scale unit.
    fn mul(self, rhs: Unit128) -> Unit128 {
        if !self.is_si_compatible() || !rhs.is_si_compatible() {
            panic!("This operation is not possible for units incompatible with the SI")
        } else if self.is_scale() || rhs.is_scale() {
            todo!("This method is not implemented for scale units")
            // Whether to enable arithmetic for scale units is an open question.
            // As they always have a value in linear units, arithmetic could in theory
            // be performed via that linear value – but the result is then no longer
            // a scale quantity, which may be surprising behaviour.
        } else {
            Unit128::new(
                self.factor()
                    .expect("We already checked it's a linear SI unit")
                    * rhs
                        .factor()
                        .expect("We already checked it's a linear SI unit"),
                self.dimensions() * rhs.dimensions(),
            )
        }
    }
}

impl Div for Unit128 {
    type Output = Self;

    /// Divides the unit by `rhs`, where both are linear, SI-compatible units.
    ///
    /// # Panics
    ///
    /// Panics if either unit is not SI-compatible, or is a scale unit.
    fn div(self, rhs: Unit128) -> Unit128 {
        if !self.is_si_compatible() || !rhs.is_si_compatible() {
            panic!("This operation is not possible for units incompatible with the SI")
        } else if self.is_scale() || rhs.is_scale() {
            todo!("This method is not implemented for scale units")
            // Whether to enable arithmetic for scale units is an open question (see Mul)
        } else {
            Unit128::new(
                self.factor()
                    .expect("We already checked it's a linear SI unit")
                    / rhs
                        .factor()
                        .expect("We already checked it's a linear SI unit"),
                self.dimensions() / rhs.dimensions(),
            )
        }
    }
}

impl fmt::Debug for Unit128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit128(0x{:X})", self.0)
    }
}

impl fmt::Display for Unit128 {
    /// Writes the unit as the hexadecimal representation of the 128-bit UoMID.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

impl FromStr for Unit128 {
    type Err = QuanstantsError;

    /// Creates a new unit from a hexadecimal string representation of the 128-bit UoMID.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("0x").unwrap_or(s);
        let bits = u128::from_str_radix(hex, 16).map_err(|_e| QuanstantsError::Parse(s.into()))?;
        Ok(Self(bits))
    }
}

/// Casts a (pre-validated) `i16` to the equivalent `i7` (as a `u8` with a leading 0).
///
/// This function relies on the `i16` having already been confirmed as fitting
/// into an `i7` (i.e. it is in the range `-63..=64`).
fn i16_to_i7(n: i16) -> u8 {
    // Can't just cast to `i8` and then `u8`, need to move the sign bits
    let n = n as u16;
    let sign = ((n & 0b1000000000000000) >> 9) as u8; // Bit 15 moved to bit 6
    let value = (n & 0b0000000000111111) as u8; // Bits 5–0 only
    sign | value
}

/// Casts an `i7` (as a `u8` with a leading 0) to the equivalent `i16`.
fn i7_to_i16(n: u8) -> i16 {
    let sign = ((n & 0b01000000) << 9) as u16; // Bit 6 moved to bit 15
    let value = (n & 0b00111111) as u16; // Bits 5–0 only
    (sign | value) as i16
}

#[allow(dead_code)]
// Functions for converting between `SciDecimal`s and the 64-bit numeric component
// of `Unit128`
impl Unit128 {
    // Maximum and minimum values for a simple 64-bit numeric component, which just encodes
    // the proportionality factor _k_.
    pub const MAX_SIGNIFICAND_SIMPLE: i64 = 10_i64.pow(16) - 1;
    pub const MIN_SIGNIFICAND_SIMPLE: i64 = -(10_i64.pow(16) - 1);
    pub const MAX_EXPONENT_SIMPLE: i8 = i8::MAX;
    pub const MIN_EXPONENT_SIMPLE: i8 = i8::MIN;

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
    /// < [`MIN_EXPONENT_SIMPLE`]) or is not finite (i.e. it is infinity or NaN).
    pub(crate) fn factor_to_bits(factor: SciDecimal) -> Result<u64, QuanstantsError> {
        if !factor.is_finite() {
            Err(QuanstantsError::Range)
        } else if let Ok(exp) = i8::try_from(factor.exponent()) {
            Ok((factor.sign() as u64) << 62 | (factor.significand() - 1) << 8 | exp as u8 as u64)
        } else {
            Err(QuanstantsError::Range)
        }
    }

    // Maximum and minimum values for a narrow 36-bit numeric component, which just encodes
    // the proportionality factor _k_.
    // Used when the dimensional component encodes fractional exponents and is therefore widened.
    pub const MAX_SIGNIFICAND_SIMPLE_NARROW: i64 = 10_i64.pow(8) - 1;
    pub const MIN_SIGNIFICAND_SIMPLE_NARROW: i64 = -(10_i64.pow(8) - 1);
    pub const MAX_EXPONENT_SIMPLE_NARROW: i8 = i8::MAX;
    pub const MIN_EXPONENT_SIMPLE_NARROW: i8 = i8::MIN;

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
    /// or is not finite (i.e. it is infinity or NaN).
    pub(crate) fn factor_to_bits_narrow(factor: SciDecimal) -> Result<u64, QuanstantsError> {
        if !factor.is_finite() {
            return Err(QuanstantsError::Range);
        }

        let shortened: SciDecimal =
            if factor.significand() > Unit128::MAX_SIGNIFICAND_SIMPLE_NARROW as u64 {
                factor.round_sf(8, scinum::RoundingMode::HalfUp)
            } else {
                factor
            };

        if let Ok(exp) = i8::try_from(shortened.exponent()) {
            Ok(
                (shortened.sign() as u64) << 35
                    | (factor.significand() - 1) << 8
                    | exp as u8 as u64,
            )
        } else {
            Err(QuanstantsError::Range)
        }
    }

    // Maximum and minimum values for a referenced numeric component, which encodes
    // both a "reference" _y_ and a "constant" _k_.
    pub const MAX_SIGNIFICAND_REFERENCED: i64 = 10_i64.pow(7) - 1;
    pub const MIN_SIGNIFICAND_REFERENCED: i64 = -(10_i64.pow(7) - 1);
    pub const MAX_EXPONENT_REFERENCED: i8 = 63;
    pub const MIN_EXPONENT_REFERENCED: i8 = -64;

    /// Calculates the 64-bit referenced numeric component that encodes the
    /// provided [`SciDecimal`]s.
    ///
    /// A reference defined by *y* = (−1)^*h* (*b* + 1) × 10^*f* and a constant defined by
    /// *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e* are encoded by a form of BID decimal floating
    /// point, with:
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
    /// Fails if `factor` is too large or small to be represented (*e* > [`MAX_EXPONENT_REFERENCED`]
    /// or < [`MIN_EXPONENT_REFERENCED`]) or is not finite (i.e. it is infinity or NaN).
    pub(crate) fn reference_and_constant_to_bits(
        reference: SciDecimal,
        constant: SciDecimal,
    ) -> Result<u64, QuanstantsError> {
        if !(reference.is_finite() && constant.is_finite()) {
            return Err(QuanstantsError::Range);
        }

        let shortened_reference: SciDecimal =
            if reference.significand() > Unit128::MAX_SIGNIFICAND_REFERENCED as u64 {
                reference.round_sf(7, scinum::RoundingMode::HalfUp)
            } else {
                reference
            };
        let shortened_constant: SciDecimal =
            if constant.significand() > Unit128::MAX_SIGNIFICAND_REFERENCED as u64 {
                constant.round_sf(7, scinum::RoundingMode::HalfUp)
            } else {
                constant
            };

        // First check that the exponents are representable as `i7`s
        // That allows us to use our `i16_to_i7()` function without worry
        let allowed_exp =
            (Unit128::MIN_EXPONENT_REFERENCED as i16)..=(Unit128::MAX_EXPONENT_REFERENCED as i16);
        if !(allowed_exp.contains(&reference.exponent())
            && allowed_exp.contains(&constant.exponent()))
        {
            return Err(QuanstantsError::Range);
        }

        Ok((shortened_reference.sign() as u64) << 63
            | (shortened_reference.significand() - 1) << 39
            | (i16_to_i7(shortened_reference.exponent()) as u64) << 32
            | (shortened_constant.sign() as u64) << 31
            | (shortened_constant.significand() - 1) << 7
            | (i16_to_i7(shortened_constant.exponent()) as u64))
    }

    // Maximum and minimum values for a narrow referenced numeric component, which encodes
    // both a "reference" _y_ and a "constant" _k_.
    // Used when the dimensional component encodes fractional exponents and is therefore widened.
    pub const MAX_SIGNIFICAND_REFERENCED_NARROW: i64 = 10_i64.pow(3) - 1;
    pub const MIN_SIGNIFICAND_REFERENCED_NARROW: i64 = -(10_i64.pow(3) - 1);
    pub const MAX_EXPONENT_REFERENCED_NARROW: i8 = 63;
    pub const MIN_EXPONENT_REFERENCED_NARROW: i8 = -64;

    /// Calculates the 36-bit referenced numeric component that encodes the
    /// provided [`SciDecimal`]s, with zero padding up to 64 bits.
    ///
    /// A reference defined by *y* = (−1)^*h* (*b* + 1) × 10^*f* and a constant defined by
    /// *k* as *k* = (−1)^*g* (*a* + 1) × 10^*e* are encoded by a form of BID decimal floating
    /// point, with:
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
    /// Fails if `factor` is too large or small to be represented
    /// (*e* > [`MAX_EXPONENT_REFERENCED_NARROW`] or < [`MIN_EXPONENT_REFERENCED_NARROW`])
    /// or is not finite (i.e. it is infinity or NaN).
    pub(crate) fn reference_and_constant_to_bits_narrow(
        reference: SciDecimal,
        constant: SciDecimal,
    ) -> Result<u64, QuanstantsError> {
        if !(reference.is_finite() && constant.is_finite()) {
            return Err(QuanstantsError::Range);
        }

        let shortened_reference: SciDecimal =
            if reference.significand() > Unit128::MAX_SIGNIFICAND_REFERENCED_NARROW as u64 {
                reference.round_sf(3, scinum::RoundingMode::HalfUp)
            } else {
                reference
            };
        let shortened_constant: SciDecimal =
            if constant.significand() > Unit128::MAX_SIGNIFICAND_REFERENCED_NARROW as u64 {
                constant.round_sf(3, scinum::RoundingMode::HalfUp)
            } else {
                constant
            };

        // First check that the exponents are representable as `i7`s
        // That allows us to use our `i16_to_i7()` function without worry
        let allowed_exp = (Unit128::MIN_EXPONENT_REFERENCED_NARROW as i16)
            ..=(Unit128::MAX_EXPONENT_REFERENCED_NARROW as i16);
        if !(allowed_exp.contains(&reference.exponent())
            && allowed_exp.contains(&constant.exponent()))
        {
            return Err(QuanstantsError::Range);
        }

        Ok((shortened_reference.sign() as u64) << 35
            | (shortened_reference.significand() - 1) << 25
            | (i16_to_i7(shortened_reference.exponent()) as u64) << 18
            | (shortened_constant.sign() as u64) << 17
            | (shortened_constant.significand() - 1) << 7
            | (i16_to_i7(shortened_constant.exponent()) as u64))
    }

    /// Determines the [`SciDecimal`] encoded by a 64-bit numeric component.
    ///
    /// If the numeric component uses the binary encoding, the result may lose
    /// some precision.
    pub(crate) fn bits_to_factor(b: u64) -> SciDecimal {
        const BINARY_MASK: u64 =
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        const SIGN_MASK: u64 =
            0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        const EXPONENT_MASK: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
        let sign = b & SIGN_MASK;
        let significand = ((b & !SIGN_MASK & !BINARY_MASK) >> 8) + 1;
        let binary = (b & BINARY_MASK) != 0;
        if !binary {
            SciDecimal::new(
                (sign | significand) as i64,
                (b & EXPONENT_MASK) as i8 as i16, // Casting larger to smaller truncates, smaller to larger sign extends
            )
        } else {
            // First just make the significand into a number i.e. *n* = (−1)^*g* (*a* + 1)
            let n = SciDecimal::new((sign | significand) as i64, 0);
            // Create the exponent term as a second SciDecimal
            // Per the specification, the exponent is encoded as the actual value multiplied by 3
            let e = SciDecimal::new(1024, 0).powi((b & EXPONENT_MASK) as i8 as i32 / 3);
            // The result will always fit into a `SciDecimal`,
            // but full precision will not always be possible
            n * e
        }
    }

    /// Determines the [`SciDecimal`] encoded by a zero-padded 36-bit numeric component.
    pub(crate) fn bits_to_factor_narrow(b: u64) -> SciDecimal {
        // This case is even simpler than the normal case, as there's no possibility of a binary encoding
        const SIGN_MASK: u64 = 0b1000_00000000_00000000_00000000_00000000;
        const EXPONENT_MASK: u64 = 0b0000_00000000_00000000_00000000_11111111;
        let sign = (b & SIGN_MASK) << 28;
        let significand = ((b & !SIGN_MASK) >> 8) + 1;
        SciDecimal::new(
            (sign | significand) as i64,
            (b & EXPONENT_MASK) as i8 as i16, // Casting larger to smaller truncates, smaller to larger sign extends
        )
    }

    /// Determines the [`SciDecimal`]s encoded by a 64-bit referenced numeric component.
    pub(crate) fn bits_to_reference_and_constant(b: u64) -> (SciDecimal, SciDecimal) {
        const SIGN_MASK: u64 = 0b10000000_00000000_00000000_00000000;
        const EXPONENT_MASK: u64 = 0b00000000_00000000_00000000_01111111;
        let r = b >> 32; // Reference has same layout as constant, just bit shifted
        let reference_sign = (r & SIGN_MASK) << 32;
        let reference_significand = ((r & !SIGN_MASK) >> 7) + 1;
        let reference = SciDecimal::new(
            (reference_sign | reference_significand) as i64,
            i7_to_i16((r & EXPONENT_MASK) as u8),
        );
        let constant_sign = (b & SIGN_MASK) << 32;
        let constant_significand = ((b & !SIGN_MASK) >> 7) + 1;
        let constant = SciDecimal::new(
            (constant_sign | constant_significand) as i64,
            i7_to_i16((b & EXPONENT_MASK) as u8),
        );
        (reference, constant)
    }

    /// Determines the [`SciDecimal`]s encoded by a zero-padded 36-bit referenced numeric component.
    pub(crate) fn bits_to_reference_and_constant_narrow(b: u64) -> (SciDecimal, SciDecimal) {
        const SIGN_MASK: u64 = 0b10_00000000_00000000;
        const EXPONENT_MASK: u64 = 0b00_00000000_01111111;
        let r = b >> 18; // Reference has same layout as constant, just bit shifted
        let reference_sign = (r & SIGN_MASK) << 46;
        let reference_significand = ((r & !SIGN_MASK) >> 7) + 1;
        let reference = SciDecimal::new(
            (reference_sign | reference_significand) as i64,
            i7_to_i16((r & EXPONENT_MASK) as u8),
        );
        let constant_sign = (b & SIGN_MASK) << 46;
        let constant_significand = ((b & !SIGN_MASK) >> 7) + 1;
        let constant = SciDecimal::new(
            (constant_sign | constant_significand) as i64,
            i7_to_i16((b & EXPONENT_MASK) as u8),
        );
        (reference, constant)
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

    /// The scale unit of the degree Celsius, symbol **°C**, for temperatures on the Celsius scale.
    ///
    /// A temperature on the Celsius scale *x* °C is related to the absolute temperature
    /// in kelvin by *x* °C = (*x* + 273.15) K
    ///
    /// For the corresponding absolute, linear unit, equal to the kelvin and
    /// used for representing temperature differences and intervals, see
    /// [`Unit128::CELSIUS_DEGREE`].
    pub const DEGREE_CELSIUS: Unit128 = Unit128(0x00355942_00000000_00_00_01_00_00_00_00_41);

    // Numeric component gets referenced layout:
    // `|hbbbbbbb|bbbbbbbb|bbbbbbbb|bfffffff|gaaaaaaa|aaaaaaaa|aaaaaaaa|aeeeeeee|`
    // *y* = (−1)^*h* (*b* + 1) × 10^*f* and *k* = (−1)^*g* (*a* + 1) × 10^*e*
    // For °C:
    // reference y = 273.15 = 27315e-2
    //   => h = 0 (positive)
    //      b = 27315 − 1 = 0b1101010_10110010
    //      f = −2 = 0b10000010 as i8 = 0b1000010 as i7
    //   so encoded as 0b00000000_00110101_01011001_01000010 = 0x00_35_59_42
    // constant (proportionality factor) k = 1
    //   => g = 0,
    //      b = 1 − 1 = 0,
    //      f = 0 = 0b0 as i8 = 0b0 as i7
    //   so encoded as 4 bytes of zeros 0x00_00_00_00
    //   and encoded for the degree magnitude (a linear unit) as 8 bytes of zeros
    // reference unit ꟛ = kelvin => dim = 0x00_00_01_00_00_00_00
    // Celsius gets catalogue number 1 assigned => lsb = 0x01 for degree magnitude, 0x41 for scale
    // Putting it together (everything below is hex):
    // reference:                   00355942
    // constant:                             00000000
    // reference unit:                                00_00_01_00_00_00_00
    // least significant byte:                                             n1 (where n = 0 or 4)
    // UoMID for degree magnitude:                    00_00_01_00_00_00_00_01
    // UoMID for scale unit:        00355942_00000000_00_00_01_00_00_00_00_41

    /// The absolute magnitude of the degree Celsius, equal to the kelvin.
    ///
    /// This is an absolute, linear unit, used for representing temperature
    /// differences and intervals.
    ///
    /// For the corresponding scale unit, used for representing temperatures on
    /// the Celsius scale, see [`Unit128::DEGREE_CELSIUS`].
    pub const CELSIUS_DEGREE: Unit128 = Unit128(0x00_00_01_00_00_00_00_01);

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

    use super::*;

    //#[test]
    //fn new() {
    //    let s = Unit128::new(SciDecimal::ONE, Dimensions::TIME, 0x00);
    //    let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    assert_eq!(s, Unit128::SECOND);
    //    assert_eq!(s.num, 0x0);
    //    assert_eq!(s.dim, 0x1100);
    //    assert_eq!(celsius.num, 0x006AB3FE00000000);
    //    assert_eq!(celsius.dim, 0x0000110000000041);
    //    assert_eq!(ft.num, 0xBE7FC);
    //    assert_eq!(ft.dim, 0x110001);
    //}

    #[test]
    fn create_i7() {
        // Max possible value for an i7 is 63, min value is -64
        // Positive numbers first, should be straightforward
        assert_eq!(i16_to_i7(0), 0_u8);
        assert_eq!(i16_to_i7(1), 1_u8);
        assert_eq!(i16_to_i7(63), 63_u8);
        // Generate negative i7s for testing using two's complement and then
        // zeroing bit 7 (to reflect overflow)
        // Don't have to worry about actual overflow of the u8 since the value
        // is always too low for it to occur
        assert_eq!(i16_to_i7(-1), (!1_u8 + 1) & 0b01111111);
        assert_eq!(i16_to_i7(-63), (!63_u8 + 1) & 0b01111111);
        assert_eq!(i16_to_i7(-64), (!64_u8 + 1) & 0b01111111);
    }

    //#[test]
    //fn factor_to_bits() {
    //    assert_eq!(Unit128::factor_to_bits(SciDecimal::new(1, 0)), 0x0);
    //    assert_eq!(Unit128::factor_to_bits(SciDecimal::new(2, 0)), 0x100);
    //    //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(10)), 0x1); // Fails for
    //    // now, gives:
    //    assert_eq!(Unit128::factor_to_bits(SciDecimal::new(10, 0)), 0x900);
    //    //assert_eq!(Unit128::factor_to_bits(SciNum::new_exact(1000)), 0x3); // Fails
    //    // for now, gives:
    //    assert_eq!(Unit128::factor_to_bits(SciDecimal::new(1000, 0)), 0x3E700);
    //    assert_eq!(Unit128::factor_to_bits(sci!(0.1)), 0xFF);
    //    assert_eq!(Unit128::factor_to_bits(sci!(1e-3)), 0xFD);
    //    assert_eq!(
    //        Unit128::factor_to_bits(SciDecimal::new(-1, 0)),
    //        0xFFFFFFFFFFFFFE00
    //    );
    //    assert_eq!(
    //        Unit128::factor_to_bits(SciDecimal::new(-3, 0)),
    //        0xFFFFFFFFFFFFFC00
    //    );
    //    assert_eq!(Unit128::factor_to_bits(sci!(0.3048)), 0xBE7FC);
    //}

    //#[test]
    //fn bits_to_factor() {
    //    assert_eq!(Unit128::bits_to_factor(0x0), SciDecimal::new(1, 0));
    //    assert_eq!(Unit128::bits_to_factor(0x100), SciDecimal::new(2, 0));
    //    //assert_eq!(Unit128::bits_to_factor(0x1, SciNum::new_exact(10)); // Fails for
    //    // now, gives:
    //    assert_eq!(Unit128::bits_to_factor(0x900), SciDecimal::new(10, 0));
    //    //assert_eq!(Unit128::bits_to_factor(0x3, SciNum::new_exact(1000)); // Fails
    //    // for now, gives:
    //    assert_eq!(Unit128::bits_to_factor(0x3E700), SciDecimal::new(1000, 0));
    //    assert_eq!(Unit128::bits_to_factor(0xFF), sci!(0.1));
    //    assert_eq!(Unit128::bits_to_factor(0xFD), sci!(1e-3));
    //    assert_eq!(
    //        Unit128::bits_to_factor(0xFFFFFFFFFFFFFE00),
    //        SciDecimal::new(-1, 0)
    //    );
    //    assert_eq!(
    //        Unit128::bits_to_factor(0xFFFFFFFFFFFFFC00),
    //        SciDecimal::new(-3, 0)
    //    );
    //    assert_eq!(Unit128::bits_to_factor(0xBE7FC), sci!(0.3048));
    //}

    //#[test]
    //fn factor() {
    //    assert_eq!(Unit128::KILOGRAM.factor(), SciDecimal::ONE);
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    assert_eq!(ft.factor(), sci!(0.3048));
    //    // Calling factor() on this was broken, keep as regression test
    //    let x = Unit128 {
    //        num: 0x20789937226C9F0,
    //        dim: 0x1214F40D,
    //    };
    //    // The above should correspond to:
    //    // 4.184^-2 = 0.05712374190670824665757561355… = 571237419067082 * 10^-16
    //    assert_eq!(x.factor(), sci!(571237419067082e-16));
    //}

    //#[test]
    //fn dimensions() {
    //    assert_eq!(Unit128::KILOGRAM.dimensions(), Dimensions::MASS);
    //    assert_eq!(
    //        Unit128::KELVIN.dimensions(),
    //        Dimensions::THERMODYNAMIC_TEMPERATURE
    //    );
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    assert_eq!(ft.dimensions(), Dimensions::LENGTH);
    //}

    //#[test]
    //fn lsb() {
    //    assert_eq!(Unit128::ONE.least_significant_byte(), 0x00);
    //    assert_eq!(Unit128::SECOND.least_significant_byte(), 0x00);
    //    let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
    //    assert_eq!(celsius.least_significant_byte(), 0x41);
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    assert_eq!(ft.least_significant_byte(), 0x01);
    //}

    //#[test]
    //fn is_referenced() {
    //    assert!(!Unit128::ONE.is_referenced());
    //    assert!(!Unit128::SECOND.is_referenced());
    //    let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
    //    assert!(celsius.is_referenced());
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    assert!(!ft.is_referenced());
    //}

    //#[test]
    //fn to_from_str() {
    //    let s = Unit128::SECOND;
    //    let celsius = Unit128::from_bits(0x006AB3FE000000000000110000000041);
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    // Test these known examples
    //    assert_eq!(s.to_string(), "0x1100");
    //    assert_eq!(celsius.to_string(), "0x6AB3FE000000000000110000000041");
    //    assert_eq!(ft.to_string(), "0xBE7FC0000000000110001");
    //    // Test round trip
    //    assert_eq!(Unit128::from_str(&s.to_string()).unwrap(), s);
    //    assert_eq!(Unit128::from_str(&celsius.to_string()).unwrap(), celsius);
    //    assert_eq!(Unit128::from_str(&ft.to_string()).unwrap(), ft);
    //}

    //#[test]
    //fn debug() {
    //    assert_eq!(
    //        format!("{:?}", Unit128::SECOND),
    //        "Unit128 { num: 0x0, dim: 0x1100 }"
    //    );
    //}

    //#[test]
    //fn mul() {
    //    let amp_second = Unit128::AMPERE * Unit128::SECOND;
    //    let square_metre = Unit128::METRE * Unit128::METRE;
    //    let ft = Unit128::new(sci!(0.3048), Dimensions::LENGTH, 0x01);
    //    let square_foot = ft * ft;
    //    assert_eq!(amp_second.num, 0x0);
    //    assert_eq!(amp_second.dim, 0x110000110C);
    //    assert_eq!(square_metre.num, 0x0);
    //    assert_eq!(square_metre.dim, 0x12000C);
    //    assert_eq!(square_foot.num, 0x8DC23FF8);
    //    assert_eq!(square_foot.dim, 0x12000C);
    //}
}
