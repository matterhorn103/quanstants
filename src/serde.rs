use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{
    dimensions::Dimensions, fraction::Frac, prefix::Prefix, scinum::SciNum, unit128::Unit128,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DefFile {
    pub(crate) units: HashMap<String, UnitDef>,
    pub(crate) constants: HashMap<String, ConstantDef>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnitDef {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub(crate) id: Option<Unit128>,
    pub(crate) source: Option<String>,
    pub(crate) base: bool,
    pub(crate) dimensions: Option<Dimensions>,
    pub(crate) symbol: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) alt_names: Vec<String>,
    pub(crate) aliases: Vec<String>,
    pub(crate) prefix: Option<Prefix>,
    pub(crate) value: Option<QuantityDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ConstantDef {
    pub(crate) source: Option<String>,
    pub(crate) symbol: Option<String>,
    pub(crate) name: String,
    pub(crate) alt_names: Vec<String>,
    pub(crate) aliases: Vec<String>,
    pub(crate) value: QuantityDef,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QuantityDef {
    pub(crate) number: SciNum,
    pub(crate) unit: Vec<(String, Frac)>,
    pub(crate) uncertainty: SciNum,
}
