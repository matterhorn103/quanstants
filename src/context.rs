use pyo3::{pyclass, pymethods, Py, Python};

use crate::{dimensions::Dimensions, reg::UnitRegistry, unit::Unit};

#[pyclass]
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
}

#[pymethods]
impl Context {
    #[new]
    fn py_new() -> Self {
        Context::default()
    }

    #[getter]
    fn units(slf: Py<Self>) -> PyUnits {
        PyUnits { context: slf }
    }

    #[getter]
    fn second(&self) -> Unit {
        self.unit_reg.get_by_name("second")
    }

    #[getter]
    fn s(&self) -> Unit {
        self.second()
    }

    #[getter]
    fn metre(&self) -> Unit {
        self.unit_reg.get_by_name("metre")
    }

    #[getter]
    fn meter(&self) -> Unit {
        self.metre()
    }

    #[getter]
    fn m(&self) -> Unit {
        self.metre()
    }

    #[getter]
    fn kilogram(&self) -> Unit {
        self.unit_reg.get_by_name("kilogram")
    }

    #[getter]
    fn kg(&self) -> Unit {
        self.kilogram()
    }
}

#[pyclass]
pub struct PyUnits {
    context: Py<Context>,
}

#[pymethods]
impl PyUnits {
    // Square bracket notation lookup for units
    fn __getitem__(&self, py: Python, name: &str) -> Unit {
        self.context.borrow(py).unit_by_name(name)
    }
}
