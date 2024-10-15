use core::{error, fmt::Display};

#[derive(Debug)]
pub enum Error {
    Unknown,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Unknown => write!(f, "unknown error"),
        }
    }
}
