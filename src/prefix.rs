// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(non_camel_case_types)]

use num_traits::Float;
use scinum::SciDecimal;
use std::{fmt, str::FromStr};

use crate::error::QuanstantsError;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
pub enum Prefix {
    // Metric
    quecto,
    ronto,
    yocto,
    zepto,
    atto,
    femto,
    pico,
    nano,
    micro,
    milli,
    centi,
    deci,
    deca,
    hecto,
    kilo,
    mega,
    giga,
    tera,
    peta,
    exa,
    zetta,
    yotta,
    ronna,
    quetta,
    // Binary
    kibi,
    mebi,
    gibi,
    tebi,
    pebi,
    exbi,
    zebi,
    yobi,
}

impl Prefix {
    pub fn from_symbol(symbol: &str) -> Result<Self, QuanstantsError> {
        match symbol {
            "q" => Ok(Self::quecto),
            "r" => Ok(Self::ronto),
            "y" => Ok(Self::yocto),
            "z" => Ok(Self::zepto),
            "a" => Ok(Self::atto),
            "f" => Ok(Self::femto),
            "p" => Ok(Self::pico),
            "n" => Ok(Self::nano),
            "μ" => Ok(Self::micro),
            "m" => Ok(Self::milli),
            "c" => Ok(Self::centi),
            "d" => Ok(Self::deci),
            "da" => Ok(Self::deca),
            "h" => Ok(Self::hecto),
            "k" => Ok(Self::kilo),
            "M" => Ok(Self::mega),
            "G" => Ok(Self::giga),
            "T" => Ok(Self::tera),
            "P" => Ok(Self::peta),
            "E" => Ok(Self::exa),
            "Z" => Ok(Self::zetta),
            "Y" => Ok(Self::yotta),
            "R" => Ok(Self::ronna),
            "Q" => Ok(Self::quetta),
            "Ki" => Ok(Self::kibi),
            "Mi" => Ok(Self::mebi),
            "Gi" => Ok(Self::gibi),
            "Ti" => Ok(Self::tebi),
            "Pi" => Ok(Self::pebi),
            "Ei" => Ok(Self::exbi),
            "Zi" => Ok(Self::zebi),
            "Yi" => Ok(Self::yobi),
            _ => Err(QuanstantsError::Parse(symbol.to_string())),
        }
    }

    pub fn from_name(name: &str) -> Result<Self, QuanstantsError> {
        match name {
            "quecto" => Ok(Self::quecto),
            "ronto" => Ok(Self::ronto),
            "yocto" => Ok(Self::yocto),
            "zepto" => Ok(Self::zepto),
            "atto" => Ok(Self::atto),
            "femto" => Ok(Self::femto),
            "pico" => Ok(Self::pico),
            "nano" => Ok(Self::nano),
            "micro" => Ok(Self::micro),
            "milli" => Ok(Self::milli),
            "centi" => Ok(Self::centi),
            "deci" => Ok(Self::deci),
            "deca" => Ok(Self::deca),
            "hecto" => Ok(Self::hecto),
            "kilo" => Ok(Self::kilo),
            "mega" => Ok(Self::mega),
            "giga" => Ok(Self::giga),
            "tera" => Ok(Self::tera),
            "peta" => Ok(Self::peta),
            "exa" => Ok(Self::exa),
            "zetta" => Ok(Self::zetta),
            "yotta" => Ok(Self::yotta),
            "ronna" => Ok(Self::ronna),
            "quetta" => Ok(Self::quetta),
            "kibi" => Ok(Self::kibi),
            "mebi" => Ok(Self::mebi),
            "gibi" => Ok(Self::gibi),
            "tebi" => Ok(Self::tebi),
            "pebi" => Ok(Self::pebi),
            "exbi" => Ok(Self::exbi),
            "zebi" => Ok(Self::zebi),
            "yobi" => Ok(Self::yobi),
            _ => Err(QuanstantsError::Parse(name.to_string())),
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Self::quecto => String::from("q"),
            Self::ronto => String::from("r"),
            Self::yocto => String::from("y"),
            Self::zepto => String::from("z"),
            Self::atto => String::from("a"),
            Self::femto => String::from("f"),
            Self::pico => String::from("p"),
            Self::nano => String::from("n"),
            Self::micro => String::from("μ"),
            Self::milli => String::from("m"),
            Self::centi => String::from("c"),
            Self::deci => String::from("d"),
            Self::deca => String::from("da"),
            Self::hecto => String::from("h"),
            Self::kilo => String::from("k"),
            Self::mega => String::from("M"),
            Self::giga => String::from("G"),
            Self::tera => String::from("T"),
            Self::peta => String::from("P"),
            Self::exa => String::from("E"),
            Self::zetta => String::from("Z"),
            Self::yotta => String::from("Y"),
            Self::ronna => String::from("R"),
            Self::quetta => String::from("Q"),
            Self::kibi => String::from("Ki"),
            Self::mebi => String::from("Mi"),
            Self::gibi => String::from("Gi"),
            Self::tebi => String::from("Ti"),
            Self::pebi => String::from("Pi"),
            Self::exbi => String::from("Ei"),
            Self::zebi => String::from("Zi"),
            Self::yobi => String::from("Yi"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::quecto => String::from("quecto"),
            Self::ronto => String::from("ronto"),
            Self::yocto => String::from("yocto"),
            Self::zepto => String::from("zepto"),
            Self::atto => String::from("atto"),
            Self::femto => String::from("femto"),
            Self::pico => String::from("pico"),
            Self::nano => String::from("nano"),
            Self::micro => String::from("micro"),
            Self::milli => String::from("milli"),
            Self::centi => String::from("centi"),
            Self::deci => String::from("deci"),
            Self::deca => String::from("deca"),
            Self::hecto => String::from("hecto"),
            Self::kilo => String::from("kilo"),
            Self::mega => String::from("mega"),
            Self::giga => String::from("giga"),
            Self::tera => String::from("tera"),
            Self::peta => String::from("peta"),
            Self::exa => String::from("exa"),
            Self::zetta => String::from("zetta"),
            Self::yotta => String::from("yotta"),
            Self::ronna => String::from("ronna"),
            Self::quetta => String::from("quetta"),
            Self::kibi => String::from("kibi"),
            Self::mebi => String::from("mebi"),
            Self::gibi => String::from("gibi"),
            Self::tebi => String::from("tebi"),
            Self::pebi => String::from("pebi"),
            Self::exbi => String::from("exbi"),
            Self::zebi => String::from("zebi"),
            Self::yobi => String::from("yobi"),
        }
    }

    pub fn value(&self) -> SciDecimal {
        match self {
            Self::quecto => SciDecimal::new(1, -30),
            Self::ronto => SciDecimal::new(1, -27),
            Self::yocto => SciDecimal::new(1, -24),
            Self::zepto => SciDecimal::new(1, -21),
            Self::atto => SciDecimal::new(1, -18),
            Self::femto => SciDecimal::new(1, -15),
            Self::pico => SciDecimal::new(1, -12),
            Self::nano => SciDecimal::new(1, -9),
            Self::micro => SciDecimal::new(1, -6),
            Self::milli => SciDecimal::new(1, -3),
            Self::centi => SciDecimal::new(1, -2),
            Self::deci => SciDecimal::new(1, -1),
            Self::deca => SciDecimal::new(1, 1),
            Self::hecto => SciDecimal::new(1, 2),
            Self::kilo => SciDecimal::new(1, 3),
            Self::mega => SciDecimal::new(1, 6),
            Self::giga => SciDecimal::new(1, 9),
            Self::tera => SciDecimal::new(1, 12),
            Self::peta => SciDecimal::new(1, 15),
            Self::exa => SciDecimal::new(1, 18),
            Self::zetta => SciDecimal::new(1, 21),
            Self::yotta => SciDecimal::new(1, 24),
            Self::ronna => SciDecimal::new(1, 27),
            Self::quetta => SciDecimal::new(1, 30),
            Self::kibi => SciDecimal::new(1024, 0).powi(1),
            Self::mebi => SciDecimal::new(1024, 0).powi(2),
            Self::gibi => SciDecimal::new(1024, 0).powi(3),
            Self::tebi => SciDecimal::new(1024, 0).powi(4),
            Self::pebi => SciDecimal::new(1024, 0).powi(5),
            Self::exbi => SciDecimal::new(1024, 0).powi(6),
            Self::zebi => SciDecimal::new(1024, 0).powi(7),
            Self::yobi => SciDecimal::new(1024, 0).powi(8),
        }
    }

    pub fn is_binary(&self) -> bool {
        matches!(
            self,
            Prefix::kibi
                | Prefix::mebi
                | Prefix::gibi
                | Prefix::tebi
                | Prefix::pebi
                | Prefix::exbi
                | Prefix::zebi
                | Prefix::yobi
        )
    }

    /// Returns the power _n_ such that the prefix's value is 10<sup>_n_</sup>,
    /// or in the case of a binary prefix, 1024<sup>_n_/3</sup>.
    ///
    /// For example, `Prefix::milli` returns `-3`, `Prefix::mega` returns `6`,
    /// and `Prefix::mebi` also returns `6`.
    pub fn equivalent_power(&self) -> i8 {
        match self {
            Self::quecto => -30,
            Self::ronto => -27,
            Self::yocto => -24,
            Self::zepto => -21,
            Self::atto => -18,
            Self::femto => -15,
            Self::pico => -12,
            Self::nano => -9,
            Self::micro => -6,
            Self::milli => -3,
            Self::centi => -2,
            Self::deci => -1,
            Self::deca => 1,
            Self::hecto => 2,
            Self::kilo => 3,
            Self::mega => 6,
            Self::giga => 9,
            Self::tera => 12,
            Self::peta => 15,
            Self::exa => 18,
            Self::zetta => 21,
            Self::yotta => 24,
            Self::ronna => 27,
            Self::quetta => 30,
            Self::kibi => 3,
            Self::mebi => 6,
            Self::gibi => 9,
            Self::tebi => 12,
            Self::pebi => 15,
            Self::exbi => 18,
            Self::zebi => 21,
            Self::yobi => 24,
        }
    }
}

impl From<Prefix> for SciDecimal {
    fn from(p: Prefix) -> SciDecimal {
        p.value()
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for Prefix {
    type Err = QuanstantsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Prefix::from_name(s)
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::unit::py::PyUnit;

    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "Prefix")]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum PyPrefix {
        // Metric
        quecto,
        ronto,
        yocto,
        zepto,
        atto,
        femto,
        pico,
        nano,
        micro,
        milli,
        centi,
        deci,
        deca,
        hecto,
        kilo,
        mega,
        giga,
        tera,
        peta,
        exa,
        zetta,
        yotta,
        ronna,
        quetta,
        // Binary
        kibi,
        mebi,
        gibi,
        tebi,
        pebi,
        exbi,
        zebi,
        yobi,
    }

    impl PyPrefix {
        pub fn into_inner(self) -> Prefix {
            match self {
                PyPrefix::quecto => Prefix::quecto,
                PyPrefix::ronto => Prefix::ronto,
                PyPrefix::yocto => Prefix::yocto,
                PyPrefix::zepto => Prefix::zepto,
                PyPrefix::atto => Prefix::atto,
                PyPrefix::femto => Prefix::femto,
                PyPrefix::pico => Prefix::pico,
                PyPrefix::nano => Prefix::nano,
                PyPrefix::micro => Prefix::micro,
                PyPrefix::milli => Prefix::milli,
                PyPrefix::centi => Prefix::centi,
                PyPrefix::deci => Prefix::deci,
                PyPrefix::deca => Prefix::deca,
                PyPrefix::hecto => Prefix::hecto,
                PyPrefix::kilo => Prefix::kilo,
                PyPrefix::mega => Prefix::mega,
                PyPrefix::giga => Prefix::giga,
                PyPrefix::tera => Prefix::tera,
                PyPrefix::peta => Prefix::peta,
                PyPrefix::exa => Prefix::exa,
                PyPrefix::zetta => Prefix::zetta,
                PyPrefix::yotta => Prefix::yotta,
                PyPrefix::ronna => Prefix::ronna,
                PyPrefix::quetta => Prefix::quetta,
                PyPrefix::kibi => Prefix::kibi,
                PyPrefix::mebi => Prefix::mebi,
                PyPrefix::gibi => Prefix::gibi,
                PyPrefix::tebi => Prefix::tebi,
                PyPrefix::pebi => Prefix::pebi,
                PyPrefix::exbi => Prefix::exbi,
                PyPrefix::zebi => Prefix::zebi,
                PyPrefix::yobi => Prefix::yobi,
            }
        }
    }

    impl From<Prefix> for PyPrefix {
        fn from(p: Prefix) -> Self {
            match p {
                Prefix::quecto => PyPrefix::quecto,
                Prefix::ronto => PyPrefix::ronto,
                Prefix::yocto => PyPrefix::yocto,
                Prefix::zepto => PyPrefix::zepto,
                Prefix::atto => PyPrefix::atto,
                Prefix::femto => PyPrefix::femto,
                Prefix::pico => PyPrefix::pico,
                Prefix::nano => PyPrefix::nano,
                Prefix::micro => PyPrefix::micro,
                Prefix::milli => PyPrefix::milli,
                Prefix::centi => PyPrefix::centi,
                Prefix::deci => PyPrefix::deci,
                Prefix::deca => PyPrefix::deca,
                Prefix::hecto => PyPrefix::hecto,
                Prefix::kilo => PyPrefix::kilo,
                Prefix::mega => PyPrefix::mega,
                Prefix::giga => PyPrefix::giga,
                Prefix::tera => PyPrefix::tera,
                Prefix::peta => PyPrefix::peta,
                Prefix::exa => PyPrefix::exa,
                Prefix::zetta => PyPrefix::zetta,
                Prefix::yotta => PyPrefix::yotta,
                Prefix::ronna => PyPrefix::ronna,
                Prefix::quetta => PyPrefix::quetta,
                Prefix::kibi => PyPrefix::kibi,
                Prefix::mebi => PyPrefix::mebi,
                Prefix::gibi => PyPrefix::gibi,
                Prefix::tebi => PyPrefix::tebi,
                Prefix::pebi => PyPrefix::pebi,
                Prefix::exbi => PyPrefix::exbi,
                Prefix::zebi => PyPrefix::zebi,
                Prefix::yobi => PyPrefix::yobi,
            }
        }
    }

    #[pymethods]
    impl PyPrefix {
        fn __str__(&self) -> String {
            self.into_inner().to_string()
        }

        fn __mul__(&self, rhs: &PyUnit) -> PyUnit {
            (self.into_inner() * rhs.owned_inner()).into()
        }
    }
}
