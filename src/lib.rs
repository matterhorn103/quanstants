// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(mixed_script_confusables)]
#![allow(non_ascii_idents)]

pub mod context;
pub mod defs;
pub mod dimensions;
pub mod error;
pub mod fraction;
mod num;
pub mod ops;
pub mod prefix;
pub mod quantity;
pub mod reg;
pub mod unit;
pub mod unit128;

pub use scinum::{SciDecimal, SciFloat, SciNum};

#[cfg(feature = "python")]
mod bindings {
    use pyo3::prelude::*;

    #[pymodule]
    fn _quanstants(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<crate::fraction::py::PyFrac>()?;
        m.add_class::<crate::dimensions::py::PyDimensions>()?;
        m.add_class::<crate::num::py::PySciDecimal>()?;
        m.add_class::<crate::unit128::py::PyUnitId>()?;
        m.add_class::<crate::unit::py::PyUnit>()?;
        m.add_class::<crate::prefix::py::PyPrefix>()?;
        m.add_class::<crate::context::py::PyContext>()?;
        m.add_class::<crate::defs::units::py::PyUnitModule>()?;
        Ok(())
    }
}
