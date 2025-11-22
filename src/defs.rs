use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as, skip_serializing_none};

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
    pub enum UnitModule {
        Si,
    }

    impl UnitModule {
        const SI: &str = include_str!("definitions/units/si.toml");

        pub fn toml(self) -> &'static str {
            match self {
                Self::Si => Self::SI,
            }
        }
    }
}

pub mod constants {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST: &str = include_str!("definitions/units/test.toml");

    #[test]
    fn parse_si() {
        let _: DefFile = toml::from_str(units::UnitModule::Si.toml()).unwrap();
    }
}
