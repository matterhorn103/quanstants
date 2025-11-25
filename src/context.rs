// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use crate::{
    error::QuanstantsError, fraction::Frac, prefix::Prefix, quantity::Quantity, reg::UnitRegistry, scinum::SciNum, unit::Unit, unit128::Unit128
};

#[derive(Debug, Default)]
pub struct Context {
    pub units: UnitRegistry,
}

impl Context {
    /// Creates a new `Context` pre-populated with:
    /// - SI base units
    /// - SI derived units
    /// - non-SI units officially approved for use with the SI
    /// - the seven defining fundamental constants of the SI
    pub fn new() -> Self {
        Self {
            units: UnitRegistry::default(), // Always pre-populate with SI units
        }
    }

    /// Creates a new `Context` with minimal pre-population (just the SI base units).
    pub fn new_empty() -> Self {
        Self {
            units: UnitRegistry::new(), // Currently just adds unitless and SI base units
        }
    }

    /// Creates a new `Quantity` from a number and a unit.
    #[inline]
    pub fn quantity(&self, number: SciNum, unit: Unit) -> Quantity {
        Quantity { number, unit }
    }

    /// Creates a new `Quantity` from a string.
    pub fn quantity_from_str(&self, s: &str) -> Result<Quantity, QuanstantsError> {
        let s = s.to_owned();
        let mut parts = s.split_whitespace();
        if s.contains("+/-") || s.contains("±") {
            todo!("Uncertainties must be denoted using parentheses for now")
        }
        let number = SciNum::from_str(parts.next().expect("String shouldn't be empty"))?;
        let mut unit_vec = Vec::new();
        for term_string in parts {
            let mut unit_string = String::new();
            let mut exponent_string = String::new();
            for c in term_string.chars() {
                if c.is_alphabetic() {
                    unit_string.push(c);
                } else if c == '^' {
                    continue;
                } else {
                    exponent_string.push(c);
                }
            }
            let unit = self.units.get_by_symbol(&unit_string).or(self.units.get_by_name(&unit_string)).ok_or(QuanstantsError::Lookup(unit_string))?;
            let term = if exponent_string.is_empty() {
                unit
            } else {
                unit.pow(Frac::from_str(&exponent_string)?)
            };
            unit_vec.push(term);
        }
        let product_unit = unit_vec.into_iter().reduce(|acc, e| acc * e).unwrap_or(self.unitless());
        Ok(Quantity { number, unit: product_unit })
    }
}

/// Macro to generate convenience functions for units
macro_rules! unit_getter {
    ($name:ident, $id:expr) => {
        #[inline]
        pub fn $name(&self) -> Unit {
            self.units
                .get_by_id($id)
                .expect("Should not be called if unit known to be absent")
        }
    };
}

// Generate convenience functions for the pre-populated units
#[allow(dead_code)]
impl Context {
    #[inline]
    pub fn unitless(&self) -> Unit {
        self.units.unitless()
    }

    // Make sure to get the meter with the US spelling
    #[inline]
    pub fn meter(&self) -> Unit {
        self.units
            .get_by_name("meter")
            .expect("Context is always pre-populated with the SI base units, including the metre/meter")
    }

    // Make sure to get the liter with the US spelling
    #[inline]
    pub fn liter(&self) -> Unit {
        self.units
            .get_by_name("liter")
            .expect("Context is pre-populated with common SI-compatible units, including the litre/liter")
    }

    unit_getter!(second, Unit128::SECOND);
    unit_getter!(metre, Unit128::METRE);
    unit_getter!(kilogram, Unit128::KILOGRAM);
    unit_getter!(ampere, Unit128::AMPERE);
    unit_getter!(kelvin, Unit128::KELVIN);
    unit_getter!(mole, Unit128::MOLE);
    unit_getter!(candela, Unit128::CANDELA);
    unit_getter!(radian, Unit128::RADIAN);
    unit_getter!(steradian, Unit128::STERADIAN);
    unit_getter!(hertz, Unit128::HERTZ);
    unit_getter!(newton, Unit128::NEWTON);
    unit_getter!(pascal, Unit128::PASCAL);
    unit_getter!(joule, Unit128::JOULE);
    unit_getter!(watt, Unit128::WATT);
    unit_getter!(coulomb, Unit128::COULOMB);
    unit_getter!(volt, Unit128::VOLT);
    unit_getter!(farad, Unit128::FARAD);
    unit_getter!(ohm, Unit128::OHM);
    unit_getter!(siemens, Unit128::SIEMENS);
    unit_getter!(weber, Unit128::WEBER);
    unit_getter!(tesla, Unit128::TESLA);
    unit_getter!(henry, Unit128::HENRY);
    //unit_getter!(celsius_degree, Unit128::CELSIUS_DEGREE);
    unit_getter!(lumen, Unit128::LUMEN);
    unit_getter!(lux, Unit128::LUX);
    unit_getter!(becquerel, Unit128::BECQUEREL);
    unit_getter!(gray, Unit128::GRAY);
    unit_getter!(sievert, Unit128::SIEVERT);
    unit_getter!(katal, Unit128::KATAL);
    unit_getter!(gram, Unit128{ num: 0xFD, dim: 0x0000000011000001 });
    unit_getter!(litre, Unit128{ num: 0xFD, dim: 0x0000000000130001 });
}

/// Macro to generate convenience functions for prefixes
macro_rules! prefix_getter {
    ($name:ident) => {
        #[inline]
        pub fn $name(&self) -> Prefix {
            Prefix::$name
        }
    };
}

// Generate convenience functions for the most common prefixes
#[allow(dead_code)]
impl Context {
    prefix_getter!(nano);
    prefix_getter!(micro);
    prefix_getter!(milli);
    prefix_getter!(kilo);
    prefix_getter!(mega);
    prefix_getter!(giga);
}

#[cfg(feature = "python")]
pub(crate) mod py {
    use crate::{prefix::py::PyPrefix, quantity::py::PyQuantity, reg::py::PyUnits, scinum::py::PyIntoSciNum, unit::py::PyUnit};

    use super::*;
    use pyo3::{prelude::*, types::PyType};

    #[pyclass(name = "Context")]
    #[derive(Debug, Default)]
    pub(crate) struct PyContext(pub(crate) Context);

    #[allow(non_snake_case)]
    #[pymethods]
    impl PyContext {
        #[new]
        fn new() -> Self {
            PyContext::default()
        }

        /// Looks up a unit using square bracket notation.
        fn __getitem__(&self, name: &str) -> PyUnit {
            self.0.units.get_by_name(name).unwrap().into()
        }

        /// Creates a new `Quantity` from a string.
        fn __call__(&self, s: &str) -> PyQuantity {
            self.0.quantity_from_str(s).unwrap().into()
        }

        /// Creates a new `Quantity` from a number and a unit.
        #[pyo3(signature = (number, unit, uncertainty=None))]
        fn quantity(&self, number: PyIntoSciNum, unit: PyUnit, uncertainty: Option<PyIntoSciNum>) -> PyQuantity {
            let num: SciNum = if let Some(u) = uncertainty {
                let num: SciNum = number.try_into().unwrap();
                num.with_uncertainty(u.try_into().unwrap())
            } else {
                number.try_into().unwrap()
            };
            Quantity { number: num, unit: unit.into_inner() }.into()
        }

        /// Provides access to the context's unit registry.
        #[getter]
        fn units(slf: Py<Self>) -> PyUnits {
            PyUnits { parent: slf }
        }

        /// Makes the `Prefix` enum conveniently accessible as a class attribute.
        #[classattr]
        fn Prefix() -> PyResult<Py<PyType>> {
            Python::attach(|py| {
                Ok(py.get_type::<PyPrefix>().unbind())
            })
        }

        // Unit getters

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
            self.0.meter().into()
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

        //#[getter]
        //fn celsius_degree(&self) -> PyUnit {
        //    self.0.celsius_degree().into()
        //}

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

        #[getter]
        fn gram(&self) -> PyUnit {
            self.0.gram().into()
        }

        #[getter]
        fn g(&self) -> PyUnit {
            self.0.gram().into()
        }

        #[getter]
        fn litre(&self) -> PyUnit {
            self.0.litre().into()
        }

        #[getter]
        fn liter(&self) -> PyUnit {
            self.0.liter().into()
        }

        #[getter]
        fn L(&self) -> PyUnit {
            self.0.litre().into()
        }

        // Prefix getters

        #[getter]
        fn nano(&self) -> PyPrefix {
            self.0.nano().into()
        }

        #[getter]
        fn micro(&self) -> PyPrefix {
            self.0.micro().into()
        }

        #[getter]
        fn milli(&self) -> PyPrefix {
            self.0.milli().into()
        }

        #[getter]
        fn kilo(&self) -> PyPrefix {
            self.0.kilo().into()
        }

        #[getter]
        fn mega(&self) -> PyPrefix {
            self.0.mega().into()
        }

        #[getter]
        fn giga(&self) -> PyPrefix {
            self.0.giga().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_getters() {
        // Just make sure that a default context has all the SI units in it and their getters work
        let context = Context::new();
        context.second();
        context.metre();
        context.meter();
        context.kilogram();
        context.ampere();
        context.kelvin();
        context.mole();
        context.candela();
        context.radian();
        context.steradian();
        context.hertz();
        context.newton();
        context.pascal();
        context.joule();
        context.watt();
        context.coulomb();
        context.volt();
        context.farad();
        context.ohm();
        context.siemens();
        context.weber();
        context.tesla();
        context.henry();
        //context.celsius_degree();
        context.lumen();
        context.lux();
        context.becquerel();
        context.gray();
        context.sievert();
        context.katal();
    }

    #[test]
    fn quantity_from_str() {
        let context = Context::new();
        assert_eq!(context.quantity_from_str("3 m").unwrap(), SciNum::new_exact(3) * context.metre());
        assert_eq!(context.quantity_from_str("3 metre").unwrap(), SciNum::new_exact(3) * context.metre());
        assert_eq!(context.quantity_from_str("3 m2").unwrap(), SciNum::new_exact(3) * context.metre().pow(2));
    }
}
