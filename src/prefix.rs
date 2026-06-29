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
    robi,
    quebi,
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
            "Ri" => Ok(Self::robi),
            "Qi" => Ok(Self::quebi),
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
            "robi" => Ok(Self::robi),
            "quebi" => Ok(Self::quebi),
            _ => Err(QuanstantsError::Parse(name.to_string())),
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Prefix::quecto => String::from("q"),
            Prefix::ronto => String::from("r"),
            Prefix::yocto => String::from("y"),
            Prefix::zepto => String::from("z"),
            Prefix::atto => String::from("a"),
            Prefix::femto => String::from("f"),
            Prefix::pico => String::from("p"),
            Prefix::nano => String::from("n"),
            Prefix::micro => String::from("μ"),
            Prefix::milli => String::from("m"),
            Prefix::centi => String::from("c"),
            Prefix::deci => String::from("d"),
            Prefix::deca => String::from("da"),
            Prefix::hecto => String::from("h"),
            Prefix::kilo => String::from("k"),
            Prefix::mega => String::from("M"),
            Prefix::giga => String::from("G"),
            Prefix::tera => String::from("T"),
            Prefix::peta => String::from("P"),
            Prefix::exa => String::from("E"),
            Prefix::zetta => String::from("Z"),
            Prefix::yotta => String::from("Y"),
            Prefix::ronna => String::from("R"),
            Prefix::quetta => String::from("Q"),
            Prefix::kibi => String::from("Ki"),
            Prefix::mebi => String::from("Mi"),
            Prefix::gibi => String::from("Gi"),
            Prefix::tebi => String::from("Ti"),
            Prefix::pebi => String::from("Pi"),
            Prefix::exbi => String::from("Ei"),
            Prefix::zebi => String::from("Zi"),
            Prefix::yobi => String::from("Yi"),
            Prefix::robi => String::from("Ri"),
            Prefix::quebi => String::from("Qi"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Prefix::quecto => String::from("quecto"),
            Prefix::ronto => String::from("ronto"),
            Prefix::yocto => String::from("yocto"),
            Prefix::zepto => String::from("zepto"),
            Prefix::atto => String::from("atto"),
            Prefix::femto => String::from("femto"),
            Prefix::pico => String::from("pico"),
            Prefix::nano => String::from("nano"),
            Prefix::micro => String::from("micro"),
            Prefix::milli => String::from("milli"),
            Prefix::centi => String::from("centi"),
            Prefix::deci => String::from("deci"),
            Prefix::deca => String::from("deca"),
            Prefix::hecto => String::from("hecto"),
            Prefix::kilo => String::from("kilo"),
            Prefix::mega => String::from("mega"),
            Prefix::giga => String::from("giga"),
            Prefix::tera => String::from("tera"),
            Prefix::peta => String::from("peta"),
            Prefix::exa => String::from("exa"),
            Prefix::zetta => String::from("zetta"),
            Prefix::yotta => String::from("yotta"),
            Prefix::ronna => String::from("ronna"),
            Prefix::quetta => String::from("quetta"),
            Prefix::kibi => String::from("kibi"),
            Prefix::mebi => String::from("mebi"),
            Prefix::gibi => String::from("gibi"),
            Prefix::tebi => String::from("tebi"),
            Prefix::pebi => String::from("pebi"),
            Prefix::exbi => String::from("exbi"),
            Prefix::zebi => String::from("zebi"),
            Prefix::yobi => String::from("yobi"),
            Prefix::robi => String::from("robi"),
            Prefix::quebi => String::from("quebi"),
        }
    }

    pub fn value(&self) -> SciDecimal {
        match self {
            Prefix::quecto => SciDecimal::new(1, -30),
            Prefix::ronto => SciDecimal::new(1, -27),
            Prefix::yocto => SciDecimal::new(1, -24),
            Prefix::zepto => SciDecimal::new(1, -21),
            Prefix::atto => SciDecimal::new(1, -18),
            Prefix::femto => SciDecimal::new(1, -15),
            Prefix::pico => SciDecimal::new(1, -12),
            Prefix::nano => SciDecimal::new(1, -9),
            Prefix::micro => SciDecimal::new(1, -6),
            Prefix::milli => SciDecimal::new(1, -3),
            Prefix::centi => SciDecimal::new(1, -2),
            Prefix::deci => SciDecimal::new(1, -1),
            Prefix::deca => SciDecimal::new(1, 1),
            Prefix::hecto => SciDecimal::new(1, 2),
            Prefix::kilo => SciDecimal::new(1, 3),
            Prefix::mega => SciDecimal::new(1, 6),
            Prefix::giga => SciDecimal::new(1, 9),
            Prefix::tera => SciDecimal::new(1, 12),
            Prefix::peta => SciDecimal::new(1, 15),
            Prefix::exa => SciDecimal::new(1, 18),
            Prefix::zetta => SciDecimal::new(1, 21),
            Prefix::yotta => SciDecimal::new(1, 24),
            Prefix::ronna => SciDecimal::new(1, 27),
            Prefix::quetta => SciDecimal::new(1, 30),
            Prefix::kibi => SciDecimal::new(1024, 0).powi(1),
            Prefix::mebi => SciDecimal::new(1024, 0).powi(2),
            Prefix::gibi => SciDecimal::new(1024, 0).powi(3),
            Prefix::tebi => SciDecimal::new(1024, 0).powi(4),
            Prefix::pebi => SciDecimal::new(1024, 0).powi(5),
            Prefix::exbi => SciDecimal::new(1024, 0).powi(6),
            Prefix::zebi => SciDecimal::new(1024, 0).powi(7),
            Prefix::yobi => SciDecimal::new(1024, 0).powi(8),
            Prefix::robi => SciDecimal::new(1024, 0).powi(9),
            Prefix::quebi => SciDecimal::new(1024, 0).powi(10),
        }
    }

    pub fn value_f64(&self) -> f64 {
        match self {
            Prefix::quecto => 1e-30,
            Prefix::ronto => 1e-27,
            Prefix::yocto => 1e-24,
            Prefix::zepto => 1e-21,
            Prefix::atto => 1e-18,
            Prefix::femto => 1e-15,
            Prefix::pico => 1e-12,
            Prefix::nano => 1e-9,
            Prefix::micro => 1e-6,
            Prefix::milli => 1e-3,
            Prefix::centi => 1e-2,
            Prefix::deci => 1e-1,
            Prefix::deca => 1e1,
            Prefix::hecto => 1e2,
            Prefix::kilo => 1e3,
            Prefix::mega => 1e6,
            Prefix::giga => 1e9,
            Prefix::tera => 1e12,
            Prefix::peta => 1e15,
            Prefix::exa => 1e18,
            Prefix::zetta => 1e21,
            Prefix::yotta => 1e24,
            Prefix::ronna => 1e27,
            Prefix::quetta => 1e30,
            Prefix::kibi => 2_f64.powi(10),
            Prefix::mebi => 2_f64.powi(20),
            Prefix::gibi => 2_f64.powi(30),
            Prefix::tebi => 2_f64.powi(40),
            Prefix::pebi => 2_f64.powi(50),
            Prefix::exbi => 2_f64.powi(60),
            Prefix::zebi => 2_f64.powi(70),
            Prefix::yobi => 2_f64.powi(80),
            Prefix::robi => 2_f64.powi(90),
            Prefix::quebi => 2_f64.powi(100),
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
            Prefix::quecto => -30,
            Prefix::ronto => -27,
            Prefix::yocto => -24,
            Prefix::zepto => -21,
            Prefix::atto => -18,
            Prefix::femto => -15,
            Prefix::pico => -12,
            Prefix::nano => -9,
            Prefix::micro => -6,
            Prefix::milli => -3,
            Prefix::centi => -2,
            Prefix::deci => -1,
            Prefix::deca => 1,
            Prefix::hecto => 2,
            Prefix::kilo => 3,
            Prefix::mega => 6,
            Prefix::giga => 9,
            Prefix::tera => 12,
            Prefix::peta => 15,
            Prefix::exa => 18,
            Prefix::zetta => 21,
            Prefix::yotta => 24,
            Prefix::ronna => 27,
            Prefix::quetta => 30,
            Prefix::kibi => 3,
            Prefix::mebi => 6,
            Prefix::gibi => 9,
            Prefix::tebi => 12,
            Prefix::pebi => 15,
            Prefix::exbi => 18,
            Prefix::zebi => 21,
            Prefix::yobi => 24,
            Prefix::robi => 27,
            Prefix::quebi => 30,
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
        robi,
        quebi,
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
                PyPrefix::robi => Prefix::robi,
                PyPrefix::quebi => Prefix::quebi,
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
                Prefix::robi => PyPrefix::robi,
                Prefix::quebi => PyPrefix::quebi,
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
