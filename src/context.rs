// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use crate::{
    error::QuanstantsError, prefix::Prefix, reg::UnitRegistry, unit::Unit, unit128::Unit128,
};

#[derive(Debug, Default)]
pub struct Context {
    pub units: UnitRegistry,
}

impl Context {
    pub fn new() -> Self {
        Self {
            units: UnitRegistry::default(), // Always pre-populate with SI units
        }
    }

    pub fn new_empty() -> Self {
        Self {
            units: UnitRegistry::new(),
        }
    }

    pub fn unit_by_name(&self, name: &str) -> Option<Unit> {
        self.units.get_by_name(name)
    }

    pub fn unit_by_id(&self, id: &Unit128) -> Option<Unit> {
        self.units.get_by_id(id)
    }

    pub fn prefix_by_name(&self, name: &str) -> Result<Prefix, QuanstantsError> {
        Prefix::from_name(name)
    }
}

// Convenience functions for pre-populated units
#[allow(dead_code)]
impl Context {
    pub fn unitless(&self) -> Unit {
        self.units.unitless()
    }

    pub fn second(&self) -> Unit {
        self.units
            .get_by_name("second")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn metre(&self) -> Unit {
        self.units
            .get_by_name("metre")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn kilogram(&self) -> Unit {
        self.units
            .get_by_name("kilogram")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn ampere(&self) -> Unit {
        self.units
            .get_by_name("ampere")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn kelvin(&self) -> Unit {
        self.units
            .get_by_name("kelvin")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn mole(&self) -> Unit {
        self.units
            .get_by_name("mole")
            .expect("Should not be called if unit known to be absent")
    }

    pub fn candela(&self) -> Unit {
        self.units
            .get_by_name("candela")
            .expect("Should not be called if unit known to be absent")
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::{prefix::py::PyPrefix, unit::py::PyUnit};

    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "Context")]
    #[derive(Debug, Default)]
    pub struct PyContext(Context);

    #[allow(non_snake_case)]
    #[pymethods]
    impl PyContext {
        #[new]
        fn new() -> Self {
            PyContext::default()
        }

        #[getter]
        fn units(slf: Py<Self>) -> PyUnits {
            PyUnits { parent: slf }
        }

        fn unit_by_name(&self, name: &str) -> PyUnit {
            self.0.unit_by_name(name).unwrap().into()
        }

        fn unit_by_id(&self, id: u128) -> PyUnit {
            self.0.unit_by_id(&Unit128::from_bits(id)).unwrap().into()
        }

        // Square bracket notation lookup for units
        fn __getitem__(&self, name: &str) -> PyUnit {
            self.unit_by_name(name)
        }

        #[getter]
        fn prefixes(slf: Py<Self>) -> PyPrefixes {
            PyPrefixes { parent: slf }
        }

        fn prefix_by_name(&self, name: &str) -> PyPrefix {
            self.0.prefix_by_name(name).unwrap().into()
        }

        #[getter]
        fn unitless(&self) -> PyUnit {
            self.0.unitless().into()
        }

        #[getter]
        fn second(&self) -> PyUnit {
            self.0.second().into()
        }

        #[getter]
        fn s(&self) -> PyUnit {
            self.0.second().into()
        }

        #[getter]
        fn metre(&self) -> PyUnit {
            self.0.metre().into()
        }

        #[getter]
        fn meter(&self) -> PyUnit {
            self.0.metre().into()
        }

        #[getter]
        fn m(&self) -> PyUnit {
            self.0.metre().into()
        }

        #[getter]
        fn kilogram(&self) -> PyUnit {
            self.0.kilogram().into()
        }

        #[getter]
        fn kg(&self) -> PyUnit {
            self.0.kilogram().into()
        }

        #[getter]
        fn ampere(&self) -> PyUnit {
            self.0.ampere().into()
        }

        #[getter]
        fn amp(&self) -> PyUnit {
            self.0.ampere().into()
        }

        #[getter]
        fn A(&self) -> PyUnit {
            self.0.ampere().into()
        }

        #[getter]
        fn kelvin(&self) -> PyUnit {
            self.0.kelvin().into()
        }

        #[getter]
        fn K(&self) -> PyUnit {
            self.0.kelvin().into()
        }

        #[getter]
        fn mole(&self) -> PyUnit {
            self.0.mole().into()
        }

        #[getter]
        fn mol(&self) -> PyUnit {
            self.0.mole().into()
        }

        #[getter]
        fn candela(&self) -> PyUnit {
            self.0.candela().into()
        }

        #[getter]
        fn cd(&self) -> PyUnit {
            self.0.candela().into()
        }
    }

    #[pyclass]
    pub struct PyUnits {
        parent: Py<PyContext>,
    }

    #[pymethods]
    impl PyUnits {
        // Square bracket notation lookup for units
        fn __getitem__(&self, py: Python, name: &str) -> PyUnit {
            self.parent.borrow(py).unit_by_name(name)
        }
    }

    #[pyclass]
    pub struct PyPrefixes {
        parent: Py<PyContext>,
    }

    #[pymethods]
    impl PyPrefixes {
        // Square bracket notation lookup for prefixes
        fn __getitem__(&self, py: Python, name: &str) -> PyPrefix {
            self.parent.borrow(py).prefix_by_name(name)
        }
    }
}
