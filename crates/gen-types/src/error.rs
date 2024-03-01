use std::{error::Error as StdError, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IriParseError(oxiri::IriParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IriParseError(_) => write!(f, "Failed parse Iri"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IriParseError(err) => Some(err),
        }
    }
}

impl From<oxiri::IriParseError> for Error {
    fn from(err: oxiri::IriParseError) -> Self {
        Self::IriParseError(err)
    }
}
