use pyo3::prelude::*;

use crate::{dimensions::Dimensions, unit::BaseUnit};

#[pyclass]
pub struct Context;

#[pymethods]
impl Context {
    #[new]
    fn new() -> Self {
        Self
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

    // For square bracket syntax lookup
    //fn __getitem__(&self) -> 
}
