// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT

#[cfg(feature = "python")]
pub(crate) mod py {
    use std::str::FromStr;

    use bigdecimal::BigDecimal;
    use pyo3::prelude::*;
    use scinum::{SciDecimal, SciNum};

    use crate::error::QuanstantsError;

    /// Types that can be converted into `SciDecimal`.
    #[derive(Debug, FromPyObject)]
    pub(crate) enum PyIntoSciDecimal {
        #[pyo3(transparent, annotation = "int")]
        Int(i32),
        #[pyo3(transparent, annotation = "float")]
        Float(f64),
        #[pyo3(transparent, annotation = "Decimal")]
        Decimal(BigDecimal),
        #[pyo3(transparent, annotation = "str")]
        String(String),
    }

    impl TryFrom<PyIntoSciDecimal> for SciDecimal {
        type Error = QuanstantsError;

        fn try_from(n: PyIntoSciDecimal) -> Result<SciDecimal, QuanstantsError> {
            match n {
                PyIntoSciDecimal::Int(i) => Ok(SciDecimal::new(i.into(), 0)),
                PyIntoSciDecimal::Float(f) => Ok(SciDecimal::from(f)),
                PyIntoSciDecimal::Decimal(d) => {
                    SciDecimal::try_from(d).or(Err(QuanstantsError::Cast))
                }
                PyIntoSciDecimal::String(s) => {
                    SciDecimal::from_str(&s).or(Err(QuanstantsError::Cast))
                }
            }
        }
    }

    #[pyclass(frozen, name = "SciDecimal")]
    #[derive(Clone, PartialEq, PartialOrd, Debug)]
    pub(crate) struct PySciDecimal(pub(crate) SciDecimal);

    impl From<SciDecimal> for PySciDecimal {
        #[inline]
        fn from(n: SciDecimal) -> Self {
            Self(n)
        }
    }

    #[pymethods]
    impl PySciDecimal {
        fn __str__(&self) -> String {
            format!("{}", self.0)
        }

        fn __repr__(&self) -> String {
            format!("{:?}", self.0)
        }

        /// Returns the number as an exact `SciDecimal` without its uncertainty.
        #[getter]
        fn number(&self) -> Self {
            self.0.number().into()
        }

        /// Returns the absolute uncertainty as an exact `SciDecimal`.
        /// The uncertainty is always positive.
        #[getter]
        fn uncertainty(&self) -> Self {
            self.0.uncertainty().into()
        }
    }
}
