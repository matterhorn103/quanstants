// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

#![allow(mixed_script_confusables)]
#![allow(non_ascii_idents)]

pub mod context;
pub mod defs;
pub mod dimensions;
pub mod error;
pub mod fraction;
pub mod prefix;
pub mod quantity;
pub mod reg;
pub mod scinum;
pub mod unit;
pub mod unit128;
pub mod ops;

#[cfg(feature = "python")]
mod bindings {
    use pyo3::prelude::*;

    #[pymodule]
    fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<crate::fraction::py::PyFrac>()?;
        m.add_class::<crate::dimensions::py::PyDimensions>()?;
        m.add_class::<crate::unit128::py::PyUnitId>()?;
        m.add_class::<crate::unit::py::PyUnit>()?;
        m.add_class::<crate::prefix::py::PyPrefix>()?;
        m.add_class::<crate::context::py::PyContext>()?;
        Ok(())
    }
}
