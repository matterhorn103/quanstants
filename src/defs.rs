use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    dimensions::Dimensions, fraction::Frac, prefix::Prefix, scinum::SciNum, unit128::Unit128,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefFile {
    pub units: HashMap<String, UnitDef>,
    pub constants: HashMap<String, ConstantDef>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct UnitDef {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub id: Option<Unit128>,
    pub source: Option<String>,
    pub base: bool,
    pub dimensions: Option<Dimensions>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub alt_names: Vec<String>,
    pub aliases: Vec<String>,
    pub prefix: Option<Prefix>,
    pub value: Option<QuantityDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantDef {
    pub source: Option<String>,
    pub symbol: Option<String>,
    pub name: String,
    pub alt_names: Vec<String>,
    pub aliases: Vec<String>,
    pub value: QuantityDef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityDef {
    pub number: SciNum,
    pub unit: Vec<(String, Frac)>,
    pub uncertainty: SciNum,
}

pub mod units {

}

pub mod constants {
    
}
