// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

#![allow(non_camel_case_types)]

use std::{fmt, str::FromStr};

use crate::{error::QuanstantsError, scinum::SciNum};

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
            _ => Err(QuanstantsError::Parse),
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
            _ => Err(QuanstantsError::Parse),
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

    pub fn value(&self) -> SciNum {
        match self {
            Self::quecto => SciNum::exact_from_scientific_parts(1, -30),
            Self::ronto => SciNum::exact_from_scientific_parts(1, -27),
            Self::yocto => SciNum::exact_from_scientific_parts(1, -24),
            Self::zepto => SciNum::exact_from_scientific_parts(1, -21),
            Self::atto => SciNum::exact_from_scientific_parts(1, -18),
            Self::femto => SciNum::exact_from_scientific_parts(1, -15),
            Self::pico => SciNum::exact_from_scientific_parts(1, -12),
            Self::nano => SciNum::exact_from_scientific_parts(1, -9),
            Self::micro => SciNum::exact_from_scientific_parts(1, -6),
            Self::milli => SciNum::exact_from_scientific_parts(1, -3),
            Self::centi => SciNum::exact_from_scientific_parts(1, -2),
            Self::deci => SciNum::exact_from_scientific_parts(1, -1),
            Self::deca => SciNum::exact_from_scientific_parts(1, 1),
            Self::hecto => SciNum::exact_from_scientific_parts(1, 2),
            Self::kilo => SciNum::exact_from_scientific_parts(1, 3),
            Self::mega => SciNum::exact_from_scientific_parts(1, 6),
            Self::giga => SciNum::exact_from_scientific_parts(1, 9),
            Self::tera => SciNum::exact_from_scientific_parts(1, 12),
            Self::peta => SciNum::exact_from_scientific_parts(1, 15),
            Self::exa => SciNum::exact_from_scientific_parts(1, 18),
            Self::zetta => SciNum::exact_from_scientific_parts(1, 21),
            Self::yotta => SciNum::exact_from_scientific_parts(1, 24),
            Self::ronna => SciNum::exact_from_scientific_parts(1, 27),
            Self::quetta => SciNum::exact_from_scientific_parts(1, 30),
            Self::kibi => SciNum::new_exact(1).powi(1),
            Self::mebi => SciNum::new_exact(1).powi(2),
            Self::gibi => SciNum::new_exact(1).powi(3),
            Self::tebi => SciNum::new_exact(1).powi(4),
            Self::pebi => SciNum::new_exact(1).powi(5),
            Self::exbi => SciNum::new_exact(1).powi(6),
            Self::zebi => SciNum::new_exact(1).powi(7),
            Self::yobi => SciNum::new_exact(1).powi(8),
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Prefix::kibi
            | Prefix::mebi
            | Prefix::gibi
            | Prefix::tebi
            | Prefix::pebi
            | Prefix::exbi
            | Prefix::zebi
            | Prefix::yobi => true,
            _ => false,
        }
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

/*
quecto = Prefix("q", "quecto", "1E-30")
ronto  = Prefix("r", "ronto", "1E-27")
yocto  = Prefix("y", "yocto", "1E-24")
zepto  = Prefix("z", "zepto", "1E-21")
atto   = Prefix("a", "atto", "1E-18")
femto  = Prefix("f", "femto", "1E-15")
pico   = Prefix("p", "pico", "1E-12")
nano   = Prefix("n", "nano", "1E-9")
micro  = Prefix("μ", "micro", "1E-6")
milli  = Prefix("m", "milli", "1E-3")
centi  = Prefix("c", "centi", "1E-2")
deci   = Prefix("d", "deci", "1E-1")
deca   = Prefix("da", "deca", "1E+1")
hecto  = Prefix("h", "hecto", "1E+2")
kilo   = Prefix("k", "kilo", "1E+3")
mega   = Prefix("M", "mega", "1E+6")
giga   = Prefix("G", "giga", "1E+9")
tera   = Prefix("T", "tera", "1E+12")
peta   = Prefix("P", "peta", "1E+15")
exa    = Prefix("E", "exa", "1E+18")
zetta  = Prefix("Z", "zetta", "1E+21")
yotta  = Prefix("Y", "yotta", "1E+24")
ronna  = Prefix("R", "ronna", "1E+27")
quetta = Prefix("Q", "quetta", "1E+30")

quecto = Prefix("q", "quecto", "1E-30")
ronto = Prefix("r", "ronto", "1E-27")
yocto = Prefix("y", "yocto", "1E-24")
zepto = Prefix("z", "zepto", "1E-21")
atto = Prefix("a", "atto", "1E-18")
femto = Prefix("f", "femto", "1E-15")
pico = Prefix("p", "pico", "1E-12")
nano = Prefix("n", "nano", "1E-9")
micro = Prefix("μ", "micro", "1E-6")
milli = Prefix("m", "milli", "1E-3")
centi = Prefix("c", "centi", "1E-2")
deci = Prefix("d", "deci", "1E-1")
deca = Prefix("da", "deca", "1E+1")
hecto = Prefix("h", "hecto", "1E+2")
kilo = Prefix("k", "kilo", "1E+3")
mega = Prefix("M", "mega", "1E+6")
giga = Prefix("G", "giga", "1E+9")
tera = Prefix("T", "tera", "1E+12")
peta = Prefix("P", "peta", "1E+15")
exa = Prefix("E", "exa", "1E+18")
zetta = Prefix("Z", "zetta", "1E+21")
yotta = Prefix("Y", "yotta", "1E+24")
ronna = Prefix("R", "ronna", "1E+27")
quetta = Prefix("Q", "quetta", "1E+30")

kibi = Prefix("Ki", "kibi", 1024**1)
mebi = Prefix("Mi", "mebi", 1024**2)
gibi = Prefix("Gi", "gibi", 1024**3)
tebi = Prefix("Ti", "tebi", 1024**4)
pebi = Prefix("Pi", "pebi", 1024**5)
exbi = Prefix("Ei", "exbi", 1024**6)
zebi = Prefix("Zi", "zebi", 1024**7)
yobi = Prefix("Yi", "yobi", 1024**8)
*/

#[cfg(feature = "python")]
pub(crate) mod py {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "Prefix")]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct PyPrefix(Prefix);

    impl PyPrefix {
        pub fn into_inner(self) -> Prefix {
            self.0
        }
    }

    impl From<Prefix> for PyPrefix {
        fn from(value: Prefix) -> Self {
            Self(value)
        }
    }

    #[pymethods]
    impl PyPrefix {
        fn __str__(&self) -> String {
            self.0.to_string()
        }
    }
}
