// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

use crate::{
    dimensions::Dimensions, fraction::Frac, prefix::Prefix, scinum::SciNum, unit128::Unit128,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefFile {
    #[serde(default)]
    pub units: HashMap<String, UnitDef>,
    #[serde(default)]
    pub constants: HashMap<String, ConstantDef>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct UnitDef {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub id: Option<Unit128>,
    pub source: Option<String>,
    #[serde(default)]
    pub base: bool,
    pub dimensions: Option<Dimensions>,
    pub symbol: Option<String>,
    pub name: String,
    #[serde(default)]
    pub alt_names: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub prefix: Option<Prefix>,
    pub value: Option<QuantityDef>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantDef {
    pub source: Option<String>,
    pub symbol: Option<String>,
    pub name: String,
    pub alt_names: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub value: QuantityDef,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityDef {
    pub number: SciNum,
    pub unit: Vec<(String, Frac)>,
    pub uncertainty: Option<SciNum>,
}

pub mod units {
    #[derive(Copy, Clone, Debug)]
    pub enum UnitModule {
        Si,
        SiCompatible,
    }

    impl UnitModule {
        const SI: &str = include_str!("definitions/units/si.toml");
        const SI_COMPATIBLE: &str = include_str!("definitions/units/si_compatible.toml");

        pub fn toml(self) -> &'static str {
            match self {
                Self::Si => Self::SI,
                Self::SiCompatible => Self::SI_COMPATIBLE,
            }
        }
    }

    #[cfg(feature = "python")]
    pub(crate) mod py {
        use super::*;
        use pyo3::prelude::*;

        #[pyclass(name = "UnitModule")]
        #[derive(Copy, Clone, Debug)]
        pub(crate) enum PyUnitModule {
            Si,
            SiCompatible,
        }

        impl From<PyUnitModule> for UnitModule {
            fn from(m: PyUnitModule) -> UnitModule {
                match m {
                    PyUnitModule::Si => UnitModule::Si,
                    PyUnitModule::SiCompatible => UnitModule::SiCompatible,
                }
            }
        }

        impl From<UnitModule> for PyUnitModule {
            fn from(m: UnitModule) -> PyUnitModule {
                match m {
                    UnitModule::Si => PyUnitModule::Si,
                    UnitModule::SiCompatible => PyUnitModule::SiCompatible,
                }
            }
        }
    }
}

pub mod constants {}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("definitions/units/test.toml");

    #[test]
    fn parse_si() {
        let _: DefFile = toml::from_str(units::UnitModule::Si.toml()).unwrap();
    }
}
