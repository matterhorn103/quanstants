#![allow(mixed_script_confusables)]

use pyo3::prelude::*;

pub mod dimensions;
pub mod prefix;
pub mod unit;
pub mod quantity;
pub mod reg;
pub mod context;
pub mod id;
pub mod exponent;

pub use dimensions::Dimensions;
pub use unit::Unit;
pub use prefix::Prefix;
//pub use quantity::LinearQuantity;

#[pymodule]
fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<dimensions::Dimensions>()?;
    m.add_class::<unit::BaseUnit>()?;
    m.add_class::<prefix::Prefix>()?;
    m.add_class::<context::Context>()?;
    Ok(())
}
