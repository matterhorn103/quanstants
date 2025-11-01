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
            units: UnitRegistry::new(), // Currently just adds unitless and SI base units
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

macro_rules! unit_getter {
    ($name:ident) => {
        pub fn $name(&self) -> Unit {
            self.units
                .get_by_name(stringify!($name))
                .expect("Should not be called if unit known to be absent")
        }
    };
}

// Convenience functions for pre-populated units
#[allow(dead_code)]
impl Context {
    pub fn unitless(&self) -> Unit {
        self.units.unitless()
    }

    unit_getter!(second);
    unit_getter!(metre);
    unit_getter!(meter);
    unit_getter!(kilogram);
    unit_getter!(ampere);
    unit_getter!(kelvin);
    unit_getter!(mole);
    unit_getter!(candela);
    unit_getter!(radian);
    unit_getter!(steradian);
    unit_getter!(hertz);
    unit_getter!(newton);
    unit_getter!(pascal);
    unit_getter!(joule);
    unit_getter!(watt);
    unit_getter!(coulomb);
    unit_getter!(volt);
    unit_getter!(farad);
    unit_getter!(ohm);
    unit_getter!(siemens);
    unit_getter!(weber);
    unit_getter!(tesla);
    unit_getter!(henry);
    unit_getter!(celsius_degree);
    unit_getter!(lumen);
    unit_getter!(lux);
    unit_getter!(becquerel);
    unit_getter!(gray);
    unit_getter!(sievert);
    unit_getter!(katal);
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

        #[getter]
        fn radian(&self) -> PyUnit {
            self.0.radian().into()
        }

        #[getter]
        fn rad(&self) -> PyUnit {
            self.0.radian().into()
        }

        #[getter]
        fn steradian(&self) -> PyUnit {
            self.0.steradian().into()
        }

        #[getter]
        fn sr(&self) -> PyUnit {
            self.0.steradian().into()
        }

        #[getter]
        fn hertz(&self) -> PyUnit {
            self.0.hertz().into()
        }

        #[getter]
        fn Hz(&self) -> PyUnit {
            self.0.hertz().into()
        }

        #[getter]
        fn newton(&self) -> PyUnit {
            self.0.newton().into()
        }

        #[getter]
        fn N(&self) -> PyUnit {
            self.0.newton().into()
        }

        #[getter]
        fn pascal(&self) -> PyUnit {
            self.0.pascal().into()
        }

        #[getter]
        fn Pa(&self) -> PyUnit {
            self.0.pascal().into()
        }

        #[getter]
        fn joule(&self) -> PyUnit {
            self.0.joule().into()
        }

        #[getter]
        fn J(&self) -> PyUnit {
            self.0.joule().into()
        }

        #[getter]
        fn watt(&self) -> PyUnit {
            self.0.watt().into()
        }

        #[getter]
        fn W(&self) -> PyUnit {
            self.0.watt().into()
        }

        #[getter]
        fn coulomb(&self) -> PyUnit {
            self.0.coulomb().into()
        }

        #[getter]
        fn C(&self) -> PyUnit {
            self.0.coulomb().into()
        }

        #[getter]
        fn volt(&self) -> PyUnit {
            self.0.volt().into()
        }

        #[getter]
        fn V(&self) -> PyUnit {
            self.0.volt().into()
        }

        #[getter]
        fn farad(&self) -> PyUnit {
            self.0.farad().into()
        }

        #[getter]
        fn F(&self) -> PyUnit {
            self.0.farad().into()
        }

        #[getter]
        fn ohm(&self) -> PyUnit {
            self.0.ohm().into()
        }

        #[getter]
        fn Ω(&self) -> PyUnit {
            self.0.ohm().into()
        }

        #[getter]
        fn siemens(&self) -> PyUnit {
            self.0.siemens().into()
        }

        #[getter]
        fn S(&self) -> PyUnit {
            self.0.siemens().into()
        }

        #[getter]
        fn weber(&self) -> PyUnit {
            self.0.weber().into()
        }

        #[getter]
        fn Wb(&self) -> PyUnit {
            self.0.weber().into()
        }

        #[getter]
        fn tesla(&self) -> PyUnit {
            self.0.tesla().into()
        }

        #[getter]
        fn T(&self) -> PyUnit {
            self.0.tesla().into()
        }

        #[getter]
        fn henry(&self) -> PyUnit {
            self.0.henry().into()
        }

        #[getter]
        fn H(&self) -> PyUnit {
            self.0.henry().into()
        }

        #[getter]
        fn celsius_degree(&self) -> PyUnit {
            self.0.celsius_degree().into()
        }

        #[getter]
        fn lumen(&self) -> PyUnit {
            self.0.lumen().into()
        }

        #[getter]
        fn lm(&self) -> PyUnit {
            self.0.lumen().into()
        }

        #[getter]
        fn lux(&self) -> PyUnit {
            self.0.lux().into()
        }

        #[getter]
        fn lx(&self) -> PyUnit {
            self.0.lux().into()
        }

        #[getter]
        fn becquerel(&self) -> PyUnit {
            self.0.becquerel().into()
        }

        #[getter]
        fn Bq(&self) -> PyUnit {
            self.0.becquerel().into()
        }

        #[getter]
        fn gray(&self) -> PyUnit {
            self.0.gray().into()
        }

        #[getter]
        fn Gy(&self) -> PyUnit {
            self.0.gray().into()
        }

        #[getter]
        fn sievert(&self) -> PyUnit {
            self.0.sievert().into()
        }

        #[getter]
        fn Sv(&self) -> PyUnit {
            self.0.sievert().into()
        }

        #[getter]
        fn katal(&self) -> PyUnit {
            self.0.katal().into()
        }

        #[getter]
        fn kat(&self) -> PyUnit {
            self.0.katal().into()
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
