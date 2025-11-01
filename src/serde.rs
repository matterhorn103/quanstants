use serde::{Serialize, Deserialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::{dimensions::Dimensions, scinum::SciNum, unit128::Unit128};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UnitDef {
    #[serde_as(as = "Option<DisplayFromStr>")]
    id: Option<Unit128>,
    base: bool,
    dimensions: Option<Dimensions>,
    symbol: Option<String>,
    name: Option<String>,
    alt_spellings: Vec<String>,
    aliases: Vec<String>,
    value: Option<QuantityDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QuantityDef {
    number: SciNum,
    unit: Vec<Vec<String>>,
    uncertainty: SciNum,
}
