#![allow(mixed_script_confusables)]

pub mod context;
pub mod dimensions;
pub mod fraction;
pub mod id;
pub mod numeric;
pub mod prefix;
pub mod quantity;
pub mod reg;
pub mod unit;
pub mod error;

#[cfg(feature = "python")]
mod bindings {
    use pyo3::prelude::*;

    #[pymodule]
    fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<crate::fraction::py::PyFrac>()?;
        m.add_class::<crate::dimensions::py::PyDimensions>()?;
        m.add_class::<crate::id::py::PyUnitId>()?;
        m.add_class::<crate::unit::py::PyUnit>()?;
        m.add_class::<crate::prefix::py::PyPrefix>()?;
        m.add_class::<crate::context::py::PyContext>()?;
        Ok(())
    }
}
