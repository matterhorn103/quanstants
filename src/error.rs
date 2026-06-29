// SPDX-FileCopyrightText: 2025 Matthew Milner <matterhorn103@proton.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{error::Error, fmt, num::TryFromIntError};

use scinum::SciNumError;

use crate::unit::Unit;

#[derive(Clone, Debug)]
pub enum QuanstantsError {
    Parse(String),
    Cast,
    Range,
    Overflow,
    Definition(String),
    Lookup(String),
    InvalidId,
    MismatchedUnits(Unit, Unit),
    NonLinearUnit(Unit),
}

impl fmt::Display for QuanstantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuanstantsError::Parse(string) => write!(f, "Failed to parse: {string}"),
            QuanstantsError::Cast => write!(f, "Failed to cast"),
            QuanstantsError::Range => write!(f, "Input lies outside of valid range"),
            QuanstantsError::Overflow => {
                write!(f, "Operation would cause type to exceed valid range")
            }
            QuanstantsError::Definition(field) => {
                write!(f, "Definition is incomplete, missing entry {field}")
            }
            QuanstantsError::Lookup(search) => {
                write!(f, "{search} not found")
            }
            QuanstantsError::MismatchedUnits(u1, u2) => {
                write!(f, "Incompatible units: {u1} and {u2}")
            }
            QuanstantsError::NonLinearUnit(u) => write!(
                f,
                "The attempted operation is only valid for linear units, and {u} is not linear"
            ),
            QuanstantsError::InvalidId => write!(
                f,
                "Provided UoMID does not encode a valid unit of this type"
            ),
        }
    }
}

impl Error for QuanstantsError {}

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
