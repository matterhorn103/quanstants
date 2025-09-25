#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
            Self::quecto =>   String::from("quecto"),
            Self::ronto =>    String::from("ronto"),
            Self::yocto =>    String::from("yocto"),
            Self::zepto =>    String::from("zepto"),
            Self::atto =>     String::from("atto"),
            Self::femto =>    String::from("femto"),
            Self::pico =>     String::from("pico"),
            Self::nano =>     String::from("nano"),
            Self::micro =>    String::from("micro"),
            Self::milli =>    String::from("milli"),
            Self::centi =>    String::from("centi"),
            Self::deci =>     String::from("deci"),
            Self::deca =>     String::from("deca"),
            Self::hecto =>    String::from("hecto"),
            Self::kilo =>     String::from("kilo"),
            Self::mega =>     String::from("mega"),
            Self::giga =>     String::from("giga"),
            Self::tera =>     String::from("tera"),
            Self::peta =>     String::from("peta"),
            Self::exa =>      String::from("exa"),
            Self::zetta =>    String::from("zetta"),
            Self::yotta =>    String::from("yotta"),
            Self::ronna =>    String::from("ronna"),
            Self::quetta =>   String::from("quetta"),
            Self::kibi =>     String::from("kibi"),
            Self::mebi =>     String::from("mebi"),
            Self::gibi =>     String::from("gibi"),
            Self::tebi =>     String::from("tebi"),
            Self::pebi =>     String::from("pebi"),
            Self::exbi =>     String::from("exbi"),
            Self::zebi =>     String::from("zebi"),
            Self::yobi =>     String::from("yobi"),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::quecto =>   1e-30,
            Self::ronto =>    1e-27,
            Self::yocto =>    1e-24,
            Self::zepto =>    1e-21,
            Self::atto =>     1e-18,
            Self::femto =>    1e-15,
            Self::pico =>     1e-12,
            Self::nano =>     1e-9,
            Self::micro =>    1e-6,
            Self::milli =>    1e-3,
            Self::centi =>    1e-2,
            Self::deci =>     1e-1,
            Self::deca =>     1e+1,
            Self::hecto =>    1e+2,
            Self::kilo =>     1e+3,
            Self::mega =>     1e+6,
            Self::giga =>     1e+9,
            Self::tera =>     1e+12,
            Self::peta =>     1e+15,
            Self::exa =>      1e+18,
            Self::zetta =>    1e+21,
            Self::yotta =>    1e+24,
            Self::ronna =>    1e+27,
            Self::quetta =>   1e+30,
            Self::kibi =>     1024_f64.powf(1.0),
            Self::mebi =>     1024_f64.powf(2.0),
            Self::gibi =>     1024_f64.powf(3.0),
            Self::tebi =>     1024_f64.powf(4.0),
            Self::pebi =>     1024_f64.powf(5.0),
            Self::exbi =>     1024_f64.powf(6.0),
            Self::zebi =>     1024_f64.powf(7.0),
            Self::yobi =>     1024_f64.powf(8.0),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
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

    #[pymethods]
    impl PyPrefix {
        fn __str__(&self) -> String {
            self.0.to_string()
        }
    }
}
