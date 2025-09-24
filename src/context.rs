use crate::{id::Unit128, reg::UnitRegistry, unit::Unit};

#[derive(Debug, Default)]
pub struct Context {
    unit_reg: UnitRegistry,
}

impl Context {
    pub fn new() -> Self {
        Self {
            unit_reg: UnitRegistry::new(),
        }
    }

    pub fn unit_by_name(&self, name: &str) -> Unit {
        self.unit_reg.get_by_name(name)
    }

    pub fn unit_by_id(&self, id: &Unit128) -> Unit {
        self.unit_reg.get_by_id(id)
    }
}

impl Context {
    fn second(&self) -> Unit {
        self.unit_reg.get_by_name("second")
    }

    fn s(&self) -> Unit {
        self.second()
    }

    fn metre(&self) -> Unit {
        self.unit_reg.get_by_name("metre")
    }

    fn meter(&self) -> Unit {
        self.metre()
    }

    fn m(&self) -> Unit {
        self.metre()
    }

    fn kilogram(&self) -> Unit {
        self.unit_reg.get_by_name("kilogram")
    }

    fn kg(&self) -> Unit {
        self.kilogram()
    }
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::unit::py::PyUnit;

    use super::*;
    use pyo3::prelude::*;

    #[pyclass(name = "Context")]
    #[derive(Debug, Default)]
    pub struct PyContext(Context);

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
            self.0.unit_by_name(name).into()
        }

        fn unit_by_id(&self, id: u128) -> PyUnit {
            self.0.unit_by_id(&id.into()).into()
        }

        #[getter]
        fn second(&self) -> PyUnit {
            self.0.unit_by_name("second").into()
        }

        #[getter]
        fn s(&self) -> PyUnit {
            self.second()
        }

        #[getter]
        fn metre(&self) -> PyUnit {
            self.0.unit_by_name("metre").into()
        }

        #[getter]
        fn meter(&self) -> PyUnit {
            self.metre()
        }

        #[getter]
        fn m(&self) -> PyUnit {
            self.metre()
        }

        #[getter]
        fn kilogram(&self) -> PyUnit {
            self.0.unit_by_name("kilogram").into()
        }

        #[getter]
        fn kg(&self) -> PyUnit {
            self.kilogram()
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
}
