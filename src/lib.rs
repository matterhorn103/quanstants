#![allow(mixed_script_confusables)]

use pyo3::prelude::*;

pub mod fraction;
pub mod dimensions;
pub mod id;
pub mod prefix;
pub mod unit;
pub mod quantity;
pub mod reg;
pub mod context;

#[pymodule]
fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<fraction::Frac>()?;
    m.add_class::<dimensions::Dimensions>()?;
    m.add_class::<id::Unit128>()?;
    m.add_class::<unit::Unit>()?;
    m.add_class::<prefix::Prefix>()?;
    m.add_class::<context::Context>()?;
    Ok(())
}
