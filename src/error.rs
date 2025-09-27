use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub enum QuanstantsError {
    Parse,
}

impl fmt::Display for QuanstantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuanstantsError::Parse => write!(f, "Failed to parse"),
        }
    }
}

impl Error for QuanstantsError {
    fn description(&self) -> &str {
        match *self {
            QuanstantsError::Parse => "Failed to parse",
        }
    }
}
