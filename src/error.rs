// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::num::TryFromIntError;

use scinum::SciNumError;

use crate::unit::Unit;

/// Quanstants' error type.
#[derive(thiserror::Error, Clone, Debug)]
pub enum QuanstantsError {
    #[error("Failed to parse: {0}")]
    Parse(String),
    #[error("Failed to cast between types")]
    Cast,
    #[error("Input lies outside of valid range")]
    Range,
    #[error("Operation would cause type to exceed valid range")]
    Overflow,
    #[error("Definition is incomplete, missing entry {0}")]
    Definition(String),
    #[error("{0} not found")]
    Lookup(String),
    #[error("Provided UoMID does not encode a valid unit of this type")]
    InvalidId,
    #[error("Incompatible units: {0} and {1}")]
    MismatchedUnits(Unit, Unit),
    #[error("The attempted operation is only valid for linear units, and {0} is not linear")]
    NonLinearUnit(Unit),
}

impl From<TryFromIntError> for QuanstantsError {
    fn from(_err: TryFromIntError) -> Self {
        QuanstantsError::Cast
    }
}

impl From<SciNumError> for QuanstantsError {
    fn from(e: SciNumError) -> Self {
        match e {
            SciNumError::Parse(s) => QuanstantsError::Parse(s),
            SciNumError::Cast(_) => QuanstantsError::Cast,
        }
    }
}
