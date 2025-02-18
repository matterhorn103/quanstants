#![allow(mixed_script_confusables)]

use pyo3::prelude::*;

pub mod dimensions;
pub mod prefix;
pub mod unit;
pub mod quantity;

pub use dimensions::Dimensions;
pub use unit::Unit;
pub use quantity::LinearQuantity;

#[pymodule]
fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Dimensions>()?;
    m.add_class::<unit::BaseUnit>()?;
    Ok(())
}
