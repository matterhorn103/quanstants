use pyo3::prelude::*;

use crate::{dimensions::Dimensions, reg::UnitRegistry, unit::BaseUnit, Unit};

#[pyclass]
pub struct Context {
    unit_reg: UnitRegistry,
}

#[pymethods]
impl Context {
    #[new]
    fn new() -> Self {
        Self {
            unit_reg: UnitRegistry::new(),
        }
    }

    fn get_unit(&self, name: &str) -> BaseUnit {
        self.unit_reg.get_unit(name)
    }

    // Square bracket notation lookup for units
    fn __getitem__(&self, name: &str) -> BaseUnit {
        self.get_unit(name)
    }

    #[getter]
    fn second(&self) -> BaseUnit {
        BaseUnit::new(
            String::from("s"),
            String::from("second"),
            Dimensions::new(1, 0, 0, 0, 0, 0, 0),
        )
    }

    #[getter]
    fn s(&self) -> BaseUnit {
        self.second()
    }

    #[getter]
    fn meter(&self) -> BaseUnit {
        BaseUnit::new(
            String::from("m"),
            String::from("meter"),
            Dimensions::new(0, 1, 0, 0, 0, 0, 0),
        )
    }

    #[getter]
    fn m(&self) -> BaseUnit {
        self.meter()
    }

    #[getter]
    fn kilogram(&self) -> BaseUnit {
        BaseUnit::new(
            String::from("kg"),
            String::from("kilogram"),
            Dimensions::new(0, 0, 1, 0, 0, 0, 0),
        )
    }

    #[getter]
    fn kg(&self) -> BaseUnit {
        self.kilogram()
    }

    // For catching dot syntax lookup
    //pub fn __get_attr__(&self, name: String) {
    //    
    //}
}
