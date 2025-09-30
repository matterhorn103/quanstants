use std::{error::Error, fmt, num::TryFromIntError};

#[derive(Clone, Debug)]
pub enum QuanstantsError {
    Parse,
    Cast,
}

impl fmt::Display for QuanstantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuanstantsError::Parse => write!(f, "Failed to parse"),
            QuanstantsError::Cast => write!(f, "Failed to cast"),
        }
    }
}

impl Error for QuanstantsError {}

impl From<TryFromIntError> for QuanstantsError {
    fn from(_err: TryFromIntError) -> Self {
        QuanstantsError::Cast
    }
}
